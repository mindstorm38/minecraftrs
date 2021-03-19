use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mc_core::block::{Blocks, StaticBlocks, BlockState, vanilla::VanillaBlocks};


fn pick_states(blocks: impl StaticBlocks) -> Vec<&'static BlockState> {
    let last_uid = blocks.get_last_uid();

}


fn state_benchmark(c: &mut Criterion) {

    // Lazy load blocks
    *VanillaBlocks;

    c.bench_function("Blocks::register", |b| {
        b.iter(|| {
            let mut blocks = Blocks::new();
            blocks.register(&*VanillaBlocks);
        })
    });

    let mut blocks = Blocks::new();
    blocks.register(&*VanillaBlocks);

    c.bench_function("Blocks::register", |b| {
        b.iter(|| {
            let mut blocks = Blocks::new();
            blocks.register(&*VanillaBlocks);
        })
    });

}


criterion_group!(benches, state_benchmark);
criterion_main!(benches);
