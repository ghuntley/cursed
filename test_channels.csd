// Test the new channel implementation
fn main() {
    // Create a channel
    let (sender, receiver) = dm::<i32>();
    
    // Send a value
    sender.send(42);
    
    // Receive the value
    let value = receiver.recv();
    
    // Print the result
    println!("Received: {}", value);
}
