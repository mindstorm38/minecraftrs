
/// A palette is a vector with limited capacity that allows searching indices of elements.
pub struct Palette<T> {
    items: Vec<T>,
    capacity: usize
}

impl<T> Palette<T>
where
    T: Clone + Copy + Eq
{

    /// Create a new empty palette with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Given capacity is zero.");
        Self {
            items: Vec::with_capacity(capacity),
            capacity
        }
    }

    /// Create a new palette of the specified capacity and fill it with the given iterator.
    /// At most `capacity` elements are took from the iterator.
    pub fn with_defaults<I>(defaults: I, capacity: usize) -> Self
    where
        I: IntoIterator<Item = T>
    {
        assert_ne!(capacity, 0, "Given capacity is zero.");
        let mut items = Vec::with_capacity(capacity);
        items.extend(defaults.into_iter().take(capacity));
        Self {
            items,
            capacity
        }
    }

    /// Create a new palette with a default element into it and with the specified capacity.
    pub fn with_default(default: T, capacity: usize) -> Self {
        Self::with_defaults(Some(default), capacity)
    }

    pub fn from_raw(items: Vec<T>, capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Given capacity is zero.");
        Self {
            items,
            capacity
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn ensure_index(&mut self, target_item: T) -> Option<usize> {
        match self.search_index(target_item) {
            Some(index) => Some(index),
            None => self.insert_index(target_item)
        }
    }

    pub fn insert_index(&mut self, target_item: T) -> Option<usize> {
        let length = self.items.len();
        if length < self.capacity {
            self.items.push(target_item);
            Some(length)
        } else {
            None
        }
    }

    pub fn search_index(&self, target_item: T) -> Option<usize> {
        return self.items.iter()
            .position(|item| *item == target_item);
    }

    pub fn get_item(&self, index: usize) -> Option<T> {
        self.items.get(index).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.items.iter().copied()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn invalid_capacity() {
        Palette::with_default("default", 0);
    }

    #[test]
    fn valid_usage() {

        let mut palette = Palette::with_default("default", 5);
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