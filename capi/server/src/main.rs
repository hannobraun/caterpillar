mod args;
mod server;

use std::str;

use capi_compiler::compiler::compile;
use capi_protocol::update::SourceCode;
use tokio::process::Command;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = args::Args::parse();

    let script = Command::new("cargo")
        .arg("run")
        .args(["--package", "snake"])
        .output()
        .await?
        .stdout;
    let script = str::from_utf8(&script).unwrap();
    let script = ron::from_str(script).unwrap();

    let (bytecode, source_map) = compile(&script);
    let source_code = SourceCode {
        functions: script.functions,
        source_map,
    };

    server::start(args.address, args.serve_dir, source_code, bytecode).await?;

    info!("`capi-server` shutting down.");
    Ok(())
}
