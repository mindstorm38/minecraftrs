use crate::block::BlockState;


/// A structure used to statically define an heightmap type that
pub struct HeightmapType {
    name: &'static str,
    predicate: fn(&'static BlockState) -> bool
}

impl HeightmapType {
    pub const fn new(name: &'static str, predicate: fn(&'static BlockState) -> bool) -> Self {
        Self { name, predicate }
    }
}

pub struct Heightmap {
    typ: &'static HeightmapType
}


#[macro_export]
macro_rules! heightmaps {
    ($($vis:vis $id:ident: $predicate:ident;)*) => {
        $($vis static $id: $crate::world::heightmap::HeightmapType = $crate::world::heightmap::HeightmapType::new(stringify!($id), $predicate);)*
    };
}
