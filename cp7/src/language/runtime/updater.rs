use crate::language::{
    syntax::Syntax,
    tokens::{AddressedToken, Tokens},
};

use super::evaluator::Evaluator;

pub fn update(
    old_tokens: &Tokens,
    new_tokens: &Tokens,
    syntax: &Syntax,
    evaluator: &mut Evaluator,
) {
    let common_token_left = search_common_token(
        old_tokens.left_to_right(),
        new_tokens.left_to_right(),
        |token| token.left_neighbor,
    );
    let common_token_right = search_common_token(
        old_tokens.right_to_left(),
        new_tokens.right_to_left(),
        |token| token.right_neighbor,
    );

    match common_token_left {
        Some(address) => {
            let token = &old_tokens
                .right_to_left
                .get(&address)
                .expect("Using address that I got from same map")
                .token;
            eprintln!("Common token on left side: {}", token);
        }
        None => {
            eprintln!("No common token on left side.");
        }
    }
    match common_token_right {
        Some(address) => {
            let token = &old_tokens
                .left_to_right
                .get(&address)
                .expect("Using address that I got from same map")
                .token;
            eprintln!("Common token on right side: {}", token);
        }
        None => {
            eprintln!("No common token on right side.");
        }
    }

    for ((old, _), (new, _)) in syntax.find_replaced_fragments() {
        evaluator.functions.replace(old, new);
    }
}

fn search_common_token<'r, T>(
    mut old_tokens: impl Iterator<Item = &'r AddressedToken>,
    mut new_tokens: impl Iterator<Item = &'r AddressedToken>,
    next: impl Fn(&AddressedToken) -> Option<T>,
) -> Option<T>
where
    T: Eq,
{
    let mut old_token_left = old_tokens.next();
    let mut new_token_left = new_tokens.next();

    let mut common_token_left = None;

    loop {
        let (Some(old), Some(new)) = (old_token_left, new_token_left) else {
            // We've reached the end of one of our token streams. Either we
            // found a commonality of not, but either way, the search is over.
            break;
        };

        if next(old) == next(new) {
            // We found a commonality!
            common_token_left = next(old);

            // Advance the old token, so we can check in the next loop iteration
            // whether there is a deeper commonality.
            old_token_left = old_tokens.next();

            continue;
        }

        // The current new token is not the same as the current old one. Advance
        // the new token, maybe we'll find a commonality yet.
        new_token_left = new_tokens.next();
    }

    common_token_left
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
