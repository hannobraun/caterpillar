use std::iter;

use crate::draw_target::DrawTarget;

pub struct Cells {
    pub buffer: Vec<u8>,
    pub cell_size: usize,
}

impl Cells {
    pub fn new(cell_size: usize, draw_target: &DrawTarget) -> Self {
        let width = draw_target.width / cell_size;
        let height = draw_target.height / cell_size;

        let buffer = iter::repeat(0).take(width * height).collect();

        Self { buffer, cell_size }
    }
}
