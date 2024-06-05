use std::{
    cmp::min,
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

    cursor: Vector,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            previous: Vec::new(),
            current: Vec::new(),

            previous_size: Vector { x: 0, y: 0 },
            current_size: Vector { x: 0, y: 0 },

            cursor: Vector { x: 0, y: 0 },
        }
    }

    pub fn prepare(&mut self, size: Vector) {
        self.previous_size = self.current_size;
        self.current_size = size;

        self.previous.clear();
        self.previous.extend(self.current.iter().cloned());

        let size = size.x * size.y;
        self.current.clear();
        self.current.extend(iter::repeat(' ').take(size));
    }

    pub fn move_cursor(&mut self, position: Vector) {
        self.cursor = position;
    }

    pub fn write(&mut self, s: &str) {
        for ch in s.chars() {
            let index = self.index(self.cursor.x, self.cursor.y);
            self.current[index] = ch;
            self.cursor.x += 1;
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for (y, line) in self.current.chunks(self.current_size.x).enumerate() {
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

        stdout.queue(cursor::MoveTo(
            self.cursor.x as u16,
            self.cursor.y as u16,
        ))?;

        stdout.flush()?;

        Ok(())
    }

    fn index(&self, x: usize, y: usize) -> usize {
        let x = min(x, self.current_size.x - 1);
        let y = min(y, self.current_size.y - 1);

        y * self.current_size.x + x
    }
}
