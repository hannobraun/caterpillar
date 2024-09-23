use crate::syntax::{Expression, Function};

pub fn determine_tail_positions(functions: &mut Vec<Function>) {
    for function in functions {
        analyze_function(function);
    }
}

fn analyze_function(function: &mut Function) {
    for branch in &mut function.branches {
        analyze_branch(&mut branch.body);
    }
}

fn analyze_branch(body: &mut [Expression]) {
    for expression in body.iter_mut().rev() {
        if let Expression::Comment { .. } = expression {
            continue;
        }

        if let Expression::Identifier {
            is_known_to_be_in_tail_position,
            ..
        } = expression
        {
            *is_known_to_be_in_tail_position = true;
        }

        break;
    }

    for expression in body {
        if let Expression::Function { function } = expression {
            analyze_function(function);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{Expression, Script};

    use super::determine_tail_positions;

    #[test]
    fn last_in_named_function() {
        let mut script = Script::default();
        script.function("f", |b| {
            b.branch(
                |p| p,
                |s| {
                    s.ident("not_tail").ident("tail");
                },
            )
        });

        determine_tail_positions(&mut script.functions);

        let mut function = script.functions.remove(0);
        let branch = function.branches.remove(0);
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn last_in_block() {
        let mut script = Script::default();
        script.function("f", |b| {
            b.branch(
                |p| p,
                |s| {
                    s.ident("a")
                        .fun(|b| {
                            b.branch(
                                |b| b,
                                |s| {
                                    s.ident("not_tail").ident("tail");
                                },
                            )
                        })
                        .ident("b");
                },
            )
        });

        determine_tail_positions(&mut script.functions);

        let mut function = script.functions.remove(0);
        let mut branch = function.branches.remove(0);
        let Expression::Function { function } = branch.body.remove(1) else {
            panic!("Expected block.");
        };
        let identifiers =
            function.branches.first().unwrap().body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn ignore_comments() {
        let mut script = Script::default();
        script.function("f", |b| {
            b.branch(
                |p| p,
                |s| {
                    s.ident("not_tail").ident("tail").c("This is a comment.");
                },
            )
        });

        determine_tail_positions(&mut script.functions);

        let mut function = script.functions.remove(0);
        let branch = function.branches.remove(0);
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    trait ToIdentifiers {
        fn to_identifiers(&self) -> Vec<(&str, bool)>;
    }

    impl ToIdentifiers for Vec<Expression> {
        fn to_identifiers(&self) -> Vec<(&str, bool)> {
            self.iter()
                .filter_map(|expression| {
                    if let Expression::Identifier {
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
