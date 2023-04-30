pub fn tokenize(code: &str) -> impl Iterator<Item = &str> {
    code.split_whitespace()
}
