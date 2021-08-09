use std::io::{Error as IoError, Result as IoResult, Seek, SeekFrom, Read, Write, Cursor, ErrorKind};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use bit_vec::BitVec;


const SECTOR_SIZE: u64 = 4096;
const MAX_SECTOR_OFFSET: u64 = 0xFFFFFF;
const MAX_SECTOR_LENGTH: u64 = 0xFF;
const MAX_CHUNK_SIZE: u64 = MAX_SECTOR_LENGTH * SECTOR_SIZE;

// /// Internal empty sector array used to fill the region file with an entire section.
// static EMPTY_SECTOR: [u8; SECTOR_SIZE as usize] = [0; SECTOR_SIZE as usize];

/// Error type used together with `RegionResult` for every call on region file methods.
#[derive(Debug)]
pub enum RegionError {
    /// The region file is shorter than 8192 bytes.
    FileTooSmall,
    /// The region file size is not a multiple of 4096 (4096 = 1 sector).
    FileNotPadded,
    /// The region file has an invalid chunk metadata that leads to sectors out of the range.
    IllegalMetadata,
    /// The compression method in the chunk header is unknown, the ID is given as parameter.
    UnknownCompression(u8),
    /// The external chunk file was not found. This is used if the chunk is too large.
    ExternalChunkNotFound,
    /// For common IO errors that can happen and can't be reduced to another `RegionError`.
    Io(IoError)
}

/// A result type with an error of type `RegionError`, it is used in region file methods.
pub type RegionResult<T> = Result<T, RegionError>;


