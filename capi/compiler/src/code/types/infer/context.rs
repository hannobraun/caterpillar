use std::collections::BTreeMap;

use crate::code::{syntax::FunctionLocation, Index, Signature};

use super::types::{InferredType, InferredTypes};

#[derive(Default)]
pub struct InferenceContext {
    pub types: InferredTypes,
    pub functions: BTreeMap<FunctionLocation, Signature<Index<InferredType>>>,
}
