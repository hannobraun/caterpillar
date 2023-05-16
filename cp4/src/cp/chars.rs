use std::collections::VecDeque;

pub struct Chars {
    pub inner: VecDeque<char>,
}

impl Chars {
    pub async fn next(&mut self) -> Option<char> {
        self.inner.pop_front()
    }
}
