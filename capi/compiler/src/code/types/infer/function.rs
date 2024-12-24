use crate::code::{Index, Signature};

use super::{
    signature::unify_type_list,
    types::{InferredType, InferredTypes},
};

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
