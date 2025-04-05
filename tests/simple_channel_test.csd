fr fr Simple channel test in CURSED

vibe main

yeet "vibez"

slay main() {
    fr fr Creating a channel
    sus ch = dm smol
    
    fr fr Send a value
    ch <- 42
    
    fr fr Receive the value
    sus result = <-ch
    
    fr fr Check the result
    lowkey result == 42 {
        vibez.spill("Channel test passed: received 42 from channel")
    } highkey {
        vibez.spill("Channel test failed: expected 42, got", result)
    }
}