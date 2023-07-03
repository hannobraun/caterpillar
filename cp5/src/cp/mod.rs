mod code;
mod execute;
mod namespace;
mod pipeline;
mod runtime;
mod test_runner;

pub use self::{
    code::{define_code, intrinsics, std, tests},
    execute::{execute, Error},
    namespace::{
        Bindings, Function, FunctionBody, Functions, Intrinsic, Module,
    },
    pipeline::{error::PipelineError, ir::analyzer_output::AnalyzerEvent},
    runtime::{
        data_stack::{DataStack, DataStackError},
        evaluate::{Evaluator, EvaluatorError},
    },
    test_runner::{run_tests, TestReport},
};
