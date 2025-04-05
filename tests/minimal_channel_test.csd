fr fr Minimal channel test

vibe main

yeet "vibez"

slay main() {
    vibez.spill("Channel Test")
    
    fr fr Create a channel
    sus ch = dm smol
    
    fr fr Send a value
    ch <- 42
    
    fr fr Receive the value
    sus result = <-ch
    
    vibez.spill("Received from channel:", result)
}