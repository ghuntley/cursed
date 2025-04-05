fr fr Test file for channel operations
slay main() {
    fr fr Creating a channel
    sus ch = dm smol
    
    fr fr Start a goroutine to send a value
    stan {
        ch <- 42
        fr fr Send an integer to the channel
    }
    
    fr fr Receive a value from the channel
    sus value = <-ch
    
    fr fr Test that the received value is correct
    lowkey value == 42 {
        fr fr Success
    } highkey {
        fr fr Failure
        yolo -1
    }
    
    yolo 0
}