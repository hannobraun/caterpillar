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
        line: u16,
        num_columns: u16,
        stdout: &mut Stdout,
    ) -> anyhow::Result<()> {
        let mut column = num_columns - cells::NUM_CELLS as u16 - 2;

        print_vertical_border(column, line, stdout)?;
        column += 1;

        for &cell in &self.cells {
            let content = if cell { "#" } else { " " };
            stdout
                .queue(cursor::MoveTo(column, line))?
                .queue(style::PrintStyledContent(content.stylize()))?;
            column += 1;
        }

        print_vertical_border(column, line, stdout)?;

        Ok(())
    }
}

fn print_vertical_border(
    column: u16,
    line: u16,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    stdout
        .queue(cursor::MoveTo(column, line))?
        .queue(style::PrintStyledContent("┃".stylize()))?;
    Ok(())
}
