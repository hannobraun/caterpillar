use std::path::PathBuf;

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::files::FILES;

pub async fn deploy(path: PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&path).await?;

    let deployment = ["index.html", "capi_host.wasm"];

    for name in deployment {
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
