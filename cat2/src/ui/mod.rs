mod area;
mod border;
mod buffer;
mod generations;
mod vector;

pub use self::buffer::Buffer;

use std::{collections::VecDeque, io::Stdout, iter};

use crossterm::terminal;

use crate::cells::{self, NUM_CELLS};

use self::vector::Vector;

pub struct Lines {
    inner: VecDeque<[bool; NUM_CELLS]>,
}

impl Lines {
    pub fn new() -> Self {
        let inner = VecDeque::new();
        Self { inner }
    }

    pub fn current(&self) -> [bool; NUM_CELLS] {
        self.inner.back().cloned().unwrap_or_else(cells::init)
    }

    pub fn push_next(&mut self, next: [bool; NUM_CELLS]) {
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
            .chain(iter::repeat_with(|| [false; NUM_CELLS]))
            .take(lines_height)
        {
            generations::write_generation(&mut area, line);
        }

        buffer.draw(stdout)?;

        Ok(())
    }
}
