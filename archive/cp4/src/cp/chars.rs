use std::collections::VecDeque;

pub struct Chars {
    inner: VecDeque<char>,
}

impl Chars {
    pub fn new(code: &str) -> Self {
        Self {
            inner: code.chars().collect(),
        }
    }

    pub async fn next(&mut self) -> Option<char> {
        self.inner.pop_front()
    }
}
