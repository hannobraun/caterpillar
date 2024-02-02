use crate::{
    builtins::core::CorePlatform, platform::BuiltinFn,
    repr::eval::fragments::FragmentId,
};

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

    pub fn push(&mut self, next: StackFrame) {
        self.frames.push(next);
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
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
        word: FragmentId,
        function: BuiltinFn<CorePlatform>,
        step: usize,
    },
}
