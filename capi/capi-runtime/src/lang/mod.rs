mod builtins;
mod data_stack;

use self::{
    builtins::{add, mul, store},
    data_stack::DataStack,
};

pub fn lang(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    set_all_pixels(canvas_width, canvas_height, mem);
}

fn set_all_pixels(canvas_width: usize, canvas_height: usize, mem: &mut [u8]) {
    let mut data_stack = DataStack::new();

    data_stack.push(canvas_width);
    data_stack.push(canvas_height);
    compute_draw_buffer_len(&mut data_stack);
    let buffer_len = data_stack.pop();

    draw_buffer_addr(&mut data_stack);

    loop {
        let addr = data_stack.pop();
        if addr >= buffer_len {
            break;
        }
        data_stack.push(addr);

        set_pixel(&mut data_stack, mem);
    }

    assert_eq!(data_stack.num_values(), 0);
}

fn compute_draw_buffer_len(data_stack: &mut DataStack) {
    mul(data_stack);
    data_stack.push(4);
    mul(data_stack);
}

fn draw_buffer_addr(data_stack: &mut DataStack) {
    data_stack.push(0);
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
    inc_addr(data_stack);
}

fn inc_addr(data_stack: &mut DataStack) {
    data_stack.push(1);
    add(data_stack);
}
