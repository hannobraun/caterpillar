pub mod a_loader;
pub mod b_tokenizer;
pub mod c_parser;
pub mod d_evaluator;

pub fn run(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let code = a_loader::load(path)?;
    let tokens = b_tokenizer::tokenize(&code);
    let (syntax, syntax_tree) = c_parser::parse(tokens)?;
    d_evaluator::evaluate(syntax_tree.first, syntax)?;

    Ok(())
}
