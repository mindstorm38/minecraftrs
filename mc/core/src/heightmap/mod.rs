use crate::block::BlockState;


/// A structure used to statically define an heightmap type.
pub struct HeightmapType {
    name: &'static str,
    predicate: fn(&'static BlockState) -> bool
}

impl HeightmapType {

    pub const fn new(name: &'static str, predicate: fn(&'static BlockState) -> bool) -> Self {
        Self { name, predicate }
    }

}

impl PartialEq for &'static HeightmapType {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl Eq for &'static HeightmapType { }


/// A global heightmaps palette used by level, each chunk created into it will include all
/// these heightmaps and automatically computed and updated when modifying the chunk.
pub struct GlobalHeightmaps {
    heightmap_types: Vec<&'static HeightmapType>
}

impl GlobalHeightmaps {

    pub fn new() -> Self {
        Self {
            heightmap_types: Vec::new()
        }
    }

    pub fn with_all(slice: &[&'static HeightmapType]) -> Self {
        Self {
            heightmap_types: slice.iter().collect()
        }
    }

    pub fn register(&mut self, heightmap_type: &'static HeightmapType) {
        self.heightmap_types.push(heightmap_type);
    }

    pub fn register_all(&mut self, slice: &[&'static HeightmapType]) {
        self.heightmap_types.extend(slice);
    }

    pub fn has_heightmap_type(&self, heightmap_type: &'static HeightmapType) -> bool {
        self.heightmap_types.iter().any(move |&t| t == heightmap_type)
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

        $($global_vis static $heightmap_id: $crate::heightmap::HeightmapType = $crate::heightmap::HeightmapType::new(
            $heightmap_name,
            $heightmap_predicate
        );)*

        $global_vis static $static_id: [&'static $crate::heightmap::HeightmapType; $crate::count!($($heightmap_id)*)] = [
            $(&$heightmap_id),*
        ];

    };
}
