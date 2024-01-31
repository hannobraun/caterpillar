use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let script_path =
        env::var("CAPI_SCRIPT").map(Some).or_else(|err| match err {
            env::VarError::NotPresent => Ok(None),
            err @ env::VarError::NotUnicode(_) => Err(err),
        })?;

    let script = match script_path {
        Some(path) => load_script(&path)
            .with_context(|| format!("Loading script `{path}`"))?,
        None => String::from(r#":main { "Hello, world!" print } fn"#),
    };

    let out_dir = env::var_os("OUT_DIR").expect("Cargo did not set `OUT_DIR`");
    let generated = Path::new(&out_dir).join("script.rs");
    fs::write(
        generated,
        format!("const SCRIPT: &str = r##\"{script}\"##;\n"),
    )?;

    Ok(())
}

fn load_script(path: impl AsRef<Path>) -> io::Result<String> {
    let mut script = String::new();
    File::open(path)?.read_to_string(&mut script)?;
    Ok(script)
}
