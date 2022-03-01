use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static OUTPUT: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() {
    thread::spawn(|| fill_vector());
    loop {
        println!("{:?}", OUTPUT.lock().unwrap());
        thread::sleep(Duration::from_millis(100));
    }
}

fn fill_vector() {
    for i in 0.. {
        OUTPUT.lock().unwrap().push(i);
        thread::sleep(Duration::from_secs(1));
    }
}
