fr fr Minimal test for channel operations

vibe main;

yeet "vibez";

slay main() {
    fr fr Create a channel
    sus ch = dm smol;
    
    fr fr Send a value
    dm_send(ch, 42);
    
    fr fr Receive the value
    sus result = dm_recv(ch);
    
    fr fr Check the result
    lowkey result == 42 {
        vibez.spill("Channel test passed: value received is 42");
        damn 0;
    } highkey {
        vibez.spill("Channel test failed: expected 42, got");
        vibez.spill(result);
        damn 1;
    }
}