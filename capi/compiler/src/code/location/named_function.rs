use crate::code::{Index, NamedFunction};

use super::located::HasLocation;

impl HasLocation for NamedFunction {
    type Location = Index<NamedFunction>;
}
