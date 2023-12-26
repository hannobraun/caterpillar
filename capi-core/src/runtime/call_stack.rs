use crate::repr::eval::fragments::FragmentId;

#[derive(Debug, Default)]
pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<FragmentId> {
        self.frames.last().copied().map(|stack_frame| {
            let StackFrame::Fragment(fragment_id) = stack_frame;
            fragment_id
        })
    }

    pub fn advance(&mut self, next: Option<FragmentId>) {
        if let Some(current) = self.frames.last_mut() {
            match next {
                Some(next) => {
                    *current = StackFrame::Fragment(next);
                }
                None => {
                    self.frames.pop();
                }
            }
        }
    }

    pub fn push(&mut self, next: FragmentId) {
        self.frames.push(StackFrame::Fragment(next));
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for frame in &mut self.frames {
            if *frame == StackFrame::Fragment(old) {
                *frame = StackFrame::Fragment(new);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackFrame {
    Fragment(FragmentId),
}
