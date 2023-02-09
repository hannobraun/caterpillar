mod area;
mod border;
mod buffer;
mod editor;
mod generations;
mod vector;

pub use self::buffer::Buffer;

use std::io::Stdout;

use crossterm::terminal;

use crate::{
    cells::{self, Generation},
    cp,
};

use self::{border::BORDER_OVERHEAD, vector::Vector};

pub fn draw(
    interpreter: &cp::Interpreter,
    generations: &[Generation],
    buffer: &mut Buffer,
    stdout: &mut Stdout,
) -> anyhow::Result<()> {
    let (num_columns, num_rows) = terminal::size()?;
    let (num_columns, num_rows) = (num_columns as usize, num_rows as usize);

    buffer.prepare(Vector {
        x: num_columns,
        y: num_rows,
    });

    let generations_width = cells::NUM_CELLS + BORDER_OVERHEAD;

    {
        let offset = Vector {
            x: num_columns - generations_width,
            y: 0,
        };
        let size = Vector {
            x: generations_width,
            y: num_rows,
        };
        let area = area::new(buffer, offset, size);

        generations::draw(area, generations.iter());
    }
    {
        let editor_height = 1 + BORDER_OVERHEAD;

        let offset = Vector {
            x: 0,
            y: num_rows - editor_height,
        };
        let size = Vector {
            x: num_columns - generations_width,
            y: editor_height,
        };
        let area = area::new(buffer, offset, size);

        editor::draw(
            area,
            interpreter
                .functions
                .get("cell_is_born", [cp::Type::U8])
                .tokens
                .join(" ")
                .as_str(),
        );
    }

    buffer.draw(stdout)?;

    Ok(())
}
