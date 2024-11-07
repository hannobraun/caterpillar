use anyhow::anyhow;
use capi_compiler::{
    code::{
        Branch, BranchLocation, Cluster, ExpressionLocation, NamedFunctions,
        Pattern, Types,
    },
    source_map::SourceMap,
};
use capi_runtime::Effect;

use super::{Breakpoints, DebugExpression};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugBranch {
    pub parameters: Vec<String>,
    pub body: Vec<DebugExpression>,
    pub is_active: bool,
}

impl DebugBranch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        branch: Branch,
        location: BranchLocation,
        active_expression: Option<&ExpressionLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        named_functions: &NamedFunctions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        let body = branch
            .body
            .into_iter()
            .map(|(index, expression)| {
                let location = ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index,
                };
                DebugExpression::new(
                    expression,
                    location,
                    active_expression,
                    is_in_innermost_active_function,
                    cluster,
                    named_functions,
                    types,
                    source_map,
                    breakpoints,
                    effect,
                )
            })
            .collect::<Vec<_>>();
        let parameters = branch
            .parameters
            .into_iter()
            .map(|pattern| match pattern {
                Pattern::Identifier { name } => name,
                Pattern::Literal { value } => format!("{value:?}"),
            })
            .collect();

        let is_active = body
            .iter()
            .any(|expression| expression.data.state.is_active());

        Self {
            parameters,
            body,
            is_active,
        }
    }

    pub fn active_expression(&self) -> anyhow::Result<&DebugExpression> {
        self.body
            .iter()
            .find(|expression| expression.data.state.is_active())
            .ok_or_else(|| {
                anyhow!(
                    "Expected active expression in branch, but could not find \
                    any. Branch:\n\
                    {self:#?}"
                )
            })
    }

    pub fn expression_after(
        &self,
        expression: &ExpressionLocation,
    ) -> anyhow::Result<Option<&DebugExpression>> {
        if !self.body.iter().any(|f| f.data.location == *expression) {
            return Err(anyhow!(
                "Expected expression to be in branch, but could not find it. \
                Expression:\n\
                {expression:#?}\n\
                Branch:\n\
                {self:#?}"
            ));
        }

        let mut expressions = self
            .body
            .iter()
            .skip_while(|f| f.data.location != *expression);

        // This is the fragment we've been passed as an argument. Need to ignore
        // it, to advance the iterator to the one we're actually looking for.
        assert_eq!(
            expressions
                .next()
                .as_ref()
                .map(|fragment| &fragment.data.location),
            Some(expression)
        );

        Ok(expressions.next())
    }
}
