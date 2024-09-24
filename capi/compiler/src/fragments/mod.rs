#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod function;
mod map;

pub use self::{
    fragment::Fragment,
    fragments::Fragments,
    function::{Branch, Function, Parameters},
    map::{FoundFunction, FragmentId, FragmentMap},
};
