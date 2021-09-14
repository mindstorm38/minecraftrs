use mc_worldgen::layer_new::{Layer, IslandLayer};

fn main() {

    let mut layer = IslandLayer::new(0);
    let layer_ref: &dyn Layer = &layer;

    println!("Size 'IslandLayer' {}o", std::mem::size_of_val(&layer));
    println!("Size 'dyn Layer' {}o", std::mem::size_of_val(layer_ref));
    println!("Size '&dyn Layer' {}o", std::mem::size_of_val(&layer_ref));

}
