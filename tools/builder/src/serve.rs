use std::{path::PathBuf, process::Stdio};

use capi_watch::Watcher;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    select,
};

use crate::build;

pub async fn start() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("capi").canonicalize()?;

    let watcher = Watcher::new(&crates_dir)?;
    let mut updates = build::start(watcher.changes);

    let mut current_server: Option<Child> = None;

    let Some(mut serve_dir) = updates.recv().await else {
        // The sender has been dropped, which means the process is shutting
        // down.
        return Ok(());
    };

    'updates: loop {
        println!();

        if let Some(mut server) = current_server.take() {
            println!("⏳ Killing previous instance of Caterpillar server...");
            server.kill().await?;
        }

        println!("⏳ Starting Caterpillar server...");
        println!();

        let mut new_server = Command::new("cargo")
            .arg("run")
            .args(["--package", "capi-cli"])
            .arg("--")
            .arg("serve")
            .env("FILES", serve_dir.display().to_string())
            .kill_on_drop(true)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = new_server.stdout.take().expect(
            "Expecting stdio to be captured, according to configuration above",
        );
        let mut stdout = BufReader::new(stdout);

        current_server = Some(new_server);

        let mut line = String::new();
        loop {
            line.clear();

            select! {
                result = stdout.read_line(&mut line) => {
                    result?;
                }
                update = updates.recv() => {
                    let Some(update) = update else {
                        // The sender has been dropped, which means the process
                        // is shutting down.
                        return Ok(());
                    };
                    serve_dir = update;
                    continue 'updates;
                }
            }

            print!("\t{line}");
        }
    }
}
