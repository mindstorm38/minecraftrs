#[derive(Debug, Default)]
pub struct ParrotEntity {
    variant: ParrotVariant
}

#[derive(Debug)]
pub enum ParrotVariant {
    Red,
    Blue,
    Green,
    Cyan,
    Gray
}

impl Default for ParrotVariant {
    fn default() -> Self {
        Self::Red
    }
}

impl ParrotVariant {

    pub fn get_id(self) -> u8 {
        match self {
            ParrotVariant::Red => 0,
            ParrotVariant::Blue => 1,
            ParrotVariant::Green => 2,
            ParrotVariant::Cyan => 3,
            ParrotVariant::Gray => 4,
        }
    }

}
