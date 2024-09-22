#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod function;
mod id;

pub use self::{
    fragment::{Fragment, FragmentKind, FragmentLocation},
    fragments::{FoundFunction, FragmentMap, Fragments},
    function::{Branch, Function, Parameters},
    id::Hash,
};
