use std::collections::VecDeque;

use super::tokenize::tokenize;

/// # The tokens in a script
///
/// See [parent module](super).
pub struct Tokens {
    inner: VecDeque<Token>,
}

impl Tokens {
    /// # Tokenize the provided input
    ///
    /// Takes raw text, as input by the developer, and creates its tokenized
    /// form.
    pub fn tokenize(input: &str) -> Self {
        let tokens = tokenize(input);
        Self {
            inner: tokens.into(),
        }
    }

    /// # Peek at the next token without taking it
    pub fn peek(&self) -> Option<&Token> {
        self.inner.front()
    }

    /// # Take the next token
    pub fn take(&mut self) -> Option<Token> {
        self.inner.pop_front()
    }
}

/// # A token
///
/// See [parent module](super).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// # A comment
    ///
    /// ## Implementation Note
    ///
    /// This is actually _one line_ of a comment. Multi-line comments are thus
    /// represented by multiple tokens.
    ///
    /// I don't know if that's good. It seems that multi-line comments should
    /// end up as a single unit at some point, as that would provide flexibility
    /// in displaying them. I'm not sure though, if that should already happen
    /// in the tokenizer, of if that's a task for parsing.
    Comment {
        /// # The contents of the comment
        text: String,
    },

    /// # A delimiter in a list, rendered as `,`
    Delimiter,

    /// # The `end` keyword
    KeywordEnd,

    /// # The `fn` keyword
    KeywordFn,

    /// # A function name
    FunctionName {
        /// # The name of the function
        name: String,
    },

    /// # The start of a branch, rendered as `\``
    BranchStart,

    /// # The start of the body of a branch, rendered as `->`
    BranchBodyStart,

    /// # An identifier
    Identifier {
        /// # The name of the identifier
        name: String,
    },

    /// # An integer literal
    IntegerLiteral {
        /// # The value of the integer literal
        value: i32,
    },
}
