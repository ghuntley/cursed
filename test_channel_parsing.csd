// Test channel operations and select statement parsing

slay main_character() {
    // Channel declaration  
    sus my_channel = dm<normie>()
    
    // Goroutine that sends to channel
    stan send_worker(my_channel)
    
    // Select statement for channel multiplexing
    select {
        mood value <- my_channel {
            vibez.spill("Received: ", value)
        }
        basic {
            vibez.spill("No data available")
        }
    }
    
    // Direct channel operations
    my_channel <- 42
    sus result = <-my_channel
    vibez.spill("Result: ", result)
}

slay send_worker(ch) {
    ch <- 100
    ch <- 200
}
