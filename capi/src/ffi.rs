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
static UPDATES_TX: SharedFramedBuffer<UPDATES_BUFFER_SIZE> =
    SharedFramedBuffer::new();

/// This is a workaround for not being able to return a tuple from
/// `updates_read`. That should work in principle (see [1]), but Rust warns
/// about the flag being unstable, and `wasm-bindgen-cli-support` crashes on an
/// assertion. It seems to be possible to make `wasm-bindgen` work[2], but that
/// doesn't seem worth the effort right now.
///
/// It might be worth revisiting this, once this crate no longer depends on
/// `wasm-bindgen`. There's also discussion about enabling the required flag by
/// default in LLVM[3], so long-term, this might take care of itself.
///
/// [1]: https://github.com/rust-lang/rust/issues/73755#issuecomment-1577586801
/// [2]: https://github.com/rustwasm/wasm-bindgen/issues/3552
/// [3]: https://github.com/WebAssembly/tool-conventions/issues/158
static LAST_UPDATE_READ: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[no_mangle]
pub fn updates_read() {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { UPDATES_TX.access() };
    let update = buffer.read_frame();

    *LAST_UPDATE_READ.lock().unwrap() =
        Some((update.as_ptr() as usize, update.len()));
}

#[no_mangle]
pub fn updates_read_ptr() -> usize {
    let (ptr, _) = LAST_UPDATE_READ.lock().unwrap().unwrap();
    ptr
}

#[no_mangle]
pub fn updates_read_len() -> usize {
    let (_, len) = LAST_UPDATE_READ.lock().unwrap().unwrap();
    len
}

#[no_mangle]
pub fn on_key(key_code: u8) {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub fn on_frame() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.update();

    for update in state.updates.take_queued_updates() {
        // Sound, because the reference is dropped before we call the method
        // again or we give back control to the host.
        let buffer = unsafe { UPDATES_TX.access() };
        buffer.write_frame(update.len()).copy_from_slice(&update);

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
pub struct SharedFramedBuffer<const SIZE: usize> {
    inner: UnsafeCell<FramedBuffer<SIZE>>,
}

impl<const SIZE: usize> SharedFramedBuffer<SIZE> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(FramedBuffer::new()),
        }
    }

    /// # Gain access to the shared buffer
    ///
    /// ## `&self` argument
    ///
    /// This method returns a mutable reference, despite only requiring `&self`.
    /// This is fine, as the method is `unsafe` and the requirements that derive
    /// from this are documented.
    ///
    /// If this took `&mut self`, the `SharedFrameBuffer` would need to live in
    /// a `static mut`, which would have the same pitfalls and more. With the
    /// current design, `SharedFrameBuffer` can live in a non-`mut` `static`.
    ///
    /// ## Safety
    ///
    /// The caller must drop the returned reference before giving back control
    /// to the JavaScript host.
    ///
    /// The caller must not call this method again, while the returned reference
    /// still exists.
    #[allow(clippy::mut_from_ref)] // function is `unsafe` and well-documented
    pub unsafe fn access(&self) -> &mut FramedBuffer<SIZE> {
        &mut *self.inner.get()
    }
}

// Safe to implement, since with WebAssembly, this lives in a single-threaded
// context.
unsafe impl<const SIZE: usize> Sync for SharedFramedBuffer<SIZE> {}

pub struct FramedBuffer<const SIZE: usize> {
    buffer: [u8; SIZE],
    frames: VecDeque<BufferFrame>,
}

impl<const SIZE: usize> FramedBuffer<SIZE> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; SIZE],
            frames: VecDeque::new(),
        }
    }

    pub fn write_frame(&mut self, len: usize) -> &mut [u8] {
        let next_free =
            self.frames.back().copied().unwrap_or_default().ends_before;

        let new_frame = BufferFrame {
            starts_at: next_free,
            ends_before: next_free + len,
        };
        self.frames.push_back(new_frame);

        &mut self.buffer[new_frame.starts_at..new_frame.ends_before]
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
