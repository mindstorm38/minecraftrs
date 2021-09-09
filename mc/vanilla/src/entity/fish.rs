use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::entity_component;
use mc_core::util::NbtExt;
use crate::util::DyeColor;

#[derive(Debug, Default)]
pub struct SalmonEntity;
entity_component!(SalmonEntity: default);

#[derive(Debug, Default)]
pub struct CodEntity;
entity_component!(CodEntity: default);

#[derive(Debug, Default)]
pub struct PufferfishEntity {
    state: PuffState
}

entity_component!(PufferfishEntity: PufferfishEntityCodec);

pub struct PufferfishEntityCodec;
impl SingleEntityCodec for PufferfishEntityCodec {

    type Comp = PufferfishEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("PuffState", match src.state {
            PuffState::Deflated => 0,
            PuffState::HalfwayPuffedUp => 1,
            PuffState::FullyPuffedUp => 2,
        });
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        PufferfishEntity {
            state: match src.get_i32_or("PuffState", 0) {
                1 => PuffState::HalfwayPuffedUp,
                2 => PuffState::FullyPuffedUp,
                _ => PuffState::Deflated
            }
        }
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

entity_component!(TropicalFishEntity: TropicalFishEntityCodec);

pub struct TropicalFishEntityCodec;
impl SingleEntityCodec for TropicalFishEntityCodec {

    type Comp = TropicalFishEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("Variant", src.encode_variant() as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        if let Ok(encoded_variant) = src.get_i32("Variant") {

            let (
                pattern,
                body_color,
                pattern_color
            ) = TropicalFishEntity::decode_variant(encoded_variant as u32);

            TropicalFishEntity {
                pattern,
                body_color,
                pattern_color
            }

        } else {
            TropicalFishEntity::default()
        }
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
