pub mod command;
pub mod host;
pub mod memory;
pub mod update;

pub const CODE_BUFFER_SIZE: usize = 1024 * 1024;

/// The size of the updates buffer
///
/// This is a ridiculous 1 MiB large. It should be possible to make this much
/// smaller, but for now, we're using a very space-inefficient serialization
/// format.
pub const UPDATES_BUFFER_SIZE: usize = 1024 * 1024;

pub const COMMANDS_BUFFER_SIZE: usize = 1024;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Versioned<T> {
    pub version: u64,
    pub inner: T,
}
