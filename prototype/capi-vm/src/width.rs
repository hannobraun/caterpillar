pub const W8: Width = Width { flag: 0b00 << 6 };
pub const W16: Width = Width { flag: 0b01 << 6 };
pub const W32: Width = Width { flag: 0b10 << 6 };
pub const W64: Width = Width { flag: 0b11 << 6 };

#[derive(Eq, PartialEq)]
pub struct Width {
    pub flag: u8,
}
