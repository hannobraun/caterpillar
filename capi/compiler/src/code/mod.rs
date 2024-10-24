#![allow(clippy::module_inception)]

pub mod search;

mod call_graph;
mod changes;
mod fragment;
mod functions;
mod location;
mod type_;

pub use self::{
    call_graph::{CallGraph, Cluster},
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    fragment::{Fragment, UnresolvedCallToUserDefinedFunction},
    functions::{Branch, Function, NamedFunctions, Pattern},
    location::{
        BranchIndex, BranchLocation, FragmentIndexInBranchBody,
        FragmentLocation, FunctionIndexInCluster, FunctionIndexInRootContext,
        FunctionLocation,
    },
    type_::Type,
};
