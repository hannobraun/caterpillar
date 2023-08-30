use enum_variant_type::EnumVariantType;

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumVariantType)]
#[evt(module = "token")]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Number(i64),
    Symbol(String),
    Word(String),
}
