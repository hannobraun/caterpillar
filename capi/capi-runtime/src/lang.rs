pub fn lang(data: &mut [u8]) {
    for chunk in data.chunks_mut(4) {
        chunk[0] = 0;
        chunk[1] = 255;
        chunk[2] = 0;
        chunk[3] = 255;
    }
}
