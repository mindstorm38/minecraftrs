use mc_core::entity::{EntityComponent, EntityCodec, DefaultEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use std::convert::TryFrom;


#[derive(Debug, Default)]
pub struct SquidEntity;
impl EntityComponent for SquidEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<SquidEntity>::new();
}

#[derive(Debug, Default)]
pub struct GlowSquidEntity {
    dark_ticks_remaining: u32
}

impl EntityComponent for GlowSquidEntity {
    const CODEC: &'static dyn EntityCodec = &GlowSquidEntityCodec;
}

pub struct GlowSquidEntityCodec;
impl EntityCodec for GlowSquidEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<GlowSquidEntity>() {
            dst.insert_i32("DarkTicksRemaining", i32::try_from(comp.dark_ticks_remaining).unwrap_or_default());
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(GlowSquidEntity {
            dark_ticks_remaining: src.get_i32("DarkTicksRemaining").unwrap_or_default() as u32
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(GlowSquidEntity::default());
    }

}