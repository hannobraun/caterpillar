use std::fs::File;

use chrono::Local;

fn main() -> anyhow::Result<()> {
    let date = Local::now().format("%Y-%m-%d");
    let path = format!("website/content/daily/{date}.md");

    File::create_new(&path)?;
    println!();
    println!("\t{path}");
    println!();

    Ok(())
}
