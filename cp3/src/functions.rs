use crate::cp::{self, Functions};

pub fn define() -> anyhow::Result<Functions> {
    Ok(cp::Functions::new())
}
