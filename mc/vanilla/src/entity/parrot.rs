use mc_core::entity::SingleEntityCodec;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct ParrotEntity {
    variant: ParrotVariant
}

entity_component!(ParrotEntity: ParrotEntityCodec);

pub struct ParrotEntityCodec;
impl SingleEntityCodec for ParrotEntityCodec {

    type Comp = ParrotEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("Variant", src.variant.get_id() as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        ParrotEntity {
            variant: ParrotVariant::from_id(src.get_i32_or("Variant", 0) as u8)
        }
    }

}


#[derive(Debug, Copy, Clone)]
pub enum ParrotVariant {
    Red,
    Blue,
    Green,
    Cyan,
    Gray
}

impl Default for ParrotVariant {
    fn default() -> Self {
        Self::Red
    }
}

impl ParrotVariant {

    pub fn get_id(self) -> u8 {
        use ParrotVariant::*;
        match self {
            Red => 0,
            Blue => 1,
            Green => 2,
            Cyan => 3,
            Gray => 4,
        }
    }

    pub fn from_id(id: u8) -> Self {
        use ParrotVariant::*;
        match id {
            0 => Red,
            1 => Blue,
            2 => Green,
            3 => Cyan,
            4 => Gray,
            _ => Self::default()
        }
    }

}
