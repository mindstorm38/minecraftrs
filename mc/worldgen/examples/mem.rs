use mc_worldgen::layer_new::{LayerSystem, Layer, island, zoom, snow, State, debug_layer_data, debug_biomes_grid};
use mc_worldgen::layer_it::Layer as ItLayer;
use mc_worldgen::layer_it;
use std::time::Instant;

fn main() {

    let layer = island::IslandLayer::new(0);
    let layer_ref: &dyn Layer = &layer;

    println!("Size 'IslandLayer' {}o", std::mem::size_of_val(&layer));
    println!("Size 'dyn Layer' {}o", std::mem::size_of_val(layer_ref));
    println!("Size '&dyn Layer' {}o", std::mem::size_of_val(&layer_ref));
    println!("Size 'State' {}o", std::mem::size_of::<State>());

    // let _gen = LevelGenRelease102::new(0);

    let mut system = LayerSystem::new();
    system.push(island::IslandLayer::new(1));
    system.push(zoom::ZoomLayer::new_fuzzy(2000));
    system.push(island::AddIslandLayer::new(1));
    system.push(zoom::ZoomLayer::new_smart(2001));
    system.push(island::AddIslandLayer::new(2));
    system.push(snow::AddSnowLayer::new(2));
    system.push(zoom::ZoomLayer::new_smart(2002));
    system.push(island::AddIslandLayer::new(3));
    system.push(zoom::ZoomLayer::new_smart(2003));
    system.push(island::AddIslandLayer::new(4));

    let start = Instant::now();
    let layer = system.borrow_root().unwrap().generate_size(16, 16, 16, 16);
    println!("Sequential (time: {}us):", start.elapsed().as_micros());
    debug_layer_data(&layer);

    let (
        it_river,
        it_biome
    ) = layer_it::island::IslandLayer::new(1)
        .zoom_fuzzy(2000)
        .add_island(1)
        .zoom_smart(2001)
        .add_island(2)
        .add_snow(2)
        .zoom_smart(2002)
        .add_island(3)
        .zoom_smart(2003)
        .add_island(4)
        .add_mushroom_island(5)
        .into_box()
        .into_shared_split();

    let mut it_river = it_river
        .init_river(100)
        .zoom_smart(1000)
        .zoom_smart(1001)
        .zoom_smart(1002)
        .zoom_smart(1003)
        .zoom_smart(1004)
        .zoom_smart(1005)
        .add_river()
        .smooth(1000)
        .into_box();

    let mut it_biome = it_biome
        .biome(200, (1, 2)).unwrap()
        .zoom_smart(1000)
        .zoom_smart(1001)
        .into_box();

    let start = Instant::now();
    let biomes = it_biome.next_grid(16, 16, 16, 16);
    println!("Iterative (time: {}us):", start.elapsed().as_micros());
    debug_biomes_grid(&biomes[..], 16);

}

fn type_name_of<T>(val: &T) -> &'static str {
    std::any::type_name::<T>()
}