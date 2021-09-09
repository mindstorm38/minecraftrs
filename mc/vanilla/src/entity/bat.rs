use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::entity_component;

#[derive(Debug, Default)]
pub struct BatEntity {
    hanging: bool
}

entity_component!(BatEntity: BatEntityCodec);

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
