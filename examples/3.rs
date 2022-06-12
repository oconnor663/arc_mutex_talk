use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let number = Arc::new(0u64);
    let alias = number.clone();
    thread::spawn(move || add_loop(alias));
    loop {
        println!("{}", *number);
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop(number: Arc<u64>) {
    loop {
        *number += 1;
    }
}
