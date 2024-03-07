use std::iter;

pub struct RenderTarget {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        let buffer = iter::repeat(0).take(len).collect();

        Self {
            buffer,
            width,
            height,
        }
    }
}
