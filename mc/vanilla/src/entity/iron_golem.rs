use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct IronGolemEntity {
    player_created: bool
}

impl EntityComponent for IronGolemEntity {
    const CODEC: &'static dyn EntityCodec = &IronGolemEntityCodec;
}

pub struct IronGolemEntityCodec;
impl EntityCodec for IronGolemEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<IronGolemEntity>() {
            dst.insert_bool("PlayerCreated", comp.player_created);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(IronGolemEntity {
            player_created: src.get_bool("PlayerCreated").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(IronGolemEntity::default());
    }

}