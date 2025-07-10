# Test select/ready statement with timeout functionality
yeet "vibez"

slay main() {
    # Create an empty channel
    sus ch := dm<normie>()
    
    # Test select with timeout using basic case
    ready {
        case val := <-ch:
            vibez.spill("Received value: " + val)
        basic:
            vibez.spill("Timeout: No value received")
    }
    
    # Test sending to channel and then receiving
    yolo {
        # Send after a brief delay
        ch.send(99)
    }
    
    # This should receive the value
    ready {
        case val := <-ch:
            vibez.spill("Received delayed value: " + val)
        basic:
            vibez.spill("No delayed value received")
    }
}
