use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::pos::BlockPos;
use mc_core::util::NbtExt;
use mc_core::entity_component;


#[derive(Debug, Default)]
pub struct BeeEntity {
    /// Time left in ticks until the bee can enter a beehive. Used when the bee is angered and
    /// released from the hive by a player, but the hive is smoked by a campfire.
    enter_hive_cooldown: i32,
    /// Number of ticks passed since the bee's last pollination.
    ticks_since_pollination: i32,
    /// How many crops the bee has grown since its last pollination. Used to limit number of
    /// crops it can grow.
    crops_grown_since_pollination: i32,
    /// True if the bee is carrying pollen.
    has_nectar: bool,
    /// True if the bee has stung a mob or player.
    has_stung: bool,
    /// Coordinates of the flower the bee is circling.
    flower_pos: BlockPos,
    /// Coordinates of the bee's hive.
    hive_pos: BlockPos
}

entity_component!(BeeEntity: BeeEntityCodec);

pub struct BeeEntityCodec;
impl SingleEntityCodec for BeeEntityCodec {

    type Comp = BeeEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("CannotEnterHiveTicks", src.enter_hive_cooldown);
        dst.insert_i32("TicksSincePollination", src.ticks_since_pollination);
        dst.insert_i32("CropsGrownSincePollination", src.crops_grown_since_pollination);
        dst.insert_bool("HasNectar", src.has_nectar);
        dst.insert_bool("HasStung", src.has_stung);
        dst.insert_block_pos("FlowerPos", &src.flower_pos);
        dst.insert_block_pos("HivePos", &src.hive_pos);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        BeeEntity {
            enter_hive_cooldown: src.get_i32_or("CannotEnterHiveTicks", 0),
            ticks_since_pollination: src.get_i32_or("TicksSincePollination", 0),
            crops_grown_since_pollination: src.get_i32_or("CropsGrownSincePollination", 0),
            has_nectar: src.get_bool_or("HasNectar", false),
            has_stung: src.get_bool_or("HasStung", false),
            flower_pos: src.get_block_pos("FlowerPos").unwrap_or_default(),
            hive_pos: src.get_block_pos("HivePos").unwrap_or_default(),
        }
    }

}
