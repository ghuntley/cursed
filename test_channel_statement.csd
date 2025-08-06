slay main_character() {
    sus ch = dm(drip, 5)
    
    // Send value to channel
    ch <- 42
    
    // Receive value from channel
    sus received = <-ch
    vibez.spill(received)
    
    vibez.spill("Channel operations completed")
}
