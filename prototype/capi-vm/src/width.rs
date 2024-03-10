pub trait Width {
    const ENCODING: u8;
    const SIZE: usize;

    const INFO: WidthInfo = WidthInfo {
        size: Self::SIZE,
        flag: Self::ENCODING << 6,
    };
}

pub struct W8;

impl Width for W8 {
    const ENCODING: u8 = 0b00;
    const SIZE: usize = 1;
}

pub struct W16;

impl Width for W16 {
    const ENCODING: u8 = 0b01;
    const SIZE: usize = 2;
}

pub struct W32;

impl Width for W32 {
    const ENCODING: u8 = 0b10;
    const SIZE: usize = 4;
}

pub struct W64;

impl Width for W64 {
    const ENCODING: u8 = 0b11;
    const SIZE: usize = 8;
}

#[derive(Eq, PartialEq)]
pub struct WidthInfo {
    pub size: usize,
    pub flag: u8,
}
