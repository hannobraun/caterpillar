use std::iter;

use crate::{evaluator::Evaluator, world::World};

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

    pub fn draw(&mut self, world: &World, evaluator: &mut Evaluator) {
        for x in 0..world.cells.size[0] {
            for y in 0..world.cells.size[1] {
                let cell_x = x * world.cells.cell_size;
                let cell_y = y * world.cells.cell_size;

                let color = world.cells.buffer[x + y * world.cells.size[0]];

                self.draw_cell(
                    world.cells.cell_size,
                    cell_x,
                    cell_y,
                    color,
                    evaluator,
                );
            }
        }
    }

    fn draw_cell(
        &mut self,
        cell_size: usize,
        cell_x: usize,
        cell_y: usize,
        color: u8,
        evaluator: &mut Evaluator,
    ) {
        for x in 0..cell_size {
            for y in 0..cell_size {
                let pixel_x = cell_x + x;
                let pixel_y = cell_y + y;

                let index = (pixel_x + pixel_y * self.width)
                    * RenderTarget::NUM_COLOR_CHANNELS;

                let data = evaluator.evaluate([color]);
                assert_eq!(data[..4], [color, color, color, 255]);

                self.buffer[index + 0] = color;
                self.buffer[index + 1] = color;
                self.buffer[index + 2] = color;
                self.buffer[index + 3] = 255;
            }
        }
    }
}
