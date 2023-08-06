pub mod a_loader;
pub mod b_tokenizer;
pub mod c_parser;
pub mod d_evaluator;

pub fn run(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let code = a_loader::load(path)?;
    let tokens = b_tokenizer::tokenize(&code);
    let (syntax, start) = c_parser::parse(tokens)?;

    if let Some(start) = start {
        let mut evaluator = d_evaluator::Evaluator::new(start);
        d_evaluator::evaluate_syntax(&mut evaluator, &syntax)?;
    }

    Ok(())
}
