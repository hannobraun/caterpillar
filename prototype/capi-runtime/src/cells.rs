use std::iter;

use crate::render_target::RenderTarget;

pub struct Cells {
    pub buffer: Vec<u8>,
    pub cell_size: usize,
    pub size: [usize; 2],
}

impl Cells {
    pub fn new(draw_target: &RenderTarget) -> Self {
        let cell_size = 32;

        let width = draw_target.width / cell_size;
        let height = draw_target.height / cell_size;

        let buffer = iter::repeat(0).take(width * height).collect();

        Self {
            buffer,
            cell_size,
            size: [width, height],
        }
    }
}
