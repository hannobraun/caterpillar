use std::{
    collections::VecDeque,
    io::{Stdout, Write},
    iter,
};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};

use crate::cells;

pub struct Lines {
    inner: VecDeque<Line>,
}

impl Lines {
    pub fn new() -> Self {
        let inner = VecDeque::new();
        Self { inner }
    }

    pub fn current(&self) -> Line {
        self.inner.back().cloned().unwrap_or_else(Line::init)
    }

    pub fn push_next(&mut self, next: Line) {
        self.inner.push_back(next);
    }

    pub fn print(&mut self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let (num_columns, num_rows) = terminal::size()?;

        while self.inner.len() > num_rows as usize {
            self.inner.pop_front();
        }

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        let lines_width = cells::NUM_CELLS as u16 + 2;
        let x = num_columns - lines_width;
        let mut y = 0;

        print_top_border(x, &mut y, lines_width, stdout)?;

        for line in self
            .inner
            .iter()
            .cloned()
            .chain(iter::repeat_with(Line::empty))
            .take(num_rows as usize - 2)
        {
            line.print(x, &mut y, stdout)?;
        }

        print_bottom_border(x, &mut y, lines_width, stdout)?;

        stdout.flush()?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Line {
    cells: [bool; cells::NUM_CELLS],
}

impl Line {
    pub fn init() -> Self {
        let cells = cells::init();
        Self { cells }
    }

    pub fn empty() -> Self {
        let cells = [false; cells::NUM_CELLS];
        Self { cells }
    }

    pub fn from_cells(cells: [bool; cells::NUM_CELLS]) -> Self {
        Self { cells }
    }

    pub fn cells(&self) -> [bool; cells::NUM_CELLS] {
        self.cells
    }

    pub fn print(
        &self,
        mut x: u16,
        y: &mut u16,
        stdout: &mut Stdout,
    ) -> anyhow::Result<()> {
        print_vertical_border(&mut x, *y, stdout)?;
        print_cells(self.cells, &mut x, *y, stdout)?;
        print_vertical_border(&mut x, *y, stdout)?;

        *y += 1;

        Ok(())
    }
}

fn print_top_border(
    mut x: u16,
    y: &mut u16,
    width: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    print("┏", &mut x, *y, stdout)?;
    (0..width).try_for_each(|_| print("━", &mut x, *y, stdout))?;
    print("┓", &mut x, *y, stdout)?;

    *y += 1;

    Ok(())
}

fn print_bottom_border(
    mut x: u16,
    y: &mut u16,
    width: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    print("┗", &mut x, *y, stdout)?;
    (0..width).try_for_each(|_| print("━", &mut x, *y, stdout))?;
    print("┛", &mut x, *y, stdout)?;

    *y += 1;

    Ok(())
}

fn print_vertical_border(
    x: &mut u16,
    y: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    print("┃", x, y, stdout)
}

fn print_cells(
    cells: [bool; cells::NUM_CELLS],
    x: &mut u16,
    y: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    for cell in cells {
        print_cell(cell, x, y, stdout)?;
    }

    Ok(())
}

fn print_cell(
    cell: bool,
    x: &mut u16,
    y: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    let content = if cell { "#" } else { " " };
    print(content, x, y, stdout)?;
    Ok(())
}

fn print(
    s: &str,
    x: &mut u16,
    y: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    stdout
        .queue(cursor::MoveTo(*x, y))?
        .queue(style::PrintStyledContent(s.stylize()))?;

    let num_chars: u16 = s.chars().count().try_into().expect("String too long");
    *x += num_chars;

    Ok(())
}
