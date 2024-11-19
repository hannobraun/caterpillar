mod args;
mod headless;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use self::args::Args;

    let args = Args::parse();

    match args {
        Args::Headless => {
            headless::run().await?;
        }
    }

    Ok(())
}
