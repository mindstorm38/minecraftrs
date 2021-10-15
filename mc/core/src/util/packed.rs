
/// A packed array is an array of cell (`u64`) with a fixed length and a varying bits
/// length (byte size) that allows it to store multiple values in the same cell.
///
/// Its content can be modified and queried by using methods of this method, the byte
/// size can also be changed and methods allows to replace its content in-place.
pub struct PackedArray {
    cells: Vec<u64>,
    length: usize,
    byte_size: u8,
}


impl PackedArray {

    /// Create a new packed array with specific length and byte size (the bit size of each value,
    /// smallest addressable unit), the default value is 0 if `None` is given.
    pub fn new(length: usize, byte_size: u8, default: Option<u64>) -> Self {

        Self::check_byte_size(byte_size);

        let default_value = match default {
            Some(default) if default != 0 => {
                assert!(default <= Self::calc_mask(byte_size), "Given default value does not fit in {} bits.", byte_size);
                let vpc = Self::calc_values_per_cell(byte_size);
                let mut value = default;
                for _ in 1..vpc {
                    value <<= byte_size;
                    value |= default;
                }
                value
            },
            _ => 0
        };

        Self {
            cells: vec![default_value; Self::calc_cells_capacity(length, byte_size)],
            length,
            byte_size,
        }

    }

