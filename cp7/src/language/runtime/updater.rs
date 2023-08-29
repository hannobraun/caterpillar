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

        let mut change_starts_within_function = false;
        let mut change_ends_within_function = false;

        for token in old_tokens.left_to_right_from(token_range.start) {
            if Some(token) == change_start {
                change_starts_within_function = true;
            }
        }
        for token in old_tokens.right_to_left_from(token_range.end) {
            if Some(token) == change_end {
                change_ends_within_function = true;
            }
        }

        match (change_starts_within_function, change_ends_within_function) {
            (true, true) => {}
            (false, false) => continue,
            _ => {
                todo!(
                    "Change overlaps function, but is not contained within it. \
                    Not supported yet."
                )
            }
        }

        eprintln!("Function changed");

        // So, I made it this far, but I'm honestly not sure what to do from
        // here. I think this might have been the wrong approach.
        //
        // To finish this, I would need to update the existing function to look
        // like the new one. Specifically, I would need to update the `start`
        // and `token_range` fields. At the very least, I don't have the
        // information I need to do that available here, but currently I'm a bit
        // hazy on *what* information I need in the first place.
        //
        // I will have to think about this more, but currently I suspect that
        // addressing the tokens and wanting to do all decisions based on that,
        // might have been the wrong approach. I still need to figure out which
        // syntax replaces which other syntax, and while I *might* be able to
        // use the tokens to do that, why didn't I uniquely address the syntax
        // instead?
        //
        // To proceed, I see the following options:
        //
        // 1. Keep going along this path. Figure out how to do the update based
        //    on the token replacement, and get whatever information I need to
        //    do that here.
        // 2. Uniquely address syntax fragments, figure out replacements, then
        //    do the update based on this information.
        // 3. Reconsider the general approach. Should I actually update the
        //    existing functions? Maybe, instead of updating the existing
        //    program, I can create a new program and inject my runtime data
        //    there? The I would "just" have to figure out, how to transform
        //    the data and call stacks to make them work with the new program.
        //
        // That last point would require a different approach to the language
        // design. Right now, function definition is completely dynamic, which
        // is probably both unnecessary and suboptimal. I could introduce a
        // distinction between run- and "compile"-time (would need a different
        // name, I guess, since there's no compilation), with function
        // definition being a "compile"-time thing.
        //
        // On the other hand, that wouldn't help me with anonymous functions, I
        // guess. So maybe it would just result in the language being more
        // complicated, and the same problem re-appearing in the next step
        // anyway.
    }

    for (old, new) in syntax.find_replaced_fragments() {
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
