use std::{collections::VecDeque, iter};

use crate::{cells::Cells, ffi_out::random};

pub struct State {
    pub positions: VecDeque<[i32; 2]>,
    pub velocity: [i32; 2],
    pub food_pos: [i32; 2],
    pub growth_left: i32,
    pub time_since_last_update_ms: f64,
    pub lost: bool,
    pub cells: Cells,
}

impl State {
    pub fn new(cells: Cells) -> Self {
        let x = cells.size[0] as i32 / 2;
        let y = cells.size[1] as i32 / 2;

        let mut self_ = Self {
            positions: iter::once([x, y]).collect(),
            velocity: [1, 0],
            food_pos: [0, 0],
            growth_left: 2,
            time_since_last_update_ms: 0.,
            lost: false,
            cells,
        };

        self_.randomize_food_pos();

        self_
    }

    pub fn randomize_food_pos(&mut self) {
        self.food_pos = self
            .cells
            .size
            .map(|dim| (random() * dim as f32).floor() as i32);
    }

    pub fn head_position(&self) -> [i32; 2] {
        self.positions[0]
    }

    pub fn body_positions(&self) -> impl Iterator<Item = [i32; 2]> + '_ {
        self.positions.iter().copied().skip(1)
    }
}
