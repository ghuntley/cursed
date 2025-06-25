vibe main;

slay main() {
    // Create a buffered channel with capacity 1
    sus ch = dm lit(1);
    
    // Try to send without blocking - should succeed
    bestie ok := ch <-? 42 {
        lowkey ok {
            puts("Send successful");
        } highkey {
            puts("Send would block");
        }
    }
    
    // Try to send again - should block since capacity is 1
    bestie ok := ch <-? 100 {
        lowkey ok {
            puts("Second send successful");
        } highkey {
            puts("Would block");
        }
    }
    
    // Try to receive without blocking - should succeed
    bestie ok, val := <-? ch {
        lowkey ok {
            puts(val);
        } highkey {
            puts("Receive would block");
        }
    }
    
    // Now the channel is empty, another receive should block
    bestie ok, val := <-? ch {
        lowkey ok {
            puts(val);
        } highkey {
            puts("Second receive would block");
        }
    }
}