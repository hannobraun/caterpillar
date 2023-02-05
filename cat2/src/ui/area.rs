use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

pub struct Area<'a> {
    pub out: &'a mut Stdout,
}

pub fn move_cursor(area: &mut Area, x: u16, y: u16) -> anyhow::Result<()> {
    area.out.queue(cursor::MoveTo(x, y))?;
    Ok(())
}

pub fn write(
    area: &mut Area,
    x: &mut u16,
    y: u16,
    s: &str,
) -> anyhow::Result<()> {
    move_cursor(area, *x, y)?;
    area.out.queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    *x += num_chars;

    Ok(())
}
