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

        let mut data_stack = DataStack::new();

        data_stack.push(addr);
        set_pixel(&mut data_stack, mem);
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

fn set_pixel(data_stack: &mut DataStack, mem: &mut [u8]) {
    store_red(data_stack, mem);
    store_green(data_stack, mem);
    store_blue(data_stack, mem);
    store_alpha(data_stack, mem);
}

fn store_red(data_stack: &mut DataStack, mem: &mut [u8]) {
    data_stack.push(0);
    store_channel(data_stack, mem);
}

fn store_green(data_stack: &mut DataStack, mem: &mut [u8]) {
    data_stack.push(255);
    store_channel(data_stack, mem);
}

fn store_blue(data_stack: &mut DataStack, mem: &mut [u8]) {
    data_stack.push(0);
    store_channel(data_stack, mem);
}

fn store_alpha(data_stack: &mut DataStack, mem: &mut [u8]) {
    data_stack.push(255);
    store_channel(data_stack, mem);
}

fn store_channel(data_stack: &mut DataStack, mem: &mut [u8]) {
    store(data_stack, mem);
    inc_channel(data_stack);
}

fn inc_channel(data_stack: &mut DataStack) {
    data_stack.push(1);
    add(data_stack);
}
