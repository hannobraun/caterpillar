mod args;
mod data_stack;
mod pipeline;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;

    let code = pipeline::a_loader::load(example)?;
    let tokens = pipeline::b_tokenizer::tokenize(&code);
    let syntax_tree = pipeline::c_parser::parse(tokens)?;
    pipeline::d_evaluator::evaluate(syntax_tree)?;

    Ok(())
}
