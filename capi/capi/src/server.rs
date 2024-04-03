use std::{panic::catch_unwind, process::exit, thread};

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
    println!("Hello, world!");
    Ok(())
}
