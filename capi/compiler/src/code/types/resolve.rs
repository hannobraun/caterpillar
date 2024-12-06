use std::collections::BTreeMap;

use crate::code::syntax::{
    MemberLocation, SyntaxSignature, SyntaxTree, SyntaxType,
};

use super::{Signature, Type};

pub fn resolve_type_annotations(
    syntax_tree: &SyntaxTree,
) -> BTreeMap<MemberLocation, Signature> {
    let mut types_ = BTreeMap::new();

    for function in syntax_tree.all_functions() {
        for branch in function.branches() {
            for (expression, signature) in branch.annotated_expressions() {
                let Some(signature) = signature else {
                    continue;
                };

                let signature = resolve_signature(signature);

                types_.insert(expression.location, signature);
            }
        }
    }

    types_
}

fn resolve_signature(signature: &SyntaxSignature) -> Signature {
    Signature {
        inputs: resolve_types(&signature.inputs),
        outputs: resolve_types(&signature.outputs),
    }
}

fn resolve_types(types: &[SyntaxType]) -> Vec<Type> {
    types.iter().map(resolve_type).collect()
}

fn resolve_type(type_: &SyntaxType) -> Type {
    match type_ {
        SyntaxType::Function { signature } => {
            let signature = resolve_signature(signature);
            Type::Function { signature }
        }
        SyntaxType::Identifier { name } => match name.as_str() {
            "Number" => Type::Number,
            type_ => {
                panic!("Unknown type `{type_}`");
            }
        },
    }
}
