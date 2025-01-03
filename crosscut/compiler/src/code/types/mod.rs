mod infer;
mod repr;
mod resolve;

pub use self::repr::{Signature, Type, TypeAnnotations, Types};

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
                    br value ->
                        value
                        not
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
    fn infer_type_of_binding_from_other_branch() {
        // The type of a binding can be inferred, if its type is specified in
        // the parameter list of another branch.

        let branch_with_known_type = r"
            br 0 ->
                0
            end
        ";
        let branch_with_unknown_type = r"
            br x ->
                x
            end
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
                    br value ->
                        # We should know the type of `value` from its use within
                        # the local function.
                        value

                        fn
                            br ->
                                value
                                not # type of `value` can be inferred here
                            end
                        end
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
                    br ->
                        1
                    end
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
                    br ->
                        0 g
                    end
                end

                g: fn
                    br x ->
                        x not
                    end
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
                    br ->
                        0 0 g
                    end
                end

                g: fn
                    br 0, x ->
                        x
                    end

                    br x, 0 ->
                        x
                    end
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
            br 0 ->
                1 g
            end
        ";
        let branch_non_recursive = r"
            br x ->
                x
            end
        ";

        test(branch_recursive, branch_non_recursive);
        test(branch_non_recursive, branch_recursive);

        fn test(branch_a: &str, branch_b: &str) {
            let (syntax_tree, types) = infer_types(&format!(
                r"
                    f: fn
                        br ->
                            0 g
                        end
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
                    br ->
                        0 g
                        0 h
                    end
                end

                g: fn
                    br 0 ->
                        0 h
                    end
                    
                    br _ ->
                        1 h
                    end
                end

                h: fn
                    br 0 ->
                        1 g
                    end

                    br _ ->
                        0
                    end
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
                    br ->
                        0
                        fn
                            br 0 ->
                                0
                            end

                            # Add this branch to make sure that `f` and the
                            # local function are in the same cluster. This more
                            # complicated to handle, resulting in this test
                            # covering a bit more ground.
                            br _ ->
                                f
                            end
                        end
                        eval
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
            .nth(1)
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
        assert_eq!(types.stack_at(&f_local).unwrap(), &[Type::Number]);
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

        // Don't use any of the type annotations for the inference. Only use
        // them to _check_ the inference later.
        let _ = type_annotations;
        let types = Types::infer(
            &syntax_tree,
            &bindings,
            &identifiers,
            &dependencies,
            TypeAnnotations::empty(),
        );

        (syntax_tree, types)
    }
}
