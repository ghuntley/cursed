// CURSED Compiler Status Test
// Testing what currently works and what doesn't

// Global variables work
sus message = "CURSED compiler is partially working!"
sus number = 42
sus flag = true

// Main function works
slay main() {
    // Basic output works
    vibez.spill("=== CURSED Compiler Test Results ===");
    
    // Variable access works
    vibez.spill("Message: ");
    vibez.spill(message);
    
    // Basic arithmetic works
    sus result = number + 8
    vibez.spill("Arithmetic (42 + 8): ");
    vibez.spill(result);
    
    // Boolean values work
    vibez.spill("Boolean flag: ");
    vibez.spill(flag);
    
    // String concatenation may work
    sus greeting = "Hello, " + "World!"
    vibez.spill("String concat: ");
    vibez.spill(greeting);
    
    vibez.spill("=== End Test ===");
}
