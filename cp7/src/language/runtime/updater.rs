use crate::language::{
    syntax::{Syntax, SyntaxToTokens},
    tokens::{
        AddressedToken, LeftNeighborAddress, RightNeighborAddress, Token,
        Tokens,
    },
};

use super::evaluator::Evaluator;

pub fn update(
    old_tokens: &Tokens,
    new_tokens: &Tokens,
    syntax: &Syntax,
    syntax_to_tokens: &SyntaxToTokens,
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

    eprint!("Updated token in range: ");
    print_token_range_from_addresses(
        common_token_left,
        common_token_right,
        old_tokens,
    );

    for function in evaluator.functions.user_defined_mut() {
        if let Some(body) = &function.body.start {
            let token_range = syntax_to_tokens.get(body);

            let (left, right) = match token_range {
                Some(token_range) => (
                    Some(&token_range.start.token),
                    Some(&token_range.end.token),
                ),
                None => (None, None),
            };

            eprint!("Token range of user-defined function: ");
            print_token_range_from_tokens(left, right);
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
            // found a commonality or not, but either way, the search is over.
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

fn print_token_range_from_addresses(
    left: Option<LeftNeighborAddress>,
    right: Option<RightNeighborAddress>,
    tokens: &Tokens,
) {
    let left = left.map(|address| {
        &tokens
            .right_to_left
            .get(&address)
            .expect("Using address that I got from same map")
            .token
    });
    let right = right.map(|address| {
        &tokens
            .left_to_right
            .get(&address)
            .expect("Using address that I got from same map")
            .token
    });
    print_token_range_from_tokens(left, right);
}

fn print_token_range_from_tokens(left: Option<&Token>, right: Option<&Token>) {
    if let Some(token) = left {
        eprint!("{token}");
    }
    eprint!(" ... ");
    if let Some(token) = right {
        eprintln!("{token}");
    }
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
