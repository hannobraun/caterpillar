use std::{fs::File, io::Write};

fn main() -> anyhow::Result<()> {
    let program = [
        0x04, // clone
        0x01, // push
        0,    // address
        0x03, // store
        0x04, // clone
        0x01, // push
        1,    // address
        0x03, // store
        0x01, // push
        2,    // address
        0x03, // store
        0x01, // push
        255,  // alpha channel
        0x01, // push
        3,    // address
        0x03, // store
        0x00, // terminate
    ];

    File::create("program.bc.capi")?.write_all(&program)?;

    Ok(())
}
