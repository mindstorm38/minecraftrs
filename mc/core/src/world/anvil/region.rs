use std::io::{Error as IoError, Result as IoResult, Seek, SeekFrom, Read, Write, Cursor, ErrorKind};
use std::fs::{File, OpenOptions};
use std::time::SystemTime;
use std::fmt::Arguments;
use std::path::PathBuf;

use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use thiserror::Error;
use bit_vec::BitVec;


const SECTOR_SIZE: u64 = 4096;
const MAX_SECTOR_OFFSET: u64 = 0xFFFFFF;
const MAX_SECTOR_LENGTH: u64 = 0xFF;
const MAX_CHUNK_SIZE: u64 = MAX_SECTOR_LENGTH * SECTOR_SIZE;


/// Error type used together with `RegionResult` for every call on region file methods.
#[derive(Error, Debug)]
pub enum RegionError {
    #[error("The region file was not found in the level directory at {0}.")]
    FileNotFound(PathBuf),
    #[error("The region file size ({0}) is shorter than 8192 bytes.")]
    FileTooSmall(u64),
    #[error("The region file size ({0}) is not a multiple of 4096 (4096 = 1 sector).")]
    FileNotPadded(u64),
    #[error("The region file has an invalid chunk (#{0}) metadata that leads to sectors out of the range.")]
    IllegalMetadata(u16),
    #[error("The required chunk is empty, it has no sector allocated in the region file.")]
    EmptyChunk,
    #[error("The compression method {0} in the chunk header is unknown.")]
    UnknownCompression(u8),
    #[error("The external chunk file was not found. This is used if the chunk is too large.")]
    ExternalChunkNotFound,
    #[error("No more sectors are available in the region file, really unlikely to happen.")]
    OutOfSectors,
    #[error("{0}")]
    Io(#[from] IoError)
}

/// A result type with an error of type `RegionError`, it is used in region file methods.
pub type RegionResult<T> = Result<T, RegionError>;


/// This structure holds a region file and all its metadata, it is used
pub struct RegionFile {
    /// The base directory path that contains all regions and chunks files.
    dir: PathBuf,
    /// The file object of the region.
    file: File,
    /// Chunk metadata for each chunk in the 32x32 region.
    metadata: [ChunkMetadata; 1024],
    /// A vector of bits for each section, this does not include the 2x headers sectors.
    /// True if the section is free.
    sectors: BitVec
}

impl RegionFile {

    /// Build a new region file, this method will open the region file from the given directory
    /// and the region coordinates. The directory should be the one of region, not the level dir.
    /// This method can return the following errors:
    /// - `RegionError::FileTooSmall` The given file is too small, smaller than 2 sectors.
    /// - `RegionError::FileNotPadded` The file size is not a multiple of sector size, 4096 bytes.
    /// - `RegionError::IllegalMetadata` Failed to parse one of the chunk's metadata.
    /// - `RegionError::Io` An IO error happened.
    pub fn new(dir: PathBuf, rx: i32, rz: i32, create: bool) -> RegionResult<Self> {

        if create {
            std::fs::create_dir_all(&dir)?;
        }

        let file_path = get_region_file_path(&dir, rx, rz);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(create)
            .open(&file_path)
            .map_err(|err| match err.kind() {
                ErrorKind::NotFound => RegionError::FileNotFound(file_path),
                _ => RegionError::Io(err)
            })?;

        let file_len = file.seek(SeekFrom::End(0))?;

        let mut metadata = [ChunkMetadata { location: 0, timestamp: 0 }; 1024];
        let mut sectors;

        if file_len == 0 && create {
            file.write_all(&[0; 8192])?;
            sectors = BitVec::new();
        } else {

            // The following conditions are used to fix the file
            if file_len < 8192 {
                return Err(RegionError::FileTooSmall(file_len));
            } else if (file_len & 0xFFF) != 0 {
                return Err(RegionError::FileNotPadded(file_len));
            }

            file.seek(SeekFrom::Start(0))?;

            // The sectors_count take the two headers sectors into account.
            let sectors_count = file_len / SECTOR_SIZE;
            sectors = BitVec::from_elem(sectors_count as usize - 2, true);

            // let mut metadata = [ChunkMetadata { location: 0, timestamp: 0 }; 1024];

            // Reading the first sector containing location information of each chunk.
            for (idx, meta) in metadata.iter_mut().enumerate() {
                let mut data = [0u8; 4];
                file.read_exact(&mut data)?;
                meta.location = u32::from_be_bytes(data);

                let offset = meta.offset();
                let length = meta.length();

                if length != 0 {
                    if (offset + length) <= sectors_count {
                        fill_sectors(&mut sectors, offset as usize - 2, length as usize, false);
                    } else {
                        return Err(RegionError::IllegalMetadata(idx as u16));
                    }
                }
            }

            // Reading the second sector containing last modification times for each chunk.
            for meta in &mut metadata {
                let mut data = [0u8; 4];
                file.read_exact(&mut data)?;
                meta.timestamp = u32::from_be_bytes(data);
            }

        }

        Ok(Self {
            dir,
            file,
            metadata,
            sectors
        })

    }

