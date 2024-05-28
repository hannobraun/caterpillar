use std::fs::File;

use chrono::Local;

fn main() -> anyhow::Result<()> {
    let date = Local::now().format("%Y-%m-%d");
    let path = format!("website/daily/{date}.md");

    File::create(&path)?;
    println!("Created {path}");

    Ok(())
}
