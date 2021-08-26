use crate::util::DyeColor;

#[derive(Debug)]
pub struct WolfEntity {
    collar_color: DyeColor
}

impl Default for WolfEntity {
    fn default() -> Self {
        Self {
            collar_color: DyeColor::Red
        }
    }
}
