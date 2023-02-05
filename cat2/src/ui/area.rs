use std::{io::Stdout, ops};

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

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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
