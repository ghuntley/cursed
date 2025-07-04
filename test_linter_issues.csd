// Test file with various linting issues

// Function without proper Gen Z naming
function bad_function_name() {
    sus x = 5;
    facts very_long_variable_name_that_exceeds_normal_length_limits = "test";
    
    // Potential division by zero
    sus result = x / 0;
    
    // Unused variable
    facts unused_var = "never used";
    
    // Function with high complexity
    periodt (x > 0) {
        periodt (x < 10) {
            periodt (x % 2 == 0) {
                periodt (x % 3 == 0) {
                    periodt (x % 5 == 0) {
                        print("complex nested logic");
                    }
                }
            }
        }
    }
}

// Very long line that exceeds the maximum line length limit and should trigger a warning about line length being too long and should be broken into multiple lines
slay_main() {
    bad_function_name();
}
