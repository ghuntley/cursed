# Test comprehensive select/ready statement functionality
yeet "vibez"

slay main() {
    # Create multiple channels
    sus ch1 := dm_buffered<normie>(1)
    sus ch2 := dm_buffered<normie>(1)
    sus ch3 := dm_buffered<normie>(1)
    
    # Send values to different channels
    ch1.send(10)
    ch2.send(20)
    
    # Use select to handle multiple channels
    ready {
        case val := <-ch1: 
            vibez.spill("Received from ch1: " + val)
        case val := <-ch2: 
            vibez.spill("Received from ch2: " + val)
        case val := <-ch3: 
            vibez.spill("Received from ch3: " + val)
        basic:
            vibez.spill("No value received from any channel")
    }
    
    # Test select with send operations
    ready {
        case ch3 <- 30:
            vibez.spill("Sent value 30 to ch3")
        basic:
            vibez.spill("Could not send to ch3")
    }
    
    # Test receiving from the channel we just sent to
    ready {
        case val := <-ch3:
            vibez.spill("Received from ch3: " + val)
        basic:
            vibez.spill("No value in ch3")
    }
}
