//! # A basic code representation, mirroring the raw input text
//!
//! Tokens almost fully mirror the raw text that the developer inputs, only
//! ignoring whitespace. They are the first semi-structured code representation
//! that the compiler produces, laying the groundwork for parsing.

/// # A token
///
/// See [parent module][self].
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

    /// # A function name, represented
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
