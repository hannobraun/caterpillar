use std::collections::VecDeque;

pub struct Input {
    pub events: VecDeque<InputEvent>,
}

#[derive(Eq, PartialEq)]
#[repr(i32)]
pub enum InputEvent {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}
