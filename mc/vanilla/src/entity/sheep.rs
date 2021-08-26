use crate::util::DyeColor;

#[derive(Debug, Default)]
pub struct SheepEntity {
    /// The color of the sheep
    color: DyeColor,
    /// True if the sheep has been shorn.
    sheared: bool
}
