// Test control flow structures
vibe main

sus x = 10
sus y = 5

slay main() {
    vibez.spill("Testing control structures");
    
    // Test if-else
    if (x > y) {
        vibez.spill("x is greater than y - PASS");
    } else {
        vibez.spill("x is not greater than y - FAIL");
    }
    
    // Test while loop
    sus counter = 0
    vibez.spill("Testing while loop:");
    while (counter < 3) {
        vibez.spill("Counter: ");
        vibez.spill(counter);
        counter = counter + 1
    }
    
    vibez.spill("Control structures test completed");
}
