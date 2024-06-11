use std::collections::VecDeque;

#[derive(Default)]
pub struct RuntimeState {
    pub input: Input,
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}
