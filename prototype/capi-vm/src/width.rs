pub trait Width {
    const SIZE: usize;
    const ENCODING: u8;
    const INFO: WidthInfo;
}

pub struct W8;

impl Width for W8 {
    const SIZE: usize = 1;
    const ENCODING: u8 = 0b00;
    const INFO: WidthInfo = WidthInfo {
        size: Self::SIZE,
        flag: Self::ENCODING << 6,
    };
}

pub struct W16;

impl Width for W16 {
    const SIZE: usize = 2;
    const ENCODING: u8 = 0b01;
    const INFO: WidthInfo = WidthInfo {
        size: Self::SIZE,
        flag: Self::ENCODING << 6,
    };
}

pub struct W32;

impl Width for W32 {
    const SIZE: usize = 4;
    const ENCODING: u8 = 0b10;
    const INFO: WidthInfo = WidthInfo {
        size: Self::SIZE,
        flag: Self::ENCODING << 6,
    };
}

pub struct W64;

impl Width for W64 {
    const SIZE: usize = 8;
    const ENCODING: u8 = 0b11;
    const INFO: WidthInfo = WidthInfo {
        size: Self::SIZE,
        flag: Self::ENCODING << 6,
    };
}

#[derive(Eq, PartialEq)]
pub struct WidthInfo {
    pub size: usize,
    pub flag: u8,
}
