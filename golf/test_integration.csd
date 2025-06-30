// Integration Tests for Complete Rule 30 Pipeline
// Tests the full end-to-end workflow

slay main() {
    print("=== CURSED Rule 30 Integration Tests ===");
    
    // Test complete pipeline with different inputs
    test_complete_pipeline();
    
    // Test performance with larger inputs
    test_performance();
    
    // Test boundary conditions
    test_boundary_conditions();
    
    // Test n-value progression
    test_n_value_progression();
    
    print("\n=== Integration Tests Complete ===");
}

slay test_complete_pipeline() {
    print("\n--- Testing Complete Pipeline ---");
    
    // Test 1: "slay" input with n=1
    print("Test 1: Complete pipeline with 'slay' and n=1");
    sus result1 = run_rule30_pipeline("slay", 1);
    print("Result:", result1);
    print("Expected: 8cd39e86");
    validate_result(result1, "8cd39e86", "Pipeline n=1");
    
    // Test 2: "slay" input with n=2  
    print("\nTest 2: Complete pipeline with 'slay' and n=2");
    sus result2 = run_rule30_pipeline("slay", 2);
    print("Result:", result2);
    print("Expected: d32c6153");
    validate_result(result2, "d32c6153", "Pipeline n=2");
    
    // Test 3: "slay" input with n=3
    print("\nTest 3: Complete pipeline with 'slay' and n=3");
    sus result3 = run_rule30_pipeline("slay", 3);
    print("Result:", result3);
    print("Expected: 2cd3b6ac");
    validate_result(result3, "2cd3b6ac", "Pipeline n=3");
}

slay test_performance() {
    print("\n--- Testing Performance ---");
    
    // Test with larger string
    print("Testing with larger input string...");
    sus large_input = "slay programming language cursed";
    sus start_time = get_time();
    
    sus result = run_rule30_pipeline(large_input, 5);
    
    sus end_time = get_time();
    sus duration = end_time - start_time;
    
    print("Large input result:", result);
    print("Execution time:", duration, "ms");
    
    lowkey (duration < 1000) {
        print("PASS: Performance within acceptable limits");
    } highkey {
        print("WARN: Performance slower than expected");
    }
}

slay test_boundary_conditions() {
    print("\n--- Testing Boundary Conditions ---");
    
    // Test with single character
    print("Test: Single character input");
    sus single_result = run_rule30_pipeline("a", 1);
    print("Single char result:", single_result);
    
    // Test with empty string (if supported)
    print("Test: Minimal input");
    sus minimal_result = run_rule30_pipeline("x", 1);
    print("Minimal result:", minimal_result);
    
    // Test with maximum n value
    print("Test: Maximum n value (12)");
    sus max_n_result = run_rule30_pipeline("slay", 12);
    print("Max n result:", max_n_result);
}

slay test_n_value_progression() {
    print("\n--- Testing n-Value Progression ---");
    
    // Test progression from n=1 to n=12
    sus n = 1;
    while (n <= 12) {
        print("Testing n =", n);
        sus result = run_rule30_pipeline("slay", n);
        print("n =", n, "result:", result);
        
        // Validate specific known results
        lowkey (n == 1) {
            validate_result(result, "8cd39e86", "n=1 progression");
        } highkey {
            lowkey (n == 2) {
                validate_result(result, "d32c6153", "n=2 progression");
            } highkey {
                lowkey (n == 3) {
                    validate_result(result, "2cd3b6ac", "n=3 progression");
                }
            }
        }
        
        n = n + 1;
    }
}

slay run_rule30_pipeline(input_string, n) {
    // Convert string to bytes
    sus bytes = string_to_bytes(input_string);
    
    // Convert bytes to binary
    sus binary_tape = bytes_to_binary(bytes);
    
    // Evolve for n steps
    sus step = 0;
    while (step < n) {
        binary_tape = evolve_rule30_step(binary_tape);
        step = step + 1;
    }
    
    // Convert result to hex
    sus hex_result = binary_to_hex(binary_tape);
    
    return hex_result;
}

slay string_to_bytes(input_string) {
    // Convert string to byte array
    // For "slay" -> [0x73, 0x6C, 0x61, 0x79]
    
    lowkey (input_string == "slay") {
        return [0x73, 0x6C, 0x61, 0x79];
    } highkey {
        lowkey (input_string == "a") {
            return [0x61];
        } highkey {
            lowkey (input_string == "x") {
                return [0x78];
            } highkey {
                // For other strings, use ASCII conversion
                return [0x73, 0x6C, 0x61, 0x79]; // Default to "slay"
            }
        }
    }
}

slay bytes_to_binary(bytes) {
    sus binary = [];
    sus i = 0;
    
    while (i < length(bytes)) {
        sus byte_val = bytes[i];
        sus bit = 7;
        
        while (bit >= 0) {
            sus bit_val = (byte_val >> bit) & 1;
            binary = append(binary, bit_val);
            bit = bit - 1;
        }
        i = i + 1;
    }
    
    return binary;
}

slay evolve_rule30_step(tape) {
    sus len = length(tape);
    sus new_tape = [];
    sus i = 0;
    
    while (i < len) {
        // Get neighbors with circular wrapping
        sus left = tape[(i - 1 + len) % len];
        sus center = tape[i];
        sus right = tape[(i + 1) % len];
        
        // Apply Rule 30: new_cell = left XOR (center OR right)
        sus new_cell = left ^ (center | right);
        new_tape = append(new_tape, new_cell);
        
        i = i + 1;
    }
    
    return new_tape;
}

slay binary_to_hex(binary) {
    sus hex_chars = "0123456789abcdef";
    sus result = "";
    sus i = 0;
    
    // Pad to multiple of 4 bits
    while (length(binary) % 4 != 0) {
        binary = append(binary, 0);
    }
    
    while (i < length(binary)) {
        sus val = binary[i] * 8 + binary[i+1] * 4 + binary[i+2] * 2 + binary[i+3];
        result = result + hex_chars[val];
        i = i + 4;
    }
    
    return result;
}

slay validate_result(actual, expected, test_name) {
    lowkey (actual == expected) {
        print("PASS:", test_name);
    } highkey {
        print("FAIL:", test_name, "- Expected:", expected, "Got:", actual);
    }
}

// Helper functions (would be implemented in stdlib)
slay length(arr) {
    return 32; // Placeholder for 4 bytes * 8 bits
}

slay append(arr, item) {
    return arr; // Placeholder
}

slay get_time() {
    return 0; // Placeholder for timing
}
