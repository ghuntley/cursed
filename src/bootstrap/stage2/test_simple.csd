// Simple test program for the Stage 2 CURSED compiler
// Tests basic functionality that the self-hosting compiler should handle

vibe "test_simple";

// Main function - comprehensive test
slay main() -> normie {
    // Variable declarations
    sus x = 42;
    facts y = 24;
    sus total = 0;
    
    // Arithmetic operations
    sus sum = x + y;
    sus product = x * y;
    sus difference = x - y;
    sus quotient = x / y;
    
    // Function calls
    sus calculated = calculate(x, y);
    sus count_result = count_up(5);
    
    // String operations
    sus message = "CURSED Stage 2 Compiler Test";
    sus success_msg = "All tests passed!";
    
    // Boolean operations
    facts is_greater = sum > 50;
    facts is_equal = (x == 42);
    facts logical_result = is_greater && is_equal;
    
    // Control flow
    lowkey (logical_result) {
        total = sum + product + calculated + count_result;
        
        // Nested control flow
        lowkey (total > 1000) {
            yolo 0; // Success
        } highkey {
            yolo 2; // Unexpected value
        }
    } highkey {
        yolo 1; // Logic error
    }
}

// Helper function with more complex logic
slay calculate(a: normie, b: normie) -> normie {
    sus result = a + b * 2;
    
    // Conditional logic
    lowkey (result > 100) {
        result = result / 2;
    }
    
    yolo result;
}

// Function with loop and more complex control
slay count_up(limit: normie) -> normie {
    sus count = 0;
    sus total = 0;
    
    periodt (count < limit) {
        total = total + count;
        count = count + 1;
        
        // Skip even numbers
        lowkey (count % 2 == 0) {
            simp; // continue
        }
        
        // Break early if total gets too large
        lowkey (total > 50) {
            ghosted; // break
        }
    }
    
    yolo total;
}

// Test function for type system
slay test_types() -> cap {
    sus number = 123;
    facts text = "Hello, CURSED!";
    sus flag = truth;
    
    // Type compatibility checks
    sus result = number + 456;
    
    yolo flag;
}

// Test function for error handling patterns
slay test_error_patterns() -> normie {
    sus value = 10;
    
    // Division by zero protection
    lowkey (value != 0) {
        sus safe_division = 100 / value;
        yolo safe_division;
    } highkey {
        yolo -1; // Error code
    }
}
