fr fr Minimal test for channel operations

vibe main;

yeet "vibez";

slay main() {
    fr fr Create a channel
    sus ch = dm smol;
    
    fr fr Send a value
    ch <- 42;
    
    fr fr Receive the value
    sus result = <-ch;
    
    fr fr Check the result
    lowkey result == 42 {
        vibez.spill("Channel test passed: value received is 42");
        yolo 0;
    } highkey {
        vibez.spill("Channel test failed: expected 42, got", result);
        yolo 1;
    }
}