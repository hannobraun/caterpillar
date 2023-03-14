use std::{
    future,
    io::{stdout, Stdout},
};

use crate::{
    terminal::{self, Terminal},
    ui,
};

pub async fn run() -> anyhow::Result<()> {
    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();

    Terminal::run(|| {
        let size = match terminal::Size::get() {
            Ok(size) => size,
            Err(err) => return future::ready(Err(err)),
        };
        future::ready(run_once(size, &mut buffer, &mut stdout))
    })
    .await?;

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
