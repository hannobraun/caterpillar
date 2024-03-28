mod builtins;
mod data_stack;

use self::{
    builtins::{add, mul, store},
    data_stack::DataStack,
};

pub struct Lang<'r> {
    data_stack: DataStack,
    frame: &'r mut [u8],
}

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut lang = Lang {
        data_stack: DataStack::new(),
        frame,
    };

    lang.data_stack.push(frame_width);
    lang.data_stack.push(frame_height);

    store_all_pixels(&mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
}

fn store_all_pixels(lang: &mut Lang) {
    compute_draw_buffer_len(&mut lang.data_stack);
    let buffer_len = lang.data_stack.pop();

    frame_addr(&mut lang.data_stack);

    loop {
        let addr = lang.data_stack.pop();
        if addr >= buffer_len {
            break;
        }
        lang.data_stack.push(addr);

        store_pixel(&mut lang.data_stack, lang.frame);
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
