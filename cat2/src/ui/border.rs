use super::area::{self, Area};

pub fn print_vertical(area: &mut Area) -> anyhow::Result<()> {
    area::write(area, "┃")
}
