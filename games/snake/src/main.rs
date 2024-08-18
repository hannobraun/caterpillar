use std::{fs::File, io::Read};

use capi_compiler::{parse, tokenize};

pub fn main() -> anyhow::Result<()> {
    let mut source = String::new();
    File::open("games/snake/snake.capi")?.read_to_string(&mut source)?;
    let tokens = tokenize(source);
    let script = parse(tokens);

    let script = ron::to_string(&script).unwrap();
    println!("{script}");

    Ok(())
}
