use std::{
    cmp::Ordering,
    collections::{BTreeSet, VecDeque},
};

use capi_process::Value;

use crate::syntax::{ExpressionKind, Location, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut by_function = Vec::new();

    for function in script.functions.inner {
        let mut fragments = VecDeque::new();
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
            fragments.push_front(fragment);
        }

        by_function.push(Function {
            name: function.name,
            args: function.args,
            fragments: FunctionFragments { inner: fragments },
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
    inner: VecDeque<Fragment>,
}

impl IntoIterator for FunctionFragments {
    type Item = <FragmentsIter as Iterator>::Item;
    type IntoIter = FragmentsIter;

    fn into_iter(self) -> Self::IntoIter {
        FragmentsIter { fragments: self }
    }
}

pub struct FragmentsIter {
    fragments: FunctionFragments,
}

impl Iterator for FragmentsIter {
    type Item = Fragment;

    fn next(&mut self) -> Option<Self::Item> {
        self.fragments.inner.pop_front()
    }
}

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
