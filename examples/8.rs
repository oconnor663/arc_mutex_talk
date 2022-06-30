use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static NUMBER: AtomicU64 = AtomicU64::new(0);

fn main() {
    thread::spawn(|| loop {
        NUMBER.fetch_add(1, Ordering::SeqCst);
    });
    loop {
        println!("{}", NUMBER.load(Ordering::SeqCst));
        thread::sleep(Duration::from_secs(1));
    }
}
