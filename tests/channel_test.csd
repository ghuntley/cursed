fr fr Test file for channel operations
slay main() {
    fr fr Creating a channel
    sus ch = dm<smol>;
    
    fr fr Start a goroutine to send a value
    stan sendValue();
    
    fr fr Sleep to give goroutine time to run
    sleep(0.1);
    
    fr fr Receive a value from the channel
    sus value = <-ch;
    
    fr fr Test that the received value is correct
    lowkey value == 42 {
        fr fr Success
    } highkey {
        fr fr Failure
        yolo -1;
    }
    
    yolo 0;
}

fr fr Helper function that will run as a goroutine
slay sendValue() {
    fr fr Send an integer to the channel
    ch <- 42;
}