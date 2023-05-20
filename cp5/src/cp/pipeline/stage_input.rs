use std::collections::VecDeque;

#[derive(Debug)]
pub struct StageInput<T> {
    pub elements: VecDeque<T>,
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
}
