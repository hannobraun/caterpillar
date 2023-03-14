use std::io::{stdout, Stdout};

use futures::executor::block_on;

use crate::{
    terminal::{self, Terminal},
    ui,
};

pub async fn run() -> anyhow::Result<()> {
    Terminal::run(run_inner).await?;
    Ok(())
}

pub async fn run_inner(mut terminal: Terminal) -> anyhow::Result<()> {
    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();

    let size = match terminal::Size::get() {
        Ok(size) => size,
        Err(err) => return Err(err),
    };

    match run_once(size, &mut buffer, &mut stdout) {
        Ok(()) => (),
        Err(err) => return Err(err),
    }

    loop {
        let () = match block_on(terminal.next_event()) {
            Ok(Some(())) => (),
            Ok(None) => break,
            Err(err) => return Err(err),
        };
        match run_once(size, &mut buffer, &mut stdout) {
            Ok(()) => (),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

pub fn run_once(
    terminal_size: terminal::Size,
    buffer: &mut ui::Buffer,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    let terminal_size = ui::Vector {
        x: terminal_size.num_columns,
        y: terminal_size.num_rows,
    };

    buffer.prepare(terminal_size);

    let offset = ui::Vector { x: 0, y: 0 };

    let area = ui::area::new(buffer, offset, terminal_size);
    ui::border::draw(area);

    buffer.draw(stdout)?;

    Ok(())
}
