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
    types: &InferredTypes,
) -> Result<Option<Signature<Type>>> {
    let try_map = |from: &Vec<Index<InferredType>>| {
        from.iter()
            .map(|index| {
                let type_ = types.resolve(index)?;
                Ok(type_.into_type())
            })
            .collect::<Result<Option<_>>>()
    };

    let inputs = try_map(&signature.inputs)?;
    let outputs = try_map(&signature.outputs)?;

    let signature = inputs
        .zip(outputs)
        .map(|(inputs, outputs)| Signature { inputs, outputs });

    Ok(signature)
}

pub fn unify(
    [a, b]: [&Signature<Index<InferredType>>; 2],
    types: &mut InferredTypes,
) {
    assert_eq!(
        a.inputs.len(),
        b.inputs.len(),
        "Expecting signatures to have same number of inputs.",
    );
    assert_eq!(
        a.outputs.len(),
        b.outputs.len(),
        "Expecting signatures to have same number of outputs.",
    );

    let unify = |[a, b]: [&Vec<Index<InferredType>>; 2],
                 types: &mut InferredTypes| {
        for (a, b) in a.iter().zip(b.iter()) {
            types.unify([a, b]);
        }
    };

    unify([&a.inputs, &b.inputs], types);
    unify([&a.outputs, &b.outputs], types);
}
