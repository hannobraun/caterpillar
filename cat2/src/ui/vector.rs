use std::ops;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: u16,
    pub y: u16,
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