    // Metadata //

    pub fn has_chunk(&self, cx: i32, cz: i32) -> bool {
        self.metadata[calc_chunk_index_from_pos(cx, cz)].length() != 0
    }

    // Reading //

    /// Get a reader for a specific chunk, the position is not checked and you must ensure that
    /// the chunk belong to this region. Some errors can be returned by this method:
    /// - `RegionError::EmptyChunk` The chunk is not yet saved in the region.
    /// - `RegionError::UnknownCompression` The compression method can't be decoded.
    /// - `RegionError::ExternalChunkNotFound` The chunk should be saved externally but its file
    /// is not found.
    /// - `RegionError::Io` An IO error happened.
    pub fn get_chunk_reader(&mut self, cx: i32, cz: i32) -> RegionResult<Box<dyn Read>> {

        let metadata = self.metadata[calc_chunk_index_from_pos(cx, cz)];
        if metadata.length() == 0 {
            return Err(RegionError::EmptyChunk);
        }

        self.file.seek(SeekFrom::Start(metadata.offset() * SECTOR_SIZE))?;

        let mut length_data = [0u8; 4];
        self.file.read_exact(&mut length_data)?;
        let data_length = u32::from_be_bytes(length_data) - 1;

        let mut compression_id = [0u8; 1];
        self.file.read_exact(&mut compression_id)?;
        let compression_id = compression_id[0];

        let compression = CompressionMethod::from_id(compression_id)
            .ok_or_else(|| RegionError::UnknownCompression(compression_id))?;

        let (compression_method, external) = compression;

        let data = if external {

            let mut external_file = match File::open(get_chunk_file_path(&self.dir, cx, cz)) {
                Ok(file) => file,
                Err(e) => return match e.kind() {
                    ErrorKind::NotFound => Err(RegionError::ExternalChunkNotFound),
                    _ => Err(RegionError::Io(e))
                }
            };

            let mut data = Vec::new();
            external_file.read_to_end(&mut data)?;
            data

        } else {
            let mut data = vec![0u8; data_length as usize];
            self.file.read_exact(&mut data[..])?;
            data
        };

        let cursor = Cursor::new(data);

        Ok(match compression_method {
            CompressionMethod::Gzip => Box::new(GzDecoder::new(cursor)),
            CompressionMethod::Zlib => Box::new(ZlibDecoder::new(cursor)),
            CompressionMethod::None => Box::new(cursor)
        })

    }

    // Writing //

    pub fn get_chunk_writer(&mut self, cx: i32, cz: i32, method: CompressionMethod) -> ChunkWriter {

        let vec: Vec<u8> = Vec::new();
        let inner = match method {
            CompressionMethod::Gzip => ChunkWriterInner::Gzip(GzEncoder::new(vec, Compression::best())),
            CompressionMethod::Zlib => ChunkWriterInner::Zlib(ZlibEncoder::new(vec, Compression::best())),
            CompressionMethod::None => ChunkWriterInner::None(vec)
        };

        ChunkWriter {
            cx,
            cz,
            region: self,
            inner
        }

    }

