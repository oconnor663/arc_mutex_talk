use std::thread;
use std::time::Duration;

static mut OUTPUT: Vec<u64> = Vec::new();

fn main() {
    thread::spawn(|| fill_vector());
    loop {
        println!("{:?}", OUTPUT);
        thread::sleep(Duration::from_millis(100));
    }
}

fn fill_vector() {
    for i in 0.. {
        OUTPUT.push(i);
        thread::sleep(Duration::from_secs(1));
    }
}
