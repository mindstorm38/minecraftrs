use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct ParrotEntity {
    variant: ParrotVariant
}

impl EntityComponent for ParrotEntity {
    const CODEC: &'static dyn EntityCodec = &ParrotEntityCodec;
}

pub struct ParrotEntityCodec;
impl EntityCodec for ParrotEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<ParrotEntity>() {
            dst.insert_i32("Variant", comp.variant.get_id() as i32);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(ParrotEntity {
           variant: ParrotVariant::from_id(src.get_i32("Variant").unwrap_or_default() as u8)
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(ParrotEntity::default());
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
