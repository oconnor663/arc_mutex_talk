use std::mem::MaybeUninit;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static mut NUMBER: MaybeUninit<Mutex<u64>> = MaybeUninit::uninit();

fn main() {
    unsafe {
        NUMBER.write(Mutex::new(0u64));
    }
    thread::spawn(add_loop);
    loop {
        unsafe {
            println!("{}", *NUMBER.assume_init_ref().lock().unwrap());
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop() {
    loop {
        unsafe {
            *NUMBER.assume_init_ref().lock().unwrap() += 1;
        }
    }
}
