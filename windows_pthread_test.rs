use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        println!("Hello from thread!");
    });
    
    handle.join().unwrap();
    println!("Thread finished");
}
