use std::{cell::UnsafeCell, panic, sync::Mutex};

use crate::{ffi_out::print, input::InputEvent, state::State};

// I want to know when I go beyond certain thresholds, just out of interest.
// Keeping the limits as low as possible here, to make sure I notice.
pub const CODE_SIZE: usize = 32;
pub const DATA_SIZE: usize = 8;

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// The virtual machine's code memory
pub static CODE: SharedMemory<CODE_SIZE> = SharedMemory::new();

/// The virtual machine's data memory
pub static DATA: SharedMemory<DATA_SIZE> = SharedMemory::new();

#[no_mangle]
pub extern "C" fn code_ptr() -> usize {
    &CODE as *const _ as usize
}

#[no_mangle]
pub extern "C" fn code_len() -> usize {
    CODE_SIZE
}

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
    }));

    // This is sound, as the reference is dropped at the end of this function.
    let data = unsafe { DATA.access_read() };

    let state = State::new(width, height, data);

    STATE
        .lock()
        .expect("Expected exclusive access")
        .insert(state)
        .render_target
        .buffer
        .as_mut_ptr();
}

#[no_mangle]
pub extern "C" fn get_render_target_buffer() -> *mut u8 {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.render_target.buffer.as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn get_render_target_buffer_len() -> usize {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.render_target.buffer.len()
}

#[no_mangle]
pub extern "C" fn on_input(key: i32) {
    let input = match key {
        0 => InputEvent::Up,
        1 => InputEvent::Left,
        2 => InputEvent::Down,
        3 => InputEvent::Right,
        _ => return,
    };

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.world.input.events.push_back(input);
}

#[no_mangle]
pub extern "C" fn on_frame(delta_time_ms: f64) {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    // This is sound, as the reference is dropped at the end of this function.
    let code = unsafe { CODE.access_read() };

    // This is sound, as the reference is dropped at the end of this function.
    let data = unsafe { DATA.access_write() };

    state.world.update(delta_time_ms);
    state
        .render_target
        .draw(&state.world, &mut state.evaluator, &code, data);
}

/// Virtual machine memory that is shared with the JavaScript host
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

    /// Gain read access to the shared memory
    ///
    /// This method is private, to prevent any access within Rust code that
    /// doesn't come from the top-level FFI functions.
    ///
    /// # Safety
    ///
    /// The caller must drop the returned reference before returning control to
    /// the JavaScript host.
    ///
    /// The caller must not call [`SharedMemory::access_write`], while the
    /// returned reference still exists.
    unsafe fn access_read(&self) -> &[u8] {
        &*self.inner.get()
    }

    /// Gain write access to the shared memory
    ///
    /// This method is private, to prevent any access within Rust code that
    /// doesn't come from the top-level FFI functions.
    ///
    /// # Safety
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
