use mc_worldgen::layer_new::{LayerSystem, Layer, island, zoom, snow, State, debug_layer_data, debug_biomes_grid, debug_rivers_grid, debug_pot_rivers_grid};
use mc_worldgen::layer_it::{Layer as ItLayer, LayerBuilder};
use mc_worldgen::layer_it;
use std::time::Instant;

use mc_worldgen::gen::release102::LevelGenRelease102;

fn main() {

    let layer = island::IslandLayer::new(0);
    let layer_ref: &dyn Layer = &layer;

    println!("Size 'IslandLayer' {}o", std::mem::size_of_val(&layer));
    println!("Size 'dyn Layer' {}o", std::mem::size_of_val(layer_ref));
    println!("Size '&dyn Layer' {}o", std::mem::size_of_val(&layer_ref));
    println!("Size 'State' {}o", std::mem::size_of::<State>());

    let gen = LevelGenRelease102::new(0);

    /*let mut system = LayerSystem::new();
    system.push(island::IslandLayer::new(1));
    system.push(zoom::ZoomLayer::new_fuzzy(2000));
    system.push(island::AddIslandLayer::new(1));
    system.push(zoom::ZoomLayer::new_smart(2001));
    system.push(island::AddIslandLayer::new(2));
    system.push(snow::AddSnowLayer::new(2));
    system.push(zoom::ZoomLayer::new_smart(2002));
    system.push(island::AddIslandLayer::new(3));
    system.push(zoom::ZoomLayer::new_smart(2003));
    system.push(island::AddIslandLayer::new(4));*/

    let start = Instant::now();
    let layer = gen.layer_system.borrow_root().unwrap()
        .generate_size(512, 512, 16, 16);
    println!("Sequential (time: {}us):", start.elapsed().as_micros());
    debug_layer_data(&layer);

    let (
        it_river,
        it_biome
    ) = LayerBuilder::with_island(1)
        .then_zoom_fuzzy(2000)
        .then_add_island(1)
        .then_zoom_smart(2001)
        .then_add_island(2)
        .then_add_snow(2)
        .then_zoom_smart(2002)
        .then_add_island(3)
        .then_zoom_smart(2003)
        .then_add_island(4)
        .then_add_mushroom_island(5)
        .into_box()
        .into_shared_split();

    let it_river = it_river
        .then_init_river(100)
        .then_zoom_smart(1000)
        .then_zoom_smart(1001)
        .then_zoom_smart(1002)
        .then_zoom_smart(1003)
        .then_zoom_smart(1004)
        .then_zoom_smart(1005)
        .then_add_river()
        .then_smooth(1000)
        .into_box()
        .build();

    let it_biome = it_biome
        .then_biome(200, (1, 2)).unwrap()
        .then_zoom_smart(1000)
        .then_zoom_smart(1001)
        .then_hills(1000)
        .then_zoom_smart(1000)
        .then_add_island(3)
        .then_zoom_smart(1001)
        .then_shore()
        .then_biome_river(1000)
        .then_zoom_smart(1002)
        .then_zoom_smart(1003)
        .then_smooth(1000)
        .into_box()
        .build();

    let mut it_mix = LayerBuilder::with_biome_and_river_mix(it_biome, it_river)
        .then_zoom_voronoi(10)
        .into_box()
        .build();

    it_mix.seed(0);

    let start = Instant::now();
    let biomes = it_mix.next_grid(512, 512, 16, 16);
    println!("Iterative (time: {}us):", start.elapsed().as_micros());
    debug_biomes_grid(&biomes[..], 16);

}

fn type_name_of<T>(val: &T) -> &'static str {
    std::any::type_name::<T>()
}