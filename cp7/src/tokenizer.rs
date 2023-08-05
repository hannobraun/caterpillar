pub fn tokenize(code: &str) -> Vec<String> {
    code.split_whitespace().map(Into::into).collect()
}
