use crate::language::syntax::Syntax;

use super::evaluator::Evaluator;

pub fn update(syntax: &Syntax, evaluator: &mut Evaluator) {
    for ((old, _), (new, _)) in syntax.find_replaced_fragments() {
        evaluator.functions.replace(old, new);
    }
}

#[cfg(test)]
mod tests {
    use crate::language::runtime::{
        functions::Function, interpreter::Interpreter,
    };

    #[test]
    fn update_at_beginning_of_named_function() -> anyhow::Result<()> {
        let original = ":f { 1 + } fn";
        let updated = ":f { 2 + } fn";

        let mut interpreter = Interpreter::new(original)?;
        while interpreter.step()?.in_progress() {}

        let f_original = extract_f(&interpreter)?;

        interpreter.update(updated)?;
        let f_updated = extract_f(&interpreter)?;

        assert_ne!(f_original, f_updated);

        fn extract_f(
            interpreter: &Interpreter,
        ) -> anyhow::Result<blake3::Hash> {
            let function = interpreter.evaluator.functions.resolve("f")?;
            let Function::UserDefined { body } = function else {
                panic!("Just defined function, but somehow not user-defined");
            };
            let handle =
                body.0.expect("Function not empty, but body has no syntax");

            Ok(handle.hash)
        }

        Ok(())
    }
}
