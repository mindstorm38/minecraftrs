use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct BatEntity {
    hanging: bool
}

impl EntityComponent for BatEntity {
    const CODEC: &'static dyn EntityCodec = &BatEntityCodec;
}

pub struct BatEntityCodec;
impl EntityCodec for BatEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<BatEntity>() {
            dst.insert_bool("BatFlags", comp.hanging);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(BatEntity {
            hanging: src.get_bool("BatFlags").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(BatEntity::default());
    }

}