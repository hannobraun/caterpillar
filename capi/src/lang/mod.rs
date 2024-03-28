mod builtins;
mod data_stack;

use self::{
    builtins::{add, mul, store},
    data_stack::DataStack,
};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut data_stack = DataStack::new();

    data_stack.push(frame_width);
    data_stack.push(frame_height);

    store_all_pixels(&mut data_stack, frame);

    assert_eq!(data_stack.num_values(), 0);
}

fn store_all_pixels(data_stack: &mut DataStack, frame: &mut [u8]) {
    compute_draw_buffer_len(data_stack);
    let buffer_len = data_stack.pop();

    frame_addr(data_stack);

    loop {
        let addr = data_stack.pop();
        if addr >= buffer_len {
            break;
        }
        data_stack.push(addr);

        store_pixel(data_stack, frame);
    }
}

fn compute_draw_buffer_len(data_stack: &mut DataStack) {
    mul(data_stack);
    data_stack.push(4);
    mul(data_stack);
}

fn frame_addr(data_stack: &mut DataStack) {
    data_stack.push(0);
}

fn store_pixel(data_stack: &mut DataStack, frame: &mut [u8]) {
    store_red(data_stack, frame);
    store_green(data_stack, frame);
    store_blue(data_stack, frame);
    store_alpha(data_stack, frame);
}

fn store_red(data_stack: &mut DataStack, frame: &mut [u8]) {
    data_stack.push(0);
    store_channel(data_stack, frame);
}

fn store_green(data_stack: &mut DataStack, frame: &mut [u8]) {
    data_stack.push(255);
    store_channel(data_stack, frame);
}

fn store_blue(data_stack: &mut DataStack, frame: &mut [u8]) {
    data_stack.push(0);
    store_channel(data_stack, frame);
}

fn store_alpha(data_stack: &mut DataStack, frame: &mut [u8]) {
    data_stack.push(255);
    store_channel(data_stack, frame);
}

fn store_channel(data_stack: &mut DataStack, frame: &mut [u8]) {
    store(data_stack, frame);
    inc_addr(data_stack);
}

fn inc_addr(data_stack: &mut DataStack) {
    data_stack.push(1);
    add(data_stack);
}
