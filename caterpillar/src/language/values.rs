pub enum Value {
    U8(u8),
    Color(Color),
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
