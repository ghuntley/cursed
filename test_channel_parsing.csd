// Test channel type declarations
sus ch dm<normie>                     // Unbuffered channel declaration
sus buffered dm<tea>[10]              // Buffered channel declaration

// Test channel operations
ch <- 42                             // Send operation
sus value := <-ch                    // Receive operation
sus value2, ok := <-ch               // Receive with closed check (TODO: multiple assignment)

// Test channel creation with make
sus ch2 := make(dm<normie>)          // Create unbuffered channel
sus buffered2 := make(dm<tea>, 10)   // Create buffered channel

// Test close function
close(ch)                            // Close channel
