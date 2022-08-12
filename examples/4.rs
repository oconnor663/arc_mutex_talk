use num::bigint::BigUint;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Mutex<BigUint> = Mutex::new(BigUint::from(0u64));

fn main() {
    thread::spawn(add_loop);
    loop {
        println!("{}", *NUMBER.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop() {
    loop {
        *NUMBER.lock().unwrap() += 1u64;
    }
}
