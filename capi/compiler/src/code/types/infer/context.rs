use std::collections::BTreeMap;

use crate::code::{
    syntax::{FunctionLocation, ParameterLocation},
    Index, Signature,
};

use super::{
    signature,
    types::{InferredType, InferredTypes},
};

#[derive(Default)]
pub struct InferenceContext {
    pub types: InferredTypes,
    pub functions: BTreeMap<FunctionLocation, Signature<Index<InferredType>>>,
    pub bindings: BTreeMap<ParameterLocation, Index<InferredType>>,
}

impl InferenceContext {
    pub fn function(
        &mut self,
        location: &FunctionLocation,
        functions: &BTreeMap<FunctionLocation, Signature>,
    ) -> Option<Signature<Index<InferredType>>> {
        functions
            .get(location)
            .map(|signature| {
                signature::make_indirect(signature.clone(), &mut self.types)
            })
            .or_else(|| self.functions.get(location).cloned())
    }
}
