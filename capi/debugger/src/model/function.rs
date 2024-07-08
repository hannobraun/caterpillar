use capi_compiler::{
    repr::{
        fragments::FragmentPayload,
        syntax::{self, ExpressionKind},
    },
    source_map::SourceMap2,
};
use capi_process::Process;

use super::Fragment;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub fragments: Vec<Fragment>,
}

impl Function {
    pub fn new(
        function: syntax::Function,
        source_map: &SourceMap2,
        process: &Process,
    ) -> Self {
        let fragments = function
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
            fragments,
        }
    }
}
