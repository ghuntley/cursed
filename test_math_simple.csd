// Simple test for pure CURSED math
yeet "testz"
yeet "math"

slay test_basic_math() {
    test_start("Basic math operations");
    
    # Test constants
    vibez.spill("Testing constants...");
    sus pi_val meal = math_pi();
    vibez.spill("PI: ");
    vibez.spill(pi_val);
    
    # Test basic operations
    vibez.spill("Testing abs...");
    sus abs_result meal = math_abs(-5.0);
    vibez.spill("abs(-5): ");
    vibez.spill(abs_result);
    
    # Test sqrt
    vibez.spill("Testing sqrt...");
    sus sqrt_result meal = math_sqrt(4.0);
    vibez.spill("sqrt(4): ");
    vibez.spill(sqrt_result);
    
    # Test sin
    vibez.spill("Testing sin...");
    sus sin_result meal = math_sin(0.0);
    vibez.spill("sin(0): ");
    vibez.spill(sin_result);
    
    print_test_summary();
}

slay main() {
    vibez.spill("Simple math test starting...");
    test_basic_math();
    vibez.spill("Test completed.");
}
