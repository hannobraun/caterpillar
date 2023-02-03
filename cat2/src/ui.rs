use std::iter;

use crossterm::terminal;

use crate::cells;

#[derive(Clone)]
pub struct Line {
    pub cells: [bool; cells::NUM_CELLS],
}

impl Line {
    pub fn init() -> Self {
        let cells = cells::init();
        Self { cells }
    }

    pub fn from_cells(cells: [bool; cells::NUM_CELLS]) -> Self {
        Self { cells }
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let (num_columns, _) = terminal::size()?;
        iter::repeat(' ')
            .take(num_columns as usize - cells::NUM_CELLS - 2)
            .for_each(|c| print!("{c}"));

        print!("┃");
        for &cell in &self.cells {
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
