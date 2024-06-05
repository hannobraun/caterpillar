use core::slice;
use std::iter;

use crate::{
    platform::{core::CorePlatform, BuiltinFn},
    repr::eval::fragments::FragmentId,
};

#[derive(Clone, Debug, Default)]
pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.frames.clear()
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

    pub fn iter(&self) -> impl Iterator<Item = &StackFrame> + '_ {
        self.into_iter()
    }
}

impl<'r> IntoIterator for &'r CallStack {
    type Item = &'r StackFrame;
    type IntoIter = iter::Rev<slice::Iter<'r, StackFrame>>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.iter().rev()
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
