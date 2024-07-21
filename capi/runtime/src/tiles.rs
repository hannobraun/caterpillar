use capi_protocol::host::TILES_PER_AXIS;

pub const PIXELS_PER_TILE_AXIS: usize = 8;
pub const PIXELS_PER_AXIS: usize = TILES_PER_AXIS * PIXELS_PER_TILE_AXIS;
pub const NUM_PIXELS: usize = PIXELS_PER_AXIS * PIXELS_PER_AXIS;
pub const NUM_CHANNELS: usize = 4;
pub const NUM_PIXEL_BYTES: usize = NUM_PIXELS * NUM_CHANNELS;
