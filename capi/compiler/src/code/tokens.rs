//! # A basic code representation, mirroring the raw input text
//!
//! Tokens almost fully mirror the raw text that the developer inputs, only
//! ignoring whitespace. They are the first semi-structured code representation
//! that the compiler produces, laying the groundwork for parsing.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Comment { text: String },
    Delimiter,

    KeywordEnd,
    KeywordFn,

    FunctionName { name: String },

    BranchStart,
    BranchBodyStart,

    Identifier { name: String },
    IntegerLiteral { value: i32 },
}
