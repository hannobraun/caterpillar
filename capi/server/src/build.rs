use std::str;

use capi_compiler::compiler::compile;
use capi_process::Bytecode;
use capi_protocol::update::SourceCode;
use capi_watch::DebouncedChanges;
use tokio::{process::Command, sync::watch, task};

pub type GameRx = watch::Receiver<(SourceCode, Bytecode)>;

pub async fn build_and_watch(
    mut changes: DebouncedChanges,
) -> anyhow::Result<GameRx> {
    let (source_code, bytecode) = build_once().await?;

    let (game_tx, game_rx) =
        tokio::sync::watch::channel((source_code, bytecode));

    task::spawn(async move {
        while changes.wait_for_change().await {
            dbg!("Change detected.");
            let (source_code, bytecode) = build_once().await.unwrap();
            game_tx.send((source_code, bytecode)).unwrap();
        }
    });

    Ok(game_rx)
}

async fn build_once() -> anyhow::Result<(SourceCode, Bytecode)> {
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

    Ok((source_code, bytecode))
}
