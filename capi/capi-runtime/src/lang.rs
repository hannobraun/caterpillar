pub fn lang(data: &mut [u8]) {
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
    data[i + 2] = 0;
    data[i + 3] = 255;
}

fn set_red(i: usize, data: &mut [u8]) {
    data[i] = 0;
}

fn set_green(i: usize, data: &mut [u8]) {
    data[i + 1] = 255;
}
