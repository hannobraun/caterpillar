use std::iter;

use crossterm::terminal;

use crate::cells;

pub struct Line {
    pub inner: [bool; cells::NUM_CELLS],
}

impl Line {
    pub fn init() -> Self {
        let inner = cells::init();
        Self { inner }
    }

    pub fn empty() -> Self {
        let inner = [false; cells::NUM_CELLS];
        Self { inner }
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let (num_columns, _) = terminal::size()?;
        iter::repeat(' ')
            .take(num_columns as usize - cells::NUM_CELLS - 2)
            .for_each(|c| print!("{c}"));

        print!("┃");
        for &cell in &self.inner {
            if cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("┃");

        Ok(())
    }
}
