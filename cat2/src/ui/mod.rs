mod area;
mod border;
mod vector;

use std::{
    collections::VecDeque,
    io::{Stdout, Write},
    iter,
};

use crossterm::{terminal, QueueableCommand};

use crate::cells;

use self::vector::Vector;

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

        let lines_width = cells::NUM_CELLS as u16 + 2;
        let lines_height = num_rows as usize - 2;

        while self.inner.len() > lines_height {
            self.inner.pop_front();
        }

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        let offset = Vector {
            x: num_columns - lines_width,
            y: 0,
        };
        let size = Vector {
            x: lines_width,
            y: num_rows,
        };
        let area = area::new(stdout, offset, size);

        let mut area = border::write(area)?;

        for line in self
            .inner
            .iter()
            .cloned()
            .chain(iter::repeat_with(Line::empty))
            .take(lines_height)
        {
            line.print(&mut area)?;
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

    pub fn print(&self, area: &mut area::Area) -> anyhow::Result<()> {
        print_cells(area, self.cells)?;
        area::move_to_new_line(area);

        Ok(())
    }
}

fn print_cells(
    area: &mut area::Area,
    cells: [bool; cells::NUM_CELLS],
) -> anyhow::Result<()> {
    for cell in cells {
        print_cell(area, cell)?;
    }

    Ok(())
}

fn print_cell(area: &mut area::Area, cell: bool) -> anyhow::Result<()> {
    let content = if cell { "#" } else { " " };
    area::write(area, content)
}
