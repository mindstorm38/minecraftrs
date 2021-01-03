use std::collections::HashMap;
use std::hash::Hash;

/// Common trait for all object usable in the registry.
pub trait Registrable<ID: Eq + Hash> {
    fn get_name(&self) -> &'static str;
    fn get_id(&self) -> ID;
}


/// A block registry, used to store all available blocks for a specific
/// versions and get them by their identifier or legacy ids.
pub struct Registry<ID: Eq + Hash, T: Registrable<ID>> {
    data: Vec<T>,
    by_names: HashMap<&'static str, usize>,
    by_ids: HashMap<ID, usize>
}


impl<ID, T> Registry<ID, T>
where
    ID: Eq + Hash,
    T: Registrable<ID>
{

    pub fn new() -> Self {
        Registry {
            data: Vec::new(),
            by_names: HashMap::new(),
            by_ids: HashMap::new()
        }
    }

    pub fn register(&mut self, item: T) -> bool {

        if self.by_names.contains_key(&item.get_name()) ||
            self.by_ids.contains_key(&item.get_id()) {

            false

        } else {

            let idx = self.data.len();
            self.by_names.insert(item.get_name(), idx);
            self.by_ids.insert(item.get_id(), idx);
            self.data.push(item);

            true

        }

    }

    pub fn get_from_name(&self, name: &str) -> Option<&T> {
        Some(&self.data[*self.by_names.get(&name)?])
    }

    pub fn get_from_id(&self, id: ID) -> Option<&T> {
        Some(&self.data[*self.by_ids.get(&id)?])
    }

}
