mod data_stack;
mod execute;
mod pipeline;
mod syntax;

pub use self::{
    data_stack::{DataStack, DataStackError},
    execute::{execute, Error},
    pipeline::{d_evaluator::evaluate, PipelineError},
};
