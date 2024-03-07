use std::iter;

pub struct DrawTarget {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl DrawTarget {
    pub fn new(width: usize, height: usize) -> Self {
        const NUM_COLOR_CHANNELS: usize = 4;
        let len = width * height * NUM_COLOR_CHANNELS;

        let buffer = iter::repeat(0).take(len).collect();

        Self {
            buffer,
            width,
            height,
        }
    }
}
