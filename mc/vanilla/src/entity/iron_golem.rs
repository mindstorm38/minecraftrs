use mc_core::entity::SingleEntityCodec;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct IronGolemEntity {
    player_created: bool
}

entity_component!(IronGolemEntity: IronGolemEntityCodec);

pub struct IronGolemEntityCodec;
impl SingleEntityCodec for IronGolemEntityCodec {

    type Comp = IronGolemEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("PlayerCreated", src.player_created);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        IronGolemEntity {
            player_created: src.get_bool_or("PlayerCreated", false)
        }
    }

}
