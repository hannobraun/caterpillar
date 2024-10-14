use crate::code::{Branch, Fragment, Function};

pub fn determine_tail_positions(functions: &mut Vec<Function>) {
    for function in functions {
        analyze_function(function);
    }
}

fn analyze_function(function: &mut Function) {
    for branch in function.branches.values_mut() {
        analyze_branch(branch);
    }
}

fn analyze_branch(branch: &mut Branch) {
    for expression in branch.body.values_mut().rev() {
        if let Fragment::Comment { .. } = expression {
            continue;
        }

        if let Fragment::UnresolvedIdentifier {
            is_known_to_be_in_tail_position,
            ..
        } = expression
        {
            *is_known_to_be_in_tail_position = true;
        }

        break;
    }

    for expression in branch.body.values_mut() {
        if let Fragment::Function { function } = expression {
            analyze_function(function);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        code::{Fragment, FragmentIndexInBranchBody, NamedFunctions},
        passes::{parse, tokenize},
    };

    #[test]
    fn tail_call_in_named_function() {
        // The last expression in a function should be marked as being in tail
        // position. Others should not be.

        let functions = determine_tail_positions(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                }
            ",
        );

        let (_, branch) = functions
            .into_functions()
            .next()
            .unwrap()
            .branches
            .pop_first()
            .unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn tail_call_in_anonymous_function() {
        // The compiler pass that determines tail positions should step into
        // nested functions.

        let functions = determine_tail_positions(
            r"
                f: {
                    \ ->
                        a
                        {
                            \ ->
                                not_tail
                                tail
                        }
                        b
                }
            ",
        );

        let mut function = functions.into_functions().next().unwrap();
        let (_, branch) = function.branches.pop_first().unwrap();
        let Fragment::Function { function } =
            branch.body.values().nth(1).unwrap()
        else {
            panic!("Expected block.");
        };
        let identifiers = function
            .branches
            .first_key_value()
            .map(|(_, branch)| branch)
            .unwrap()
            .body
            .to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn comment_after_tail_call() {
        // A comment being located after a tail call should not confuse the
        // analysis.

        let functions = determine_tail_positions(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                        # This is a comment.
                }
            ",
        );

        let mut function = functions.into_functions().next().unwrap();
        let (_, branch) = function.branches.pop_first().unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    pub fn determine_tail_positions(source: &str) -> NamedFunctions {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        super::determine_tail_positions(&mut functions);

        let mut named_functions = NamedFunctions::default();
        for function in functions {
            named_functions.insert(function);
        }

        named_functions
    }

    trait ToIdentifiers {
        fn to_identifiers(&self) -> Vec<(&str, bool)>;
    }

    impl ToIdentifiers for BTreeMap<FragmentIndexInBranchBody, Fragment> {
        fn to_identifiers(&self) -> Vec<(&str, bool)> {
            self.values()
                .filter_map(|expression| {
                    if let Fragment::UnresolvedIdentifier {
                        name,
                        is_known_to_be_in_tail_position,
                        ..
                    } = expression
                    {
                        Some((name.as_str(), *is_known_to_be_in_tail_position))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }
}
