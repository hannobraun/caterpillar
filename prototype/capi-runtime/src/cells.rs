use std::iter;

use crate::draw_target::DrawTarget;

pub struct Cells {
    pub buffer: Vec<u8>,
}

impl Cells {
    pub fn new(cell_size: usize, target: &DrawTarget) -> Self {
        let width = target.width / cell_size;
        let height = target.height / cell_size;

        Self {
            buffer: iter::repeat(0).take(width * height).collect(),
        }
    }
}
