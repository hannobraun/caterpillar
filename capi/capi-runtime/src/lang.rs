pub fn lang(data: &mut [u8]) {
    set_all_pixels(data);
}

fn set_all_pixels(data: &mut [u8]) {
    let mut i = 0;

    loop {
        if i >= data.len() {
            break;
        }

        set_pixel(i, data);

        i += 4;
    }
}

fn set_pixel(i: usize, data: &mut [u8]) {
    set_red(i, data);
    set_green(i, data);
    set_blue(i, data);
    set_alpha(i, data);
}

fn set_red(i: usize, data: &mut [u8]) {
    data[i] = 0;
}

fn set_green(i: usize, data: &mut [u8]) {
    data[i + 1] = 255;
}

fn set_blue(i: usize, data: &mut [u8]) {
    data[i + 2] = 0;
}

fn set_alpha(i: usize, data: &mut [u8]) {
    data[i + 3] = 255;
}
