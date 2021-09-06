#[derive(Debug, Default)]
pub struct RabbitEntity {
    /// The rabbit variant, "RabbitType" .
    variant: RabbitVariant
}

#[derive(Debug, Copy, Clone)]
pub enum RabbitVariant {
    Brown,
    White,
    Black,
    BlackAndWhite,
    Gold,
    SaltAndPepper,
    TheKillerBunny,
    Toast
}

impl Default for RabbitVariant {
    fn default() -> Self {
        Self::Brown
    }
}

impl RabbitVariant {

    pub fn get_id(self) -> u8 {
        use RabbitVariant::*;
        match self {
            Brown => 0,
            White => 1,
            Black => 2,
            BlackAndWhite => 3,
            Gold => 4,
            SaltAndPepper => 5,
            TheKillerBunny => 99,
            // Actually not saved because it depends on the custom name.
            // Might be useless here as a distinct variant.
            Toast => u8::MAX
        }
    }

    pub fn from_id(id: u8) -> Self {
        use RabbitVariant::*;
        match id {
            0 => Brown,
            1 => White,
            2 => Black,
            3 => BlackAndWhite,
            4 => Gold,
            5 => SaltAndPepper,
            99 => TheKillerBunny,
            _ => Self::default()
        }
    }

}
