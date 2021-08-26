
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black
}

impl Default for DyeColor {
    fn default() -> Self {
        Self::White
    }
}

impl DyeColor {

    pub fn get_id(self) -> u8 {
        match self {
            DyeColor::White => 0,
            DyeColor::Orange => 1,
            DyeColor::Magenta => 2,
            DyeColor::LightBlue => 3,
            DyeColor::Yellow => 4,
            DyeColor::Lime => 5,
            DyeColor::Pink => 6,
            DyeColor::Gray => 7,
            DyeColor::LightGray => 8,
            DyeColor::Cyan => 9,
            DyeColor::Purple => 10,
            DyeColor::Blue => 11,
            DyeColor::Brown => 12,
            DyeColor::Green => 13,
            DyeColor::Red => 14,
            DyeColor::Black => 15
        }
    }

    pub fn get_diffuse_color(self) -> u32 {
        match self {
            DyeColor::White => 0xF9FFFE,
            DyeColor::Orange => 0xF9801D,
            DyeColor::Magenta => 0xC74EBD,
            DyeColor::LightBlue => 0x3AB3DA,
            DyeColor::Yellow => 0xFED83D,
            DyeColor::Lime => 0x80C71F,
            DyeColor::Pink => 0xF38BAA,
            DyeColor::Gray => 0x474F52,
            DyeColor::LightGray => 0x9D9D97,
            DyeColor::Cyan => 0x169C9C,
            DyeColor::Purple => 0x8932B8,
            DyeColor::Blue => 0x3C44AA,
            DyeColor::Brown => 0x835432,
            DyeColor::Green => 0x5E7C16,
            DyeColor::Red => 0xB02E26,
            DyeColor::Black => 0x1D1D21
        }
    }

}
