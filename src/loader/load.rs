use std::{fs::File, io::Read, path::Path};

pub fn load(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let path = path.as_ref();

    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;

    Ok(code)
}
