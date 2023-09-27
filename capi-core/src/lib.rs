pub mod intrinsics;
pub mod pipeline;
pub mod repr;
pub mod runtime;

pub use self::{
    repr::eval::value,
    runtime::{
        data_stack::DataStackResult,
        evaluator::{Evaluator, EvaluatorState},
        functions::{PlatformFunction, RuntimeContext, RuntimeState},
        interpreter::Interpreter,
    },
};
