pub enum Value {
    Color(Color),
    U8(u8),
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
