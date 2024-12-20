use crate::code::{Index, Signature, Type};

use super::types::{InferredType, InferredTypes, Result};

pub fn make_indirect(
    signature: Signature,
    types: &mut InferredTypes,
) -> Signature<Index<InferredType>> {
    let mut map = |from: Vec<Type>| {
        from.into_iter()
            .map(|type_| types.push(InferredType::Known(type_)))
            .collect()
    };

    Signature {
        inputs: map(signature.inputs),
        outputs: map(signature.outputs),
    }
}

pub fn make_direct(
    signature: &Signature<Index<InferredType>>,
    local_types: &InferredTypes,
) -> Result<Option<Signature<Type>>> {
    let try_map = |from: &Vec<Index<InferredType>>| {
        from.iter()
            .map(|index| Ok(local_types.resolve(index)?.into_type()))
            .collect::<Result<Option<_>>>()
    };

    let inputs = try_map(&signature.inputs)?;
    let outputs = try_map(&signature.outputs)?;

    let signature = inputs
        .zip(outputs)
        .map(|(inputs, outputs)| Signature { inputs, outputs });

    Ok(signature)
}
