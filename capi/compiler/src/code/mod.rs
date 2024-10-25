#![allow(clippy::module_inception)]

pub mod search;

mod call_graph;
mod changes;
mod fragment;
mod functions;
mod index;
mod location;
mod types;

pub use crate::hash::Hash;

pub use self::{
    call_graph::{CallGraph, Cluster},
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    fragment::{Fragment, UnresolvedCallToUserDefinedFunction},
    functions::{Branch, Function, NamedFunctions, Pattern, TypedFragment},
    index::Index,
    location::{BranchLocation, FragmentLocation, FunctionLocation},
    types::{Signature, Type},
};
