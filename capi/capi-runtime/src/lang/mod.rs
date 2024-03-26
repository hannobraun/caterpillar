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
    let mut addr = draw_buffer_offset();

    loop {
        if addr >= buffer_len {
            break;
        }

        set_pixel(addr, mem);

        let mut data_stack = DataStack::new();
        data_stack.push(addr);
        inc_pixel(&mut data_stack);
        addr = data_stack.pop();
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

fn set_red(base_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    let offset = 0;
    let value = 0;

    data_stack.push(value);
    data_stack.push(base_addr);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn set_green(base_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    let offset = 1;
    let value = 255;

    data_stack.push(value);
    data_stack.push(base_addr);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn set_blue(base_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    let offset = 2;
    let value = 0;

    data_stack.push(value);
    data_stack.push(base_addr);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn set_alpha(base_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    let offset = 3;
    let value = 255;

    data_stack.push(value);
    data_stack.push(base_addr);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn set_channel(data_stack: &mut DataStack, mem: &mut [u8]) {
    add(data_stack);
    swap(data_stack);
    store(data_stack, mem);
}

fn inc_pixel(data_stack: &mut DataStack) {
    data_stack.push(4);
    add(data_stack);
}
