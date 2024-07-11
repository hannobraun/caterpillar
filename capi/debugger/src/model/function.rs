use capi_compiler::{
    repr::fragments::{self, Fragments},
    source_map::SourceMap,
};
use capi_process::Process;

use super::Expression;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub fragments: Vec<Expression>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        let mut fragment_models = Vec::new();

        if let Some(start) = function.start {
            fragment_models.extend(
                fragments.inner.iter_from(start).cloned().map(|fragment| {
                    Expression::new(fragment, source_map, process)
                }),
            );
        }

        Self {
            name: function.name,
            fragments: fragment_models,
        }
    }
}
