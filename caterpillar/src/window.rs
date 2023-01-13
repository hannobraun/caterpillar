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
