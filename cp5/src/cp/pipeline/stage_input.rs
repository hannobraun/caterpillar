use std::collections::VecDeque;

#[derive(Debug)]
pub struct StageInput<T> {
    elements: VecDeque<T>,
}

impl<T> StageInput<T> {
    pub fn new() -> Self {
        Self {
            elements: VecDeque::new(),
        }
    }

    pub fn add(&mut self, element: T) {
        self.elements.push_back(element)
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn next(&mut self) -> Option<T> {
        self.elements.pop_front()
    }
}
