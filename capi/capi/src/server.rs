use std::thread;

pub fn start() {
    thread::spawn(|| {
        serve();
    });
}

fn serve() {
    println!("Hello, world!");
}
