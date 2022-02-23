use std::thread;
use std::time::Duration;

static mut OUTPUT: Vec<u64> = Vec::new();

fn main() {
    thread::spawn(|| background_count());
    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", OUTPUT);
    }
}

fn background_count() {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));
        OUTPUT.push(i);
    }
}
