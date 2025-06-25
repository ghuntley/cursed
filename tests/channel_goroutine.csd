vibe main;

slay main() {
    // Create a channel for integers
    sus ch = dm lit();
    
    // Launch a goroutine that sends a value to the channel
    stan () {
        // Send 42 to the channel
        ch <- 42;
    }();
    
    // Receive the value from the channel
    sus x = <-ch;
    
    // Print the received value
    puts(x);
}