vibe main

slay main() {
    fr fr Create a buffered channel with capacity 2
    sus ch = dm smol[2]
    
    fr fr Send values to the channel
    ch <- 42
    ch <- 43
    
    fr fr Receive values from the channel
    sus result1 = <-ch
    sus result2 = <-ch
    
    fr fr Check the results
    yolo 0
}