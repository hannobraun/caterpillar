use std::{
    io::{Stdout, Write},
    iter,
};

use super::vector::Vector;

pub struct Buffer {
    chars: Vec<char>,
    width: usize,
}

impl Buffer {
    pub fn new(size: Vector) -> Self {
        let width = size.x.into();

        let size = (size.x * size.y) as usize;
        let chars = iter::repeat(' ').take(size).collect();

        Self { chars, width }
    }

    pub fn write(&mut self, x: u16, y: u16, s: &str) {
        let mut x: usize = x.into();
        let y: usize = y.into();

        for ch in s.chars() {
            self.chars[y * self.width + x] = ch;
            x += 1;
        }
    }

    pub fn print(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let mut lines = self.chars.chunks(self.width).peekable();

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
