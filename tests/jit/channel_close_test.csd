vibe main;

slay main() {
    // Create an unbuffered channel
    sus ch = dm lit();
    
    // Send a value to the channel
    stan () {
        ch <- 42;
    }();
    
    // Receive the value from the channel
    sus val = <-ch;
    puts(val);
    
    // Close the channel
    close(ch);
    
    // Try to receive from closed channel - should fail
    bestie ok, val := <-? ch {
        lowkey ok {
            puts(val);
        } highkey {
            puts("Channel closed");
        }
    }
}