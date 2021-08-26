#[derive(Debug, Default)]
pub struct RabbitEntity {
    /// The rabbit variant, "RabbitType" .
    variant: RabbitVariant
}

#[derive(Debug)]
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
        match self {
            RabbitVariant::Brown => 0,
            RabbitVariant::White => 1,
            RabbitVariant::Black => 2,
            RabbitVariant::BlackAndWhite => 3,
            RabbitVariant::Gold => 4,
            RabbitVariant::SaltAndPepper => 5,
            RabbitVariant::TheKillerBunny => 99,
            // Actually not saved because it depends on the custom name.
            // Might be useless here as a distinct variant.
            RabbitVariant::Toast => u8::MAX
        }
    }

}
