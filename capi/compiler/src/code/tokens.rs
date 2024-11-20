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
