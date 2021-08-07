

pub struct PackedArray {
    byte_size: u8,
    length: usize,
    cells: Vec<u64>,
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
            length,
            byte_size,
            cells: vec![default_value; Self::needed_cells(length, byte_size)]
        }

    }

    pub fn get(&self, index: usize) -> Option<u64> {
        if index < self.length {
            let (cell_index, bit_index) = self.get_indexes(index);
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
                let (cell_index, bit_index) = self.get_indexes(index);
                let cell = &mut self.cells[cell_index];
                *cell = (*cell & !(mask << bit_index)) | (value << bit_index); // Clear and Set
            } else {
                panic!("Given value is out of range, {} is greater than maximum {}.", value, mask);
            }
        } else {
            panic!("Index out of bounds, {} with length {}.", index, self.length);
        }
    }

    fn get_indexes(&self, index: usize) -> (usize, usize) {
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

                let old_bit_index = value_index * old_byte_size;
                let mut value = (old_cell >> old_bit_index) & old_mask;
                let index = old_cell_index * old_vpc + value_index;

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
    fn values_per_cell(byte_size: u8) -> usize {
        64 / byte_size as usize
    }

    #[inline]
    fn needed_cells(length: usize, byte_size: u8) -> usize {
        let vpc = Self::values_per_cell(byte_size);
        (length + vpc - 1) / vpc
    }

    #[inline]
    fn calc_mask(byte_size: u8) -> u64 {
        if byte_size == 64 {
            u64::MAX
        } else {
            (1u64 << byte_size) - 1
        }
    }

    #[inline]
    fn check_byte_size(byte_size: u8) {
        assert!(byte_size <= 64, "Byte size is greater than 64.");
    }

    #[inline]
    pub fn calc_min_byte_size(value: u64) -> u8 {
        (u64::BITS - value.leading_zeros()) as u8
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
        Self::new(T::default(), 0, capacity)
    }

    pub fn new(default: T, length: usize, capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Given capacity is zero.");
        assert!(length <= capacity, "Given length is larger than the given capacity.");
        let mut items = Vec::with_capacity(capacity);
        items.resize(length, default);
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

    /*pub fn remove_index(&mut self, target_item: T) -> Option<usize> {
        if let Some(idx) = self.search_index(target_item) {
            for i in idx..(self.length - 1) {
                self.items[i] = self.items[i + 1];
            }
            self.length -= 1;
            Some(idx)
        } else {
            None
        }
    }*/

    pub fn search_index(&self, target_item: T) -> Option<usize> {
        return self.0.iter()
            .position(|item| *item == target_item);
    }

    pub fn get_item(&self, index: usize) -> Option<T> {
        self.0.get(index).copied()
    }

}


/*pub trait GlobalPalette<T> {
    fn len(&self) -> usize;
    fn has_item(&self, item: &'static T) -> bool;
    fn get_item_from_sid(&self, sid: u32) -> &'static T;
    fn get_sid_from_item(&self, item: &'static T) -> u32;
}


pub struct PaletteArray<'g, T, G: GlobalPalette<T>> {
    data: PackedArray,
    palette: Option<Palette<*const T>>,
    global_palette: &'g G
}

impl<'g, T, G: GlobalPalette<T>> PaletteArray<'g, T, G> {

    pub fn new(global_palette: &'g G, length: usize, byte_size: u8, palette_capacity: usize) -> Self {
        Self {
            data: PackedArray::new(length, byte_size),
            palette: Some(Palette::new(global_palette.get_item_from_sid(0) as *const T, 1, palette_capacity)),
            global_palette
        }
    }

    pub fn get(&self, index: usize) -> &'static T {
        let sid = self.data.get(index).unwrap() as u32;
        match self.palette {
            Some(ref palette) => unsafe {
                // SAFETY: Transmuting `*const T` to `&'static T`: safe.
                std::mem::transmute(palette.get_item(sid as usize).unwrap())
            },
            None => self.global_palette.get_item_from_sid(sid)
        }
    }

    pub fn set(&mut self, index: usize, value: &'static T) -> bool {
        match self.ensure_sid(value) {
            Some(sid) => {
                self.data.set(index, sid as u64);
                true
            },
            None => false
        }
    }

    fn ensure_sid(&mut self, value: &'static T) -> Option<u32> {

        if let Some(ref mut palette) = self.palette {
            let ptr = value as *const T;
            match palette.search_index(ptr) {
                Some(sid) => return Some(sid as u32),
                None => {
                    if self.global_palette.has_item(value) {
                        match palette.insert_index(ptr) {
                            Some(sid) => {
                                if sid as u64 > self.data.max_value() {
                                    self.data.resize_byte(self.data.byte_size + 1, None);
                                }
                                return Some(sid as u32);
                            },
                            None => {
                                // In this case, the local palette is full, we have to switch to
                                // the global one. So we don't return anything to skip the match.
                            }
                        }
                    } else {
                        return None;
                    }
                }
            }
        }

        self.global_palette.get_sid_from_item()

    }

    fn use_global(&mut self) {
        if let Some(ref local_palette) = self.palette {
            let global_palette = self.env.blocks();
            let new_byte_size = PackedArray::calc_min_byte_size(global_palette.states_count() as u64);
            self.blocks.resize_byte(new_byte_size, Some(move |sid| unsafe {
                global_palette.get_sid_from(std::mem::transmute(
                    local_palette.get_item(sid as usize).unwrap()
                )).unwrap() as u64
            }));
            self.blocks_palette = None;
        }
    }

}*/
