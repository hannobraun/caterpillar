use crate::code::syntax::Expression;

use super::Signature;

pub fn infer_expression(_: &Expression) -> Option<Signature> {
    None
}
