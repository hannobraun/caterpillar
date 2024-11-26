use std::path::{Path, PathBuf};

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::files::FILES;

pub async fn deploy(
    games_path: PathBuf,
    target_path: PathBuf,
) -> anyhow::Result<()> {
    let mut games = fs::read_dir(&games_path).await?;

    while let Some(game) = games.next_entry().await? {
        if !game.file_type().await?.is_dir() {
            continue;
        }

        let game_dir = game.path();
        let path_within_games = game_dir.strip_prefix(&games_path)?;
        let target_path = target_path.join(path_within_games);

        prepare_directory(&target_path).await?;
        deploy_static_files(&target_path).await?;
    }

    Ok(())
}

async fn prepare_directory(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).await?;
    }
    fs::create_dir_all(path).await?;

    Ok(())
}

async fn deploy_static_files(path: &Path) -> anyhow::Result<()> {
    let static_files = ["index.html", "capi_host.wasm"];

    for name in static_files {
        let file_path = PathBuf::from(name);
        let Some(file) = FILES.get(&file_path) else {
            unreachable!(
                "Only accessing files that were statically included into the \
                binary, but `{name}` is not available",
            );
        };

        File::create(path.join(file_path))
            .await?
            .write_all(file)
            .await?;
    }

    Ok(())
}
