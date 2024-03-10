pub trait Word<const N: usize> {
    fn to_bytes(self) -> [u8; N];
}

impl Word<1> for u8 {
    fn to_bytes(self) -> [u8; 1] {
        [self]
    }
}

impl Word<4> for u32 {
    fn to_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
