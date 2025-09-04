fr fr Basic Channel Send/Receive Example
fr fr Demonstrates the fundamentals of channel communication

vibe main

slay main_character() {
    puts("=== CURSED Channel Hello World ===")
    
    // Create an unbuffered channel
    sus greeting_ch = make(dm<tea>)
    sus number_ch = make(dm<normie>)
    
    puts("Creating channels...")
    
    // Start a goroutine that sends greetings
    stan {
        puts("Sender goroutine: Sending greeting...")
        greeting_ch <- "Hello from CURSED channels!"
        
        puts("Sender goroutine: Sending number...")
        number_ch <- 42
        
        puts("Sender goroutine: Done sending")
    }
    
    // Receive from channels in main
    puts("Main: Waiting for greeting...")
    sus greeting = <-greeting_ch
    puts(sprintf("Main: Received greeting: %s", greeting))
    
    puts("Main: Waiting for number...")
    sus number = <-number_ch
    puts(sprintf("Main: Received number: %d", number))
    
    puts("=== Basic Channel Demo Complete ===")
    
    // Demonstrate buffered channels
    buffered_channel_demo()
    
    // Demonstrate channel closing
    channel_closing_demo()
}

slay buffered_channel_demo() {
    puts("\n=== Buffered Channel Demo ===")
    
    // Create a buffered channel with capacity 3
    sus buffer_ch = make(dm<normie>, 3)
    
    puts("Sending to buffered channel (non-blocking)...")
    
    // These sends won't block because of the buffer
    buffer_ch <- 1
    puts("Sent: 1")
    
    buffer_ch <- 2
    puts("Sent: 2")
    
    buffer_ch <- 3
    puts("Sent: 3")
    
    puts(sprintf("Channel length: %d, capacity: %d", len(buffer_ch), cap(buffer_ch)))
    
    // Now receive the values
    puts("Receiving from buffered channel...")
    sus val1 = <-buffer_ch
    sus val2 = <-buffer_ch
    sus val3 = <-buffer_ch
    
    puts(sprintf("Received: %d, %d, %d", val1, val2, val3))
    puts(sprintf("Channel length after receiving: %d", len(buffer_ch)))
}

slay channel_closing_demo() {
    puts("\n=== Channel Closing Demo ===")
    
    sus ch = make(dm<normie>, 5)
    
    // Send some values
    ch <- 10
    ch <- 20
    ch <- 30
    
    puts("Sent values: 10, 20, 30")
    
    // Close the channel
    close(ch)
    puts("Channel closed")
    
    // Receive values and detect closure
    periodt based {
        sus value, ok = <-ch
        lowkey !ok {
            puts("Channel is closed, no more values")
            stan_it
        }
        puts(sprintf("Received: %d", value))
    }
    
    // Try to receive from closed channel
    sus final_value, still_open = <-ch
    puts(sprintf("Final receive - value: %d, channel open: %s", 
                final_value, still_open ? "based" : "sus"))
}
