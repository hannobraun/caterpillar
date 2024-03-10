pub const W8: Width = Width {
    size: 1,
    flag: encoding::W8 << 6,
};
pub const W16: Width = Width {
    size: 2,
    flag: encoding::W16 << 6,
};
pub const W32: Width = Width {
    size: 4,
    flag: encoding::W32 << 6,
};
pub const W64: Width = Width {
    size: 8,
    flag: encoding::W64 << 6,
};

#[derive(Eq, PartialEq)]
pub struct Width {
    pub size: usize,
    pub flag: u8,
}

pub mod encoding {
    pub const W8: u8 = 0b00;
    pub const W16: u8 = 0b01;
    pub const W32: u8 = 0b10;
    pub const W64: u8 = 0b11;
}
