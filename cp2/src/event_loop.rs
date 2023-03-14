use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::{terminal, ui};

pub async fn run() -> anyhow::Result<()> {
    let frame_time = Duration::from_millis(125);
    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();

    terminal::run(frame_time, |size| {
        std::future::ready(run_once(size, &mut buffer, &mut stdout))
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
