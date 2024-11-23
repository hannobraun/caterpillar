use std::net::SocketAddr;

#[derive(clap::Parser)]
pub enum Args {
    Headless,
    Serve {
        /// Address to serve at
        #[arg(short, long, default_value = "127.0.0.1:34480")]
        address: SocketAddr,
    },
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
