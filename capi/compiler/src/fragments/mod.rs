#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod functions;
mod location;
mod map;

pub use self::{
    fragment::Fragment,
    fragments::{Cluster, Fragments},
    functions::{Branch, Function, Parameters},
    location::{
        BranchIndex, FragmentIndexInBranchBody, FragmentLocation,
        FunctionIndexInCluster, FunctionIndexInRootContext,
    },
    map::{FoundFunction, FragmentId, FragmentMap},
};
