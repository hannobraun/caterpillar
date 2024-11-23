pub const FILES: Files = Files::new();

#[derive(Debug)]
pub struct Files {
    pub index_html: &'static [u8],
}

impl Files {
    pub const fn new() -> Self {
        Self {
            index_html: include_bytes!(concat!(env!("FILES"), "/index.html")),
        }
    }
}
