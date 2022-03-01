use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let output_clone = output.clone();
    thread::spawn(move || fill_vector(&output_clone));
    loop {
        println!("{:?}", output.lock().unwrap());
        thread::sleep(Duration::from_millis(100));
    }
}

fn fill_vector(output: &Mutex<Vec<u64>>) {
    for i in 0.. {
        output.lock().unwrap().push(i);
        thread::sleep(Duration::from_secs(1));
    }
}
