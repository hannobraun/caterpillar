#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod function;
mod hash;

pub use self::{
    fragment::{Fragment, FragmentKind, FragmentLocation},
    fragments::{FoundFunction, FragmentMap, Fragments},
    function::{Branch, Function, Parameters},
    hash::Hash,
};
