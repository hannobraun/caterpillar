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
    let tests = [("test 1", true), ("test 2", false)];

    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();

    let size = terminal.size()?;
    run_once(&tests, size, &mut buffer, &mut stdout)?;

    loop {
        let () = match block_on(terminal.next_event())? {
            Some(()) => (),
            None => break,
        };

        let size = terminal.size()?;
        run_once(&tests, size, &mut buffer, &mut stdout)?;
    }

    Ok(())
}

pub fn run_once(
    tests: &[(&str, bool)],
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
    let mut area = ui::border::draw(area);

    for &(test, pass) in tests {
        if pass {
            ui::area::draw(&mut area, "PASS");
        } else {
            ui::area::draw(&mut area, "FAIL");
        }

        ui::area::draw(&mut area, " ");
        ui::area::draw(&mut area, test);

        ui::area::move_to_next_line(&mut area);
    }

    buffer.draw(stdout)?;

    Ok(())
}
