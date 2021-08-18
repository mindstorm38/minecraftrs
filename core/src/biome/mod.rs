use std::collections::HashMap;

use crate::util::StaticPtr;


/// A basic biome structure. This structure is made for static definition.
#[derive(Debug)]
pub struct Biome {
    name: &'static str,
    id: i32
}


impl Biome {

    pub const fn new(name: &'static str, id: i32) -> Self {
        Self { name, id }
    }

    #[inline]
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn get_id(&self) -> i32 {
        self.id
    }

}


/// This is a global biomes palette, it is used in chunk storage to store biomes.
/// It allows you to register individual biomes in it as well as static biomes
/// arrays defined using the macro `biomes!`.
pub struct GlobalBiomes {
    next_sid: u16,
    biome_to_sid: HashMap<StaticPtr<Biome>, u16>,
    sid_to_biome: Vec<&'static Biome>,
    name_to_biome: HashMap<&'static str, &'static Biome>,
    id_to_biome: HashMap<i32, &'static Biome>
}

impl GlobalBiomes {

    pub fn new() -> Self {
        Self {
            next_sid: 0,
            biome_to_sid: HashMap::new(),
            sid_to_biome: Vec::new(),
            name_to_biome: HashMap::new(),
            id_to_biome: HashMap::new()
        }
    }

    pub fn from_static(static_biomes: &[&'static Biome]) -> Result<Self, ()> {
        let mut biomes = Self::new();
        biomes.register_static(static_biomes)?;
        Ok(biomes)
    }

    #[inline]
    fn get_biome_key(biome: &'static Biome) -> StaticPtr<Biome> {
        StaticPtr(biome)
    }

    pub fn register(&mut self, biome: &'static Biome) -> Result<(), ()> {

        let sid = self.next_sid;
        let next_sid = sid.checked_add(1).ok_or(())?;

        if let None = self.biome_to_sid.insert(Self::get_biome_key(biome), sid) {
            self.next_sid = next_sid;
            self.sid_to_biome.push(biome);
            self.name_to_biome.insert(biome.name, biome);
            self.id_to_biome.insert(biome.id, biome);
        }

        Ok(())

    }

    pub fn register_static(&mut self, static_biomes: &[&'static Biome]) -> Result<(), ()> {
        let count = static_biomes.len();
        self.biome_to_sid.reserve(count);
        self.sid_to_biome.reserve(count);
        self.name_to_biome.reserve(count);
        self.id_to_biome.reserve(count);
        for &biome in static_biomes {
            self.register(biome)?;
        }
        Ok(())
    }

    pub fn get_sid_from(&self, biome: &'static Biome) -> Option<u16> {
        Some(*self.biome_to_sid.get(&Self::get_biome_key(biome))?)
    }

    pub fn get_biome_from(&self, sid: u16) -> Option<&'static Biome> {
        Some(*self.sid_to_biome.get(sid as usize)?)
    }

    pub fn get_biome_from_name(&self, name: &str) -> Option<&'static Biome> {
        self.name_to_biome.get(name).cloned()
    }

    pub fn get_biome_from_id(&self, id: i32) -> Option<&'static Biome> {
        self.id_to_biome.get(&id).cloned()
    }

    pub fn has_biome(&self, biome: &'static Biome) -> bool {
        self.biome_to_sid.contains_key(&Self::get_biome_key(biome))
    }

    pub fn check_biome<E>(&self, biome: &'static Biome, err: impl FnOnce() -> E) -> Result<&'static Biome, E> {
        if self.has_biome(biome) { Ok(biome) } else { Err(err()) }
    }

    pub fn biomes_count(&self) -> usize {
        self.sid_to_biome.len()
    }

}


#[macro_export]
macro_rules! biomes {
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($biome_id:ident $biome_name:literal $biome_numeric_id:literal),*
        $(,)?
    ]) => {

        $($global_vis static $biome_id: $crate::biome::Biome = $crate::biome::Biome::new(
            concat!($namespace, ':', $biome_name),
            $biome_numeric_id
        );)*

        $global_vis static $static_id: [&'static $crate::biome::Biome; $crate::count!($($biome_id)*)] = [
            $(&$biome_id),*
        ];

    };
}
