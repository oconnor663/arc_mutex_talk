use num::bigint::BigUint;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Mutex<BigUint> = Mutex::new(BigUint::from(0u64));

fn main() {
    thread::spawn(|| loop {
        *NUMBER.lock().unwrap() += 1u64;
    });
    loop {
        println!("{}", *NUMBER.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
