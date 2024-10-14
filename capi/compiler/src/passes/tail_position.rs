use crate::{
    fragments::{Fragment, Function},
    syntax::Branch,
};

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
        fragments::{Fragment, FragmentIndexInBranchBody, Function},
        passes::{parse, tokenize},
    };

    use super::determine_tail_positions;

    #[test]
    fn tail_call_in_named_function() {
        // The last expression in a function should be marked as being in tail
        // position. Others should not be.

        let mut functions = tokenize_and_parse(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                }
            ",
        );

        determine_tail_positions(&mut functions);

        let (_, branch) = functions.remove(0).branches.pop_first().unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn tail_call_in_anonymous_function() {
        // The compiler pass that determines tail positions should step into
        // nested functions.

        let mut functions = tokenize_and_parse(
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

        determine_tail_positions(&mut functions);

        let mut function = functions.remove(0);
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

        let mut functions = tokenize_and_parse(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                        # This is a comment.
                }
            ",
        );

        determine_tail_positions(&mut functions);

        let mut function = functions.remove(0);
        let (_, branch) = function.branches.pop_first().unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    pub fn tokenize_and_parse(source: &str) -> Vec<Function> {
        let tokens = tokenize(source);
        parse(tokens)
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
