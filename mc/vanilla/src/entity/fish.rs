use crate::util::DyeColor;


#[derive(Debug, Default)]
pub struct FishEntity {
    /// I true, the fish has been released from a bucket.
    from_bucket: bool
}

#[derive(Debug, Default)]
pub struct SalmonEntity;

#[derive(Debug, Default)]
pub struct CodEntity;

#[derive(Debug, Default)]
pub struct PufferfishEntity {
    state: PuffState
}

#[derive(Debug, Default)]
pub struct TropicalFishEntity {
    variant: TropicalFishPattern,
    body_color: DyeColor,
    pattern_color: DyeColor
}

// Utils //

#[derive(Debug)]
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

#[derive(Debug)]
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

}
