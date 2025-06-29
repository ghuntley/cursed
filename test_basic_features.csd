// Test basic CURSED language features
vibe main

// Basic variables
sus number = 42
sus text = "CURSED is working!"
sus flag = true

slay main() {
    // Basic print statements
    vibez.spill("Testing basic CURSED features");
    vibez.spill(text);
    
    // Variable access
    vibez.spill("Number value:");
    vibez.spill(number);
    
    // Basic arithmetic
    sus result = number + 8
    vibez.spill("Arithmetic result (42 + 8):");
    vibez.spill(result);
    
    // Function call
    test_function()
}

slay test_function() {
    vibez.spill("Function calls work!");
}
