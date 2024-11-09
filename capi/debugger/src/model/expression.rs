use capi_compiler::{
    code::{
        Cluster, Expression, ExpressionLocation, FunctionLocation, Functions,
        Types,
    },
    host::Host,
    source_map::SourceMap,
};
use capi_game_engine::host::GameEngineHost;
use capi_runtime::Effect;

use super::{Breakpoints, DebugFunction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugExpression {
    pub data: DebugExpressionData,
    pub kind: DebugExpressionKind,
}

impl DebugExpression {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expression: Expression,
        location: ExpressionLocation,
        active_expression: Option<&ExpressionLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        functions: &Functions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        let signature = DebugExpressionSignature::new(&location, types);

        let state = if Some(&location) == active_expression {
            if is_in_innermost_active_function {
                DebugExpressionState::InnermostActiveExpression
            } else {
                DebugExpressionState::ActiveCaller
            }
        } else {
            DebugExpressionState::NotActive
        };

        let has_durable_breakpoint = source_map
            .expression_to_instructions(&location)
            .iter()
            .any(|instruction| breakpoints.durable_at(instruction));

        let active_effect = effect.and_then(|effect| {
            if state.is_innermost_active_expression() {
                Some(*effect)
            } else {
                None
            }
        });

        let data = DebugExpressionData {
            expression: expression.clone(),
            location: location.clone(),
            signature,
            state,
            has_durable_breakpoint,
            effect: active_effect,
        };
        let kind = DebugExpressionKind::new(
            expression,
            location,
            active_expression,
            is_in_innermost_active_function,
            cluster,
            functions,
            types,
            source_map,
            breakpoints,
            effect,
        );

        Self { kind, data }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugExpressionData {
    pub expression: Expression,
    pub location: ExpressionLocation,
    pub signature: DebugExpressionSignature,
    pub state: DebugExpressionState,
    pub has_durable_breakpoint: bool,
    pub effect: Option<Effect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugExpressionSignature {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

impl DebugExpressionSignature {
    pub fn new(location: &ExpressionLocation, types: &Types) -> Self {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        if let Some(signature) = types.of_expressions.get(location) {
            let convert = |index| {
                let type_ = types.inner.get(index).expect(
                    "Got type index from signature; must exist in `types`.",
                );
                format!("{type_:?}")
            };

            inputs.extend(signature.inputs.iter().map(convert));
            outputs.extend(signature.outputs.iter().map(convert));
        }

        Self { inputs, outputs }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugExpressionState {
    InnermostActiveExpression,
    ActiveCaller,
    NotActive,
}

impl DebugExpressionState {
    /// # Indicate whether this is the innermost active expression
    ///
    /// The innermost active expression is the active expression in the
    /// innermost active function. The expression where the process is currently
    /// stopped at.
    pub fn is_innermost_active_expression(&self) -> bool {
        matches!(self, Self::InnermostActiveExpression)
    }

    /// # Indicate whether the expression is active
    ///
    /// A expression is active, either if the process is currently stopped here,
    /// or if it calls an active function (which is a function that contains an
    /// active expression).
    pub fn is_active(&self) -> bool {
        matches!(self, Self::InnermostActiveExpression | Self::ActiveCaller)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugExpressionKind {
    Binding { name: String },
    CallToFunction { name: String },
    CallToFunctionRecursive { name: String },
    CallToHostFunction { name: String },
    CallToIntrinsic { name: String },
    Comment { text: String },
    Function { function: DebugFunction },
    UnresolvedIdentifier { name: String },
    Value { as_string: String },
}

impl DebugExpressionKind {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expression: Expression,
        location: ExpressionLocation,
        active_expression: Option<&ExpressionLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        functions: &Functions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        match expression {
            Expression::Binding { name, .. } => Self::Binding { name },
            Expression::CallToUserDefinedFunction { hash, .. } => {
                let function = functions
                    .find_named_by_hash(&hash)
                    .expect("Expecting function referenced by call to exist.");
                let name = function.name.clone();

                Self::CallToFunction { name }
            }
            Expression::CallToUserDefinedFunctionRecursive {
                index, ..
            } => {
                let called_function_index = cluster
                    .functions
                    .get(&index)
                    .expect(
                    "The index of a recursive call must be valid within the \
                    calling function's cluster.",
                );
                let called_function =
                    functions.named.get(called_function_index).expect(
                        "Expecting to find expression referred to from a \
                        cluster.",
                    );
                let name = called_function.name.clone();

                Self::CallToFunctionRecursive { name }
            }
            Expression::CallToHostFunction { number } => {
                let name = GameEngineHost
                    .function_by_number(number)
                    .expect("Expected effect number in code to be valid.")
                    .name()
                    .to_string();

                Self::CallToHostFunction { name }
            }
            Expression::CallToIntrinsicFunction { intrinsic, .. } => {
                Self::CallToIntrinsic {
                    name: intrinsic.to_string(),
                }
            }
            Expression::Comment { text } => Self::Comment {
                text: format!("# {text}"),
            },
            Expression::LiteralFunction { function, .. } => {
                let function = DebugFunction::new(
                    function,
                    None,
                    FunctionLocation::AnonymousFunction { location },
                    active_expression,
                    is_in_innermost_active_function,
                    cluster,
                    functions,
                    types,
                    source_map,
                    breakpoints,
                    effect,
                );

                Self::Function { function }
            }
            Expression::LiteralNumber { value } => Self::Value {
                as_string: value.to_string(),
            },
            Expression::UnresolvedIdentifier { name, .. } => {
                Self::UnresolvedIdentifier { name }
            }
        }
    }
}
