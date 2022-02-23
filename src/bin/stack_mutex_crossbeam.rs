use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let output = Mutex::new(Vec::new());
    crossbeam::scope(|scope| {
        scope.spawn(|_| background_count(&output));
        loop {
            thread::sleep(Duration::from_millis(100));
            println!("{:?}", output.lock().unwrap());
        }
    })
    .unwrap();
}

fn background_count(output: &Mutex<Vec<u64>>) {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));
        output.lock().unwrap().push(i);
    }
}