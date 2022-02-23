use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let output = Arc::new(Vec::new());
    let output_clone = output.clone();
    thread::spawn(|| background_count(output_clone));
    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", output);
    }
}

fn background_count(output: Arc<Vec<u64>>) {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));
        output.push(i);
    }
}
