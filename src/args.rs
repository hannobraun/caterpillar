use std::path::PathBuf;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();
    Ok(args.example)
}

#[derive(clap::Parser)]
struct Args {
    example: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
