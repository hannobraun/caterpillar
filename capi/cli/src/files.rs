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

        let files = [(self.index_html, "index.html")];

        for (file, name) in files {
            if file.is_empty() {
                invalid_files.push(name);
            }
        }

        invalid_files
    }
}
