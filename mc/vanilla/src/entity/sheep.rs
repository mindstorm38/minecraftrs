use crate::util::DyeColor;
use mc_core::entity::{EntityCodec, EntityComponent, SingleEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;

#[derive(Debug, Default)]
pub struct SheepEntity {
    /// The color of the sheep
    color: DyeColor,
    /// True if the sheep has been shorn.
    sheared: bool
}

impl EntityComponent for SheepEntity {
    const CODEC: &'static dyn EntityCodec = &SheepEntityCodec;
}

pub struct SheepEntityCodec;
impl SingleEntityCodec for SheepEntityCodec {

    type Comp = SheepEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i8("Color", src.color.get_id() as i8);
        dst.insert_bool("Sheared", src.sheared);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        SheepEntity {
            color: DyeColor::from_id(src.get_i8_or("Color", 0) as u8),
            sheared: src.get_bool_or("Sheared", false)
        }
    }

}
