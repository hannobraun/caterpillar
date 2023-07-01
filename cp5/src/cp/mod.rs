mod execute;
mod namespace;
mod pipeline;
mod runtime;

pub use self::{
    execute::{execute, Error},
    namespace::{
        Bindings, Function, FunctionBody, Functions, Intrinsic, Module,
    },
    pipeline::{error::PipelineError, ir::analyzer_output::AnalyzerEvent},
    runtime::{
        data_stack::{DataStack, DataStackError},
        evaluate::{Evaluator, EvaluatorError},
    },
};
