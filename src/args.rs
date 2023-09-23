use std::path::PathBuf;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();
    Ok(PathBuf::from(args.example))
}

#[derive(clap::Parser)]
struct Args {
    example: String,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
