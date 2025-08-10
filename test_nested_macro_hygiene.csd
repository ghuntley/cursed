# Test for critical P1 issue #16: Nested macro hygiene implementation

yeet "vibez"

# Define a macro that captures a variable
#define OUTER_MACRO 
    sus captured_x drip = 42
    @INNER_MACRO
#end

# Define a nested macro that might accidentally capture 'x'
#define INNER_MACRO
    sus x drip = captured_x + 1
    vibez.spill("Inner macro result:", x)
#end

# Test case 1: Basic nested macro hygiene
slay test_basic_nested_hygiene() void {
    sus x drip = 10  # This should not be captured by macros
    @OUTER_MACRO
    vibez.spill("Outer x unchanged:", x)  # Should still be 10
}

# Test case 2: Multiple nesting levels
#define LEVEL_1
    sus temp drip = 100
    @LEVEL_2
#end

#define LEVEL_2
    sus temp drip = temp + 50  # Should use hygienic renaming
    @LEVEL_3
#end

#define LEVEL_3
    vibez.spill("Deep nested temp:", temp)
#end

slay test_deep_nesting() void {
    sus temp drip = 1
    @LEVEL_1
    vibez.spill("Original temp:", temp)  # Should still be 1
}

# Test case 3: Variable capture detection
#define CAPTURE_TEST(var_name)
    sus captured_var drip = var_name * 2
    vibez.spill("Captured:", captured_var)
#end

slay test_variable_capture() void {
    sus important_value drip = 25
    @CAPTURE_TEST(important_value)  # Should detect capture
}

# Test case 4: Scope escape prevention
#define SCOPE_ESCAPE_TEST
    sus local_only drip = 999
    # This should not escape the macro scope
#end

slay test_scope_escape() void {
    @SCOPE_ESCAPE_TEST
    # sus leaked drip = local_only  # This should fail or be hygienically renamed
}

# Main test function
slay main() drip {
    vibez.spill("Testing critical P1 macro hygiene fixes...")
    
    test_basic_nested_hygiene()
    test_deep_nesting()
    test_variable_capture()
    test_scope_escape()
    
    vibez.spill("Macro hygiene tests completed!")
    damn 0
}
