//! # A basic code representation, mirroring the raw input
//!
//! Tokens almost fully mirror the raw text that the developer inputs, only
//! ignoring whitespace. They are the first semi-structured code representation
//! that the compiler produces, laying the groundwork for parsing.

mod repr;
mod tokenize;

pub use self::repr::{Token, Tokens};
