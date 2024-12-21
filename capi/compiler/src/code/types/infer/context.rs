use std::collections::BTreeMap;

use crate::code::{syntax::FunctionLocation, Index, Signature};

use super::types::InferredType;

pub struct InferenceContext {
    pub functions: ClusterFunctions,
}

pub type ClusterFunctions =
    BTreeMap<FunctionLocation, Signature<Index<InferredType>>>;
