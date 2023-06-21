use std::convert::Infallible;

use crate::cp::syntax::SyntaxElement;

use super::{stage_input::StageInputReader, PipelineError};

pub fn analyze(
    mut syntax_elements: StageInputReader<SyntaxElement>,
) -> Result<Expression, PipelineError<Infallible>> {
    let expression =
        Expression::RawSyntaxElement(syntax_elements.read()?.clone());
    syntax_elements.take();
    Ok(expression)
}

pub enum Expression {
    RawSyntaxElement(SyntaxElement),
}
