use crate::cells::Generation;

use super::{
    area::{self, Area},
    border,
};

pub fn draw(area: Area, generations: impl Iterator<Item = Generation>) {
    let mut area = border::draw(area);

    for generation in generations {
        draw_generation(&mut area, generation)
    }
}

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
