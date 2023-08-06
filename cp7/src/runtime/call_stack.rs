use crate::syntax::SyntaxHandle;

pub struct CallStack {
    pub current: Option<SyntaxHandle>,
}

impl CallStack {
    pub fn new(start: Option<SyntaxHandle>) -> Self {
        Self { current: start }
    }
}
