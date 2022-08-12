use num::bigint::BigUint;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Mutex<Option<BigUint>> = Mutex::new(None);

fn main() {
    *NUMBER.lock().unwrap() = Some(BigUint::from(0u64));
    thread::spawn(|| loop {
        *NUMBER.lock().unwrap().as_mut().unwrap() += 1u64;
    });
    loop {
        println!("{}", *NUMBER.lock().unwrap().as_ref().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
