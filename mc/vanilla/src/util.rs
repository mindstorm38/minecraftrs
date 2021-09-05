
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
        use DyeColor::*;
        match self {
            White => 0,
            Orange => 1,
            Magenta => 2,
            LightBlue => 3,
            Yellow => 4,
            Lime => 5,
            Pink => 6,
            Gray => 7,
            LightGray => 8,
            Cyan => 9,
            Purple => 10,
            Blue => 11,
            Brown => 12,
            Green => 13,
            Red => 14,
            Black => 15
        }
    }
    
    pub fn from_id(id: u8) -> Self {
        use DyeColor::*;
        match id {
            0 => White,
            1 => Orange,
            2 => Magenta,
            3 => LightBlue,
            4 => Yellow,
            5 => Lime,
            6 => Pink,
            7 => Gray,
            8 => LightGray,
            9 => Cyan,
            10 => Purple,
            11 => Blue,
            12 => Brown,
            13 => Green,
            14 => Red,
            15 => Black,
            _ => Self::default()
        }
    }

    pub fn get_diffuse_color(self) -> u32 {
        use DyeColor::*;
        match self {
            White => 0xF9FFFE,
            Orange => 0xF9801D,
            Magenta => 0xC74EBD,
            LightBlue => 0x3AB3DA,
            Yellow => 0xFED83D,
            Lime => 0x80C71F,
            Pink => 0xF38BAA,
            Gray => 0x474F52,
            LightGray => 0x9D9D97,
            Cyan => 0x169C9C,
            Purple => 0x8932B8,
            Blue => 0x3C44AA,
            Brown => 0x835432,
            Green => 0x5E7C16,
            Red => 0xB02E26,
            Black => 0x1D1D21
        }
    }

}
