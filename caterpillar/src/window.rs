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
        // This is the initial default. Good enough for now, but should be
        // adapted to always return the current size.
        [300, 150]
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
