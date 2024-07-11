#![allow(clippy::module_inception)]

mod fragment;
mod fragments;
mod id;
mod payload;

pub use self::{
    fragment::{Fragment, FragmentAddress, FragmentAddressParent},
    fragments::{FragmentMap, Fragments, Function},
    id::FragmentId,
    payload::FragmentPayload,
};
