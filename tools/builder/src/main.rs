mod build;
mod pipeline;
mod serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;

    tracing_subscriber::fmt().init();

    let args = Args::parse();
    match args.command {
        None | Some(Command::Serve) => {
            pipeline::pipeline().await?;
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    Serve,
}
