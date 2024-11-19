#[derive(clap::Parser)]
pub enum Args {
    Headless,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
