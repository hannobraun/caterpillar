pub trait Argument<const N: usize> {
    fn to_bytes(self) -> [u8; N];
}

impl Argument<1> for u8 {
    fn to_bytes(self) -> [u8; 1] {
        [self]
    }
}

impl Argument<4> for u32 {
    fn to_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
