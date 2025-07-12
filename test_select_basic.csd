yeet "core"

slay test_select_basic() {
    vibez.spill("Testing basic select statement...")
    
    # Create a channel
    sus ch := core.make_channel(normie, 1)
    
    # Test select with default case
    ready {
        mood value := <-ch:
            vibez.spill("Received value (should not happen)")
        basic:
            vibez.spill("Default case executed")
    }
    
    # Test select with send operation
    ready {
        mood ch <- 42:
            vibez.spill("Sent value to channel")
        basic:
            vibez.spill("Send would block")
    }
    
    # Test select with receive operation
    ready {
        mood received := <-ch:
            vibez.spill("Received from channel: ", received)
        basic:
            vibez.spill("Receive would block")
    }
    
    vibez.spill("Select test completed!")
}

test_select_basic()