#[inline]
fn calc_chunk_metadata_index(cx: i32, cz: i32) -> usize {
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

/// Internal function used to pass to `.map_err` of `IoResult`.
fn map_io_err(e: IoError) -> RegionError {
    RegionError::Io(e)
}


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

    pub fn new(dir: PathBuf, rx: i32, rz: i32) -> RegionResult<Self> {

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(get_region_file_path(&dir, rx, rz))
            .map_err(map_io_err)?;

        let file_len = file.seek(SeekFrom::End(0)).map_err(map_io_err)?;

        // The following conditions are used to fix the file
        if file_len < 8192 {
            return Err(RegionError::FileTooSmall);
        } else if (file_len & 0xFFF) != 0 {
            return Err(RegionError::FileNotPadded);
        }

        file.seek(SeekFrom::Start(0)).map_err(map_io_err)?;

        // The sectors_count take the two headers sectors into account.
        let sectors_count = file_len / SECTOR_SIZE;
        let mut sectors = BitVec::from_elem(sectors_count as usize - 2, true);

        let mut metadata = [ChunkMetadata { location: 0, timestamp: 0 }; 1024];

        // Reading the first sector containing location information of each chunk.
        for meta in &mut metadata {

            let mut data = [0u8; 4];
            file.read_exact(&mut data).map_err(map_io_err)?;
            meta.location = u32::from_be_bytes(data);

            let offset = meta.offset();
            let length = meta.length();

            if length != 0 && (offset + length) <= sectors_count {
                for sector in (offset - 2)..(offset + length - 2) {
                    sectors.set(sector as usize, false);
                }
            } else {
                return Err(RegionError::IllegalMetadata);
            }

        }

        // Reading the second sector containing last modification times for each chunk.
        for meta in &mut metadata {
            let mut data = [0u8; 4];
            file.read_exact(&mut data).map_err(map_io_err)?;
            meta.timestamp = u32::from_be_bytes(data);
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
        self.metadata[calc_chunk_metadata_index(cx, cz)].length() != 0
    }

    // Reading //

    pub fn get_chunk_reader(&mut self, cx: i32, cz: i32) -> RegionResult<Box<dyn Read>> {

        let metadata = self.metadata[calc_chunk_metadata_index(cx, cz)];
        self.file.seek(SeekFrom::Start(metadata.offset() * SECTOR_SIZE)).map_err(map_io_err)?;

        let mut length_data = [0u8; 4];
        self.file.read_exact(&mut length_data).map_err(map_io_err)?;
        let data_length = u32::from_be_bytes(length_data) - 1;

        let mut compression_id = [0u8; 1];
        self.file.read_exact(&mut compression_id).map_err(map_io_err)?;
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
            external_file.read_to_end(&mut data).map_err(map_io_err)?;
            data

        } else {
            let mut data = vec![0u8; data_length as usize];
            self.file.read_exact(&mut data[..]).map_err(map_io_err)?;
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

    pub fn get_latest_chunk_writer(&mut self, cx: i32, cz: i32) -> LatestChunkWriter {
        LatestChunkWriter {
            cx,
            cz,
            region: self,
            inner: ZlibEncoder::new(Vec::new(), Compression::best())
        }
    }

    fn write_chunk(&mut self, cx: i32, cz: i32, data: &[u8], method: CompressionMethod) -> RegionResult<()> {

        let metadata_index = calc_chunk_metadata_index(cx, cz);
        let mut metadata = self.metadata[metadata_index];
        let mut offset = metadata.offset();
        let mut length = metadata.length();

        // Here, adding 1 to count the compression method byte ID.
        let needed_byte_length = data.len() as u64 + 1;
        let mut external = needed_byte_length > MAX_CHUNK_SIZE;

        let needed_length = if external {
            1 // If external, only one sector is needed to store chunk header.
        } else {
            // Adding 4 to the byte length to count the 32 bits length of (data.len() + 1).
            (needed_byte_length + 4 - 1) / SECTOR_SIZE + 1
        };

        if needed_length != length {

            for sector in (offset - 2)..(offset + length - 2) {
                self.sectors.set(sector as usize, true);
            }

            offset = 0;
            length = 0;

            let mut first_free_sector: Option<usize> = None;

            for (sector, free) in self.sectors.iter().enumerate() {
                if free {
                    if let None = first_free_sector {
                        first_free_sector = Some(sector);
                    }
                    length += 1;
                    if length == needed_length {
                        break;
                    }
                } else {
                    length = 0;
                    offset = sector as u64 + 1;
                }
            }

            if offset > MAX_SECTOR_OFFSET {

                // Here we switch to external chunk storage, then we can only use 1 sector and
                // store the chunk header. This is why we keep track of the first free sector.
                if let Some(free_sector) = first_free_sector {
                    external = true;
                    offset = free_sector as u64;
                    length = 1;
                } else {
                    // TODO: Return err, no sector available for external chunk header.
                    //       This is really unlikely to happen but we need to take this case into
                    //       account.
                }

            }

            if length != needed_length {

                // This should be always true according to the sectors finding algorithm that break
                //  when reaching the needed length.
                // However, the offset should be valid here and point to the first free sector even
                //  if the length is not enough.
                debug_assert!(length < needed_length);
                let missing_length = needed_length - length;

                self.file.set_len((missing_length + self.sectors.len() as u64 + 2) * SECTOR_SIZE)
                    .map_err(map_io_err)?;

                self.sectors.extend((0..missing_length).map(|_| true));

            }

            // Mark all new sectors to "not free".
            for sector in (offset - 2)..(offset + length - 2) {
                self.sectors.set(sector as usize, false);
            }

            // Update metadata for new offset and length.
            metadata.set_location(offset, length);
            self.write_metadata(metadata_index, metadata).map_err(map_io_err)?;

        }

        // Actually write the data
        if external {

            self.write_chunk_at(offset, 1, &[], method, true)
                .map_err(map_io_err)?;

            File::create(get_chunk_file_path(&self.dir, cx, cz))
                .map_err(map_io_err)?
                .write_all(data)
                .map_err(map_io_err)

        } else {

            self.write_chunk_at(offset, needed_byte_length as u32, data, method, false)
                .map_err(map_io_err)

        }

    }

    fn write_chunk_at(&mut self, sector_offset: u64, length: u32, data: &[u8], method: CompressionMethod, external: bool) -> IoResult<()> {
        self.file.seek(SeekFrom::Start(sector_offset * SECTOR_SIZE))?;
        self.file.write_all(&u32::to_be_bytes(length))?;
        self.file.write_all(&[method.get_id(external)])?;
        self.file.write_all(data)?;
        Ok(())
    }

    fn write_metadata(&mut self, index: usize, metadata: ChunkMetadata) -> IoResult<()> {
        self.file.seek(SeekFrom::Start(index as u64 * SECTOR_SIZE))?;
        self.file.write_all(&u32::to_be_bytes(metadata.location))?;
        self.file.seek(SeekFrom::Start(SECTOR_SIZE + index as u64 * SECTOR_SIZE))?;
        self.file.write_all(&u32::to_be_bytes(metadata.timestamp))?;
        self.metadata[index] = metadata;
        Ok(())
    }

}


#[derive(Copy, Clone, Debug)]
pub struct ChunkMetadata {
    location: u32,
    timestamp: u32
}

impl ChunkMetadata {

    pub fn offset(&self) -> u64 {
        ((self.location >> 8) & 0xFFFFFF) as u64
    }

    pub fn length(&self) -> u64 {
        (self.location & 0xFF) as u64
    }

    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn set_location(&mut self, offset: u64, length: u64) {
        self.location = (((offset & 0xFFFFFF) as u32) << 8) | ((length & 0xFF) as u32);
    }

    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
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


/// A chunk writer for the latest compression method.
pub struct LatestChunkWriter<'region> {
    cx: i32,
    cz: i32,
    region: &'region mut RegionFile,
    inner: ZlibEncoder<Vec<u8>>
}

impl LatestChunkWriter<'_> {

    pub fn write_chunk(&mut self) -> RegionResult<()> {
        self.region.write_chunk(self.cx, self.cz, &self.inner.get_ref()[..], CompressionMethod::Zlib)
    }

}

impl Write for LatestChunkWriter<'_> {

    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
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

    fn inner_write(&mut self) -> &mut dyn Write {
        match &mut self.inner {
            ChunkWriterInner::Gzip(encoder) => encoder,
            ChunkWriterInner::Zlib(encoder) => encoder,
            ChunkWriterInner::None(vec) => vec
        }
    }

    pub fn write_chunk(&mut self) -> RegionResult<()> {

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

}
