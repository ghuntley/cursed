fr fr A simpler test for channels

vibe main

slay main() {
    fr fr Create a channel
    sus ch = dm smol
    
    fr fr Send a value
    ch <- 42
    
    fr fr Receive the value
    sus result = <-ch
    
    fr fr Check the result
    lowkey result == 42 {
        yolo 0
    }
    
    yolo 1
}