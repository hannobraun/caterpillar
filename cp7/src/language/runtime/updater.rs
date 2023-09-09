use crate::language::repr::eval::fragments::{Fragments, Replacement};

use super::evaluator::Evaluator;

pub fn update(fragments: &mut Fragments, evaluator: &mut Evaluator) {
    for Replacement { old, new } in fragments.take_replacements() {
        evaluator.functions.replace(old, new);
    }
}

#[cfg(test)]
mod tests {
    use crate::language::runtime::interpreter::Interpreter;

    #[test]
    fn update_at_beginning_of_named_function() -> anyhow::Result<()> {
        let original = ":f { 1 ping f } fn f";
        let updated = ":f { 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_in_middle_of_named_function() -> anyhow::Result<()> {
        let original = ":f { 1 1 ping f } fn f";
        let updated = ":f { 1 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let original = ":f { 1 1 ping f } fn f";
        let updated = ":f { 1 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }
}