    fn write_chunk(&mut self, cx: i32, cz: i32, data: &[u8], method: CompressionMethod) -> RegionResult<()> {

        let metadata_index = calc_chunk_index_from_pos(cx, cz);
        let mut metadata = self.metadata[metadata_index];
        let mut offset = metadata.offset();
        let mut length = metadata.length();

        // Here, adding 1 to count the compression method byte ID.
        let needed_byte_length = data.len() as u64 + 1;
        let mut external = needed_byte_length > MAX_CHUNK_SIZE;

        let mut needed_length = if external {
            1 // If external, only one sector is needed to store chunk header.
        } else {
            // Adding 4 to the byte length to count the 32 bits length of (data.len() + 1).
            (needed_byte_length + 4 - 1) / SECTOR_SIZE + 1
        };

        if needed_length != length {

            if length != 0 {
                fill_sectors(&mut self.sectors, offset as usize - 2, length as usize, true);
            }

            offset = 2;
            length = 0;

            let mut first_free_sector: Option<usize> = None;

            for (sector, free) in self.sectors.iter().enumerate() {
                if free {
                    if first_free_sector.is_none() {
                        first_free_sector.insert(sector + 2);
                    }
                    length += 1;
                    if length == needed_length {
                        break;
                    }
                } else {
                    length = 0;
                    offset = sector as u64 + 2 + 1;
                }
            }

            if offset > MAX_SECTOR_OFFSET {

                // Here we switch to external chunk storage, then we can only use 1 sector and
                // store the chunk header. This is why we keep track of the first free sector
                // even if no free sector were found.
                if let Some(free_sector) = first_free_sector {
                    external = true;
                    offset = free_sector as u64;
                    needed_length = 1; // Needed length is now 1 because we have switched to external.
                    length = 1;
                } else {
                    // Revert the change to sectors "free state".
                    fill_sectors(&mut self.sectors, metadata.offset() as usize - 2, length as usize, false);
                    return Err(RegionError::OutOfSectors);
                }

            } else if length < needed_length {
                let missing_length = needed_length - length;
                self.file.set_len((missing_length + self.sectors.len() as u64 + 2) * SECTOR_SIZE)?;
                self.sectors.extend((0..missing_length).map(|_| true));
                length = needed_length;
            }

            // Mark all new sectors to "not free".
            fill_sectors(&mut self.sectors, offset as usize - 2, length as usize, false);

            // Update metadata for new offset and length.
            metadata.set_location(offset, length);

        }

        // Update metadata for new timestamp.
        metadata.set_timestamp_now();
        self.write_metadata(metadata_index, metadata)?;

        // Actually write the data
        if external {
            self.write_chunk_at(offset, 1, &[], method, true)?;
            File::create(get_chunk_file_path(&self.dir, cx, cz))?.write_all(data)?;
        } else {
            self.write_chunk_at(offset, needed_byte_length as u32, data, method, false)?;
        }

        Ok(())

    }

    fn write_chunk_at(&mut self, sector_offset: u64, length: u32, data: &[u8], method: CompressionMethod, external: bool) -> IoResult<()> {
        self.file.seek(SeekFrom::Start(sector_offset * SECTOR_SIZE))?;
        self.file.write_all(&u32::to_be_bytes(length))?;
        self.file.write_all(&[method.get_id(external)])?;
        self.file.write_all(data)?;
        self.file.flush()?;
        Ok(())
    }

    fn write_metadata(&mut self, index: usize, metadata: ChunkMetadata) -> IoResult<()> {
        self.file.seek(SeekFrom::Start(index as u64 * 4))?;
        self.file.write_all(&u32::to_be_bytes(metadata.location))?;
        self.file.seek(SeekFrom::Start(SECTOR_SIZE + index as u64 * 4))?;
        self.file.write_all(&u32::to_be_bytes(metadata.timestamp))?;
        self.metadata[index] = metadata;
        Ok(())
    }

}


#[derive(Copy, Clone, Debug)]
struct ChunkMetadata {
    location: u32,
    timestamp: u32
}

impl ChunkMetadata {

