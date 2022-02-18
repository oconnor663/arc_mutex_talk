use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

fn main() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let output_clone = output.clone();
    thread::spawn(|| background_count(output_clone));
    loop {
        thread::sleep(Duration::from_millis(100));

        let mutex: &Mutex<Vec<u64>> = output.deref();
        let lock_result: Result<MutexGuard<Vec<u64>>, _> = mutex.lock();
        let guard: MutexGuard<Vec<u64>> = lock_result.unwrap();
        let v: &Vec<u64> = guard.deref();

        println!("{:?}", v);
    }
}

fn background_count(output: Arc<Mutex<Vec<u64>>>) {
    for i in 0.. {
        thread::sleep(Duration::from_secs(1));

        let mutex: &Mutex<Vec<u64>> = output.deref();
        let lock_result: Result<MutexGuard<Vec<u64>>, _> = mutex.lock();
        let mut guard: MutexGuard<Vec<u64>> = lock_result.unwrap();
        let v: &mut Vec<u64> = guard.deref_mut();

        v.push(i);
    }
}
