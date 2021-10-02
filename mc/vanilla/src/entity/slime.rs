use mc_core::entity::SingleEntityCodec;
use mc_core::entity_component;
use nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct SlimeEntity {
    size: u8
}

entity_component!(SlimeEntity: SlimeEntityCodec);

pub struct SlimeEntityCodec;
impl SingleEntityCodec for SlimeEntityCodec {

    type Comp = SlimeEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("Size", src.size as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        SlimeEntity {
            size: src.get_i32("Size").unwrap_or_default() as u8
        }
    }

}
