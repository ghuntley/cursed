yeet "core"

slay test_select_implementation() {
    vibez.spill("Testing complete select statement implementation...")
    
    # Test 1: Basic select with default case
    sus ch1 := core.make_channel(normie, 1)
    sus ch2 := core.make_channel(normie, 1)
    
    # Send to ch1
    ch1 <- 42
    
    # Select should pick ch1 since it's ready
    ready {
        mood val := <-ch1:
            vibez.spill("Received from ch1: ", val)
        mood val := <-ch2:
            vibez.spill("Received from ch2: ", val)
        basic:
            vibez.spill("Default case executed")
    }
    
    # Test 2: Select with timeout
    sus timeout_ch := core.make_channel(normie, 1)
    
    # Start a goroutine to send after delay
    yolo slay() {
        core.sleep(100)
        timeout_ch <- 999
    }()
    
    # Select with timeout
    ready {
        mood val := <-timeout_ch:
            vibez.spill("Received before timeout: ", val)
        mood <-core.timeout(50):
            vibez.spill("Timeout occurred")
    }
    
    # Test 3: Select with multiple send operations
    sus send_ch1 := core.make_channel(normie, 1)
    sus send_ch2 := core.make_channel(normie, 1)
    
    ready {
        mood send_ch1 <- 100:
            vibez.spill("Sent to ch1")
        mood send_ch2 <- 200:
            vibez.spill("Sent to ch2")
        basic:
            vibez.spill("All sends would block")
    }
    
    # Test 4: Select with mixed send/receive operations
    sus mixed_ch := core.make_channel(normie, 2)
    mixed_ch <- 1
    
    ready {
        mood val := <-mixed_ch:
            vibez.spill("Received: ", val)
        mood mixed_ch <- 2:
            vibez.spill("Sent: 2")
        basic:
            vibez.spill("All operations would block")
    }
    
    vibez.spill("Select implementation test completed!")
}

test_select_implementation()
