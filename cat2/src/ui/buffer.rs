use std::{io::Stdout, iter};

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
    size: Vector,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            previous: Vec::new(),
            current: Vec::new(),

            previous_size: Vector { x: 0, y: 0 },
            size: Vector { x: 0, y: 0 },
        }
    }

    pub fn prepare(&mut self, size: Vector) {
        self.previous_size = self.size;
        self.size = size;

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
            self.current[y * self.size.x as usize + x] = ch;
            x += 1;
        }
    }

    pub fn print(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for (y, line) in self.current.chunks(self.size.x as usize).enumerate() {
            for (x, ch) in line.iter().enumerate() {
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::PrintStyledContent(ch.stylize()))?;
            }
        }

        Ok(())
    }
}
