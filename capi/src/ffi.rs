use std::{cell::UnsafeCell, sync::Mutex};

use crate::{state::RuntimeState, tiles::NUM_TILES};

pub static STATE: StaticRuntimeState = StaticRuntimeState {
    inner: Mutex::new(None),
};

pub static TILES: SharedMemory<NUM_TILES> = SharedMemory::new();

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.inner.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    let mut state = STATE.inner.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    let Some(display) = state.display.as_mut() else {
        // Display not initialized yet.
        return;
    };

    let tiles = unsafe { TILES.access_write() };

    display.handle_effects(&mut state.input, &mut state.runner, tiles);
    display.render(tiles);
}

pub struct StaticRuntimeState {
    pub inner: Mutex<Option<RuntimeState>>,
}

/// # Virtual machine memory that is shared with the JavaScript host
///
/// ## Safety
///
/// We are in a single-threaded context. Shared memory is only accessed by top-
/// level FFI functions in this module and the JavaScript host. Since neither of
/// those can run concurrently, this doesn't constitute concurrent access.
///
/// As a consequence, access is sound, as long as no reference to this static is
/// lives longer than the local scope of the FFI function that creates it.
#[repr(transparent)]
pub struct SharedMemory<const SIZE: usize> {
    inner: UnsafeCell<[u8; SIZE]>,
}

impl<const SIZE: usize> SharedMemory<SIZE> {
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new([0; SIZE]),
        }
    }

    /// # Gain write access to the shared memory
    ///
    /// This method is private, to prevent any access within Rust code that
    /// doesn't come from the top-level FFI functions.
    ///
    /// ## `&self` argument
    ///
    /// This method returns a mutable reference, despite only requiring `&self`.
    /// This is fine, as method is `unsafe` and the requirements that come from
    /// this are documented.
    ///
    /// If this took `&mut self`, the `SharedMemory` would need to live in a
    /// `static mut`, which would have the same pitfalls, and more. With the
    /// current design, `SharedMemory` can live in a non-`mut` `static`.
    ///
    /// ## Safety
    ///
    /// The caller must drop the returned reference before returning control to
    /// the JavaScript host.
    ///
    /// The caller must not call [`SharedMemory::access_write`] again, while the
    /// returned reference still exists.
    #[allow(clippy::mut_from_ref)] // it's `unsafe` and well-documented
    unsafe fn access_write(&self) -> &mut [u8] {
        &mut *self.inner.get()
    }
}

// Safe to implement, since with WebAssembly, this lives in a single-threaded
// context.
unsafe impl<const SIZE: usize> Sync for SharedMemory<SIZE> {}
