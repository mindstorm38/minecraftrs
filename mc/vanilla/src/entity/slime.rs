use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct SlimeEntity {
    size: u8
}

impl EntityComponent for SlimeEntity {
    const CODEC: &'static dyn EntityCodec = &SlimeEntityCodec;
}

pub struct SlimeEntityCodec;
impl EntityCodec for SlimeEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<SlimeEntity>() {
            dst.insert_i32("Size", comp.size as i32);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(SlimeEntity {
            size: src.get_i32("Size").unwrap_or_default() as u8
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(SlimeEntity::default());
    }

}
