use std::{
    io::{Stdout, Write},
    iter,
};

use super::vector::Vector;

pub struct Buffer {
    chars: Vec<char>,
    size: Vector,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            chars: Vec::new(),
            size: Vector { x: 0, y: 0 },
        }
    }

    pub fn prepare(&mut self, size: Vector) {
        self.size = size;

        let size = (size.x * size.y) as usize;
        self.chars.clear();
        self.chars.extend(iter::repeat(' ').take(size));
    }

    pub fn write(&mut self, x: u16, y: u16, s: &str) {
        let mut x: usize = x.into();
        let y: usize = y.into();

        for ch in s.chars() {
            self.chars[y * self.size.x as usize + x] = ch;
            x += 1;
        }
    }

    pub fn print(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let mut lines = self.chars.chunks(self.size.x as usize).peekable();

        while let Some(line) = lines.next() {
            for ch in line {
                write!(stdout, "{ch}")?;
            }

            if lines.peek().is_some() {
                writeln!(stdout)?;
            }
        }

        Ok(())
    }
}
