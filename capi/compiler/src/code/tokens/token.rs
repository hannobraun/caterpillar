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

    /// # A keyword
    Keyword(Keyword),

    /// # A punctuator
    Punctuator(Punctuator),
}

/// # Keywords
///
/// A keyword is a specific word with special meaning in the language, that is
/// delimited by whitespace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    /// # The `end` keyword
    End,

    /// # The `fn` keyword
    Fn,
}

/// # Punctuators
///
/// A punctuator is a token with syntactic and semantic meaning to the compiler,
/// that in itself is never an expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Punctuator {
    /// # A delimiter in a list, rendered as `,`
    Delimiter,

    /// # A token that introduces a new syntactic element, rendered as `:`
    ///
    /// This token is used to separate the new syntactic element from the
    /// previous one, that it relates to.
    Introducer,

    /// # The start of a branch, rendered as `\`
    BranchStart,

    /// # An indication that a thing is transformed to another, rendered as `->`
    ///
    /// Shows up in the form of `a -> b`, where `a` could be the parameter list
    /// of a branch and `b` its body; or in a function type, with `a` as input
    /// and `b` as the output types.
    Transformer,

    /// # A token that ends an expression, where necessary, rendered as `.`
    Terminator,
}
