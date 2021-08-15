

pub struct PackedArray {
    cells: Vec<u64>,
    length: usize,
    byte_size: u8,
}

impl PackedArray {

    pub fn new(length: usize, byte_size: u8, default: Option<u64>) -> Self {

        Self::check_byte_size(byte_size);

        let default_value = match default {
            Some(default) if default != 0 => {
                assert!(default <= Self::calc_mask(byte_size), "Given default value does not fit in {} bits.", byte_size);
                let vpc = Self::values_per_cell(byte_size);
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
            cells: vec![default_value; Self::needed_cells(length, byte_size)],
            length,
            byte_size,
        }

    }

    pub fn from_raw(cells: Vec<u64>, length: usize, byte_size: u8) -> Option<Self> {
        Self::check_byte_size(byte_size);
        if cells.len() >= Self::needed_cells(length, byte_size) {
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

    pub fn set(&mut self, index: usize, value: u64) {
        if index < self.length {
            let mask = Self::calc_mask(self.byte_size);
            if value <= mask {
                let (cell_index, bit_index) = self.get_indices(index);
                let cell = &mut self.cells[cell_index];
                *cell = (*cell & !(mask << bit_index)) | (value << bit_index); // Clear and Set
            } else {
                panic!("Given value {} does not fit in {} bits.", value, self.byte_size);
            }
        } else {
            panic!("Index out of bounds, {} with length {}.", index, self.length);
        }
    }

    fn get_indices(&self, index: usize) -> (usize, usize) {
        let vpc = Self::values_per_cell(self.byte_size);
        let cell_index = index / vpc;
        let bit_index = (index % vpc) * self.byte_size as usize;
        (cell_index, bit_index)
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        let byte_size = self.byte_size as usize;
        let vpc = Self::values_per_cell(self.byte_size);
        let mask = Self::calc_mask(self.byte_size);
        self.cells
            .iter()
            .flat_map(move |&cell| {
                (0..vpc).map(move |idx| (cell >> (idx * byte_size)) & mask)
            })
            .take(self.length)
    }

    pub fn replace(&mut self, replacer: impl Fn(u64) -> u64) {
        let byte_size = self.byte_size as usize;
        let vpc = Self::values_per_cell(self.byte_size);
        let mask = Self::calc_mask(self.byte_size);
        for mut_cell in &mut self.cells {
            let mut cell = *mut_cell;
            for value_index in 0..vpc {
                let bit_index = value_index * byte_size;
                let new_value = replacer((cell >> bit_index) & mask);
                cell = (cell & !(mask << bit_index)) | (new_value << bit_index);
            }
            *mut_cell = cell;
        }
    }

    pub fn resize_byte(&mut self, new_byte_size: u8) {
        self.internal_resize_byte::<fn(u64) -> u64>(new_byte_size, None);
    }

    pub fn resize_byte_and_replace(&mut self, new_byte_size: u8, replacer: impl Fn(u64) -> u64) {
        self.internal_resize_byte(new_byte_size, Some(replacer));
    }

    fn internal_resize_byte<F>(&mut self, new_byte_size: u8, replacer: Option<F>)
    where
        F: Fn(u64) -> u64
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

        let new_cells_cap = Self::needed_cells(self.length, new_byte_size);

        let old_vpc = Self::values_per_cell(old_byte_size);
        let new_vpc = Self::values_per_cell(new_byte_size);

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

                    if let Some(ref replacer) = replacer {
                        value = replacer(value);
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

    // Utils //

    #[inline]
    pub fn values_per_cell(byte_size: u8) -> usize {
        64 / byte_size as usize
    }

    #[inline]
    pub fn needed_cells(length: usize, byte_size: u8) -> usize {
        let vpc = Self::values_per_cell(byte_size);
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

    #[inline]
    pub fn calc_min_byte_size(value: u64) -> u8 {
        (u64::BITS - value.leading_zeros()) as u8
    }

    #[inline]
    fn check_byte_size(byte_size: u8) {
        assert!(byte_size <= 64, "Byte size is greater than 64.");
    }

}


pub struct Palette<T>(Vec<T>);

impl<T> Palette<T>
where
    T: Clone + Copy + Eq
{

    pub fn new_default(capacity: usize) -> Self
    where
        T: Default
    {
        Self::new(None, capacity)
    }

    pub fn new(default: Option<T>, capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Given capacity is zero.");
        let mut items = Vec::with_capacity(capacity);
        if let Some(default) = default {
            items.resize(1, default);
        }
        Self(items)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn ensure_index(&mut self, target_item: T) -> Option<usize> {
        match self.search_index(target_item) {
            Some(index) => Some(index),
            None => self.insert_index(target_item)
        }
    }

    pub fn insert_index(&mut self, target_item: T) -> Option<usize> {
        let length = self.0.len();
        if length < self.0.capacity() {
            self.0.push(target_item);
            Some(length)
        } else {
            None
        }
    }

    pub fn search_index(&self, target_item: T) -> Option<usize> {
        return self.0.iter()
            .position(|item| *item == target_item);
    }

    pub fn get_item(&self, index: usize) -> Option<T> {
        self.0.get(index).copied()
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn packed_valid_byte_size() {
        PackedArray::new(1, 64, None);
    }

    #[test]
    #[should_panic]
    fn packed_invalid_byte_size() {
        PackedArray::new(1, 65, None);
    }

    #[test]
    fn packed_min_byte_size() {
        assert_eq!(PackedArray::calc_min_byte_size(63), 6);
        assert_eq!(PackedArray::calc_min_byte_size(64), 7);
        assert_eq!(PackedArray::calc_min_byte_size(127), 7);
    }

    #[test]
    #[should_panic]
    fn packed_invalid_default_value() {
        PackedArray::new(1, 4, Some(16));
    }

    #[test]
    #[should_panic]
    fn packed_invalid_value() {
        let mut array = PackedArray::new(1, 4, None);
        array.set(0, 16);
    }

    #[test]
    #[should_panic]
    fn packed_oor_get() {
        PackedArray::new(10, 4, None).get(10).unwrap();
    }

    #[test]
    #[should_panic]
    fn packed_oor_set() {
        PackedArray::new(10, 4, None).set(10, 15);
    }

    #[test]
    #[should_panic]
    fn packed_invalid_resize() {
        PackedArray::new(10, 4, None).resize_byte(4);
    }

    #[test]
    fn packed() {

        const LEN: usize = 32;

        let mut array = PackedArray::new(LEN, 4, Some(15));

        assert_eq!(array.cells.len(), 2);
        assert_eq!(array.cells[0], u64::MAX);
        assert_eq!(array.cells[1], u64::MAX);

        for i in 0..LEN {
            assert_eq!(array.get(i).unwrap(), 15);
        }

        array.set(2, 3);

        for i in 0..LEN {
            assert_eq!(array.get(i).unwrap(), if i == 2 { 3 } else { 15 });
        }

        array.replace(|val| val / 2);

        for i in 0..LEN {
            assert_eq!(array.get(i).unwrap(), if i == 2 { 1 } else { 7 });
        }

        array.resize_byte(5);
        assert_eq!(array.cells.len(), 3);
        assert_eq!(array.byte_size, 5);

        for i in 0..LEN {
            assert_eq!(array.get(i).unwrap(), if i == 2 { 1 } else { 7 });
        }

        array.resize_byte_and_replace(16, |val| val * 5678);
        assert_eq!(array.cells.len(), 8);
        assert_eq!(array.byte_size, 16);

        for i in 0..LEN {
            assert_eq!(array.get(i).unwrap(), if i == 2 { 5678 } else { 7 * 5678 });
        }

    }

    #[test]
    #[should_panic]
    fn palette_invalid_capacity() {
        Palette::new(Some("default"), 0);
    }

    #[test]
    fn palette() {

        let mut palette = Palette::new(Some("default"), 5);
        assert_eq!(palette.get_item(0).unwrap(), "default");
        assert_eq!(palette.search_index("default").unwrap(), 0);
        assert_eq!(palette.ensure_index("default").unwrap(), 0);

        assert_eq!(palette.ensure_index("str1").unwrap(), 1);
        assert_eq!(palette.ensure_index("str2").unwrap(), 2);
        assert_eq!(palette.ensure_index("str3").unwrap(), 3);
        assert_eq!(palette.ensure_index("str4").unwrap(), 4);
        assert!(palette.ensure_index("str5").is_none());

    }

}