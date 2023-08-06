use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn load(path: impl AsRef<Path>) -> io::Result<String> {
    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;
    Ok(code)
}
