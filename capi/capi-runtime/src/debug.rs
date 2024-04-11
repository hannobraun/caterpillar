use crate::{syntax::SyntaxElement, Function, Functions};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct DebugState {
    pub functions: Vec<DebugFunction>,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        let functions = functions.inner.into_iter().map(Into::into).collect();
        Self { functions }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugFunction {
    pub name: String,
    pub syntax: Vec<DebugSyntaxElement>,
}

impl From<Function> for DebugFunction {
    fn from(function: Function) -> Self {
        Self {
            name: function.name,
            syntax: function.syntax.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugSyntaxElement {
    pub inner: SyntaxElement,
    pub breakpoint: bool,
}

impl From<SyntaxElement> for DebugSyntaxElement {
    fn from(syntax_element: SyntaxElement) -> Self {
        Self {
            inner: syntax_element,
            breakpoint: false,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub enum DebugEvent {
    ToggleBreakpoint,
}
