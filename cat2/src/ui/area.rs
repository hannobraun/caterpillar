use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

pub struct Area<'a> {
    out: &'a mut Stdout,
}

pub fn new(out: &mut Stdout) -> Area {
    Area { out }
}

pub fn write(
    area: &mut Area,
    x: &mut u16,
    y: u16,
    s: &str,
) -> anyhow::Result<()> {
    area.out
        .queue(cursor::MoveTo(*x, y))?
        .queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    *x += num_chars;

    Ok(())
}
