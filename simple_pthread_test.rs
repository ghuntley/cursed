use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = data.clone();

    let handle = thread::spawn(move || {
        let mut num = data_clone.lock().unwrap();
        *num += 1;
    });

    handle.join().unwrap();
    
    println!("Result: {}", *data.lock().unwrap());
}
