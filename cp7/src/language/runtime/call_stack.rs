use crate::language::repr::syntax::FragmentId;

#[derive(Debug)]
pub struct CallStack {
    frames: Vec<FragmentId>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn current(&self) -> Option<FragmentId> {
        self.frames.last().copied()
    }

    pub fn advance(&mut self, next: Option<FragmentId>) {
        if let Some(current) = self.frames.last_mut() {
            match next {
                Some(next) => {
                    *current = next;
                }
                None => {
                    self.pop();
                }
            }
        }
    }

    pub fn push(&mut self, next: FragmentId) {
        self.frames.push(next);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}
