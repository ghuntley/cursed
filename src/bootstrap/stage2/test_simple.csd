// Simple test program for the Stage 2 CURSED compiler
// Tests basic functionality that the self-hosting compiler should handle

vibe "test_simple";

// Main function
slay main() -> normie {
    // Variable declarations
    sus x = 42;
    facts y = 24;
    
    // Arithmetic operations
    sus sum = x + y;
    sus product = x * y;
    
    // Control flow
    lowkey (sum > 50) {
        // String operations
        sus message = "The sum is greater than 50";
        yolo 0;
    } highkey {
        yolo 1;
    }
}

// Helper function
slay calculate(a: normie, b: normie) -> normie {
    sus result = a + b * 2;
    yolo result;
}

// Function with loop
slay count_up(limit: normie) -> normie {
    sus count = 0;
    
    periodt (count < limit) {
        count = count + 1;
    }
    
    yolo count;
}
