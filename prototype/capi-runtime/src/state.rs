use std::{collections::VecDeque, iter};

use crate::{cells::Cells, ffi_out::random};

pub struct State {
    pub positions: VecDeque<[i32; 2]>,
    pub food_pos: [i32; 2],
}

impl State {
    pub fn new(cells: &Cells) -> Self {
        let x = cells.size[0] as i32 / 2;
        let y = cells.size[1] as i32 / 2;

        let mut self_ = Self {
            positions: iter::once([x, y]).collect(),
            food_pos: [0, 0],
        };

        self_.randomize_food_pos(cells);

        self_
    }

    pub fn randomize_food_pos(&mut self, cells: &Cells) {
        self.food_pos =
            cells.size.map(|dim| (random() * dim as f32).floor() as i32);
    }

    pub fn body_positions(&self) -> impl Iterator<Item = [i32; 2]> + '_ {
        self.positions.iter().copied().skip(1)
    }
}
