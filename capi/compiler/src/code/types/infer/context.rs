use std::collections::BTreeMap;

use crate::code::{syntax::FunctionLocation, Index, Signature};

use super::types::InferredType;

#[derive(Default)]
pub struct InferenceContext {
    pub functions: BTreeMap<FunctionLocation, Signature<Index<InferredType>>>,
}
