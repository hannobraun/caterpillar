use std::{collections::BTreeMap, fmt};

use crate::code::{
    syntax::{MemberLocation, ParameterLocation, SyntaxTree},
    Bindings, Dependencies, Identifiers,
};

use super::{
    infer::{infer, CompilerContext, InferenceOutput},
    resolve::resolve_type_annotations,
};

/// # The types that are explicitly specified in the code
///
/// ## Implementation Note
///
/// The long-term goal for Caterpillar is to be fully inferred, with no explicit
/// types annotations being necessary at all. Then this type can be removed.
#[derive(Debug)]
pub struct TypeAnnotations {
    bindings: BTreeMap<ParameterLocation, Type>,
    expressions: BTreeMap<MemberLocation, Signature>,
}

impl TypeAnnotations {
    /// # Resolve all explicit type annotations
    pub fn resolve(syntax_tree: &SyntaxTree) -> Self {
        let (bindings, expressions) = resolve_type_annotations(syntax_tree);

        Self {
            bindings,
            expressions,
        }
    }

    /// # Access the type of the binding at the given location, if any
    pub fn type_of_binding(
        &self,
        location: &ParameterLocation,
    ) -> Option<&Type> {
        self.bindings.get(location)
    }

    /// # Access the signature of the expression at the given location, if any
    pub fn signature_of_expression(
        &self,
        location: &MemberLocation,
    ) -> Option<&Signature> {
        self.expressions.get(location)
    }
}

/// # The resolved types
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Types {
    parameters: BTreeMap<ParameterLocation, Type>,
    expressions: BTreeMap<MemberLocation, Signature>,
    stacks: Stacks,
}

impl Types {
    /// # Infer types
    ///
    /// ## Implementation Note
    ///
    /// This current also uses the explicitly specified types where it can't
    /// infer yet, but the plan is to remove any explicit annotations in favor
    /// of full type inference.
    pub fn infer(
        syntax_tree: &SyntaxTree,
        bindings: &Bindings,
        identifiers: &Identifiers,
        dependencies: &Dependencies,
        annotations: TypeAnnotations,
    ) -> Self {
        let InferenceOutput {
            functions: _,
            expressions,
            parameters,
            stacks,
        } = infer(CompilerContext {
            syntax_tree,
            bindings,
            identifiers,
            dependencies,
            annotations: &annotations,
        });
        Self {
            parameters,
            expressions,
            stacks,
        }
    }

    /// # Access the type of the parameter at the given location, if any
    pub fn type_of_parameter(
        &self,
        location: &ParameterLocation,
    ) -> Option<&Type> {
        self.parameters.get(location)
    }

    /// # Access the signature of the expression at the given location, if any
    pub fn signature_of_expression(
        &self,
        location: &MemberLocation,
    ) -> Option<&Signature> {
        self.expressions.get(location)
    }

    /// # Access the stack, as of the expression at the given location, if any
    ///
    /// This reflects what types are on the stack _before_ the expression is
    /// evaluated.
    ///
    /// The stack returned here is always local to the function that the
    /// expression is in. Tracking the global stack at compile-time is not
    /// possible, and only the local contents of the stack are relevant to what
    /// the expression can do anyway.
    pub fn stack_at(&self, location: &MemberLocation) -> Option<&[Type]> {
        self.stacks.get(location).map(|stack| &**stack)
    }
}

pub type Stacks = BTreeMap<MemberLocation, Stack>;
pub type Stack = Vec<Type>;

/// # A type signature that applies to a function or expression
///
/// This struct is generic over the type of the type in the signature. Usually
/// it's going to be [`Type`], but there are various specialized situation where
/// more specialized signatures are needed, and this type parameter enables that
/// without duplication.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Signature<T = Type> {
    /// # The inputs that the function or expression consumes
    pub inputs: Vec<T>,

    /// # The outputs that the function or expression produces
    pub outputs: Vec<T>,
}

