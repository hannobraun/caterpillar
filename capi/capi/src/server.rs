use std::{panic::catch_unwind, process::exit, thread};

use tokio::runtime::Runtime;

pub fn start() {
    thread::spawn(|| {
        let res = catch_unwind(|| {
            if let Err(err) = serve() {
                eprintln!("Server error: {err}");
                exit(1);
            }
        });

        if res.is_err() {
            exit(2);
        }
    });
}

fn serve() -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(serve_async())?;
    Ok(())
}

async fn serve_async() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
