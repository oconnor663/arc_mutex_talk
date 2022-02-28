use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static OUTPUT: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() {
    thread::spawn(|| fill_vector());
    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", OUTPUT.lock().unwrap());
    }
}

fn fill_vector() {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));
        OUTPUT.lock().unwrap().push(i);
    }
}
