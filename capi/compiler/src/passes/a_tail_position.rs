use crate::repr::syntax::{Expression, Function};

pub fn determine_tail_positions(functions: &mut Vec<Function>) {
    for function in functions {
        analyze_block(&mut function.body);
    }
}

fn analyze_block(block: &mut [Expression]) {
    for expression in block.iter_mut().rev() {
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

    for expression in block {
        if let Expression::Block { body, .. } = expression {
            analyze_block(body);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repr::syntax::{Expression, Script};

    use super::determine_tail_positions;

    #[test]
    fn last_in_function() {
        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("not_tail").ident("tail");
            },
        );

        determine_tail_positions(&mut script.functions);

        let function = script.functions.remove(0);
        let identifiers = function.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn last_in_block() {
        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("a")
                    .block(|s| {
                        s.ident("not_tail").ident("tail");
                    })
                    .ident("b");
            },
        );

        determine_tail_positions(&mut script.functions);

        let mut function = script.functions.remove(0);
        let Expression::Block { body: block, .. } = function.body.remove(1)
        else {
            panic!("Expected block.");
        };
        let identifiers = block.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn ignore_comments() {
        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("not_tail").ident("tail").c("This is a comment.");
            },
        );

        determine_tail_positions(&mut script.functions);

        let function = script.functions.remove(0);
        let identifiers = function.body.to_identifiers();
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
