use std::{
    io::{Stdout, Write},
    iter,
};

use super::vector::Vector;

pub struct Buffer {
    previous: Vec<char>,
    current: Vec<char>,
    size: Vector,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            previous: Vec::new(),
            current: Vec::new(),
            size: Vector { x: 0, y: 0 },
        }
    }

    pub fn prepare(&mut self, size: Vector) {
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
        let mut lines = self.current.chunks(self.size.x as usize).peekable();

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
