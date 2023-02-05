mod area;
mod border;
mod buffer;
mod vector;

pub use self::buffer::Buffer;

use std::{collections::VecDeque, io::Stdout, iter};

use crossterm::terminal;

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

    pub fn print(
        &mut self,
        buffer: &mut Buffer,
        stdout: &mut Stdout,
    ) -> anyhow::Result<()> {
        let (num_columns, num_rows) = terminal::size()?;
        buffer.prepare(Vector {
            x: num_columns,
            y: num_rows,
        });

        let lines_width = cells::NUM_CELLS as u16 + 2;
        let lines_height = num_rows as usize - 2;

        while self.inner.len() > lines_height {
            self.inner.pop_front();
        }

        let offset = Vector {
            x: num_columns - lines_width,
            y: 0,
        };
        let size = Vector {
            x: lines_width,
            y: num_rows,
        };
        let area = area::new(buffer, offset, size);

        let mut area = border::draw(area);

        for line in self
            .inner
            .iter()
            .cloned()
            .chain(iter::repeat_with(Line::empty))
            .take(lines_height)
        {
            print_cells(&mut area, line.cells)?;
        }

        buffer.draw(stdout)?;

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
}

fn print_cells(
    area: &mut area::Area,
    cells: [bool; cells::NUM_CELLS],
) -> anyhow::Result<()> {
    for cell in cells {
        print_cell(area, cell)?;
    }

    area::move_to_new_line(area);

    Ok(())
}

fn print_cell(area: &mut area::Area, cell: bool) -> anyhow::Result<()> {
    let content = if cell { "#" } else { " " };
    area::draw(area, content);
    Ok(())
}
