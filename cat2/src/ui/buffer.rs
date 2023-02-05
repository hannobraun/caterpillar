use std::{
    io::{Stdout, Write},
    iter,
};

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

use super::vector::Vector;

pub struct Buffer {
    previous: Vec<char>,
    current: Vec<char>,

    previous_size: Vector,
    current_size: Vector,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            previous: Vec::new(),
            current: Vec::new(),

            previous_size: Vector { x: 0, y: 0 },
            current_size: Vector { x: 0, y: 0 },
        }
    }

    pub fn prepare(&mut self, size: Vector) {
        self.previous_size = self.current_size;
        self.current_size = size;

        self.previous.clear();
        self.previous.extend(self.current.iter().cloned());

        let size = (size.x * size.y) as usize;
        self.current.clear();
        self.current.extend(iter::repeat(' ').take(size));
    }

    pub fn write(&mut self, x: u16, y: u16, s: &str) {
        let mut x: usize = x.into();
        let y: usize = y.into();

        for ch in s.chars() {
            let index = self.index(x, y);
            self.current[index] = ch;
            x += 1;
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for (y, line) in self
            .current
            .chunks(self.current_size.x as usize)
            .enumerate()
        {
            for (x, &ch) in line.iter().enumerate() {
                let index = self.index(x, y);
                if self.previous_size != self.current_size
                    || self.previous[index] != ch
                {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))?
                        .queue(style::PrintStyledContent(ch.stylize()))?;
                }
            }
        }

        stdout.flush()?;

        Ok(())
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.current_size.x as usize + x
    }
}
