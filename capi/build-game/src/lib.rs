use std::{str, time::SystemTime};

use capi_compiler::Compiler;
use capi_game_engine::host::GameEngineHost;
use capi_protocol::{updates::Code, Versioned};
use capi_watch::DebouncedChanges;
use tokio::{fs, sync::watch, task};

pub type CodeRx = watch::Receiver<Versioned<Code>>;

pub async fn build_game_once(game: &str) -> anyhow::Result<Code> {
    let mut compiler = Compiler::default();
    build_game_once_with_compiler(game, &mut compiler).await
}

pub async fn build_and_watch_game(
    game: impl Into<String>,
    mut changes: DebouncedChanges,
) -> anyhow::Result<CodeRx> {
    let game = game.into();

    let mut compiler = Compiler::default();
    let mut timestamp = Timestamp(0);

    let code = build_game_once_with_compiler(&game, &mut compiler).await?;

    timestamp.update();
    let (game_tx, game_rx) = watch::channel(Versioned {
        timestamp: timestamp.0,
        inner: code,
    });

    task::spawn(async move {
        while changes.wait_for_change().await {
            let code = build_game_once_with_compiler(&game, &mut compiler)
                .await
                .unwrap();

            timestamp.update();
            game_tx
                .send(Versioned {
                    timestamp: timestamp.0,
                    inner: code,
                })
                .unwrap();
        }
    });

    Ok(game_rx)
}

async fn build_game_once_with_compiler(
    game: &str,
    compiler: &mut Compiler,
) -> anyhow::Result<Code> {
    let path = format!("games/{game}/{game}.capi");

    let source = fs::read_to_string(path).await?;

    let (fragments, instructions, source_map) =
        compiler.compile::<GameEngineHost>(&source);

    Ok(Code {
        fragments,
        instructions,
        source_map,
    })
}

struct Timestamp(u64);

impl Timestamp {
    fn update(&mut self) {
        let timestamp = SystemTime::UNIX_EPOCH
            .elapsed()
            .expect(
                "Current system time should never be later than Unix epoch.",
            )
            .as_nanos()
            .try_into()
            .expect(
                "`u64` should be able to represent nanosecond timestamps \
                until the year 2554.",
            );

        let timestamp = if timestamp > self.0 {
            timestamp
        } else {
            // Due to various factors, the new timestamp isn't necessarily
            // larger than the previous one. We need it to be though, otherwise
            // we can't use it to distinguish new builds from previous ones.
            self.0 + 1
        };

        self.0 = timestamp;
    }
}
