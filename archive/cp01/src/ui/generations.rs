use crate::cells::Generation;

use super::{
    area::{self, Area},
    border,
};

pub fn draw<'a>(
    area: Area,
    generations: impl DoubleEndedIterator<Item = &'a Generation>,
) {
    let mut area = border::draw(area);
    area::move_to_last_line(&mut area);

    let limit = area::size(&area).y;

    for generation in generations.rev().take(limit) {
        draw_generation(&mut area, generation)
    }
}

pub fn draw_generation(area: &mut Area, generation: &Generation) {
    for &cell in generation {
        draw_cell(area, cell);
    }

    area::move_to_previous_line(area);
}

fn draw_cell(area: &mut Area, cell: bool) {
    let content = if cell { "#" } else { " " };
    area::draw(area, content);
}