    fn offset(&self) -> u64 {
        ((self.location >> 8) & 0xFFFFFF) as u64
    }

    fn length(&self) -> u64 {
        (self.location & 0xFF) as u64
    }

    fn set_location(&mut self, offset: u64, length: u64) {
        self.location = (((offset & 0xFFFFFF) as u32) << 8) | ((length & 0xFF) as u32);
    }

    #[inline]
    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    #[inline]
    fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    fn set_timestamp_now(&mut self) {
        self.set_timestamp(SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|dur| dur.as_secs() as u32)
            .unwrap_or(0));
    }

}


#[derive(Copy, Clone, Debug)]
pub enum CompressionMethod {
    Gzip,
    Zlib,
    None
}

impl CompressionMethod {

    pub fn get_id(self, external: bool) -> u8 {
        (match self {
            CompressionMethod::Gzip => 1,
            CompressionMethod::Zlib => 2,
            CompressionMethod::None => 3
        }) + if external { 128 } else { 0 }
    }

    pub fn from_id(mut id: u8) -> Option<(Self, bool)> {

        let external = id > 128;
        if external {
            id -= 128;
        }

        Some((
            match id {
                1 => CompressionMethod::Gzip,
                2 => CompressionMethod::Zlib,
                3 => CompressionMethod::None,
                _ => return None
            },
            external
        ))

    }

}

impl Default for CompressionMethod {
    fn default() -> Self {
        Self::Zlib
    }
}


/// A generic chunk writer for every compression method,
/// might be slower than `LatestChunkWriter`.
pub struct ChunkWriter<'region> {
    cx: i32,
    cz: i32,
    region: &'region mut RegionFile,
    inner: ChunkWriterInner
}

pub enum ChunkWriterInner {
    Gzip(GzEncoder<Vec<u8>>),
    Zlib(ZlibEncoder<Vec<u8>>),
    None(Vec<u8>)
}

impl ChunkWriter<'_> {

    #[inline]
    fn inner_write(&mut self) -> &mut dyn Write {
        match &mut self.inner {
            ChunkWriterInner::Gzip(encoder) => encoder,
            ChunkWriterInner::Zlib(encoder) => encoder,
            ChunkWriterInner::None(vec) => vec
        }
    }

    /// Finalize and write internal buffered data to the region.
    pub fn write_chunk(&mut self) -> RegionResult<()> {

        self.inner_write().flush()?;

        let (data, method) = match &self.inner {
            ChunkWriterInner::Gzip(encoder) => (encoder.get_ref(), CompressionMethod::Gzip),
            ChunkWriterInner::Zlib(encoder) => (encoder.get_ref(), CompressionMethod::Zlib),
            ChunkWriterInner::None(vec) => (vec, CompressionMethod::None)
        };

        self.region.write_chunk(self.cx, self.cz, &data[..], method)

    }

}

impl Write for ChunkWriter<'_> {

    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.inner_write().write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner_write().flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.inner_write().write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> IoResult<()> {
        self.inner_write().write_fmt(fmt)
    }

}


#[inline]
fn calc_chunk_index_from_pos(cx: i32, cz: i32) -> usize {
    (cx & 31) as usize | (((cz & 31) as usize) << 5)
}

#[inline]
fn get_region_file_path(dir: &PathBuf, rx: i32, rz: i32) -> PathBuf {
    dir.join(format!("r.{}.{}.mca", rx, rz))
}

#[inline]
fn get_chunk_file_path(dir: &PathBuf, cx: i32, cz: i32) -> PathBuf {
    dir.join(format!("c.{}.{}.mcc", cx, cz))
}

#[inline]
fn fill_sectors(sectors: &mut BitVec, from: usize, length: usize, value: bool) {
    for sector in from..(from + length) {
        sectors.set(sector, value);
    }
}

#[inline]
pub fn calc_region_pos(cx: i32, cz: i32) -> (i32, i32) {
    (cx >> 5, cz >> 5)
}
