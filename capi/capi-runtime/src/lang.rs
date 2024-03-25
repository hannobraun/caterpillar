pub fn lang(canvas_width: usize, canvas_height: usize, data: &mut [u8]) {
    set_all_pixels(canvas_width, canvas_height, data);
}

fn set_all_pixels(canvas_width: usize, canvas_height: usize, data: &mut [u8]) {
    let buffer_len = compute_draw_buffer_len(canvas_width, canvas_height);
    let mut i = draw_buffer_offset();

    loop {
        if i >= buffer_len {
            break;
        }

        set_pixel(i, data);
        i = inc_pixel(i);
    }
}

fn compute_draw_buffer_len(canvas_width: usize, canvas_height: usize) -> usize {
    canvas_width * canvas_height * 4
}

fn draw_buffer_offset() -> usize {
    0
}

fn set_pixel(i: usize, data: &mut [u8]) {
    set_red(i, data);
    set_green(i, data);
    set_blue(i, data);
    set_alpha(i, data);
}

fn set_red(i: usize, data: &mut [u8]) {
    let offset = i;
    data[offset] = 0;
}

fn set_green(i: usize, data: &mut [u8]) {
    let offset = i + 1;
    data[offset] = 255;
}

fn set_blue(i: usize, data: &mut [u8]) {
    let offset = i + 2;
    data[offset] = 0;
}

fn set_alpha(i: usize, data: &mut [u8]) {
    let offset = i + 3;
    data[offset] = 255;
}

fn inc_pixel(i: usize) -> usize {
    i + 4
}
