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

    // The initial value in the channel (it is initialized with `None`) is
    // considered to be "seen". This means the call to `changed` will wait for
    // an unseen value, which is the result of the initial build.
    //
    // This means that we're going to wait here until the initial build has
    // finished, before entering the loop.
    'updates: while let Ok(()) = updates.changed().await {
        let serve_dir = updates.borrow().clone().expect(
            "Should not have entered the loop until the result of the initial \
            build was available. After that, the channel should always contain \
            `Some`.",
        );

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
