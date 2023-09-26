use capi_core::{DataStackResult, Evaluator};

pub fn print(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let value = evaluator.data_stack.pop_any()?;
    println!("{}", value.kind);
    evaluator.data_stack.push(value);
    Ok(())
}
