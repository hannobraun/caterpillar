use gloo::utils::document;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
    WebDisplayHandle, WebWindowHandle,
};

pub struct Window {
    id: u32,
}

impl Window {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn size(&self) -> [u32; 2] {
        let canvas = document()
            .query_selector(&format!("[data-raw-handle=\"{}\"]", self.id))
            .expect("Error selecting canvas")
            .expect("Expected to find canvas in the DOM");

        let size = [canvas.client_width(), canvas.client_height()];
        size.map(|size| {
            size.try_into()
                .expect("Did not expect negative element size")
        })
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let handle = WebDisplayHandle::empty();
        RawDisplayHandle::Web(handle)
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = WebWindowHandle::empty();
        handle.id = self.id;

        RawWindowHandle::Web(handle)
    }
}
