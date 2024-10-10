#![allow(clippy::module_inception)]

pub mod search;

mod changes;
mod fragment;
mod fragments;
mod functions;
mod location;

pub use self::{
    changes::Changes,
    fragment::Fragment,
    fragments::Fragments,
    functions::{Branch, Function, Parameters},
    location::{
        BranchIndex, BranchLocation, FragmentIndexInBranchBody,
        FragmentLocation, FunctionIndexInCluster, FunctionIndexInRootContext,
        FunctionLocation,
    },
};
