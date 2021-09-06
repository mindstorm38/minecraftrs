use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct PigEntity {
    /// True if there is a saddle on the pig.
    saddle: bool
}

impl EntityComponent for PigEntity {
    const CODEC: &'static dyn EntityCodec = &PigEntityCodec;
}

pub struct PigEntityCodec;
impl EntityCodec for PigEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<PigEntity>() {
            dst.insert_bool("Saddle", comp.saddle);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(PigEntity {
            saddle: src.get_bool("Saddle").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(PigEntity::default());
    }

}