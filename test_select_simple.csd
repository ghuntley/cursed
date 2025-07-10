# Test basic select/ready statement functionality
yeet "vibez"

slay main() {
    # Create a channel
    sus ch := dm_buffered<normie>(1)
    
    # Send a value
    ch.send(42)
    
    # Use select to receive from channel
    ready {
        case <-ch: 
            vibez.spill("Received value from channel")
        basic:
            vibez.spill("No value received")
    }
}
