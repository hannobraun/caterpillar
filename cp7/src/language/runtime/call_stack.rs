use crate::language::syntax::SyntaxHandle;

#[derive(Debug)]
pub struct CallStack {
    frames: Vec<SyntaxHandle>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn current(&self) -> Option<SyntaxHandle> {
        self.frames.last().copied()
    }

    pub fn advance(&mut self, next: Option<SyntaxHandle>) {
        match next {
            Some(handle) => {
                self.update(handle);
            }
            None => {
                self.pop();
            }
        }
    }

    pub fn update(&mut self, next: SyntaxHandle) {
        match self.frames.last_mut() {
            Some(current) => *current = next,
            None => self.push(next),
        }
    }

    pub fn push(&mut self, next: SyntaxHandle) {
        self.frames.push(next);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}
