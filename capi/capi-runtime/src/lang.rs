pub fn lang(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    set_all_pixels(canvas_width, canvas_height, mem);
}

fn set_all_pixels(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    let buffer_len = compute_draw_buffer_len(canvas_width, canvas_height);
    let mut i = draw_buffer_offset();

    loop {
        if i >= buffer_len {
            break;
        }

        set_pixel(i, mem);

        let mut data_stack = DataStack { values: Vec::new() };
        data_stack.push(i);
        inc_pixel(&mut data_stack.values);
        i = data_stack.pop();
    }
}

fn compute_draw_buffer_len(canvas_width: usize, canvas_height: usize) -> usize {
    canvas_width * canvas_height * 4
}

fn draw_buffer_offset() -> usize {
    0
}

fn set_pixel(i: usize, mem: &mut [u8]) {
    set_red(i, mem);
    set_green(i, mem);
    set_blue(i, mem);
    set_alpha(i, mem);
}

fn set_red(i: usize, mem: &mut [u8]) {
    let offset = 0;
    let value = 0;
    set_channel(i, offset, value, mem);
}

fn set_green(i: usize, mem: &mut [u8]) {
    let offset = 1;
    let value = 255;
    set_channel(i, offset, value, mem);
}

fn set_blue(i: usize, mem: &mut [u8]) {
    let offset = 2;
    let value = 0;
    set_channel(i, offset, value, mem);
}

fn set_alpha(i: usize, mem: &mut [u8]) {
    let offset = 3;
    let value = 255;
    set_channel(i, offset, value, mem);
}

fn set_channel(i: usize, offset: usize, value: u8, mem: &mut [u8]) {
    mem[i + offset] = value;
}

fn inc_pixel(data_stack: &mut Vec<usize>) {
    let i = data_stack.pop().unwrap();
    let i = i + 4;
    data_stack.push(i);
}

pub struct DataStack {
    values: Vec<usize>,
}

impl DataStack {
    pub fn push(&mut self, value: usize) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> usize {
        self.values.pop().unwrap()
    }
}