    pub fn from_raw(length: usize, byte_size: u8, cells: Vec<u64>) -> Option<Self> {
        Self::check_byte_size(byte_size);
        if cells.len() >= Self::calc_cells_capacity(length, byte_size) {
            Some(Self {
                cells,
                length,
                byte_size
            })
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<u64> {
        if index < self.length {
            let (cell_index, bit_index) = self.get_indices(index);
            let cell = self.cells[cell_index];
            Some((cell >> bit_index) & Self::calc_mask(self.byte_size))
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: u64) -> u64 {
        if index < self.length {
            let mask = Self::calc_mask(self.byte_size);
            if value <= mask {
                self.internal_set(index, value)
            } else {
                panic!("Given value {} does not fit in {} bits.", value, self.byte_size);
            }
        } else {
            panic!("Index out of bounds, {} with length {}.", index, self.length);
        }
    }

    pub fn set_auto_resize(&mut self, index: usize, value: u64) -> u64 {
        // If the byte size if already 64, we know that the given value must fit.
        if self.byte_size != 64 && value >= (1u64 << self.byte_size) {
            self.resize_byte(Self::calc_min_byte_size(value));
        }
        self.internal_set(index, value)
    }

    #[inline]
    fn internal_set(&mut self, index: usize, value: u64) -> u64 {
        let (cell_index, bit_index) = self.get_indices(index);
        let cell = &mut self.cells[cell_index];
        let old_value = (*cell >> bit_index) & mask;
        *cell = (*cell & !(mask << bit_index)) | (value << bit_index); // Clear and Set
        old_value
    }

    fn get_indices(&self, index: usize) -> (usize, usize) {
        let vpc = Self::calc_values_per_cell(self.byte_size);
        let cell_index = index / vpc;
        let bit_index = (index % vpc) * self.byte_size as usize;
        (cell_index, bit_index)
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        self.cells.iter().copied().unpack_aligned(self.byte_size).take(self.length)
    }

    pub fn replace(&mut self, mut replacer: impl FnMut(usize, u64) -> u64) {
        let byte_size = self.byte_size as usize;
        let vpc = Self::calc_values_per_cell(self.byte_size);
        let mask = Self::calc_mask(self.byte_size);
        let mut index = 0;
        for mut_cell in &mut self.cells {
            let mut old_cell = *mut_cell;
            let mut new_cell = 0;
            for value_index in 0..vpc {
                let bit_index = value_index * byte_size;
                new_cell |= (replacer(index, old_cell & mask) & mask) << bit_index;
                old_cell >>= self.byte_size;
                index += 1;
                if index >= self.length {
                    break;
                }
            }
            *mut_cell = new_cell;
        }
    }

    pub fn resize_byte(&mut self, new_byte_size: u8) {
        self.internal_resize_byte::<fn(usize, u64) -> u64>(new_byte_size, None);
    }

    pub fn resize_byte_and_replace<F>(&mut self, new_byte_size: u8, replacer: F)
    where
        F: FnMut(usize, u64) -> u64
    {
        self.internal_resize_byte(new_byte_size, Some(replacer));
    }

    fn internal_resize_byte<F>(&mut self, new_byte_size: u8, mut replacer: Option<F>)
    where
        F: FnMut(usize, u64) -> u64
    {

        if new_byte_size == self.byte_size {
            if let Some(replacer) = replacer {
                self.replace(replacer);
                return;
            }
        }

        Self::check_byte_size(new_byte_size);
        assert!(new_byte_size > self.byte_size, "New byte size should be greater than previous.");

        let old_byte_size = self.byte_size;
        self.byte_size = new_byte_size;

        let old_mask = Self::calc_mask(old_byte_size);
        let new_mask = Self::calc_mask(new_byte_size);

        let new_cells_cap = Self::calc_cells_capacity(self.length, new_byte_size);

        let old_vpc = Self::calc_values_per_cell(old_byte_size);
        let new_vpc = Self::calc_values_per_cell(new_byte_size);

        let old_byte_size = old_byte_size as usize;
        let new_byte_size = new_byte_size as usize;

        let old_cells_cap = self.cells.len();
        self.cells.resize(new_cells_cap, 0u64);

        for old_cell_index in (0..old_cells_cap).rev() {
            let old_cell = self.cells[old_cell_index];
            for value_index in 0..old_vpc {
                let index = old_cell_index * old_vpc + value_index;
                if index < self.length {

                    let old_bit_index = value_index * old_byte_size;
                    let mut value = (old_cell >> old_bit_index) & old_mask;

                    let new_cell_index = index / new_vpc;
                    let new_bit_index = (index % new_vpc) * new_byte_size;

                    if let Some(ref mut replacer) = replacer {
                        value = replacer(index, value);
                    }

                    let cell = &mut self.cells[new_cell_index];
                    *cell = (*cell & !(new_mask << new_bit_index)) | (value << new_bit_index);

                }
            }
        }

    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[inline]
    pub fn byte_size(&self) -> u8 {
        self.byte_size
    }

    pub fn max_value(&self) -> u64 {
        Self::calc_mask(self.byte_size)
    }

    #[inline]
    pub fn cells_len(&self) -> usize {
        self.cells.len()
    }

    // Unsafe and cell-related methods //

    pub fn get_cell(&self, index: usize) -> u64 {
        self.cells[index]
    }

    pub unsafe fn set_cell(&mut self, index: usize, cell: u64) {
        self.cells[index] = cell;
    }

    pub unsafe fn resize_raw(&mut self, new_byte_size: u8) {
        Self::check_byte_size(new_byte_size);
        self.byte_size = new_byte_size;
        let new_cells_cap = Self::calc_cells_capacity(self.length, new_byte_size);
        self.cells.resize(new_cells_cap, 0u64);
    }

    pub unsafe fn clear_cells(&mut self) {
        self.cells.iter_mut().for_each(|c| *c = 0);
    }

    #[inline]
    pub fn into_inner(self) -> Vec<u64> {
        self.cells
    }

    // Utils //

    #[inline]
    pub fn calc_values_per_cell(byte_size: u8) -> usize {
        64 / byte_size as usize
    }

    #[inline]
    pub fn calc_cells_capacity(length: usize, byte_size: u8) -> usize {
        let vpc = Self::calc_values_per_cell(byte_size);
        (length + vpc - 1) / vpc
    }

    #[inline]
    pub fn calc_mask(byte_size: u8) -> u64 {
        if byte_size == 64 {
            u64::MAX
        } else {
            (1u64 << byte_size) - 1
        }
    }

    /// Calculate the minimum size in bits to store de given value, the special case is for
    /// value `0` which returns a bits size of 1, this special value is simpler to handle
    /// by the structure's methods.
    #[inline]
    pub fn calc_min_byte_size(value: u64) -> u8 {
        if value == 0 {
            1
        } else {
            (u64::BITS - value.leading_zeros()) as u8
        }
    }

    #[inline]
    fn check_byte_size(byte_size: u8) {
        assert!(byte_size <= 64, "Byte size is greater than 64.");
    }

}


// Custom iterators //

/// An iterator extension trait for unpacking aligned values from an u64 iterator.
pub trait PackedIterator: Iterator<Item = u64> {

    #[inline]
    fn unpack_aligned(self, byte_size: u8) -> UnpackAlignedIter<Self>
    where
        Self: Sized
    {
        UnpackAlignedIter::new(self, byte_size)
    }

    #[inline]
    fn pack_aligned(self, byte_size: u8) -> PackAlignedIter<Self>
    where
        Self: Sized
    {
        PackAlignedIter::new(self, byte_size)
    }

}

impl<I> PackedIterator for I
where
    I: Iterator<Item = u64>
{
    // Defaults are not redefined //
}


/// An iterator to unpack aligned values in from an u64 iterator.
/// This iterator only requires an byte size and will output every
/// value found in each cell. What means "aligned" is that value
/// cannot be defined on two different cells, if there is remaining
/// space in cells, it is ignored.
///
/// The first value in a cell is composed of the first least significant
/// 'byte size' bits. For exemple with two cells and byte size of 13
/// (underscore are unused padding bits):
///
/// ```text
/// cell #0: ____________3333333333333222222222222211111111111110000000000000
/// cell #1: ____________7777777777777666666666666655555555555554444444444444
/// ```
///
/// As you can see, in this configured, only 8 values can be stored.
pub struct UnpackAlignedIter<I> {
    inner: I,
    byte_size: u8,
    mask: u64,
    vpc: usize,
    index: usize,
    value: u64
}

impl<I> UnpackAlignedIter<I>
where
    I: Iterator<Item = u64>
{

    pub fn new(inner: I, byte_size: u8) -> Self {
        Self {
            inner,
            byte_size,
            mask: PackedArray::calc_mask(byte_size),
            vpc: PackedArray::calc_values_per_cell(byte_size),
            index: 0,
            value: 0
        }
    }

}

impl<I> Iterator for UnpackAlignedIter<I>
where
    I: Iterator<Item = u64>
{

    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

        if self.index % self.vpc == 0 {
            self.value = self.inner.next()?;
        }

        let ret = self.value & self.mask;
        self.value >>= self.byte_size;
        self.index += 1;
        Some(ret)

    }

    /*fn nth(&mut self, n: usize) -> Option<Self::Item> {
        TODO: We should redefine this method in order to speedrun skips.
    }*/

}


/// Reciprocal iterator to `UnpackAlignedIter`.
pub struct PackAlignedIter<I> {
    inner: I,
    byte_size: u8,
    mask: u64,
}

impl<I> PackAlignedIter<I>
where
    I: Iterator<Item = u64>
{

    pub fn new(inner: I, byte_size: u8) -> Self {
        Self {
            inner,
            byte_size,
            mask: PackedArray::calc_mask(byte_size)
        }
    }

}

impl<I> Iterator for PackAlignedIter<I>
where
    I: Iterator<Item = u64>
{

    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

        let mut value = match self.inner.next() {
            None => return None,
            Some(val) => val & self.mask
        };

        let mut shift = self.byte_size;
        let shift_limit = 64 - self.byte_size;

        while shift <= shift_limit {
            match self.inner.next() {
                None => break,
                Some(cell) => {
                    value |= (cell & self.mask) << shift;
                    shift += self.byte_size;
                }
            }
        }

        Some(value)

    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_byte_size() {
        PackedArray::new(1, 64, None);
    }

    #[test]
    #[should_panic]
    fn invalid_byte_size() {
        PackedArray::new(1, 65, None);
    }

    #[test]
    fn min_byte_size() {
        assert_eq!(PackedArray::calc_min_byte_size(0), 1);
        assert_eq!(PackedArray::calc_min_byte_size(1), 1);
        assert_eq!(PackedArray::calc_min_byte_size(2), 2);
        assert_eq!(PackedArray::calc_min_byte_size(7), 3);
        assert_eq!(PackedArray::calc_min_byte_size(63), 6);
        assert_eq!(PackedArray::calc_min_byte_size(64), 7);
        assert_eq!(PackedArray::calc_min_byte_size(127), 7);
    }

    #[test]
    #[should_panic]
    fn invalid_default_value() {
        PackedArray::new(1, 4, Some(16));
    }

    #[test]
    #[should_panic]
    fn invalid_value() {
        let mut array = PackedArray::new(1, 4, None);
        array.set(0, 16);
    }

    #[test]
    #[should_panic]
    fn out_of_range_get() {
        PackedArray::new(10, 4, None).get(10).unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_range_set() {
        PackedArray::new(10, 4, None).set(10, 15);
    }

    #[test]
    #[should_panic]
    fn invalid_resize() {
        PackedArray::new(10, 4, None).resize_byte(4);
    }

    #[test]
    fn valid_usage() {

        const LEN: usize = 32;

        let mut array = PackedArray::new(LEN, 4, Some(15));

        assert_eq!(array.cells.len(), 2);
        assert_eq!(array.cells[0], u64::MAX);
        assert_eq!(array.cells[1], u64::MAX);

        for value in array.iter() {
            assert_eq!(value, 15, "construction failed");
        }

        array.set(2, 3);

        for (i, value) in array.iter().enumerate() {
            assert_eq!(value, if i == 2 { 3 } else { 15 }, "set failed");
        }

        array.replace(|_, val| val / 2);

        for (i, value) in array.iter().enumerate() {
            assert_eq!(value, if i == 2 { 1 } else { 7 }, "replace failed");
        }

        array.resize_byte(5);
        assert_eq!(array.cells.len(), 3, "resize failed to change cells len");
        assert_eq!(array.byte_size, 5, "resize failed to set the new byte size");

        for (i, value) in array.iter().enumerate() {
            assert_eq!(value, if i == 2 { 1 } else { 7 }, "resize failed to transform values");
        }

        array.resize_byte_and_replace(16, |_, val| val * 5678);
        assert_eq!(array.cells.len(), 8, "resize and replace failed to change cells len");
        assert_eq!(array.byte_size, 16, "resize and replace failed to set the new byte size");

        for (i, value) in array.iter().enumerate() {
            assert_eq!(value, if i == 2 { 5678 } else { 7 * 5678 }, "resize and replace failed to transform values");
        }

    }

    #[test]
    fn iter_unpack_aligned() {

        let raw = [u64::MAX, u64::MAX, u64::MAX];

        assert!(raw.iter().copied().unpack_aligned(4).all(|v| v == 15), "byte size = 4, wrong values");
        assert_eq!(raw.iter().copied().unpack_aligned(4).count(), 48, "byte size = 4, invalid values count");

        assert!(raw.iter().copied().unpack_aligned(16).all(|v| v == 65_535), "byte size = 16, wrong values");
        assert_eq!(raw.iter().copied().unpack_aligned(16).count(), 12, "byte size = 16, invalid values count");

        assert!(raw.iter().copied().unpack_aligned(33).all(|v| v == 0x0001_FFFF_FFFF), "byte size = 33, wrong values");
        assert_eq!(raw.iter().copied().unpack_aligned(33).count(), 3, "byte size = 33, invalid values count");

    }

    #[test]
    fn iter_pack_aligned() {

        let raw = [0, u32::MAX as u64, u64::MAX];

        let out: Vec<u64> = raw.iter()
            .copied()
            .unpack_aligned(4)
            .pack_aligned(4)
            .collect();

        assert_eq!(&raw[..], &out[..]);

    }

}