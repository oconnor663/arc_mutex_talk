use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let output = Arc::new(Vec::new());
    let output_clone = output.clone();
    thread::spawn(|| fill_vector(output_clone));
    loop {
        println!("{:?}", output);
        thread::sleep(Duration::from_millis(100));
    }
}

fn fill_vector(output: Arc<Vec<u64>>) {
    for i in 0.. {
        output.push(i);
        thread::sleep(Duration::from_secs(1));
    }
}
