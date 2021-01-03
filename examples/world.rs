use minecraftrs::version::Version;
use minecraftrs::world::World;
use minecraftrs::world::gen::layer::{State, Layer};
use std::mem::size_of;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let mut world: World = World::new_seeded(Version::RELEASE_1_2_5);

    println!("World 1.2 sizeof: {}", size_of::<World>());
    println!("Layer state sizeof: {}", size_of::<State>());
    println!("Layer sizeof: {}", size_of::<Layer>());
    println!("World seed: {}", world.get_info().seed);

    println!("Vec of box size: {}", size_of::<Vec<Box<Layer>>>());
    println!("Two options of box size: {}", size_of::<Option<Box<Layer>>>() << 1);
    println!("Two options of rc size: {}", size_of::<Option<Rc<Layer>>>() << 1);
    println!("Two options of rc+refcell size: {}", size_of::<Option<Rc<RefCell<Layer>>>>() << 1);

}