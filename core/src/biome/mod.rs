use std::sync::atomic::{AtomicU32, Ordering};

use crate::util::StaticUidGenerator;
use crate::generic::RwGenericMap;
use std::collections::HashMap;


/// A basic biome structure. Allow extensions for modifying.
pub struct Biome {
    uid: u32,
    name: &'static str,
    extensions: RwGenericMap
}

impl Biome {

    pub fn new(name: &'static str) -> Biome {

        static UID: StaticUidGenerator = StaticUidGenerator::new();

        Biome {
            uid: UID.next(),
            name,
            extensions: RwGenericMap::new()
        }

    }

}



/// Trait to implement for all biomes registers, automatically implemented with the `biomes!` macro.
pub trait StaticBiomes {
    fn iter_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Biome> + 'a>;
}


/// A working biomes registry giving effective UIDs to registered biomes.
pub struct WorkBiomes<'a> {
    next_uid: u8, // 0 is reserved, like the null-ptr
    biomes_to_uid: HashMap<u32, u8>,
    uid_to_biomes: Vec<&'a Biome>
}


#[macro_export]
macro_rules! biomes {
    ($struct_id:ident $static_id:ident [
        $($biome_id:ident $biome_name:literal),*
        $(,)?
    ]) => {

        #[allow(non_snake_case)]
        pub struct $struct_id {
            biomes: Vec<std::ptr::NonNull<$crate::biome::Biome>>,
            $( pub $block_id: $crate::biome::Biome, )*
            _marker: std::marker::PhantomPinned
        }

        impl $struct_id {
            pub fn load() -> std::pin::Pin<Box<Self>> {

                use $crate::biome::Biome;
                use std::marker::PhantomPinned;
                use std::ptr::NonNull;

                let mut biomes_count = 0;

                fn inc(b: Biome, bc: &mut usize) -> Biome {
                    *bc += 1;
                    b
                }

                let mut reg = Box::pin(Self {
                    $($biome_id: inc(Biome::new($biome_name), &mut biomes_count),)*
                    biomes: Vec::with_capacity(biomes_count),
                    _marker: PhantomPinned
                });

                unsafe {
                    let reg_mut = reg.as_mut().get_unchecked_mut();
                    $(reg_mut.biomes.push(NonNull::from(&reg_mut.$biome_id));)*
                }

                reg

            }
        }

    };
}