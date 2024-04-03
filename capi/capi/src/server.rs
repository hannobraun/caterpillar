use std::{process::exit, thread};

pub fn start() {
    thread::spawn(|| {
        if let Err(err) = serve() {
            eprintln!("Server error: {err}");
            exit(1);
        }
    });
}

fn serve() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
