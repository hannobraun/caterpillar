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

fn set_pixel(addr: usize, mem: &mut [u8]) {
    set_red(addr, mem);
    set_green(addr, mem);
    set_blue(addr, mem);
    set_alpha(addr, mem);
}

fn set_red(pixel_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();
    data_stack.push(pixel_addr);

    let offset = 0;

    red_value(&mut data_stack);
    swap(&mut data_stack);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn red_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn set_green(pixel_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();
    data_stack.push(pixel_addr);

    let offset = 1;

    green_value(&mut data_stack);
    swap(&mut data_stack);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn green_value(data_stack: &mut DataStack) {
    data_stack.push(255)
}

fn set_blue(pixel_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();
    data_stack.push(pixel_addr);

    let offset = 2;

    blue_value(&mut data_stack);
    swap(&mut data_stack);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn blue_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn set_alpha(pixel_addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();
    data_stack.push(pixel_addr);

    let offset = 3;

    alpha_value(&mut data_stack);
    swap(&mut data_stack);
    data_stack.push(offset);

    set_channel(&mut data_stack, mem);
}

fn alpha_value(data_stack: &mut DataStack) {
    data_stack.push(255);
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
