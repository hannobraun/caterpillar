use std::cmp::min;

use super::{buffer::Buffer, vector::Vector};

pub struct Area<'a> {
    buffer: &'a mut Buffer,
    size: Vector,
    offset: Vector,
    cursor: Vector,
}

pub fn new(buffer: &mut Buffer, size: Vector) -> Area {
    Area {
        buffer,
        size,
        offset: Vector { x: 0, y: 0 },
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

pub fn move_to_next_line(area: &mut Area) {
    area.cursor.x = 0;
    area.cursor.y += 1;

    area.cursor.y = min(area.cursor.y, area.size.y - 1);
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
