use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::pos::BlockPos;
use mc_core::util::NbtExt;

#[derive(Debug, Default)]
pub struct TurtleEntity {
    /// True if the turtle has egg.
    has_egg: bool,
    /// The position the turtle travels toward to lay its eggs after breeding.
    home_pos: BlockPos,
    /// Used for swimming to random points in water.
    travel_pos: BlockPos
}

impl EntityComponent for TurtleEntity {
    const CODEC: &'static dyn EntityCodec = &TurtleEntityCodec;
}

pub struct TurtleEntityCodec;
impl EntityCodec for TurtleEntityCodec {
    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<TurtleEntity>() {
            dst.insert_bool("HasEgg", comp.has_egg);
            dst.insert_split_block_pos("HomePosX", "HomePosY", "HomePosZ", &comp.home_pos);
            dst.insert_split_block_pos("TravelPosX", "TravelPosY", "TravelPosZ", &comp.travel_pos);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(TurtleEntity {
            has_egg: src.get_bool("HasEgg").unwrap_or_default(),
            home_pos: src.get_split_block_pos("HomePosX", "HomePosY", "HomePosZ").unwrap_or_default(),
            travel_pos: src.get_split_block_pos("TravelPosX", "TravelPosY", "TravelPosZ").unwrap_or_default(),
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(TurtleEntity::default());
    }

}
