use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn main() {
    thread::spawn(add_loop);
    loop {
        println!("{}", *NUMBER.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop() {
    loop {
        *NUMBER.lock().unwrap() += 1;
    }
}
