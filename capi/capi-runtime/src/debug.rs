use crate::{syntax::SyntaxElement, Functions};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugState {
    pub functions: Vec<DebugFunction>,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        let functions = functions
            .inner
            .into_iter()
            .map(|function| DebugFunction {
                name: function.name,
                syntax: function.syntax.into_iter().map(Into::into).collect(),
            })
            .collect();

        Self { functions }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugFunction {
    pub name: String,
    pub syntax: Vec<DebugSyntaxElement>,
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
