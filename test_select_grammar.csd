yeet "vibez"

slay test_select_basic() lit {
    vibez.spill("=== Basic Select Test ===")
    
    # Basic select with default case
    ready {
        basic:
            vibez.spill("Default case executed")
    }
    
    damn based
}

slay test_select_with_channels() lit {
    vibez.spill("=== Select with Channels Test ===")
    
    # Create channels for testing
    sus ch1 dm<normie>
    sus ch2 dm<tea>
    
    ready {
        mood ch1 <- 42:
            vibez.spill("Send to ch1 successful")
        mood msg := <-ch2:
            vibez.spill("Received from ch2")
        basic:
            vibez.spill("No channel operations ready")
    }
    
    damn based
}

slay test_select_receive() lit {
    vibez.spill("=== Select Receive Test ===")
    
    sus ch dm<normie>
    
    ready {
        mood val := <-ch:
            vibez.spill("Received value from channel")
        basic:
            vibez.spill("No receive operation ready")
    }
    
    damn based
}

slay test_select_send() lit {
    vibez.spill("=== Select Send Test ===")
    
    sus ch dm<normie>
    
    ready {
        mood ch <- 100:
            vibez.spill("Send operation successful")
        basic:
            vibez.spill("Channel not ready for send")
    }
    
    damn based
}

slay test_select_multiple_cases() lit {
    vibez.spill("=== Select Multiple Cases Test ===")
    
    sus ch1 dm<normie>
    sus ch2 dm<tea>
    sus ch3 dm<lit>
    
    ready {
        mood ch1 <- 42:
            vibez.spill("Send to ch1")
        mood msg := <-ch2:
            vibez.spill("Received from ch2")
        mood flag := <-ch3:
            vibez.spill("Received flag from ch3")
        basic:
            vibez.spill("No operations ready")
    }
    
    damn based
}

slay main() lit {
    vibez.spill("=== Select Grammar Tests ===")
    
    test_select_basic()
    vibez.spill("")
    
    test_select_with_channels()
    vibez.spill("")
    
    test_select_receive()
    vibez.spill("")
    
    test_select_send()
    vibez.spill("")
    
    test_select_multiple_cases()
    vibez.spill("")
    
    vibez.spill("=== Tests Complete ===")
    damn based
}
