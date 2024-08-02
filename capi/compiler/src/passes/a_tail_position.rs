use crate::repr::syntax::{Expression, Function};

pub fn determine_tail_position(functions: &mut Vec<Function>) {
    for function in functions {
        analyze_block(&mut function.body);
    }
}

fn analyze_block(block: &mut [Expression]) {
    if let Some(Expression::Identifier {
        is_known_to_be_in_tail_position,
        ..
    }) = block.last_mut()
    {
        *is_known_to_be_in_tail_position = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::repr::syntax::{Expression, Script};

    use super::determine_tail_position;

    #[test]
    fn last_in_function() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.ident("not_tail").ident("tail");
        });

        determine_tail_position(&mut script.functions);

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
