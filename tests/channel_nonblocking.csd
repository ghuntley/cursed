vibe main;

slay main() {
    // Create a buffered channel with capacity 1
    sus ch = dm lit(1);
    
    // Try non-blocking send - should succeed (channel has space)
    bestie ok := ch <-? 42 {
        lowkey ok {
            puts("Send successful");
        } highkey {
            puts("Send would block");
        }
    }
    
    // Try non-blocking send again - should fail (channel is full)
    bestie ok := ch <-? 100 {
        lowkey ok {
            puts("Second send successful");
        } highkey {
            puts("Would block");
        }
    }
    
    // Try non-blocking receive - should succeed
    bestie ok, val := <-? ch {
        lowkey ok {
            puts("Received: " + val);
        } highkey {
            puts("Receive would block");
        }
    }
}