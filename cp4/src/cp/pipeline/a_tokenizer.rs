pub fn tokenize(code: &str) -> impl Iterator<Item = String> + '_ {
    code.split_whitespace().map(String::from)
}
