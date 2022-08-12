use num::bigint::BigUint;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Lazy<Mutex<BigUint>> = Lazy::new(|| Mutex::new(BigUint::from(0u64)));

fn main() {
    thread::spawn(|| loop {
        *NUMBER.lock().unwrap() += 1u64;
    });
    loop {
        println!("{}", *NUMBER.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
