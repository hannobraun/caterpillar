use crate::repr::eval::fragments::FragmentId;

use super::namespaces::IntrinsicFunction;

#[derive(Debug, Default)]
pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<StackFrame> {
        self.frames.last().copied()
    }

    pub fn advance(&mut self, next: Option<FragmentId>) {
        if let Some(current) = self.frames.last_mut() {
            match next {
                Some(next) => {
                    *current = StackFrame::Fragment { fragment_id: next };
                }
                None => {
                    self.frames.pop();
                }
            }
        }
    }

    pub fn push(&mut self, next: FragmentId) {
        self.frames.push(StackFrame::Fragment { fragment_id: next });
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for frame in &mut self.frames {
            if *frame == (StackFrame::Fragment { fragment_id: old }) {
                *frame = StackFrame::Fragment { fragment_id: new };
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackFrame {
    Fragment {
        fragment_id: FragmentId,
    },
    IntrinsicFunction {
        function: IntrinsicFunction,
        step: usize,
    },
}
