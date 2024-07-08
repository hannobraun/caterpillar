#![allow(clippy::module_inception)]

mod address;
mod fragment;
mod fragments;
mod id;
mod payload;

pub use self::{
    address::FragmentAddress,
    fragment::Fragment,
    fragments::{Fragments, Function, FunctionFragments},
    id::FragmentId,
    payload::FragmentPayload,
};
