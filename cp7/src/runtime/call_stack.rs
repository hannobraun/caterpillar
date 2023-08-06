use crate::syntax::SyntaxHandle;

pub struct CallStack {
    pub current: Option<SyntaxHandle>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { current: None }
    }
}
