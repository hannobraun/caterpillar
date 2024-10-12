#![allow(clippy::module_inception)]

pub mod search;

mod changes;
mod fragment;
mod fragments;
mod functions;
mod location;

pub use self::{
    changes::{Changes, FunctionInUpdate, FunctionUpdate},
    fragment::Fragment,
    fragments::Fragments,
    functions::{Branch, Function, NamedFunctions, Parameters},
    location::{
        BranchIndex, BranchLocation, FragmentIndexInBranchBody,
        FragmentLocation, FunctionIndexInCluster, FunctionIndexInRootContext,
        FunctionLocation,
    },
};
