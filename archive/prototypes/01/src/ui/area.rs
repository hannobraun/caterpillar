use super::{buffer::Buffer, vector::Vector};

pub struct Area<'a> {
    buffer: &'a mut Buffer,
    offset: Vector,
    size: Vector,
    cursor: Vector,
}

pub fn new(buffer: &mut Buffer, offset: Vector, size: Vector) -> Area {
    Area {
        buffer,
        offset,
        size,
        cursor: Vector { x: 0, y: 0 },
    }
}

pub fn size(area: &Area) -> Vector {
    area.size
}

pub fn slice(mut area: Area, offsets: [Vector; 2]) -> Area {
    let [d_offset, d_size] = offsets;

    area.offset += d_offset;
    area.size -= d_size;

    area.cursor = Vector { x: 0, y: 0 };

    area
}

pub fn move_to_previous_line(area: &mut Area) {
    area.cursor.x = 0;
    area.cursor.y = area.cursor.y.saturating_sub(1);
}

pub fn move_to_next_line(area: &mut Area) {
    area.cursor.x = 0;
    area.cursor.y += 1;
}

pub fn move_to_last_line(area: &mut Area) {
    area.cursor.x = 0;
    area.cursor.y = area.size.y - 1;
}

pub fn move_to_end_of_line(area: &mut Area) {
    area.cursor.x = area.size.x - 1;
}

pub fn draw(area: &mut Area, s: &str) {
    area.buffer.move_cursor(area.offset + area.cursor);
    area.buffer.write(s);

    let num_chars = s.chars().count();
    area.cursor.x += num_chars;
}
