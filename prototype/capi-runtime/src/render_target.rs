use crate::{vm::Evaluator, world::World};

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
}

impl RenderTarget {
    pub const NUM_COLOR_CHANNELS: usize = 4;

    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn buffer_size(&self) -> usize {
        self.width * self.height * Self::NUM_COLOR_CHANNELS
    }

    pub fn draw(
        &mut self,
        world: &World,
        evaluator: &mut Evaluator,
        code: &[u8],
        data: &mut [u8],
    ) {
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
                    code,
                    data,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)] // it's due for a rewrite anyway
    fn draw_cell(
        &mut self,
        cell_size: usize,
        cell_x: usize,
        cell_y: usize,
        color: u8,
        evaluator: &mut Evaluator,
        code: &[u8],
        data: &mut [u8],
    ) {
        for x in 0..cell_size {
            for y in 0..cell_size {
                let pixel_x = cell_x + x;
                let pixel_y = cell_y + y;

                let index = (pixel_x + pixel_y * self.width)
                    * RenderTarget::NUM_COLOR_CHANNELS;
                let index: u32 = index
                    .try_into()
                    .expect("Expected to run on 32-bit platform (WebAssembly)");

                evaluator
                    .push(index, data)
                    .push(color.into(), data)
                    .evaluate(code, data)
                    .unwrap();
            }
        }
    }
}
