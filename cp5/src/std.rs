use crate::cp;

pub fn define() -> anyhow::Result<cp::Functions> {
    Ok(cp::Functions::new())
}
