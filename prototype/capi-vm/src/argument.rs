pub trait Argument<const N: usize> {
    fn to_bytes(self) -> [u8; N];
}

impl Argument<1> for u8 {
    fn to_bytes(self) -> [u8; 1] {
        [self]
    }
}
