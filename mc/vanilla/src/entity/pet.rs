use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use crate::util::DyeColor;


#[derive(Debug, Default)]
pub struct PetEntity {
    collar_color: DyeColor
}

entity_component!(PetEntity: PetEntityCodec);

pub struct PetEntityCodec;
impl SingleEntityCodec for PetEntityCodec {

    type Comp = PetEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i8("CollarColor", src.collar_color.get_id() as i8);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        PetEntity {
            collar_color: DyeColor::from_id(src.get_i8_or("CollarColor", 0) as u8)
        }
    }

}


#[derive(Debug, Default)]
pub struct WolfEntity;
entity_component!(WolfEntity: default);


#[derive(Debug, Default)]
pub struct CatEntity {
    variant: CatVariant
}

entity_component!(CatEntity: CatEntityCodec);

pub struct CatEntityCodec;
impl SingleEntityCodec for CatEntityCodec {

    type Comp = CatEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("CatType", src.variant.get_id() as i32)
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        CatEntity {
            variant: CatVariant::from_id(src.get_i32_or("CatType", 0) as u8)
        }
    }

}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CatVariant {
    Tabby = 0,
    Tuxedo = 1,
    Red = 2,
    Siamese = 3,
    BritishShorthair = 4,
    Calico = 5,
    Persian = 6,
    Ragdoll = 7,
    White = 8,
    Jellie = 9,
    Black = 10
}

impl Default for CatVariant {
    fn default() -> Self {
        Self::Tabby
    }
}

impl CatVariant {

    pub fn get_id(self) -> u8 {
        self as u8
    }

    pub fn from_id(id: u8) -> Self {
        if id <= 10 {
            unsafe { std::mem::transmute(id) }
        } else {
            Self::default()
        }
    }

}
