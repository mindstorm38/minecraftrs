use mc_core::entity::SingleEntityCodec;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use nbt::CompoundTag;

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

entity_component!(SnowGolemEntity: SnowGolemEntityCodec);

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
