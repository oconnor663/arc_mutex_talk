use std::thread;
use std::time::Duration;

static mut NUMBER: u64 = 0;

fn main() {
    thread::spawn(|| loop {
        unsafe {
            NUMBER += 1;
        }
    });
    loop {
        unsafe {
            println!("{}", NUMBER);
        }
        thread::sleep(Duration::from_secs(1));
    }
}
