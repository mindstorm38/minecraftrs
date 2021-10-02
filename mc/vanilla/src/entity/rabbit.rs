use mc_core::entity::SingleEntityCodec;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct RabbitEntity {
    // TODO: Currently missing 'MoreCarrotTicks'.
    /// The rabbit variant, "RabbitType" .
    variant: RabbitVariant
}

entity_component!(RabbitEntity: RabbitEntityCodec);

pub struct RabbitEntityCodec;
impl SingleEntityCodec for RabbitEntityCodec {

    type Comp = RabbitEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("RabbitType", src.variant.get_id() as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        RabbitEntity {
            variant: RabbitVariant::from_id(src.get_i32_or("RabbitType", 0) as u8)
        }
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
