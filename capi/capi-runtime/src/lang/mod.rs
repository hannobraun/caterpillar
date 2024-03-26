mod builtins;
mod data_stack;

use self::{
    builtins::{add, store, swap},
    data_stack::DataStack,
};

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

        let mut data_stack = DataStack::new();
        data_stack.push(i);
        inc_pixel(&mut data_stack);
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
    set_channel(value, i, offset, mem);
}

fn set_green(i: usize, mem: &mut [u8]) {
    let offset = 1;
    let value = 255;
    set_channel(value, i, offset, mem);
}

fn set_blue(i: usize, mem: &mut [u8]) {
    let offset = 2;
    let value = 0;
    set_channel(value, i, offset, mem);
}

fn set_alpha(i: usize, mem: &mut [u8]) {
    let offset = 3;
    let value = 255;
    set_channel(value, i, offset, mem);
}

fn set_channel(value: usize, base_addr: usize, offset: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    data_stack.push(value);
    data_stack.push(base_addr);
    data_stack.push(offset);

    add(&mut data_stack);
    swap(&mut data_stack);
    store(&mut data_stack, mem);
}

fn inc_pixel(data_stack: &mut DataStack) {
    data_stack.push(4);
    add(data_stack);
}
