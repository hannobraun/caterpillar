use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

pub struct Area<'a> {
    out: &'a mut Stdout,
    pub cursor: [u16; 2],
}

pub fn new(out: &mut Stdout) -> Area {
    Area {
        out,
        cursor: [0; 2],
    }
}

pub fn move_cursor(area: &mut Area, x: u16, y: u16) {
    area.cursor = [x, y];
}

pub fn new_line(area: &mut Area) {
    let [x, y] = &mut area.cursor;

    *x = 0;
    *y += 1;
}

pub fn write(area: &mut Area, s: &str) -> anyhow::Result<()> {
    let [x, y] = &mut area.cursor;

    area.out
        .queue(cursor::MoveTo(*x, *y))?
        .queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    *x += num_chars;

    Ok(())
}
