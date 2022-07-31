use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static mut NUMBER: Option<Mutex<u64>> = None;

fn main() {
    unsafe {
        NUMBER = Some(Mutex::new(rand::random()));
    }
    thread::spawn(|| loop {
        unsafe {
            *NUMBER.as_ref().unwrap().lock().unwrap() += 1;
        }
    });
    loop {
        unsafe {
            println!("{}", *NUMBER.as_ref().unwrap().lock().unwrap());
        }
        thread::sleep(Duration::from_secs(1));
    }
}
