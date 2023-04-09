use crate::cp::{self, Functions};

pub fn define() -> anyhow::Result<Functions> {
    let functions = cp::Functions::new();
    Ok(functions)
}
