use capi_compiler::{
    repr::{
        fragments::FragmentPayload,
        syntax::{self, ExpressionKind},
    },
    source_map::SourceMap,
};
use capi_process::Process;

use super::Fragment;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub expressions: Vec<Fragment>,
}

impl Function {
    pub fn new(
        function: syntax::Function,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        let expressions = function
            .expressions
            .into_iter()
            .map(|expression| {
                let payload = match expression.kind {
                    ExpressionKind::Binding { names } => {
                        FragmentPayload::Binding { names }
                    }
                    ExpressionKind::Comment { text } => {
                        FragmentPayload::Comment { text }
                    }
                    ExpressionKind::Value(value) => {
                        FragmentPayload::Value(value)
                    }
                    ExpressionKind::Word { name } => {
                        FragmentPayload::Word { name }
                    }
                };

                Fragment::new(expression.location, payload, source_map, process)
            })
            .collect();

        Self {
            name: function.name,
            expressions,
        }
    }
}
