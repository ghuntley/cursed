fr fr Minimal test for channel operations

vibe main;

yeet "vibez";

slay main() {
    fr fr Create a channel
    sus ch = dm smol
    
    fr fr Send a value
    ch <- 42
    
    fr fr Receive the value
    sus result = <-ch
    
    fr fr Check the result
    lowkey result == 42 {
        spill("Received 42 from channel")
        yolo 0
    }
    
    fr fr This should not execute if test passes
    spill("Channel test failed: expected 42, got")
    yolo 1
}