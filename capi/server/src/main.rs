mod server;

use std::{path::PathBuf, str};

use capi_compiler::compiler::compile;
use capi_protocol::update::SourceCode;
use clap::Parser;
use tokio::process::Command;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

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

    server::start_server(args.address, args.serve_dir, source_code, bytecode)
        .await?;

    info!("`capi-server` shutting down.");
    Ok(())
}

/// Caterpillar server
#[derive(clap::Parser)]
pub struct Args {
    /// Address to serve at
    #[arg(short, long)]
    pub address: String,

    /// Directory to serve from
    #[arg(short, long)]
    pub serve_dir: PathBuf,
}
