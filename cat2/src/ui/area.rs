use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

use super::vector::Vector;

pub struct Area<'a> {
    out: &'a mut Stdout,
    offset: Vector,
    size: Vector,
    cursor: Vector,
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
    area.cursor.x = 0;
    area.cursor.y += 1;
}

pub fn write(area: &mut Area, s: &str) -> anyhow::Result<()> {
    let Vector { x, y } = area.offset + area.cursor;

    area.out
        .queue(cursor::MoveTo(x, y))?
        .queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    area.cursor.x += num_chars;

    Ok(())
}
