use crate::language::syntax::Syntax;

use super::evaluator::Evaluator;

pub fn update(syntax: &Syntax, evaluator: &mut Evaluator) {
    for ((old, _), (new, _)) in syntax.find_replaced_fragments() {
        evaluator.functions.replace(old, new);
    }
}
