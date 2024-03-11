pub trait Width {
    const ENCODING: u8;
    const NUM_BYTES: usize;

    const FLAG: u8 = Self::ENCODING << 6;
    const INFO: WidthInfo = WidthInfo {
        size: Self::NUM_BYTES,
        flag: Self::FLAG,
    };
}

pub struct W8;

impl Width for W8 {
    const ENCODING: u8 = 0b00;
    const NUM_BYTES: usize = 1;
}

pub struct W16;

impl Width for W16 {
    const ENCODING: u8 = 0b01;
    const NUM_BYTES: usize = 2;
}

pub struct W32;

impl Width for W32 {
    const ENCODING: u8 = 0b10;
    const NUM_BYTES: usize = 4;
}

pub struct W64;

impl Width for W64 {
    const ENCODING: u8 = 0b11;
    const NUM_BYTES: usize = 8;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WidthInfo {
    pub size: usize,
    pub flag: u8,
}
