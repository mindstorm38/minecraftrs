use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct AxolotlEntity {
    variant: AxolotlVariant
}

impl EntityComponent for AxolotlEntity {
    const CODEC: &'static dyn EntityCodec = &AxolotlEntityCodec;
}

pub struct AxolotlEntityCodec;
impl EntityCodec for AxolotlEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<AxolotlEntity>() {
            dst.insert_i32("Variant", comp.variant.get_id() as i32);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(AxolotlEntity {
            variant: AxolotlVariant::from_id(src.get_i32("Variant").unwrap_or_default() as u8)
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(AxolotlEntity::default());
    }

}

#[derive(Debug, Copy, Clone)]
pub enum AxolotlVariant {
    Lucy,
    Wild,
    Gold,
    Cyan,
    Blue
}

impl Default for AxolotlVariant {
    fn default() -> Self {
        Self::Lucy
    }
}

impl AxolotlVariant {

    pub fn get_id(self) -> u8 {
        use AxolotlVariant::*;
        match self {
            Lucy => 0,
            Wild => 1,
            Gold => 2,
            Cyan => 3,
            Blue => 4,
        }
    }

    pub fn from_id(id: u8) -> Self {
        use AxolotlVariant::*;
        match id {
            0 => Lucy,
            1 => Wild,
            2 => Gold,
            3 => Cyan,
            4 => Blue,
            _ => Self::default()
        }
    }

}