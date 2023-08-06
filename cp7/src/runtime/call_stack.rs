use crate::syntax::SyntaxHandle;

pub struct CallStack {
    current: Option<SyntaxHandle>,
}

impl CallStack {
    pub fn new(start: Option<SyntaxHandle>) -> Self {
        Self { current: start }
    }

    pub fn current(&self) -> Option<SyntaxHandle> {
        self.current
    }

    pub fn update(&mut self, next: Option<SyntaxHandle>) {
        self.current = next;
    }
}
