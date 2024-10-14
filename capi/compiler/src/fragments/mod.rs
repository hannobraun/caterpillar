#![allow(clippy::module_inception)]

pub mod search;

mod call_graph;
mod changes;
mod fragment;
mod functions;
mod location;

pub use self::{
    call_graph::{CallGraph, Cluster},
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    fragment::Fragment,
    functions::{Branch, Function, NamedFunctions, Parameters, Pattern},
    location::{
        BranchIndex, BranchLocation, FragmentIndexInBranchBody,
        FragmentLocation, FunctionIndexInCluster, FunctionIndexInRootContext,
        FunctionLocation,
    },
};
