pub const FILES: Files = Files::new();

#[derive(Debug)]
pub struct Files {
    pub capi_debugger_bg_wasm: &'static [u8],
    pub capi_debugger_js: &'static [u8],
    pub capi_host_wasm: &'static [u8],
    pub index_html: &'static [u8],
    pub tailwind_js: &'static [u8],
}

impl Files {
    pub const fn new() -> Self {
        Self {
            capi_debugger_bg_wasm: include_bytes!(concat!(
                env!("FILES"),
                "/capi-debugger_bg.wasm"
            )),
            capi_debugger_js: include_bytes!(concat!(
                env!("FILES"),
                "/capi-debugger.js"
            )),
            capi_host_wasm: include_bytes!(concat!(
                env!("FILES"),
                "/capi_host.wasm"
            )),
            index_html: include_bytes!(concat!(env!("FILES"), "/index.html")),
            tailwind_js: include_bytes!(concat!(env!("FILES"), "/tailwind.js")),
        }
    }

    pub fn list_invalid(&self) -> Vec<&'static str> {
        let mut invalid_files = Vec::new();

        let files = [
            (self.capi_debugger_bg_wasm, "capi-debugger_bg.wasm"),
            (self.capi_debugger_js, "capi-debugger.js"),
            (self.capi_host_wasm, "capi_host.wasm"),
            (self.index_html, "index.html"),
            (self.tailwind_js, "tailwind.js"),
        ];

        for (file, name) in files {
            if file.is_empty() {
                invalid_files.push(name);
            }
        }

        invalid_files
    }
}