impl<I, O> From<(I, O)> for Signature
where
    I: IntoIterator<Item = Type>,
    O: IntoIterator<Item = Type>,
{
    fn from((inputs, outputs): (I, O)) -> Self {
        Self {
            inputs: inputs.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut inputs = self.inputs.iter().peekable();
        while let Some(input) = inputs.next() {
            write!(f, "{input}")?;

            if inputs.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, " ->")?;
        if !self.outputs.is_empty() {
            write!(f, " ")?;
        }

        let mut outputs = self.outputs.iter().peekable();
        while let Some(output) = outputs.next() {
            write!(f, "{output}")?;

            if outputs.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}

/// # The type of a value
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub enum Type {
    /// # A function
    Function {
        /// # The function's signature
        signature: Signature,
    },

    /// # A number
    ///
    /// ## Implementation Note
    ///
    /// Since the language is still mostly untyped, this actually covers many
    /// different types that are all represented as a 32-bit number.
    ///
    /// I expect that this will get split into multiple, more specific numeric
    /// types at some point.
    Number,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function { signature } => {
                write!(f, "fn {signature} end")?;
            }
            Self::Number => {
                write!(f, "Number")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        code::{
            syntax::{Expression, SyntaxTree},
            Bindings, Dependencies, FunctionCalls, Identifiers, Signature,
            Tokens, Type,
        },
        host::NoHost,
    };

    use super::{TypeAnnotations, Types};

    #[test]
    fn infer_type_of_binding_from_use() {
        // The type of a binding can be inferred, if it's used by a function
        // with a known type.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ value ->
                        value not
                end
            ",
        );

        let f = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function();
        let f_branch = f.find_single_branch().unwrap();

        let value_parameter = f_branch
            .parameters()
            .map(|parameter| parameter.location)
            .next()
            .unwrap();
        let value_expression = f_branch
            .expressions()
            .map(|expression| expression.location)
            .next()
            .unwrap();

        assert_eq!(
            types.type_of_parameter(&value_parameter).cloned().unwrap(),
            Type::Number,
        );
        assert_eq!(
            types
                .signature_of_expression(&value_expression)
                .cloned()
                .unwrap(),
            Signature {
                inputs: vec![],
                outputs: vec![Type::Number],
            },
        );
        assert_eq!(types.stack_at(&value_expression).unwrap(), &[]);
    }

    #[test]
    fn infer_type_of_binding_from_other_branch() {
        // The type of a binding can be inferred, if its type is specified in
        // the parameter list of another branch.

        let branch_with_known_type = r"
            \ 0 ->
                0
        ";
        let branch_with_unknown_type = r"
            \ x ->
                x
        ";

        test(branch_with_known_type, branch_with_unknown_type);
        test(branch_with_unknown_type, branch_with_known_type);

        fn test(branch_a: &str, branch_b: &str) {
            let (syntax_tree, types) = infer_types(&format!(
                r"
                    f: fn
                        {branch_a}
                        {branch_b}
                    end
                "
            ));

            let f = syntax_tree
                .function_by_name("f")
                .unwrap()
                .into_located_function();

            let (parameter_a, parameter_b) = f
                .branches()
                .map(|branch| branch.parameters().next().unwrap().location)
                .collect_tuple()
                .unwrap();
            let (expression_a, expression_b) = f
                .branches()
                .map(|branch| branch.expressions().next().unwrap().location)
                .collect_tuple()
                .unwrap();

            for location in [parameter_a, parameter_b] {
                assert_eq!(
                    types.type_of_parameter(&location).cloned().unwrap(),
                    Type::Number,
                );
            }
            for location in [expression_a, expression_b] {
                assert_eq!(
                    types.signature_of_expression(&location).cloned().unwrap(),
                    Signature {
                        inputs: vec![],
                        outputs: vec![Type::Number],
                    },
                );
                assert_eq!(types.stack_at(&location).unwrap(), &[]);
            }
        }
    }

    #[test]
    fn infer_type_of_binding_from_use_in_local_function() {
        // If the type of a binding can be inferred in a local function, that
        // should carry over to the parent.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ value ->
                        # We should know the type of `value` from its use within
                        # the local function.
                        value

                        fn
                            \ ->
                                value not # type of `value` can be inferred here
                        end
                end
            ",
        );

        let f = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function();
        let f_branch = f.find_single_branch().unwrap();

        let value_parameter = f_branch
            .parameters()
            .map(|parameter| parameter.location)
            .next()
            .unwrap();
        let value_expression = f_branch
            .expressions()
            .map(|expression| expression.location)
            .next()
            .unwrap();

        assert_eq!(
            types.type_of_parameter(&value_parameter).cloned().unwrap(),
            Type::Number,
        );
        assert_eq!(
            types
                .signature_of_expression(&value_expression)
                .cloned()
                .unwrap(),
            Signature {
                inputs: vec![],
                outputs: vec![Type::Number],
            },
        );
        assert_eq!(types.stack_at(&value_expression).unwrap(), &[]);
    }

    #[test]
    fn infer_type_of_literal() {
        // The type of a literal can be trivially inferred.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ ->
                        1
                end
            ",
        );

        let value = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .next()
            .unwrap();

        assert_eq!(
            types.signature_of_expression(&value).cloned().unwrap(),
            Signature {
                inputs: vec![],
                outputs: vec![Type::Number],
            },
        );
        assert_eq!(types.stack_at(&value).unwrap(), &[]);
    }

    #[test]
    fn infer_type_of_call_to_single_branch_function() {
        // If the single branch of a function provides enough information to
        // infer the types of both its inputs and outputs, then the signature of
        // a call to that function should be inferred.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ ->
                        0 g
                end

                g: fn
                    \ x ->
                        x not
                end
            ",
        );

        let g = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .nth(1)
            .unwrap();

        assert_eq!(
            types.signature_of_expression(&g).cloned().unwrap(),
            Signature {
                inputs: vec![Type::Number],
                outputs: vec![Type::Number],
            },
        );
        assert_eq!(types.stack_at(&g).unwrap(), &[Type::Number]);
    }

    #[test]
    fn infer_type_of_call_to_multi_branch_function_by_combining_inputs() {
        // If a function has multiple branches, each of which contributes
        // knowledge about the inputs of the function, that knowledge should be
        // combined into a full signature.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ ->
                        0 0 g
                end

                g: fn
                    \ 0, x ->
                        x

                    \ x, 0 ->
                        x
                end
            ",
        );

        let g = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .nth(2)
            .unwrap();

        assert_eq!(
            types.signature_of_expression(&g).cloned().unwrap(),
            Signature {
                inputs: vec![Type::Number, Type::Number],
                outputs: vec![Type::Number],
            },
        );
        assert_eq!(types.stack_at(&g).unwrap(), &[Type::Number, Type::Number]);
    }

    #[test]
    fn infer_type_of_call_to_self_recursive_function() {
        // A recursive call in isolation can't be inferred, as there's not
        // return value. But another branch could provide the necessary
        // information.

        let branch_recursive = r"
            \ 0 ->
                1 g
        ";
        let branch_non_recursive = r"
            \ x ->
                x
        ";

        test(branch_recursive, branch_non_recursive);
        test(branch_non_recursive, branch_recursive);

        fn test(branch_a: &str, branch_b: &str) {
            let (syntax_tree, types) = infer_types(&format!(
                r"
                    f: fn
                        \ ->
                            0 g
                    end

                    g: fn
                        {branch_a}
                        {branch_b}
                    end
                "
            ));

            let g = syntax_tree
                .function_by_name("f")
                .unwrap()
                .into_located_function()
                .find_single_branch()
                .unwrap()
                .expressions()
                .map(|expression| expression.location)
                .nth(1)
                .unwrap();

            assert_eq!(
                types.signature_of_expression(&g).cloned().unwrap(),
                Signature {
                    inputs: vec![Type::Number],
                    outputs: vec![Type::Number],
                },
            );
            assert_eq!(types.stack_at(&g).unwrap(), &[Type::Number]);
        }
    }

    #[test]
    fn infer_type_of_calls_to_non_divergent_mutually_recursive_functions() {
        // Mutually recursive functions can not be inferred on their own. But if
        // they're not divergent, that means they have branches that we can
        // infer a return type from.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ ->
                        0 g
                        0 h
                end

                g: fn
                    \ 0 ->
                        0 h
                    
                    \ _ ->
                        1 h
                end

                h: fn
                    \ 0 ->
                        1 g

                    \ _ ->
                        0
                end
            ",
        );

        let (call_to_g_in_f, call_to_h_in_f) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .filter_map(|expression| {
                if let Expression::Identifier { .. } = expression.fragment {
                    Some(expression.location)
                } else {
                    None
                }
            })
            .collect_tuple()
            .unwrap();
        let call_to_g_in_h = syntax_tree
            .function_by_name("h")
            .unwrap()
            .into_located_function()
            .branches()
            .next()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .nth(1)
            .unwrap();

        let check = |call, stack: &[Type]| {
            assert_eq!(
                types.signature_of_expression(call).cloned().unwrap(),
                Signature {
                    inputs: vec![Type::Number],
                    outputs: vec![Type::Number],
                },
            );
            assert_eq!(types.stack_at(call).unwrap(), stack);
        };

        check(&call_to_g_in_f, &[Type::Number]);
        check(&call_to_h_in_f, &[Type::Number, Type::Number]);
        check(&call_to_g_in_h, &[Type::Number]);
    }

    #[test]
    fn infer_type_of_local_function() {
        // If the signature of a local function can be inferred, that should
        // transfer to the expression that defines it.

        let (syntax_tree, types) = infer_types(
            r"
                f: fn
                    \ ->
                        fn
                            \ x ->
                                x not
                        end
                end
            ",
        );

        let f_local = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .next()
            .unwrap();

        assert_eq!(
            types.signature_of_expression(&f_local).cloned().unwrap(),
            Signature {
                inputs: vec![],
                outputs: vec![Type::Function {
                    signature: Signature {
                        inputs: vec![Type::Number],
                        outputs: vec![Type::Number],
                    },
                }],
            },
        );
        assert_eq!(types.stack_at(&f_local).unwrap(), &[]);
    }

    fn infer_types(input: &str) -> (SyntaxTree, Types) {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let type_annotations = TypeAnnotations::resolve(&syntax_tree);
        let bindings = Bindings::resolve(&syntax_tree);
        let function_calls = FunctionCalls::resolve(&syntax_tree, &NoHost);
        let identifiers =
            Identifiers::resolve(&syntax_tree, &bindings, &function_calls);
        let dependencies = Dependencies::resolve(&syntax_tree, &function_calls);
        let types = Types::infer(
            &syntax_tree,
            &bindings,
            &identifiers,
            &dependencies,
            type_annotations,
        );

        (syntax_tree, types)
    }
}
