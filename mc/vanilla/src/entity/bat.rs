use mc_core::entity::{EntityCodec, EntityComponent, SingleEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;

#[derive(Debug, Default)]
pub struct BatEntity {
    hanging: bool
}

impl EntityComponent for BatEntity {
    const CODEC: &'static dyn EntityCodec = &BatEntityCodec;
}

pub struct BatEntityCodec;
impl SingleEntityCodec for BatEntityCodec {

    type Comp = BatEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("BatFlags", src.hanging);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        BatEntity {
            hanging: src.get_bool_or("BatFlags", false)
        }
    }

}
