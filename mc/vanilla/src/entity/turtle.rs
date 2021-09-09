use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::pos::BlockPos;
use mc_core::util::NbtExt;
use mc_core::entity_component;

#[derive(Debug, Default)]
pub struct TurtleEntity {
    /// True if the turtle has egg.
    has_egg: bool,
    /// The position the turtle travels toward to lay its eggs after breeding.
    home_pos: BlockPos,
    /// Used for swimming to random points in water.
    travel_pos: BlockPos
}

entity_component!(TurtleEntity: TurtleEntityCodec);

pub struct TurtleEntityCodec;
impl SingleEntityCodec for TurtleEntityCodec {

    type Comp = TurtleEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("HasEgg", src.has_egg);
        dst.insert_split_block_pos("HomePosX", "HomePosY", "HomePosZ", &src.home_pos);
        dst.insert_split_block_pos("TravelPosX", "TravelPosY", "TravelPosZ", &src.travel_pos);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        TurtleEntity {
            has_egg: src.get_bool("HasEgg").unwrap_or_default(),
            home_pos: src.get_split_block_pos("HomePosX", "HomePosY", "HomePosZ").unwrap_or_default(),
            travel_pos: src.get_split_block_pos("TravelPosX", "TravelPosY", "TravelPosZ").unwrap_or_default(),
        }
    }

}
