pub fn lang(data: &mut [u8]) {
    let mut i = 0;

    loop {
        if i >= data.len() {
            break;
        }

        data[i] = 0;
        data[i + 1] = 255;
        data[i + 2] = 0;
        data[i + 3] = 255;

        i += 4;
    }
}
