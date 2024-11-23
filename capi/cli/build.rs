use std::{env, fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    let out_dir_var = env::var("OUT_DIR")?;
    let out_dir_path = Path::new(&out_dir_var);

    File::create(out_dir_path.join("index.html"))?;

    println!("cargo:rustc-env=FILES={}", out_dir_var);

    Ok(())
}
