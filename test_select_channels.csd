yeet "vibez"

slay test_channel_creation() {
    vibez.spill("Creating channel...")
    
    # Create a buffered channel
    sus ch := dm normie(1)
    vibez.spill("Channel created!")
    
    # Test select with send
    ready {
        mood ch <- 42:
            vibez.spill("Sent 42 to channel")
        basic:
            vibez.spill("Send would block")
    }
    
    # Test select with receive
    ready {
        mood result := <-ch:
            vibez.spill("Received from channel:", result)
        basic:
            vibez.spill("Receive would block")
    }
    
    vibez.spill("Channel test completed!")
}

test_channel_creation()
