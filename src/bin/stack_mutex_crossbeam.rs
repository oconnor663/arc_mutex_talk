use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let output = Mutex::new(Vec::new());
    crossbeam::scope(|scope| {
        scope.spawn(|_| fill_vector(&output));
        loop {
            println!("{:?}", output.lock().unwrap());
            thread::sleep(Duration::from_millis(100));
        }
    })
    .unwrap();
}

fn fill_vector(output: &Mutex<Vec<u64>>) {
    for i in 0.. {
        output.lock().unwrap().push(i);
        thread::sleep(Duration::from_secs(1));
    }
}
