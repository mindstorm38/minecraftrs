use std::sync::Arc;

use mc_core::rand::JavaRandom;

use mc_vanilla::block::*;
use mc_vanilla::block::material::{TAG_LOG, TAG_LEAVES};

use crate::feature::Feature;
use crate::view::LevelView;
use super::{
    TreePalette, TreeHeight, RandomTreeHeight, DoubleRandomTreeHeight,
    generate_leaves_layer, generate_falling_vines
};


/// A kind of tree used for configuration of `TreeFeature` and used for position validation
/// and leaves shapes.
pub enum TreeKind {
    Default,
    /// Position validation is less strict.
    Forest,
    /// Tree crown is wider and position validation include water, also add vines falling of leaves.
    Swamp,
    /// Adds vines falling of leaves and on the trunk.
    Jungle
}


/// A common tree feature, used for well-known tree shape (oak, birch, jungle, swamp).
pub struct TreeFeature<H> {
    palette: TreePalette,
    height: H,
    kind: TreeKind
}

impl<H: TreeHeight> TreeFeature<H> {
    pub fn new(palette: TreePalette, height: H, kind: TreeKind) -> Self {
        Self { palette, height, kind }
    }
}

impl TreeFeature<RandomTreeHeight> {

    pub fn new_oak() -> Self {
        Self::new(TreePalette::new_oak(), RandomTreeHeight::new(4, 3), TreeKind::Default)
    }

    pub fn new_forest_birch() -> Self {
        Self::new(TreePalette::new_birch(), RandomTreeHeight::new(5, 3), TreeKind::Forest)
    }

    pub fn new_swamp() -> Self {
        Self::new(TreePalette::new_oak(), RandomTreeHeight::new(5, 4), TreeKind::Swamp)
    }

}

impl TreeFeature<DoubleRandomTreeHeight> {

    pub fn new_jungle() -> Self {
        Self::new(TreePalette::new_jungle(), DoubleRandomTreeHeight::new(4, 7, 3), TreeKind::Jungle)
    }

}

impl<H: TreeHeight> Feature for TreeFeature<H> {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, mut y: i32, z: i32) -> bool {

        // let height = rand.next_int_bounded(self.var_height as i32) + self.base_height as i32;
        let height = self.height.gen(rand);
        let block_air = AIR.get_default_state();

        let is_swamp_kind = matches!(self.kind, TreeKind::Swamp);
        let mut in_water = false;

        if is_swamp_kind {
            while level.get_block_at(x, y - 1, z).unwrap_or(block_air).is_block(&WATER) {
                y -= 1;
                in_water = true;
            }
        }

        let y = y;  // Re-alias to made it non-mut.
        let max_y = if is_swamp_kind { 128 } else { 256 };

        if y < 1 || y + height + 1 > max_y {
            return false;
        }

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        for dy in y..=(y + height + 1) {

            let mut radius = 1;

            if dy == y {
                radius = 0;
            }

            if dy >= (y + height - 1) {
                radius = if is_swamp_kind { 3 } else { 2 };
            }

            for dx in (x - radius)..=(x + radius) {
                for dz in (z - radius)..=(z + radius) {
                    if dy >= 0 && dy < max_y { // This condition seems useless since Y is already checked.
                        if let Ok(block) = level.get_block_at(dx, dy, dz) {
                            let block = block.get_block();
                            if block != &AIR && !env_blocks.has_block_tag(block, &TAG_LEAVES) {
                                match self.kind {
                                    TreeKind::Default | TreeKind::Jungle => {
                                        if block != &GRASS_BLOCK && block != &DIRT && !env_blocks.has_block_tag(block, &TAG_LOG) {
                                            return false;
                                        }
                                    },
                                    TreeKind::Forest => return false,
                                    TreeKind::Swamp => {
                                        if block == &WATER {
                                            if dy > y {
                                                return false;
                                            }
                                        } else {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }

        }

        let ground_block = level.get_block_at(x, y - 1, z).unwrap_or(block_air).get_block();

        // FIXME: In the following condition, the !in_water is a trick that was implemented as a
        //  fix for a specific position in a swamp, but this was only caused by decoration order.
        //  Might be removable in the future.
        if (ground_block != &GRASS_BLOCK && ground_block != &DIRT && !in_water) || y >= max_y - height - 1 { // Last condition seems redundant.
            return false;
        }

        level.set_block_at(x, y - 1, z, DIRT.get_default_state()).unwrap();

        // Get actual palette.
        let block_leaves = self.palette.get_leaves(1);
        let block_log = self.palette.log;

        // Base radius for leaves differs in swamps.
        let base_radius = if is_swamp_kind { 2 } else { 1 };

        for dy in (y + height - 3)..=(y + height) {
            let top_diff = dy - (y + height);
            let radius = base_radius - top_diff / 2;
            generate_leaves_layer(level, env_blocks, x, dy, z, radius, block_leaves, || {
                rand.next_int_bounded(2) != 0 && top_diff != 0
            });
        }

        let block_vine = VINE.get_default_state();
        let block_west_vine = block_vine.with(&PROP_WEST, true).unwrap();
        let block_east_vine = block_vine.with(&PROP_EAST, true).unwrap();
        let block_south_vine = block_vine.with(&PROP_SOUTH, true).unwrap();
        let block_north_vine = block_vine.with(&PROP_NORTH, true).unwrap();

        for dy in y..(y + height) {
            let block = level.get_block_at(x, dy, z).unwrap().get_block();
            if block == &AIR || env_blocks.has_block_tag(block, &TAG_LEAVES) || (is_swamp_kind && block == &WATER) {

                level.set_block_at(x, dy, z, block_log).unwrap();

                if matches!(self.kind, TreeKind::Jungle) && dy != y {

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x - 1, dy, z).unwrap().is_block(&AIR) {
                        level.set_block_at(x - 1, dy, z, block_east_vine).unwrap();
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x + 1, dy, z).unwrap().is_block(&AIR) {
                        level.set_block_at(x + 1, dy, z, block_west_vine).unwrap();
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x, dy, z - 1).unwrap().is_block(&AIR) {
                        level.set_block_at(x, dy, z - 1, block_south_vine).unwrap();
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x, dy, z + 1).unwrap().is_block(&AIR) {
                        level.set_block_at(x, dy, z + 1, block_north_vine).unwrap();
                    }

                }

            }
        }

        if matches!(self.kind, TreeKind::Swamp | TreeKind::Jungle) {

            for dy in (y + height - 3)..=(y + height) {

                let top_diff = dy - (y + height);
                let radius = base_radius - top_diff / 2;

                for dx in (x - radius)..=(x + radius) {
                    for dz in (z - radius)..=(z + radius) {
                        if env_blocks.has_block_tag(level.get_block_at(dx, dy, dz).unwrap().get_block(), &TAG_LEAVES) {

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(dx - 1, dy, dz).unwrap().is_block(&AIR) {
                                generate_falling_vines(level, dx - 1, dy, dz, block_east_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(dx + 1, dy, dz).unwrap().is_block(&AIR) {
                                generate_falling_vines(level, dx + 1, dy, dz, block_west_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(dx, dy, dz - 1).unwrap().is_block(&AIR) {
                                generate_falling_vines(level, dx, dy, dz - 1, block_south_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(dx, dy, dz + 1).unwrap().is_block(&AIR) {
                                generate_falling_vines(level, dx, dy, dz + 1, block_north_vine);
                            }

                        }
                    }
                }

            }

        }

        true

    }

}