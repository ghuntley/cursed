vibe main

slay close_channel(c){
    fr fr Close the channel
    c.close()
}

slay main() {
    fr fr Create a channel
    sus ch = dm smol
    
    fr fr Send a value to the channel
    ch <- 42
    
    fr fr Close the channel
    close_channel(ch)
    
    fr fr Receive the last value (should succeed)
    sus result = <-ch
    
    yolo 0
}