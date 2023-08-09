use crate::language::syntax::Syntax;

use super::evaluator::Evaluator;

pub struct Interpreter {
    pub syntax: Syntax,
    pub evaluator: Evaluator,
}
