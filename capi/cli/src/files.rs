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

    pub fn list_invalid(&self) -> Vec<&'static str> {
        let mut invalid_files = Vec::new();

        if self.index_html.is_empty() {
            invalid_files.push("index.html");
        }

        invalid_files
    }
}
