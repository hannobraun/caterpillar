#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod function;
mod hash;

pub use self::{
    fragment::{Fragment, FragmentId},
    fragments::{FoundFunction, FragmentMap, Fragments},
    function::{Branch, Function, Parameters},
};
