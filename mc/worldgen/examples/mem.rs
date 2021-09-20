use mc_worldgen::layer_new::{LayerSystem, Layer, island, zoom, State, debug_layer_data, debug_biomes_grid};
use mc_worldgen::layer_new::iter;
use mc_worldgen::layer_new::iter::Layer as _;

fn main() {

    let layer = island::IslandLayer::new(0);
    let layer_ref: &dyn Layer = &layer;

    println!("Size 'IslandLayer' {}o", std::mem::size_of_val(&layer));
    println!("Size 'dyn Layer' {}o", std::mem::size_of_val(layer_ref));
    println!("Size '&dyn Layer' {}o", std::mem::size_of_val(&layer_ref));
    println!("Size 'State' {}o", std::mem::size_of::<State>());

    // let _gen = LevelGenRelease102::new(0);

    let mut system = LayerSystem::new();
    system.push(island::IslandLayer::new(0));
    system.push(zoom::ZoomLayer::new(0, true));

    let layer = system.borrow_root().unwrap().generate_size(16, 16, 16, 16);
    println!("Sequential:");
    debug_layer_data(&layer);

    let mut it = iter::ZoomLayer::new_fuzzy(0, iter::IslandLayer::new(0));
    // let mut it = iter::IslandLayer::new(0);
    let biomes = it.next_grid(16, 16, 16, 16);
    println!("Iterative:");
    debug_biomes_grid(&biomes[..], 16);

}
