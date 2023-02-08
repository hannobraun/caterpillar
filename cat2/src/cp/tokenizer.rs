pub type Tokens = Vec<String>;

pub fn tokenize(code: &str) -> Tokens {
    code.split_whitespace().map(String::from).collect()
}
