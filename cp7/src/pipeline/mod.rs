mod a_loader;
mod b_tokenizer;
mod c_parser;
mod d_evaluator;

pub fn run(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let mut syntax = crate::syntax::Syntax::new();

    let code = a_loader::load(path)?;
    let tokens = b_tokenizer::tokenize(&code);
    let start = c_parser::parse(tokens, &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = d_evaluator::Evaluator::new(start);
        while evaluator.step(&syntax)? {}
    }

    Ok(())
}
