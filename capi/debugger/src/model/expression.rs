use capi_compiler::{
    code::{
        syntax::{
            Expression, FunctionLocation, Member, MemberLocation, Signature,
        },
        Cluster, FunctionCalls, Functions,
    },
    source_map::SourceMap,
};
use capi_runtime::Effect;

use super::{Breakpoints, DebugFunction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugMember {
    pub data: DebugMemberData,
    pub kind: DebugMemberKind,
}

impl DebugMember {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        member: Member,
        location: MemberLocation,
        active_expression: Option<&MemberLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        functions: &Functions,
        function_calls: &FunctionCalls,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        let state = if Some(&location) == active_expression {
            if is_in_innermost_active_function {
                DebugMemberState::InnermostActiveExpression
            } else {
                DebugMemberState::ActiveCaller
            }
        } else {
            DebugMemberState::NotActive
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

        let Member::Expression {
            expression,
            signature,
        } = member;

        let kind = DebugMemberKind::new(
            expression,
            location.clone(),
            active_expression,
            is_in_innermost_active_function,
            cluster,
            functions,
            function_calls,
            source_map,
            breakpoints,
            effect,
        );
        let data = DebugMemberData {
            signature,
            location,
            state,
            has_durable_breakpoint,
            effect: active_effect,
        };

        Self { kind, data }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugMemberData {
    pub signature: Option<Signature>,
    pub location: MemberLocation,
    pub state: DebugMemberState,
    pub has_durable_breakpoint: bool,
    pub effect: Option<Effect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugMemberState {
    InnermostActiveExpression,
    ActiveCaller,
    NotActive,
}

impl DebugMemberState {
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
pub enum DebugMemberKind {
    Comment { text: String },
    Function { function: DebugFunction },
    Identifier { name: String },
    Value { as_string: String },
}

impl DebugMemberKind {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expression: Expression,
        location: MemberLocation,
        active_expression: Option<&MemberLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        functions: &Functions,
        function_calls: &FunctionCalls,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        match expression {
            Expression::Comment { text } => Self::Comment {
                text: format!("# {text}"),
            },
            Expression::Identifier { name } => Self::Identifier { name },
            Expression::LiteralNumber { value } => Self::Value {
                as_string: value.to_string(),
            },
            Expression::LocalFunction { function } => {
                let function = DebugFunction::new(
                    function,
                    None,
                    FunctionLocation::AnonymousFunction { location },
                    active_expression,
                    is_in_innermost_active_function,
                    cluster,
                    functions,
                    function_calls,
                    source_map,
                    breakpoints,
                    effect,
                );

                Self::Function { function }
            }
        }
    }
}
