use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

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
impl EntityCodec for SnowGolemEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<SnowGolemEntity>() {
            dst.insert_bool("Pumpkin", comp.pumpkin);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(SnowGolemEntity {
            pumpkin: src.get_bool("Pumpkin").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(SnowGolemEntity::default());
    }

}