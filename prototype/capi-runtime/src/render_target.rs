use std::iter;

use crate::world::World;

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

pub fn draw(world: &World, render_target: &mut RenderTarget) {
    for x in 0..world.cells.size[0] {
        for y in 0..world.cells.size[1] {
            let cell_x = x * world.cells.cell_size;
            let cell_y = y * world.cells.cell_size;

            let color = world.cells.buffer[x + y * world.cells.size[0]];

            draw_cell(
                world.cells.cell_size,
                cell_x,
                cell_y,
                color,
                render_target,
            );
        }
    }
}

fn draw_cell(
    cell_size: usize,
    cell_x: usize,
    cell_y: usize,
    color: u8,
    target: &mut RenderTarget,
) {
    for x in 0..cell_size {
        for y in 0..cell_size {
            let pixel_x = cell_x + x;
            let pixel_y = cell_y + y;

            let index = (pixel_x + pixel_y * target.width)
                * RenderTarget::NUM_COLOR_CHANNELS;

            target.buffer[index + 0] = color;
            target.buffer[index + 1] = color;
            target.buffer[index + 2] = color;
            target.buffer[index + 3] = 255;
        }
    }
}
