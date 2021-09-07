use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct RabbitEntity {
    // TODO: Currently missing 'MoreCarrotTicks'.
    /// The rabbit variant, "RabbitType" .
    variant: RabbitVariant
}

impl EntityComponent for RabbitEntity {
    const CODEC: &'static dyn EntityCodec = &RabbitEntityCodec;
}

pub struct RabbitEntityCodec;
impl EntityCodec for RabbitEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<RabbitEntity>() {
            dst.insert_i32("RabbitType", comp.variant.get_id() as i32);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(RabbitEntity {
            variant: RabbitVariant::from_id(src.get_i32("RabbitType").unwrap_or_default() as u8)
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(RabbitEntity::default());
    }

}

#[derive(Debug, Copy, Clone)]
pub enum RabbitVariant {
    Brown,
    White,
    Black,
    BlackAndWhite,
    Gold,
    SaltAndPepper,
    TheKillerBunny,
    Toast
}

impl Default for RabbitVariant {
    fn default() -> Self {
        Self::Brown
    }
}

impl RabbitVariant {

    pub fn get_id(self) -> u8 {
        use RabbitVariant::*;
        match self {
            Brown => 0,
            White => 1,
            Black => 2,
            BlackAndWhite => 3,
            Gold => 4,
            SaltAndPepper => 5,
            TheKillerBunny => 99,
            // Actually not saved because it depends on the custom name.
            // Might be useless here as a distinct variant.
            Toast => u8::MAX
        }
    }

    pub fn from_id(id: u8) -> Self {
        use RabbitVariant::*;
        match id {
            0 => Brown,
            1 => White,
            2 => Black,
            3 => BlackAndWhite,
            4 => Gold,
            5 => SaltAndPepper,
            99 => TheKillerBunny,
            _ => Self::default()
        }
    }

    pub fn is_hostile(self) -> bool {
        matches!(self, Self::TheKillerBunny)
    }

}
