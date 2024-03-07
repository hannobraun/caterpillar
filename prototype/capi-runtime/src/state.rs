use std::{collections::VecDeque, iter};

use crate::cells::Cells;

pub struct State {
    pub positions: VecDeque<[i32; 2]>,
}

impl State {
    pub fn new(cells: &Cells) -> Self {
        let x = cells.size[0] as i32 / 2;
        let y = cells.size[1] as i32 / 2;

        Self {
            positions: iter::once([x, y]).collect(),
        }
    }
}
