use crate::language::repr::eval::fragments::Replacement;

use super::evaluator::Evaluator;

pub fn update(replacements: Vec<Replacement>, evaluator: &mut Evaluator) {
    for Replacement { old, new } in replacements {
        evaluator.functions.replace(old, new);
    }
}

