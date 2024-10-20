use std::{io, str, time::SystemTime};

use capi_compiler::{Compiler, CompilerOutput};
use capi_game_engine::host::GameEngineHost;
use capi_protocol::Versioned;
use capi_watch::DebouncedChanges;
use tokio::{fs, sync::watch, task};

pub type CodeRx = watch::Receiver<Versioned<CompilerOutput>>;

pub async fn build_game_once(game: &str) -> anyhow::Result<CompilerOutput> {
    let mut compiler = Compiler::default();
    let output = build_game_once_with_compiler(game, &mut compiler).await?;
    Ok(output)
}

pub async fn build_and_watch_game(
    game: impl Into<String>,
    mut changes: DebouncedChanges,
) -> anyhow::Result<CodeRx> {
    let game = game.into();

    let mut compiler = Compiler::default();
    let mut timestamp = Timestamp(0);

    println!("build:initial:start");
    let code = build_game_once_with_compiler(&game, &mut compiler).await?;
    println!("build:initial:finish");

    timestamp.update();
    let (game_tx, game_rx) = watch::channel(Versioned {
        timestamp: timestamp.0,
        inner: code,
    });

    let mut ignored_error = None;

    task::spawn(async move {
        while changes.wait_for_change().await {
            println!("build:change");
            let code = match build_game_once_with_compiler(&game, &mut compiler)
                .await
            {
                Ok(code) => code,
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => {
                        // Depending on the editor, this can happen while the
                        // file is being saved.
                        if let Some(old_err) = ignored_error {
                            panic!(
                                "Unexpected error: {err:?}\n\
                                \n\
                                Previously ignored an error, because a false \
                                positive was suspected: {old_err:?}"
                            );
                        } else {
                            ignored_error = Some(err);
                            continue;
                        }
                    }
                    _ => {
                        panic!("Unexpected error while building game: {err:?}");
                    }
                },
            };
            println!("build:finish");

            ignored_error = None;

            timestamp.update();

            let code = Versioned {
                timestamp: timestamp.0,
                inner: code,
            };
            if game_tx.send(code).is_err() {
                // Receiver dropped. We must be in the process of shutting down.
                return;
            }
        }
    });

    Ok(game_rx)
}

async fn build_game_once_with_compiler(
    game: &str,
    compiler: &mut Compiler,
) -> io::Result<CompilerOutput> {
    let path = format!("games/{game}/{game}.capi");
    let source = fs::read_to_string(path).await?;
    let output = compiler.compile::<GameEngineHost>(&source);

    Ok(output)
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
