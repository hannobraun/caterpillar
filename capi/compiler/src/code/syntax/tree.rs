use crate::code::{IndexMap, NamedFunction};

#[derive(Default)]
pub struct SyntaxTree {
    pub named_functions: IndexMap<NamedFunction>,
}
