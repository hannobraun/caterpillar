#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod functions;
mod map;

pub use self::{
    fragment::Fragment,
    fragments::Fragments,
    functions::{Branch, Function, Parameters},
    map::{FoundFunction, FragmentId, FragmentMap},
};
