pub trait Width {
    const ENCODING: u8;
    const INFO: WidthInfo;
}

pub struct W8;

impl Width for W8 {
    const ENCODING: u8 = 0b00;
    const INFO: WidthInfo = WidthInfo {
        size: 1,
        flag: Self::ENCODING << 6,
    };
}

pub struct W16;

impl Width for W16 {
    const ENCODING: u8 = 0b01;
    const INFO: WidthInfo = WidthInfo {
        size: 2,
        flag: Self::ENCODING << 6,
    };
}

pub struct W32;

impl Width for W32 {
    const ENCODING: u8 = 0b10;
    const INFO: WidthInfo = WidthInfo {
        size: 4,
        flag: Self::ENCODING << 6,
    };
}

pub struct W64;

impl Width for W64 {
    const ENCODING: u8 = 0b11;
    const INFO: WidthInfo = WidthInfo {
        size: 8,
        flag: Self::ENCODING << 6,
    };
}

#[derive(Eq, PartialEq)]
pub struct WidthInfo {
    pub size: usize,
    pub flag: u8,
}
