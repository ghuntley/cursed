slay main_character() {
    sus ch1 = dm(drip, 5)
    sus ch2 = dm(tea, 5)
    
    vibe_check_multi {
        case ch1 <- 42:
            vibez.spill("Sent to ch1")
        case msg := <-ch2:
            vibez.spill("Received from ch2")
        default:
            vibez.spill("No channel operation ready")
    }
    
    vibez.spill("Select statement completed")
}
