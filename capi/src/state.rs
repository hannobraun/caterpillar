use std::collections::VecDeque;

#[derive(Default)]
pub struct RuntimeState {
    pub input: VecDeque<u8>,
}
