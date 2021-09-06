use std::env::var;

use mc_core::entity::{EntityComponent, EntityCodec, DefaultEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

use crate::util::DyeColor;


#[derive(Debug, Default)]
pub struct FishEntity {
    /// I true, the fish has been released from a bucket.
    from_bucket: bool
}

impl EntityComponent for FishEntity {
    const CODEC: &'static dyn EntityCodec = &FishEntityCodec;
}

pub struct FishEntityCodec;
impl EntityCodec for FishEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<FishEntity>() {
            dst.insert_bool("FromBucket", comp.from_bucket);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(FishEntity {
            from_bucket: src.get_bool("FromBucket").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(FishEntity::default());
    }

}


#[derive(Debug, Default)]
pub struct SalmonEntity;
impl EntityComponent for SalmonEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<SalmonEntity>::new();
}

#[derive(Debug, Default)]
pub struct CodEntity;
impl EntityComponent for CodEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<CodEntity>::new();
}

#[derive(Debug, Default)]
pub struct PufferfishEntity {
    state: PuffState
}

impl EntityComponent for PufferfishEntity {
    const CODEC: &'static dyn EntityCodec = &PufferfishEntityCodec;
}

pub struct PufferfishEntityCodec;
impl EntityCodec for PufferfishEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<PufferfishEntity>() {
            dst.insert_i32("PuffState", match (*comp).state {
                PuffState::HalfwayPuffedUp => 1,
                PuffState::FullyPuffedUp => 2,
                _ => 0
            });
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(PufferfishEntity {
            state: match src.get_i32("PuffState").unwrap_or_default() {
                1 => PuffState::HalfwayPuffedUp,
                2 => PuffState::FullyPuffedUp,
                _ => PuffState::Deflated
            }
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(PufferfishEntity::default());
    }

}

#[derive(Debug, Default)]
pub struct TropicalFishEntity {
    pattern: TropicalFishPattern,
    body_color: DyeColor,
    pattern_color: DyeColor
}

impl TropicalFishEntity {

    fn encode_variant(&self) -> u32 {
        let pat = self.pattern;
        pat.get_base() as u32 | (pat.get_index() as u32) << 8 | (self.body_color.get_id() as u32) << 16 | (self.pattern_color.get_id() as u32) << 24
    }

    fn decode_variant(encoded: u32) -> (TropicalFishPattern, DyeColor, DyeColor) {
        (
            TropicalFishPattern::from_base_and_index(
                (encoded & 0xFF) as u8,
                ((encoded >> 8) & 0xFF) as u8
            ),
            DyeColor::from_id(((encoded >> 16) & 0xFF) as u8),
            DyeColor::from_id(((encoded >> 24) & 0xFF) as u8),
        )
    }

}

impl EntityComponent for TropicalFishEntity {
    const CODEC: &'static dyn EntityCodec = &TropicalFishEntityCodec;
}

pub struct TropicalFishEntityCodec;
impl EntityCodec for TropicalFishEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<TropicalFishEntity>() {
            dst.insert_i32("Variant", comp.encode_variant() as i32);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        if let Ok(encoded_variant) = src.get_i32("Variant") {

            let (
                pattern,
                body_color,
                pattern_color
            ) = TropicalFishEntity::decode_variant(encoded_variant as u32);

            dst.add(TropicalFishEntity {
                pattern,
                body_color,
                pattern_color
            });

        } else {
            dst.add(TropicalFishEntity::default());
        }
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(TropicalFishEntity::default());
    }

}

// Utils //

#[derive(Debug, Copy, Clone)]
pub enum PuffState {
    Deflated,
    HalfwayPuffedUp,
    FullyPuffedUp,
}

impl Default for PuffState {
    fn default() -> Self {
        Self::Deflated
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish
}

impl Default for TropicalFishPattern {
    fn default() -> Self {
        Self::Kob
    }
}

impl TropicalFishPattern {

    pub fn is_small(self) -> bool {
        use TropicalFishPattern::*;
        matches!(self, Kob | Sunstreak | Snooper | Dasher | Brinely | Spotty)
    }

    #[inline]
    pub fn is_big(self) -> bool {
        !self.is_small()
    }

    fn get_base(self) -> u8 {
        if self.is_small() {
            0
        } else {
            1
        }
    }

    fn get_index(self) -> u8 {
        use TropicalFishPattern::*;
        match self {
            Kob | Flopper => 0,
            Sunstreak | Stripey => 1,
            Snooper | Glitter => 2,
            Dasher | Blockfish => 3,
            Brinely | Betty => 4,
            Spotty | Clayfish => 5
        }
    }

    fn from_base_and_index(base: u8, index: u8) -> Self {
        use TropicalFishPattern::*;
        match (base, index) {
            // Small
            (0, 0) => Kob,
            (0, 1) => Sunstreak,
            (0, 2) => Snooper,
            (0, 3) => Dasher,
            (0, 4) => Brinely,
            (0, 5) => Spotty,
            // Big
            (1, 0) => Flopper,
            (2, 1) => Stripey,
            (3, 2) => Glitter,
            (4, 3) => Blockfish,
            (5, 4) => Betty,
            (6, 5) => Clayfish,
            // Default
            _ => Self::default()
        }
    }

}
