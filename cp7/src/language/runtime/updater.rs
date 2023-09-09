use crate::language::repr::eval::fragments::Replacement;

use super::evaluator::Evaluator;

pub fn update(replacements: Vec<Replacement>, evaluator: &mut Evaluator) {
    for Replacement { old, new } in replacements {
        evaluator.functions.replace(old, new);
    }
}

#[cfg(test)]
mod tests {
    use crate::language::runtime::interpreter::Interpreter;

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let original = ":f { 1 1 ping f } fn f";
        let updated = ":f { 1 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_functions() -> anyhow::Result<()> {
        let original = "
            :loop { f loop } fn
            :f { 1 ping } fn
            :g { 1 ping } fn
            loop";
        let updated = "
            :loop { g loop } fn
            :f { 2 ping } fn
            :g { 1 ping } fn
            loop";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

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
