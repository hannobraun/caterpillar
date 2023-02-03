use std::{
    collections::VecDeque,
    io::{Stdout, Write},
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

        for (i, line) in self.inner.iter().enumerate() {
            line.print(i as u16, num_columns, stdout)?;
        }

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

    pub fn from_cells(cells: [bool; cells::NUM_CELLS]) -> Self {
        Self { cells }
    }

    pub fn cells(&self) -> [bool; cells::NUM_CELLS] {
        self.cells
    }

    pub fn print(
        &self,
        y: u16,
        num_columns: u16,
        stdout: &mut Stdout,
    ) -> anyhow::Result<()> {
        let mut x = num_columns - cells::NUM_CELLS as u16 - 2;

        print_vertical_border(&mut x, y, stdout)?;

        for &cell in &self.cells {
            let content = if cell { "#" } else { " " };
            print(content, &mut x, y, stdout)?;
        }

        print_vertical_border(&mut x, y, stdout)?;

        Ok(())
    }
}

fn print_vertical_border(
    x: &mut u16,
    y: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    print("┃", x, y, stdout)
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
