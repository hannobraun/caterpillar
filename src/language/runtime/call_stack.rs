use crate::language::repr::eval::fragments::FragmentId;

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
                    self.frames.pop();
                }
            }
        }
    }

    pub fn push(&mut self, next: FragmentId) {
        self.frames.push(next);
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for frame in &mut self.frames {
            if *frame == old {
                *frame = new;
            }
        }
    }
}
