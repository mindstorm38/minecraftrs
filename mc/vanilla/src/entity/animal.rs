use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::entity_component;
use crate::util::DyeColor;


#[derive(Debug, Default)]
pub struct PigEntity {
    /// True if there is a saddle on the pig.
    saddle: bool
}

entity_component!(PigEntity: PigEntityCodec);

pub struct PigEntityCodec;
impl SingleEntityCodec for PigEntityCodec {

    type Comp = PigEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("Saddle", src.saddle);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        PigEntity {
            saddle: src.get_bool_or("Saddle", false)
        }
    }

}


#[derive(Debug, Default)]
pub struct SheepEntity {
    /// The color of the sheep
    color: DyeColor,
    /// True if the sheep has been shorn.
    sheared: bool
}

entity_component!(SheepEntity: SheepEntityCodec);

pub struct SheepEntityCodec;
impl SingleEntityCodec for SheepEntityCodec {

    type Comp = SheepEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i8("Color", src.color.get_id() as i8);
        dst.insert_bool("Sheared", src.sheared);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        SheepEntity {
            color: DyeColor::from_id(src.get_i8_or("Color", 0) as u8),
            sheared: src.get_bool_or("Sheared", false)
        }
    }

}


#[derive(Debug, Default)]
pub struct ChickenEntity {
    /// Number of ticks until the chicken lays its egg. Laying occurs at 0 and this timer gets
    /// reset to a new random value between 6000 and 12000.
    egg_lay_cooldown: u32,
    /// Whether or not the chicken is a jockey for a baby zombie. If true, the chicken can
    /// naturally despawn, drops 10 experience upon death instead of 1-3 and cannot spawn
    /// eggs. Baby zombies can still control a ridden chicken even if this is set false.
    is_jockey: bool
}

entity_component!(ChickenEntity: ChickenEntityCodec);

pub struct ChickenEntityCodec;
impl SingleEntityCodec for ChickenEntityCodec {

    type Comp = ChickenEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("EggLayTime", src.egg_lay_cooldown as i32);
        dst.insert_bool("IsChickenJockey", src.is_jockey);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        ChickenEntity {
            egg_lay_cooldown: src.get_i32_or("EggLayTime", 0) as u32,
            is_jockey: src.get_bool_or("IsChickenJockey", false)
        }
    }

}


#[derive(Debug, Default)]
pub struct CowEntity;
entity_component!(CowEntity: default);
