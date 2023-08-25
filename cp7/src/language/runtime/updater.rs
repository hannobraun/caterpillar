use crate::language::{
    syntax::{Syntax, SyntaxToTokens},
    tokens::{Token, TokenAddress, Tokens},
};

use super::evaluator::Evaluator;

pub fn update(
    old_tokens: &Tokens,
    new_tokens: &Tokens,
    syntax: &Syntax,
    _syntax_to_tokens: &SyntaxToTokens,
    evaluator: &mut Evaluator,
) {
    let change_start = search_common_token(
        old_tokens.left_to_right(),
        new_tokens.left_to_right(),
        |a, b| a.as_left_neighbor == b.as_left_neighbor,
    );
    let change_end = search_common_token(
        old_tokens.right_to_left(),
        new_tokens.right_to_left(),
        |a, b| a.as_right_neighbor == b.as_right_neighbor,
    );

    eprint!("Updated token in range: ");
    print_token_range_from_addresses(change_start, change_end, old_tokens);

    for function in evaluator.functions.user_defined_mut() {
        let token_range = &function.body.token_range;

        for token in old_tokens.left_to_right_from(token_range.start) {
            if Some(token) == change_start {
                eprintln!("Change starts within function.");
            }
        }
        for token in old_tokens.right_to_left_from(token_range.end) {
            if Some(token) == change_end {
                eprintln!("Change ends within function.");
            }
        }
    }

    for ((old, _), (new, _)) in syntax.find_replaced_fragments() {
        evaluator.functions.replace(old, new);
    }
}

fn search_common_token(
    mut old_tokens: impl Iterator<Item = TokenAddress>,
    mut new_tokens: impl Iterator<Item = TokenAddress>,
    relevant_address_is_equal: impl Fn(&TokenAddress, &TokenAddress) -> bool,
) -> Option<TokenAddress> {
    let mut common_token = None;

    let mut current_old = old_tokens.next();
    let mut current_new = new_tokens.next();

    loop {
        let (Some(old), Some(new)) = (current_old, current_new) else {
            // We've reached the end of one of our token streams. Whether we
            // found a commonality or not, either way, the search is over.
            break;
        };

        if relevant_address_is_equal(&old, &new) {
            // We found a commonality!
            common_token = Some(old);

            // Advance the old token, and check in the next loop iteration
            // whether there is a deeper commonality.
            current_old = old_tokens.next();
            continue;
        }

        // The current new token is not the same as the current old one. Advance
        // the new token, maybe we'll find a commonality yet.
        current_new = new_tokens.next();
    }

    common_token
}

fn print_token_range_from_addresses(
    left: Option<TokenAddress>,
    right: Option<TokenAddress>,
    tokens: &Tokens,
) {
    let left = left.map(|address| tokens.by_address.get(&address).unwrap());
    let right = right.map(|address| tokens.by_address.get(&address).unwrap());
    print_token_range_from_tokens(left, right);
}

fn print_token_range_from_tokens(left: Option<&Token>, right: Option<&Token>) {
    if let Some(token) = left {
        eprint!("{token}");
    }
    eprint!(" ... ");
    if let Some(token) = right {
        eprint!("{token}");
    }
    eprintln!();
}

#[cfg(test)]
mod tests {
    use crate::language::runtime::{
        functions::{self, Function},
        interpreter::Interpreter,
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
            let Function::UserDefined(functions::UserDefined { body }) =
                function
            else {
                panic!("Just defined function, but somehow not user-defined");
            };
            let handle = body
                .start
                .expect("Function not empty, but body has no syntax");

            Ok(handle.hash)
        }

        Ok(())
    }
}
