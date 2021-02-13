use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use std::thread;

fn fib(num: u64) -> u64 {
    if num < 2 { 1 } else { fib(num - 2) + fib(num - 1) }
}

fn main() {
    let locked_results = Arc::new(Mutex::new(BTreeMap::new()));
    let threads : Vec<_> = (0..35).map(|i| {
        let locked_results = locked_results.clone();
        thread::spawn(move || {
            let rv = fib(i);
            locked_results.lock().unwrap().insert(i, rv);
        })
    }).collect();
    for thread in threads { thread.join().unwrap(); }
    for (i, rv) in locked_results.lock().unwrap().iter() {
        println!("fib({}) = {}", i, rv);
    }
}
