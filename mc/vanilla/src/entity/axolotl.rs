use mc_core::entity::SingleEntityCodec;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct AxolotlEntity {
    variant: AxolotlVariant
}

entity_component!(AxolotlEntity: AxolotlEntityCodec);

pub struct AxolotlEntityCodec;
impl SingleEntityCodec for AxolotlEntityCodec {

    type Comp = AxolotlEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("Variant", src.variant.get_id() as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        AxolotlEntity {
            variant: AxolotlVariant::from_id(src.get_i32_or("Variant", 0) as u8)
        }
    }

}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum AxolotlVariant {
    Lucy = 0,
    Wild = 1,
    Gold = 2,
    Cyan = 3,
    Blue = 4
}

impl Default for AxolotlVariant {
    fn default() -> Self {
        Self::Lucy
    }
}

impl AxolotlVariant {

    pub fn get_id(self) -> u8 {
        self as u8
    }

    pub fn from_id(id: u8) -> Self {
        if id <= 4 {
            unsafe { std::mem::transmute(id) }
        } else {
            Self::default()
        }
    }

}
