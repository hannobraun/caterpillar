use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

pub fn load(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let path = path.as_ref();
    let code = load_inner(path)?;
    Ok(code)
}

fn load_inner(path: &Path) -> anyhow::Result<String> {
    let mut code = String::new();
    File::open(path)
        .with_context(|| format!("Opening script `{}`", path.display()))?
        .read_to_string(&mut code)?;
    Ok(code)
}
