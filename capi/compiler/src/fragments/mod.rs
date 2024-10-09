#![allow(clippy::module_inception)]

pub mod search;

mod fragment;
mod fragments;
mod functions;
mod location;
mod map;

pub use crate::syntax::Cluster;

pub use self::{
    fragment::Fragment,
    fragments::Fragments,
    functions::{Branch, Function, Parameters},
    location::{
        BranchIndex, BranchLocation, FragmentIndexInBranchBody,
        FragmentLocation, FunctionIndexInCluster, FunctionIndexInRootContext,
        FunctionLocation,
    },
    map::{FoundFunction, FragmentId, FragmentMap},
};
