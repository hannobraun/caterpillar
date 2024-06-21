use std::{cell::UnsafeCell, collections::VecDeque, sync::Mutex};

use crate::state::RuntimeState;

pub static STATE: Mutex<Option<RuntimeState>> = Mutex::new(None);

/// The size of the updates buffer
///
/// This is a ridiculous 1 MiB large. It should be possible to make this much
/// smaller, but for now, we're using a very space-inefficient serialization
/// format.
const UPDATES_BUFFER_SIZE: usize = 1024 * 1024;

/// The buffer that is used to transfer updates _to_ the host
static UPDATES_TX: SharedFrameBuffer<UPDATES_BUFFER_SIZE> =
    SharedFrameBuffer::new();

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.update();

    for update in state.updates.take_queued_updates() {
        // Sound, because the reference is dropped before we call the method
        // again or we give back control to the host.
        let buffer = unsafe { UPDATES_TX.access_write() };
        buffer.write_frame(&update);

        let update = buffer.read_frame().to_vec();
        state.updates_tx.send(update).unwrap();
    }
}

/// # A buffer that is shared with the JavaScript host
///
/// ## Safety
///
/// This data structure is designed for use in WebAssembly. It is _unsound_ to
/// use it in a multi-threaded context.
#[repr(transparent)]
pub struct SharedFrameBuffer<const SIZE: usize> {
    inner: UnsafeCell<FrameBuffer<SIZE>>,
}

impl<const SIZE: usize> SharedFrameBuffer<SIZE> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(FrameBuffer::new()),
        }
    }

    /// # Gain write access to the shared buffer
    ///
    /// ## `&self` argument
    ///
    /// This method returns a mutable reference, despite only requiring `&self`.
    /// This is fine, as the method is `unsafe` and the requirements that derive
    /// from this are documented.
    ///
    /// If this took `&mut self`, the `SharedBuffer` would need to live in a
    /// `static mut`, which would have the same pitfalls and more. With the
    /// current design, `SharedBuffer` can live in a non-`mut` `static`.
    ///
    /// ## Safety
    ///
    /// The caller must drop the returned reference before giving back control
    /// to the JavaScript host.
    ///
    /// The caller must not call this method again, while the returned reference
    /// still exists.
    #[allow(clippy::mut_from_ref)] // function is `unsafe` and well-documented
    pub unsafe fn access_write(&self) -> &mut FrameBuffer<SIZE> {
        &mut *self.inner.get()
    }
}

// Safe to implement, since with WebAssembly, this lives in a single-threaded
// context.
unsafe impl<const SIZE: usize> Sync for SharedFrameBuffer<SIZE> {}

pub struct FrameBuffer<const SIZE: usize> {
    buffer: [u8; SIZE],
    frames: VecDeque<BufferFrame>,
}

impl<const SIZE: usize> FrameBuffer<SIZE> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; SIZE],
            frames: VecDeque::new(),
        }
    }

    pub fn write_frame(&mut self, data: &[u8]) {
        let next_free =
            self.frames.back().copied().unwrap_or_default().ends_before;

        let new_frame = BufferFrame {
            starts_at: next_free,
            ends_before: next_free + data.len(),
        };

        self.buffer[new_frame.starts_at..new_frame.ends_before]
            .copy_from_slice(data);

        self.frames.push_back(new_frame);
    }

    pub fn read_frame(&mut self) -> &[u8] {
        let frame = self.frames.pop_front().unwrap_or_default();
        &self.buffer[frame.starts_at..frame.ends_before]
    }
}

#[derive(Clone, Copy, Default)]
struct BufferFrame {
    starts_at: usize,
    ends_before: usize,
}
