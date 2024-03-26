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
    let mut data_stack = DataStack::new();

    data_stack.push(addr);
    set_red(&mut data_stack, mem);
    data_stack.push(addr);
    set_green(&mut data_stack, mem);
    data_stack.push(addr);
    set_blue(&mut data_stack, mem);
    data_stack.push(addr);
    set_alpha(&mut data_stack, mem);
}

fn set_red(data_stack: &mut DataStack, mem: &mut [u8]) {
    red_value(data_stack);
    swap(data_stack);
    red_offset(data_stack);

    set_channel(data_stack, mem);
}

fn red_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn red_offset(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn set_green(data_stack: &mut DataStack, mem: &mut [u8]) {
    green_value(data_stack);
    swap(data_stack);
    green_offset(data_stack);

    set_channel(data_stack, mem);
}

fn green_value(data_stack: &mut DataStack) {
    data_stack.push(255)
}

fn green_offset(data_stack: &mut DataStack) {
    data_stack.push(1);
}

fn set_blue(data_stack: &mut DataStack, mem: &mut [u8]) {
    blue_value(data_stack);
    swap(data_stack);
    blue_offset(data_stack);

    set_channel(data_stack, mem);
}

fn blue_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn blue_offset(data_stack: &mut DataStack) {
    data_stack.push(2);
}

fn set_alpha(data_stack: &mut DataStack, mem: &mut [u8]) {
    alpha_value(data_stack);
    swap(data_stack);
    alpha_offset(data_stack);

    set_channel(data_stack, mem);
}

fn alpha_value(data_stack: &mut DataStack) {
    data_stack.push(255);
}

fn alpha_offset(data_stack: &mut DataStack) {
    data_stack.push(3);
}

fn set_channel(data_stack: &mut DataStack, mem: &mut [u8]) {
    add(data_stack);
    swap(data_stack);
    store(data_stack, mem);
    data_stack.pop();
}

fn inc_pixel(data_stack: &mut DataStack) {
    data_stack.push(4);
    add(data_stack);
}
