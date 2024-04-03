use std::thread;

pub fn start() {
    thread::spawn(|| {
        println!("Hello, world!");
    });
}
