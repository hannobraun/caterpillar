use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

pub struct Area<'a> {
    out: &'a mut Stdout,
    offset: Vector,
    size: Vector,
    cursor: Vector,
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: u16,
    pub y: u16,
}

pub fn new(out: &mut Stdout, offset: Vector, size: Vector) -> Area {
    Area {
        out,
        offset,
        size,
        cursor: Vector { x: 0, y: 0 },
    }
}

pub fn size(area: &Area) -> Vector {
    area.size
}

pub fn new_line(area: &mut Area) {
    let Vector { x, y } = &mut area.cursor;

    *x = 0;
    *y += 1;
}

pub fn write(area: &mut Area, s: &str) -> anyhow::Result<()> {
    let Vector { x, y } = &mut area.cursor;

    area.out
        .queue(cursor::MoveTo(area.offset.x + *x, area.offset.y + *y))?
        .queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    *x += num_chars;

    Ok(())
}
