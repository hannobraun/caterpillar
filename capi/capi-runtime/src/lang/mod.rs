mod builtins;
mod data_stack;

use self::{
    builtins::{add, store},
    data_stack::DataStack,
};

pub fn lang(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    set_all_pixels(canvas_width, canvas_height, mem);
}

fn set_all_pixels(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    let buffer_len = compute_draw_buffer_len(canvas_width, canvas_height);
    let mut addr = draw_buffer_addr();

    loop {
        if addr >= buffer_len {
            break;
        }

        set_pixel(addr, mem);

        let mut data_stack = DataStack::new();
        data_stack.push(addr);
        inc_pixel(&mut data_stack);
        addr = data_stack.pop();

        assert_eq!(data_stack.num_values(), 0);
    }
}

fn compute_draw_buffer_len(canvas_width: usize, canvas_height: usize) -> usize {
    canvas_width * canvas_height * 4
}

fn draw_buffer_addr() -> usize {
    0
}

fn set_pixel(addr: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();
    data_stack.push(addr);

    set_red(&mut data_stack, mem);
    set_green(&mut data_stack, mem);
    set_blue(&mut data_stack, mem);
    set_alpha(&mut data_stack, mem);

    let _addr = data_stack.pop();

    assert_eq!(data_stack.num_values(), 0);
}

fn set_red(data_stack: &mut DataStack, mem: &mut [u8]) {
    red_value(data_stack);
    set_channel(data_stack, mem);
}

fn red_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn set_green(data_stack: &mut DataStack, mem: &mut [u8]) {
    green_value(data_stack);
    set_channel(data_stack, mem);
}

fn green_value(data_stack: &mut DataStack) {
    data_stack.push(255)
}

fn set_blue(data_stack: &mut DataStack, mem: &mut [u8]) {
    blue_value(data_stack);
    set_channel(data_stack, mem);
}

fn blue_value(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn set_alpha(data_stack: &mut DataStack, mem: &mut [u8]) {
    alpha_value(data_stack);
    set_channel(data_stack, mem);
}

fn alpha_value(data_stack: &mut DataStack) {
    data_stack.push(255);
}

fn set_channel(data_stack: &mut DataStack, mem: &mut [u8]) {
    store(data_stack, mem);
    inc_channel(data_stack);
}

fn inc_channel(data_stack: &mut DataStack) {
    data_stack.push(1);
    add(data_stack);
}

fn inc_pixel(data_stack: &mut DataStack) {
    data_stack.push(4);
    add(data_stack);
}
