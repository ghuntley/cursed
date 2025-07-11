# Test fixed select/ready statement runtime
yeet "vibez"

slay main() {
    vibez.spill("Testing select statement runtime fixes...")
    
    # Test 1: Simple buffered channel receive
    vibez.spill("Test 1: Simple channel receive")
    sus ch := dm_buffered<normie>(1)
    
    # Send a value
    ch.send(42)
    
    # Use select to receive
    ready {
        case val := <-ch: 
            vibez.spill("Received value: " + val)
        basic:
            vibez.spill("No value received")
    }
    
    # Test 2: Default case when no channels ready
    vibez.spill("Test 2: Default case")
    sus ch2 := dm_buffered<normie>(1)
    
    ready {
        case val := <-ch2:
            vibez.spill("Received unexpected value: " + val)
        basic:
            vibez.spill("Default case executed correctly")
    }
    
    # Test 3: Multiple channels with one ready
    vibez.spill("Test 3: Multiple channels")
    sus ch3 := dm_buffered<normie>(1)
    sus ch4 := dm_buffered<normie>(1)
    
    # Send to one channel
    ch3.send(100)
    
    ready {
        case val := <-ch3:
            vibez.spill("Received from ch3: " + val)
        case val := <-ch4:
            vibez.spill("Received from ch4: " + val)
        basic:
            vibez.spill("No channels ready")
    }
    
    vibez.spill("Select statement runtime tests completed!")
}
