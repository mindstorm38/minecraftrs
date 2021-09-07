use mc_core::entity::{SingleEntityCodec, EntityComponent, EntityCodec, DefaultEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use crate::util::DyeColor;


#[derive(Debug, Default)]
pub struct PetEntity {
    collar_color: DyeColor
}

impl EntityComponent for PetEntity {
    const CODEC: &'static dyn EntityCodec = &PetEntityCodec;
}

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

impl EntityComponent for WolfEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<WolfEntity>::new();
}


#[derive(Debug)]
pub struct CatEntity {
    variant: CatVariant
}

impl Default for CatEntity {
    fn default() -> Self {
        Self {
            variant: CatVariant::Tabby
        }
    }
}

impl EntityComponent for CatEntity {
    const CODEC: &'static dyn EntityCodec = &CatEntityCodec;
}

pub struct CatEntityCodec;
impl SingleEntityCodec for CatEntityCodec {

    type Comp = CatEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("CatType", src.variant as i32)
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        todo!()
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
