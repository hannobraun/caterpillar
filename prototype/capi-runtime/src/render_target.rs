use std::iter;

pub struct RenderTarget {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl RenderTarget {
    pub const NUM_COLOR_CHANNELS: usize = 4;

    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height * Self::NUM_COLOR_CHANNELS;
        let buffer = iter::repeat(0).take(len).collect();

        Self {
            buffer,
            width,
            height,
        }
    }
}
