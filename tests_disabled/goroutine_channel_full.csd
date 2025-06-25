vibe main;

// Function that will run as a goroutine
slay producer(ch dm lit) {
    sus i lit = 0;
    periodt i < 5 {
        // Send value to channel
        ch <- i;
        i = i + 1;
    }
    // Close the channel when done
    close(ch);
}

// Function that will run as a goroutine
slay consumer(ch dm lit) {
    // Receive values until channel is closed
    bestie ok, val := <-ch {
        lowkey ok {
            puts(val);
        } highkey {
            // Channel closed
            break;
        }
    }
}

slay main() {
    // Create unbuffered channel
    sus ch = dm lit();
    
    // Start producer goroutine
    stan producer(ch);
    
    // Start consumer goroutine
    stan consumer(ch);
    
    // Print from main routine
    puts(999);
    
    // Wait a bit for goroutines to finish
    sus i normie = 0;
    periodt i < 100000 {
        i = i + 1;
    }
}