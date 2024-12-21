use std::collections::BTreeMap;

use crate::code::{
    syntax::{FunctionLocation, MemberLocation, ParameterLocation},
    Index, Signature, Type,
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
    pub expressions: BTreeMap<MemberLocation, Signature<Index<InferredType>>>,
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

    pub fn binding(
        &mut self,
        location: &ParameterLocation,
        bindings: &BTreeMap<ParameterLocation, Type>,
    ) -> Option<Index<InferredType>> {
        bindings
            .get(location)
            .map(|type_| self.types.push(InferredType::Known(type_.clone())))
            .or_else(|| self.bindings.get(location).cloned())
    }
}
