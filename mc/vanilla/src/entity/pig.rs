use mc_core::entity::{EntityCodec, EntityComponent, SingleEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;

#[derive(Debug, Default)]
pub struct PigEntity {
    /// True if there is a saddle on the pig.
    saddle: bool
}

impl EntityComponent for PigEntity {
    const CODEC: &'static dyn EntityCodec = &PigEntityCodec;
}

pub struct PigEntityCodec;
impl SingleEntityCodec for PigEntityCodec {

    type Comp = PigEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("Saddle", src.saddle);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        PigEntity {
            saddle: src.get_bool_or("Saddle", false)
        }
    }

}
