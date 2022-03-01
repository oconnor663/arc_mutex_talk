use std::ops::{Deref, DerefMut, RangeFrom};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use std::clone::Clone;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::option::Option::Some;
use std::result::Result;
use std::vec::Vec;

fn main() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let output_clone = output.clone();
    thread::spawn(|| {
        let moved_clone = output_clone;
        let mutex_ref: &Mutex<Vec<u64>> = moved_clone.deref();
        fill_vector(mutex_ref);
    });
    loop {
        let mutex: &Mutex<Vec<u64>> = output.deref();
        let lock_result: Result<MutexGuard<Vec<u64>>, _> = mutex.lock();
        let guard: MutexGuard<Vec<u64>> = lock_result.unwrap();
        // let v: &Vec<u64> = guard.deref();
        println!("{:?}", guard);
        drop(guard);
        thread::sleep(Duration::from_millis(100));
    }
}

fn fill_vector(output: &Mutex<Vec<u64>>) {
    let range: RangeFrom<u64> = 0..;
    let mut iterator = range.into_iter();
    while let Some(i) = iterator.next() {
        let lock_result: Result<MutexGuard<Vec<u64>>, _> = output.lock();
        let mut guard: MutexGuard<Vec<u64>> = lock_result.unwrap();
        let v: &mut Vec<u64> = guard.deref_mut();
        v.push(i);
        drop(guard);
        thread::sleep(Duration::from_secs(1));
    }
}
