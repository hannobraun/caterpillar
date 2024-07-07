use std::collections::BTreeMap;

use crate::repr::{
    fragments::{
        Fragment, FragmentAddress, FragmentPayload, Fragments, Function,
        FunctionFragments,
    },
    syntax::{ExpressionKind, Script},
};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut by_function = Vec::new();

    for function in script.functions.inner {
        let mut fragments = BTreeMap::new();
        let mut next_fragment = None;

        for expression in function.expressions.into_iter().rev() {
            let payload = match expression.kind {
                ExpressionKind::Binding { names } => {
                    FragmentPayload::Binding { names }
                }
                ExpressionKind::Comment { text } => {
                    FragmentPayload::Comment { text }
                }
                ExpressionKind::Value(value) => FragmentPayload::Value(value),
                ExpressionKind::Word { name } => FragmentPayload::Word { name },
            };

            let fragment = Fragment {
                address: FragmentAddress {
                    next: next_fragment.take(),
                },
                payload,
                location: expression.location,
            };
            next_fragment = Some(fragment.id());
            fragments.insert(fragment.id(), fragment);
        }

        let first_fragment = next_fragment;
        by_function.push(Function {
            name: function.name,
            args: function.args,
            fragments: FunctionFragments::new(first_fragment, fragments),
        });
    }

    Fragments {
        functions: script.functions.names,
        by_function,
    }
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::{
        repr::syntax::Script, stages::syntax_to_fragments::FragmentPayload,
    };

    use super::syntax_to_fragments;

    #[test]
    fn basic() {
        let mut script = Script::default();
        script.function("inc", ["x"], |s| {
            s.w("x").v(1).w("add");
        });

        let mut fragments = syntax_to_fragments(script);

        let fragments = fragments
            .by_function
            .remove(0)
            .fragments
            .map(|fragment| fragment.payload)
            .collect::<Vec<_>>();
        assert_eq!(
            fragments,
            vec![
                FragmentPayload::Word {
                    name: String::from("x")
                },
                FragmentPayload::Value(Value(1)),
                FragmentPayload::Word {
                    name: String::from("add")
                }
            ]
        );
    }
}
