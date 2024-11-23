use std::{env, fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    if let Err(err) = env::var("FILES") {
        match err {
            env::VarError::NotPresent => {
                create_dummy_files()?;
            }
            env::VarError::NotUnicode(_) => {
                return Err(err.into());
            }
        }
    }

    println!("cargo::rerun-if-env-changed=FILES");

    Ok(())
}

fn create_dummy_files() -> anyhow::Result<()> {
    let out_dir_var = env::var("OUT_DIR")?;
    let out_dir_path = Path::new(&out_dir_var);

    let files = ["index.html"];

    for file in files {
        File::create(out_dir_path.join(file))?;
    }

    println!("cargo:rustc-env=FILES={}", out_dir_var);

    Ok(())
}
