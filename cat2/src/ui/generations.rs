use crate::cells::Generation;

use super::area::{self, Area};

pub fn draw_generation(area: &mut Area, generation: Generation) {
    for cell in generation {
        draw_cell(area, cell);
    }

    area::move_to_new_line(area);
}

fn draw_cell(area: &mut Area, cell: bool) {
    let content = if cell { "#" } else { " " };
    area::draw(area, content);
}
