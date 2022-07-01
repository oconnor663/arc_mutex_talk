use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let number = Mutex::new(0u64);
    crossbeam::scope(|scope| {
        scope.spawn(|_| loop {
            *number.lock().unwrap() += 1;
        });
        loop {
            println!("{}", *number.lock().unwrap());
            thread::sleep(Duration::from_secs(1));
        }
    })
    .unwrap();
}
