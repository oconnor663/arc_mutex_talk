use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let number = Mutex::new(0u64);
    thread::scope(|scope| {
        scope.spawn(|| add_loop(&number));
        loop {
            println!("{}", *number.lock().unwrap());
            thread::sleep(Duration::from_secs(1));
        }
    });
}
fn add_loop(alias: &Mutex<u64>) {
    loop {
        *alias.lock().unwrap() += 1;
    }
}
