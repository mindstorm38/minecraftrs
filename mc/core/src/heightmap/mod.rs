use crate::block::BlockState;
use crate::util::OpaquePtr;

use std::collections::HashMap;


/// A structure used to statically define an heightmap type.
pub struct HeightmapType {
    pub name: &'static str,
    pub predicate: fn(&'static BlockState) -> bool
}

impl HeightmapType {

    #[inline]
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn get_key(&'static self) -> HeightmapTypeKey {
        OpaquePtr::new(self)
    }

    pub fn check_block(&self, state: &'static BlockState) -> bool {
        (self.predicate)(state)
    }

}

pub type HeightmapTypeKey = OpaquePtr<HeightmapType>;

impl PartialEq for &'static HeightmapType {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl Eq for &'static HeightmapType { }


/// A global heightmaps palette used by level, each chunk created into it will include all
/// these heightmaps and automatically computed and updated when modifying the chunk.
pub struct GlobalHeightmaps {
    heightmap_types: Vec<&'static HeightmapType>,
    heightmap_types_to_index: HashMap<HeightmapTypeKey, usize>,
}

impl GlobalHeightmaps {

    pub fn new() -> Self {
        Self {
            heightmap_types: Vec::new(),
            heightmap_types_to_index: HashMap::new()
        }
    }

    pub fn with_all(slice: &[&'static HeightmapType]) -> Self {
        let mut heightmaps = Self::new();
        heightmaps.register_all(slice);
        heightmaps
    }

    pub fn register(&mut self, heightmap_type: &'static HeightmapType) {
        self.heightmap_types_to_index.insert(heightmap_type.get_key(), self.heightmap_types.len());
        self.heightmap_types.push(heightmap_type);
    }

    pub fn register_all(&mut self, slice: &[&'static HeightmapType]) {
        self.heightmap_types.reserve(slice.len());
        self.heightmap_types_to_index.reserve(slice.len());
        for &heightmap_type in slice {
            self.register(heightmap_type);
        }
    }

    pub fn get_heightmap_index(&self, heightmap_type: &'static HeightmapType) -> Option<usize> {
        self.heightmap_types_to_index.get(&heightmap_type.get_key()).copied()
    }

    pub fn get_heightmap_from(&self, index: usize) -> Option<&'static HeightmapType> {
        self.heightmap_types.get(index).copied()
    }

    pub fn has_heightmap_type(&self, heightmap_type: &'static HeightmapType) -> bool {
        self.heightmap_types.iter().any(move |&t| t == heightmap_type)
    }

    pub fn iter_heightmap_types(&self) -> impl Iterator<Item = &'static HeightmapType> + '_ {
        self.heightmap_types.iter().copied()
    }

    pub fn heightmaps_count(&self) -> usize {
        self.heightmap_types.len()
    }

}


#[macro_export]
macro_rules! heightmaps {
    ($global_vis:vis $static_id:ident [
        $($heightmap_id:ident $heightmap_name:literal $heightmap_predicate:ident),*
        $(,)?
    ]) => {

        $($global_vis static $heightmap_id: $crate::heightmap::HeightmapType = $crate::heightmap::HeightmapType {
            name: $heightmap_name,
            predicate: $heightmap_predicate
        };)*

        $global_vis static $static_id: [&'static $crate::heightmap::HeightmapType; $crate::count!($($heightmap_id)*)] = [
            $(&$heightmap_id),*
        ];

    };
}
