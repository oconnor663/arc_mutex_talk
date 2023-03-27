use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Mutex<u64> = Mutex::new(0);

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
