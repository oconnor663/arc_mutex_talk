use std::thread;
use std::time::Duration;

static mut NUMBER: u64 = 0;

fn main() {
    thread::spawn(add_loop);
    loop {
        unsafe {
            println!("{}", NUMBER);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop() {
    loop {
        unsafe {
            NUMBER += 1;
        }
    }
}
