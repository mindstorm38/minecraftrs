use std::collections::HashMap;
use std::pin::Pin;
use std::any::Any;

use crate::util::generic::{RwGenericMap, GuardedRef, GuardedMut};
use crate::util::UidGenerator;

pub mod vanilla;


/// A basic biome structure. Allow extensions for modifying.
pub struct Biome {
    uid: u32,
    name: &'static str,
    extensions: RwGenericMap
}

impl Biome {

    pub fn new(name: &'static str) -> Biome {
        static UID: UidGenerator = UidGenerator::new();
        Biome {
            uid: UID.next(),
            name,
            extensions: RwGenericMap::new()
        }
    }

    pub fn get_uid(&self) -> u32 {
        self.uid
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn add_ext<E: Any + Sync + Send>(&self, ext: E) {
        self.extensions.add(ext);
    }

    pub fn get_ext<E: Any + Sync + Send>(&self) -> Option<GuardedRef<E>> {
        self.extensions.get()
    }

    pub fn get_ext_mut<E: Any + Sync + Send>(&self) -> Option<GuardedMut<E>> {
        self.extensions.get_mut()
    }

}


/// Trait to implement for all biomes registers, automatically implemented with the `biomes!` macro.
pub trait StaticBiomes {
    fn iter_biomes<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Biome> + 'a>;
    fn biomes_count(&self) -> usize;
}


/// A working biomes' registry mapping unique biomes IDs to save IDs (SID).
pub struct WorkBiomes<'a> {
    next_sid: u16, // 0 is reserved, like the null-ptr
    biomes_to_uid: HashMap<u32, u16>,
    uid_to_biomes: Vec<&'a Biome>
}

impl<'a> WorkBiomes<'a> {

    pub fn new() -> WorkBiomes<'a> {
        WorkBiomes {
            next_sid: 1,
            biomes_to_uid: HashMap::new(),
            uid_to_biomes: Vec::new()
        }
    }

    pub fn register(&mut self, biome: &'a Biome) {
        let uid = self.next_sid;
        self.next_sid = uid.checked_add(1).expect("Too much biomes in this register.");
        self.biomes_to_uid.insert(biome.uid, uid);
        self.uid_to_biomes.push(biome);
    }

    pub fn register_static(&mut self, static_biomes: &'a Pin<Box<impl StaticBiomes>>) {
        let count = static_biomes.biomes_count();
        self.biomes_to_uid.reserve(count);
        self.uid_to_biomes.reserve(count);
        for biome in static_biomes.iter_biomes() {
            self.register(biome);
        }
    }

    pub fn get_uid_from(&self, biome: &Biome) -> Option<u16> {
        let biome_uid = biome.uid;
        let biome_offset = *self.biomes_to_uid.get(&biome_uid)?;
        Some(biome_offset)
    }

    pub fn get_biome_from(&self, uid: u16) -> Option<&'a Biome> {
        match uid {
            0 => None,
            _ => Some(*self.uid_to_biomes.get((uid - 1) as usize)?)
        }
    }

    pub fn biomes_count(&self) -> usize {
        self.uid_to_biomes.len()
    }

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
            $( pub $biome_id: $crate::biome::Biome, )*
            _marker: std::marker::PhantomPinned
        }

        impl $struct_id {
            pub fn load() -> std::pin::Pin<Box<Self>> {

                use $crate::biome::Biome;
                use std::marker::PhantomPinned;
                use std::ptr::NonNull;

                let mut biomes_count = 0;

                let mut inc = |b: Biome| {
                    biomes_count += 1;
                    b
                };

                let mut reg = Box::pin(Self {
                    $($biome_id: inc(Biome::new($biome_name)),)*
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

        // Enforce Send/Sync because NonNull are pointing to pined box content.
        unsafe impl Send for $struct_id {}
        unsafe impl Sync for $struct_id {}

        impl $crate::biome::StaticBiomes for $struct_id {

            fn iter_biomes<'a>(&'a self) -> Box<dyn Iterator<Item=&'a $crate::biome::Biome> + 'a> {
                Box::new(self.biomes.iter().map(|ptr| unsafe { ptr.as_ref() }))
            }

            fn biomes_count(&self) -> usize {
                self.biomes.len()
            }

        }

        #[allow(non_upper_case_globals)]
        pub static $static_id: once_cell::sync::Lazy<std::pin::Pin<Box<$struct_id>>> = once_cell::sync::Lazy::new(|| $struct_id::load());

    };
}