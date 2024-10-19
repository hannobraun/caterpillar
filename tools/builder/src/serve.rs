use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    select,
};

use crate::build::UpdatesRx;

pub async fn start(mut updates: UpdatesRx) -> anyhow::Result<()> {
    let address = "localhost:34480";

    let mut current_server: Option<Child> = None;

    updates.mark_unchanged(); // make sure we enter the loop body immediately
    'updates: while let Ok(()) = updates.changed().await {
        let Some(serve_dir) = updates.borrow().clone() else {
            // The channel is initialized with `None`. After the initial build
            // has finished, it will always be `Some`.
            //
            // This means that if we make it here, the initial build has not
            // finished yet. Restart the loop to wait for that.
            continue;
        };

        println!();

        if let Some(mut server) = current_server.take() {
            println!("⏳ Killing previous instance of Caterpillar server...");
            server.kill().await?;
        }

        println!("⏳ Starting Caterpillar server...");
        println!();

        let mut new_server = Command::new("cargo")
            .arg("run")
            .args(["--package", "capi-server"])
            .arg("--")
            .args(["--address", address])
            .args(["--serve-dir", &serve_dir.display().to_string()])
            .kill_on_drop(true)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = new_server.stdout.take().expect(
            "Expecting stdio to be captured, according to configuration above",
        );
        let mut stdout = BufReader::new(stdout);

        current_server = Some(new_server);

        let mut line = String::new();
        while !line.starts_with("builder: ready") {
            line.clear();

            select! {
                result = stdout.read_line(&mut line) => {
                    result?;
                }
                _ = updates.changed() => {
                    updates.mark_changed();
                    continue 'updates;
                }
            }

            match line.trim() {
                "builder: ready" => {
                    println!();
                    println!("✅ Build is ready:");
                    println!();
                    println!("\t🚀 http://{address}/");
                    println!();
                    println!(
                        "================================================"
                    );
                    println!();
                }
                _ => {
                    continue;
                }
            }
        }
    }

    Ok(())
}
