mod d_evaluator;
mod stages;

pub fn run(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let mut syntax = crate::syntax::Syntax::new();

    let code = stages::a_loader::load(path)?;
    let tokens = stages::b_tokenizer::tokenize(&code);
    let start = stages::c_parser::parse(tokens, &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = d_evaluator::Evaluator::new(start);
        while evaluator.step(&syntax)? {}
    }

    Ok(())
}
