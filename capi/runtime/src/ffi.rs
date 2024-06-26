use std::sync::Mutex;

use capi_ffi::{framed_buffer::FramedBuffer, shared::Shared};
use capi_protocol::{COMMANDS_BUFFER_SIZE, UPDATES_BUFFER_SIZE};
use tokio::sync::mpsc::error::TryRecvError;

use crate::state::RuntimeState;

pub static STATE: Mutex<Option<RuntimeState>> = Mutex::new(None);

/// The buffer that is used to transfer updates _to_ the host
static UPDATES_TX: Shared<FramedBuffer<UPDATES_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());

/// The buffer that is used to transfer updates _from_ the host
static UPDATES_RX: Shared<FramedBuffer<UPDATES_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());

static COMMANDS_TX: Shared<FramedBuffer<COMMANDS_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());
static COMMANDS_RX: Shared<FramedBuffer<COMMANDS_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());

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

/// See comment on [`LAST_UPDATE_READ`]
static LAST_UPDATE_WRITE: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[no_mangle]
pub fn updates_write(len: usize) {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { UPDATES_RX.access() };
    let update = buffer.write_frame(len);

    *LAST_UPDATE_WRITE.lock().unwrap() =
        Some((update.as_ptr() as usize, update.len()));
}

#[no_mangle]
pub fn updates_write_ptr() -> usize {
    let (ptr, _) = LAST_UPDATE_WRITE.lock().unwrap().unwrap();
    ptr
}

#[no_mangle]
pub fn updates_write_len() -> usize {
    let (_, len) = LAST_UPDATE_WRITE.lock().unwrap().unwrap();
    len
}

static LAST_COMMAND_READ: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[no_mangle]
pub fn commands_read() {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { COMMANDS_TX.access() };
    let command = buffer.read_frame();

    *LAST_COMMAND_READ.lock().unwrap() =
        Some((command.as_ptr() as usize, command.len()));
}

#[no_mangle]
pub fn commands_read_ptr() -> usize {
    let (ptr, _) = LAST_COMMAND_READ.lock().unwrap().unwrap();
    ptr
}

#[no_mangle]
pub fn commands_read_len() -> usize {
    let (_, len) = LAST_COMMAND_READ.lock().unwrap().unwrap();
    len
}

static LAST_COMMAND_WRITE: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[no_mangle]
pub fn commands_write(len: usize) {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { COMMANDS_RX.access() };
    let command = buffer.write_frame(len);

    *LAST_COMMAND_WRITE.lock().unwrap() =
        Some((command.as_ptr() as usize, command.len()));
}

#[no_mangle]
pub fn commands_write_ptr() -> usize {
    let (ptr, _) = LAST_COMMAND_WRITE.lock().unwrap().unwrap();
    ptr
}

#[no_mangle]
pub fn commands_write_len() -> usize {
    let (_, len) = LAST_COMMAND_WRITE.lock().unwrap().unwrap();
    len
}

#[no_mangle]
pub fn on_key(key_code: u8) {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub fn on_command() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { COMMANDS_RX.access() };

    let command = buffer.read_frame().to_vec();
    state.commands.push(command);
}

#[no_mangle]
pub fn on_frame() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    loop {
        let command = match state.debugger.commands_rx.try_recv() {
            Ok(command) => command,
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                // The other end has hung up, which happens during
                // shutdown. Shut down this task, too.
                return;
            }
        };

        // Sound, because the reference is dropped before we call the method
        // again or we give back control to the host.
        let buffer = unsafe { COMMANDS_TX.access() };
        buffer.write_frame(command.len()).copy_from_slice(&command);
    }

    state.update();

    for update in state.updates.take_queued_updates() {
        // Sound, because the reference is dropped before we call the method
        // again or we give back control to the host.
        let buffer = unsafe { UPDATES_TX.access() };
        buffer.write_frame(update.len()).copy_from_slice(&update);
    }
}

#[no_mangle]
pub fn on_update() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { UPDATES_RX.access() };

    let update = buffer.read_frame().to_vec();
    state.debugger.updates_tx.send(update).unwrap();
}
