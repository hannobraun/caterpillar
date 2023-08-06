use crate::syntax::SyntaxHandle;

pub struct CallStack {
    frames: Vec<SyntaxHandle>,
}

impl CallStack {
    pub fn new(start: Option<SyntaxHandle>) -> Self {
        Self {
            frames: start.into_iter().collect(),
        }
    }

    pub fn current(&self) -> Option<SyntaxHandle> {
        self.frames.last().copied()
    }

    pub fn update(&mut self, next: SyntaxHandle) {
        match self.frames.last_mut() {
            Some(current) => *current = next,
            None => self.frames.push(next),
        }
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}
