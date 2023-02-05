use super::area;

pub fn print_vertical(area: &mut area::Area) -> anyhow::Result<()> {
    area::write(area, "┃")
}
