pub trait Width {
    const INFO: WidthInfo;
}

pub struct W8;

impl Width for W8 {
    const INFO: WidthInfo = WidthInfo {
        size: 1,
        flag: encoding::W8 << 6,
    };
}

pub struct W16;

impl Width for W16 {
    const INFO: WidthInfo = WidthInfo {
        size: 2,
        flag: encoding::W16 << 6,
    };
}

pub struct W32;

impl Width for W32 {
    const INFO: WidthInfo = WidthInfo {
        size: 4,
        flag: encoding::W32 << 6,
    };
}

pub struct W64;

impl Width for W64 {
    const INFO: WidthInfo = WidthInfo {
        size: 8,
        flag: encoding::W64 << 6,
    };
}

#[derive(Eq, PartialEq)]
pub struct WidthInfo {
    pub size: usize,
    pub flag: u8,
}

pub mod encoding {
    pub const W8: u8 = 0b00;
    pub const W16: u8 = 0b01;
    pub const W32: u8 = 0b10;
    pub const W64: u8 = 0b11;
}
