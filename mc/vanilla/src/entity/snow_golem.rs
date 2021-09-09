use mc_core::entity::{EntityCodec, EntityComponent, SingleEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;

#[derive(Debug)]
pub struct SnowGolemEntity {
    /// Whether or not the Snow Golem has a pumpkin on its head. True by default.
    pumpkin: bool
}

impl Default for SnowGolemEntity {
    fn default() -> Self {
        Self {
            pumpkin: true
        }
    }
}

impl EntityComponent for SnowGolemEntity {
    const CODEC: &'static dyn EntityCodec = &SnowGolemEntityCodec;
}

pub struct SnowGolemEntityCodec;
impl SingleEntityCodec for SnowGolemEntityCodec {

    type Comp = SnowGolemEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("Pumpkin", src.pumpkin);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        SnowGolemEntity {
            pumpkin: src.get_bool_or("Pumpkin", false)
        }
    }

}
