vibe main;

slay main() {
    // Create an unbuffered channel
    sus ch = dm lit();
    
    // Send a value to the channel
    ch <- 42;
    
    // Receive the value from the channel
    sus value = <-ch;
    
    // Print the value received
    puts(value);
}