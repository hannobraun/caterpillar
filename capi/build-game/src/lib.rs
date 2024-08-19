use std::str;

use capi_compiler::{compile, parse, tokenize};
use capi_game_engine::host::GameEngineHost;
use capi_protocol::{updates::Code, Versioned};
use capi_watch::DebouncedChanges;
use tokio::{fs, sync::watch, task};

pub type CodeRx = watch::Receiver<Versioned<Code>>;

pub async fn build_and_watch_game(
    game: impl Into<String>,
    mut changes: DebouncedChanges,
) -> anyhow::Result<CodeRx> {
    let game = game.into();

    let mut build_number = 0;

    let code = build_game_once(&game).await?;

    let (game_tx, game_rx) = watch::channel(Versioned {
        version: build_number,
        inner: code,
    });
    build_number += 1;

    task::spawn(async move {
        while changes.wait_for_change().await {
            dbg!("Change detected.");
            let source_code = build_game_once(&game).await.unwrap();
            dbg!("Game built.");
            game_tx
                .send(Versioned {
                    version: build_number,
                    inner: source_code,
                })
                .unwrap();

            build_number += 1;
        }
    });

    Ok(game_rx)
}

pub async fn build_game_once(game: &str) -> anyhow::Result<Code> {
    let path = format!("games/{game}/{game}.capi");

    let source = fs::read_to_string(path).await?;
    let tokens = tokenize(&source);
    let script = parse(tokens);

    let (fragments, instructions, source_map) =
        compile::<GameEngineHost>(script);

    Ok(Code {
        fragments,
        instructions,
        source_map,
    })
}
