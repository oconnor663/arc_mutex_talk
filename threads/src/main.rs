use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let output_clone = output.clone();
    thread::spawn(|| background_count(output_clone));
    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", output.lock().unwrap());
    }
}

fn background_count(output: Arc<Mutex<Vec<u64>>>) {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));
        output.lock().unwrap().push(i);
    }
}
