#![allow(clippy::module_inception)]

pub mod search;

mod call_graph;
mod changes;
mod expression;
mod functions;
mod hash;
mod index;
mod location;
mod types;

pub use self::{
    call_graph::{CallGraph, Cluster},
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    expression::{Expression, UnresolvedCallToUserDefinedFunction},
    functions::{Branch, Function, Functions, NamedFunction, Pattern},
    hash::Hash,
    index::{Index, IndexMap},
    location::{BranchLocation, ExpressionLocation, FunctionLocation},
    types::{ConcreteSignature, Signature, Type, Types},
};
