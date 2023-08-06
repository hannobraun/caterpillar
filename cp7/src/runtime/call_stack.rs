use crate::syntax::SyntaxHandle;

pub struct CallStack {
    current: Vec<SyntaxHandle>,
}

impl CallStack {
    pub fn new(start: Option<SyntaxHandle>) -> Self {
        Self {
            current: start.into_iter().collect(),
        }
    }

    pub fn current(&self) -> Option<SyntaxHandle> {
        self.current.last().copied()
    }

    pub fn update(&mut self, next: Option<SyntaxHandle>) {
        match next {
            Some(next) => match self.current.last_mut() {
                Some(current) => *current = next,
                None => self.current.push(next),
            },
            None => {
                self.current.pop();
            }
        }
    }
}
