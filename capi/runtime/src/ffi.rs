use std::{str, sync::Mutex};

use capi_compiler::{compiler::compile, games::snake::snake, syntax::Script};
use capi_ffi::{framed_buffer::FramedBuffer, shared::Shared};
use capi_protocol::{
    CODE_BUFFER_SIZE, COMMANDS_BUFFER_SIZE, UPDATES_BUFFER_SIZE,
};

use crate::{state::RuntimeState, tiles::NUM_PIXEL_BYTES};

pub static PANIC: Shared<Option<String>> = Shared::new(None);
pub static CODE: Shared<FramedBuffer<CODE_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());

pub static STATE: Mutex<Option<RuntimeState>> = Mutex::new(None);

static UPDATES: Shared<FramedBuffer<UPDATES_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());
static COMMANDS: Shared<FramedBuffer<COMMANDS_BUFFER_SIZE>> =
    Shared::new(FramedBuffer::new());
static PIXELS: Shared<[u8; NUM_PIXEL_BYTES]> =
    Shared::new([0; NUM_PIXEL_BYTES]);

#[no_mangle]
pub fn panic_ptr() -> usize {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let panic = unsafe { PANIC.access() };

    panic
        .as_ref()
        .map(|panic| panic.as_ptr() as usize)
        .unwrap_or(0)
}

#[no_mangle]
pub fn panic_len() -> usize {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let panic = unsafe { PANIC.access() };

    panic
        .as_ref()
        .map(|panic| {
            // `len` is correct here, despite this being a `String`, as the host
            // is interested in the length in bytes.
            panic.len()
        })
        .unwrap_or(0)
}

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
    let buffer = unsafe { UPDATES.access() };
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

static LAST_COMMAND_WRITE: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[no_mangle]
pub fn commands_write(len: usize) {
    // Sound, because the reference is dropped before we give back control to
    // the host.
    let buffer = unsafe { COMMANDS.access() };
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
pub fn pixels_ptr() -> usize {
    &PIXELS as *const _ as usize
}

#[no_mangle]
pub fn pixels_len() -> usize {
    NUM_PIXEL_BYTES
}

#[no_mangle]
pub fn push_random(random: f64) -> bool {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    if state.random.len() >= 1024 {
        return false;
    }

    let min: i16 = i8::MIN.into();
    let max: i16 = i8::MAX.into();

    let range: f64 = (max - min).into();
    let random = random * range;

    let random: i8 = random.floor() as i8;
    state.random.push_back(random);

    true
}

#[no_mangle]
pub fn compile_code() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    let mut script = Script::default();
    snake(&mut script);

    let (code, source_map) = compile(&script);

    state
        .updates
        .queue_source_code(script.functions, source_map);

    let code = ron::to_string(&code).unwrap();
    let code = code.as_bytes();

    // Sound, as the reference is dropped before we give back control to the
    // host.
    let buffer = unsafe { CODE.access() };
    buffer.write_frame(code.len()).copy_from_slice(code);
}

#[no_mangle]
pub fn on_code() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    // Sound, as the reference is dropped before we give back control to the
    // host.
    let buffer = unsafe { CODE.access() };

    let code = buffer.read_frame();
    let code = str::from_utf8(code).unwrap();
    let code = ron::from_str(code).unwrap();

    state.on_code(code);
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
    let buffer = unsafe { COMMANDS.access() };

    let command = buffer.read_frame().to_vec();
    state.commands.push(command);
}

#[no_mangle]
pub fn on_frame() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    // Sound, because the reference is dropped before we give back control to
    // the host.
    let pixels = unsafe { PIXELS.access() };

    state.update(pixels);

    for update in state.updates.take_queued_updates() {
        // Sound, because the reference is dropped before we call the method
        // again or we give back control to the host.
        let buffer = unsafe { UPDATES.access() };
        buffer.write_frame(update.len()).copy_from_slice(&update);
    }
}
