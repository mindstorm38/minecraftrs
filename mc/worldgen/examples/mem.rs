use mc_worldgen::layer_new::{Layer, island::IslandLayer, State};
use mc_worldgen::gen::release102::LevelGenRelease102;

fn main() {

    let layer = IslandLayer::new(0);
    let layer_ref: &dyn Layer = &layer;

    println!("Size 'IslandLayer' {}o", std::mem::size_of_val(&layer));
    println!("Size 'dyn Layer' {}o", std::mem::size_of_val(layer_ref));
    println!("Size '&dyn Layer' {}o", std::mem::size_of_val(&layer_ref));
    println!("Size 'State' {}o", std::mem::size_of::<State>());

    let _gen = LevelGenRelease102::new(0);

}
