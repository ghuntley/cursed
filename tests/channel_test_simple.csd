vibe main;

slay main() {
    // Create a channel for integers
    sus ch = dm lit();
    
    // Send a value to the channel
    ch <- 42;
    
    // Receive the value from the channel
    sus value = <-ch;
    
    // Print the value
    puts(value);
}