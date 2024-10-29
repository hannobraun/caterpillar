#![allow(clippy::module_inception)]

pub mod search;

mod call_graph;
mod changes;
mod fragment;
mod functions;
mod hash;
mod index;
mod location;
mod types;

pub use self::{
    call_graph::{CallGraph, Cluster},
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    fragment::{Fragment, UnresolvedCallToUserDefinedFunction},
    functions::{Branch, Function, NamedFunctions, Pattern},
    hash::Hash,
    index::{Index, IndexMap},
    location::{BranchLocation, FragmentLocation, FunctionLocation},
    types::{Signature, Type, Types},
};
