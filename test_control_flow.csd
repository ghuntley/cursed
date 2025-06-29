// Test control flow in CURSED
vibe main

sus x = 10
sus y = 5

slay main() {
    vibez.spill("Testing control flow");
    
    // Basic conditional
    if (x > y) {
        vibez.spill("x is greater than y");
    } else {
        vibez.spill("x is not greater than y");
    }
    
    // Basic loop
    sus i = 0
    while (i < 3) {
        vibez.spill("Loop iteration:");
        vibez.spill(i);
        i = i + 1
    }
    
    vibez.spill("Control flow test completed");
}
