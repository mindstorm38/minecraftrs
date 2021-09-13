use mc_worldgen::gen::release102::LevelGenRelease102;

fn main() {

    let size = std::mem::size_of_val(&LevelGenRelease102::new_layers(0));

    println!("Sizeof: {}", size);

}
