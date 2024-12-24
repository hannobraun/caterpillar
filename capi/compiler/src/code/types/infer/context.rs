use std::collections::BTreeMap;

use crate::code::{
    syntax::{FunctionLocation, MemberLocation, ParameterLocation},
    Index, Signature, Type,
};

use super::{
    signature::{self, unify_type_list},
    types::{InferredType, InferredTypes},
};

#[derive(Default)]
pub struct InferenceContext {
    pub types: InferredTypes,
    pub functions: BTreeMap<FunctionLocation, InferredFunction>,
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
            .or_else(|| {
                let function = self.functions.get(location).cloned()?;
                function.to_signature()
            })
    }

    pub fn binding(
        &mut self,
        location: &ParameterLocation,
        parameters: &BTreeMap<ParameterLocation, Type>,
    ) -> Option<Index<InferredType>> {
        parameters
            .get(location)
            .map(|type_| self.types.push(InferredType::Known(type_.clone())))
            .or_else(|| self.bindings.get(location).cloned())
    }
}

#[derive(Clone)]
pub struct InferredFunction {
    /// # The inferred inputs of the function
    ///
    /// Since function parameters are defined as part of each of its branches,
    /// we always know at least the number of parameters. Even if we don't know
    /// their types.
    pub inputs: Vec<Index<InferredType>>,

    /// # The inferred outputs of the function
    ///
    /// Can be `None`, if inference fails partway through a function, as we
    /// might not know anything about the output in this case.
    pub outputs: Option<Vec<Index<InferredType>>>,
}

impl InferredFunction {
    pub fn unify_with(&self, other: &Self, types: &mut InferredTypes) {
        unify_type_list([&self.inputs, &other.inputs], types);

        if let (Some(self_outputs), Some(other_outputs)) =
            (&self.outputs, &other.outputs)
        {
            unify_type_list([self_outputs, other_outputs], types);
        }
    }

    pub fn to_signature(&self) -> Option<Signature<Index<InferredType>>> {
        let inputs = self.inputs.clone();
        let outputs = self.outputs.clone()?;

        Some(Signature { inputs, outputs })
    }
}
