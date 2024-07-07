use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use capi_process::Value;

use crate::repr::syntax::{ExpressionKind, Location, Script};

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
                next: next_fragment.take(),
                payload,
                location: expression.location,
            };
            next_fragment = Some(fragment.id());
            fragments.insert(fragment.id(), fragment);
        }

        by_function.push(Function {
            name: function.name,
            args: function.args,
            fragments: FunctionFragments {
                first: next_fragment,
                inner: fragments,
            },
        });
    }

    Fragments {
        functions: script.functions.names,
        by_function,
    }
}

#[derive(Debug)]
pub struct Fragments {
    pub functions: BTreeSet<String>,
    pub by_function: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub fragments: FunctionFragments,
}

#[derive(Debug)]
pub struct FunctionFragments {
    first: Option<FragmentId>,
    inner: FunctionFragmentsInner,
}

impl FunctionFragments {
    pub fn remove_first(&mut self) -> Option<Fragment> {
        let first = self.first.take()?;
        let first = self
            .inner
            .remove(&first)
            .expect("`self.first` must be present in `self.inner`");

        self.first = first.next;

        Some(first)
    }
}

impl Iterator for FunctionFragments {
    type Item = Fragment;

    fn next(&mut self) -> Option<Self::Item> {
        self.remove_first()
    }
}

type FunctionFragmentsInner = BTreeMap<FragmentId, Fragment>;

#[derive(Debug)]
pub struct Fragment {
    pub next: Option<FragmentId>,
    pub payload: FragmentPayload,
    pub location: Location,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();
        self.payload.hash(&mut hasher);
        let hash = hasher.finalize();

        FragmentId { hash }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FragmentId {
    pub hash: blake3::Hash,
}

impl Ord for FragmentId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.as_bytes().cmp(other.hash.as_bytes())
    }
}

impl PartialOrd for FragmentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Binding { names: Vec<String> },
    Comment { text: String },
    Value(Value),
    Word { name: String },
}

impl FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            FragmentPayload::Binding { names } => {
                hasher.update(b"binding");

                for name in names {
                    hasher.update(name.as_bytes());
                }
            }
            FragmentPayload::Comment { text } => {
                hasher.update(b"comment");
                hasher.update(text.as_bytes());
            }
            FragmentPayload::Value(value) => {
                hasher.update(b"value");
                hasher.update(&value.0.to_le_bytes());
            }
            FragmentPayload::Word { name } => {
                hasher.update(b"word");
                hasher.update(name.as_bytes());
            }
        }
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
