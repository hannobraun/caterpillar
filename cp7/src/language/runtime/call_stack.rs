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
        if let Some(current) = self.frames.last_mut() {
            match next {
                Some(handle) => {
                    *current = handle;
                }
                None => {
                    self.pop();
                }
            }
        }
    }

    pub fn push(&mut self, next: SyntaxHandle) {
        self.frames.push(next);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}
