//! Utility module for game tags, used for blocks, items and biomes.

use crate::util::OpaquePtr;


/// A type that can be used as a unique key to a statically defined tag type.
pub type TagTypeKey = OpaquePtr<TagType>;

/// A type structure describing a type of tag, intended for static definition as many functions
/// will requires a static reference to it.
pub struct TagType {
    /// Name of the tag.
    pub name: &'static str,
    /// Logical tags are not saved.
    pub logical: bool
}

impl TagType {

    pub const fn new(name: &'static str) -> Self {
        Self { name, logical: false }
    }

    pub const fn new_logical(name: &'static str) -> Self {
        Self { name, logical: true }
    }

    #[inline]
    pub fn get_key(&'static self) -> TagTypeKey {
        OpaquePtr::new(self)
    }

}

impl PartialEq for &'static TagType {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for &'static TagType {}


/*pub struct TagMap<T> {
    mapping: HashMap<T, usize>,
    stores: HashMap<TagTypeKey, BitVec>,
}

impl<T> TagMap<T>
where
    T: Hash + Eq
{

    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            stores: HashMap::new()
        }
    }

    pub fn register_tag_type(&mut self, tag_type: &'static TagType) {
        match self.stores.entry(tag_type.get_key()) {
            Entry::Vacant(v) => {
                v.insert(BitVec::from_elem(self.mapping.len(), false));
            },
            _ => {}
        }
    }

    pub fn has_tag(&self, item: &T, tag: &'static TagType) -> bool {
        match self.stores.get(&tag.get_key()) {
            Some(store) => match self.mapping.get(item) {
                Some(&idx) => store.get(idx).unwrap(),
                None => false
            },
            None => false
        }
    }

    pub fn set_tag(&mut self, item: T, tag: &'static TagType, enabled: bool) {

        let len = self.mapping.len();
        let idx = match self.mapping.entry(item) {
            Entry::Occupied(o) => *o.into_mut(),
            Entry::Vacant(v) => {
                v.insert(len);
                for vec in self.stores.values_mut() {
                    vec.push(false);
                }
                len
            }
        };

        self.stores.get(&tag.get_key())
            .expect("Unsupported tag type.")
            .set(idx, enabled);

    }

}*/
