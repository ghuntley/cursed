// Test FFI replacement with direct function calls

slay main() {
    vibez.spill("=== Testing Math Functions Migration ===");
    
    # Test constants (these should work since they return constant values)
    vibez.spill("Testing PI:");
    sus pi_val meal = math_pi();
    vibez.spill(pi_val);
    
    # Test basic abs function 
    vibez.spill("Testing abs(-5):");
    sus abs_result meal = math_abs(-5.0);
    vibez.spill(abs_result);
    
    # Test sqrt function
    vibez.spill("Testing sqrt(16):");
    sus sqrt_result meal = math_sqrt(16.0);
    vibez.spill(sqrt_result);
    
    # Test sin function
    vibez.spill("Testing sin(0):");
    sus sin_result meal = math_sin(0.0);
    vibez.spill(sin_result);
    
    # Test cos function  
    vibez.spill("Testing cos(0):");
    sus cos_result meal = math_cos(0.0);
    vibez.spill(cos_result);
    
    # Test min/max functions
    vibez.spill("Testing min(3, 7):");
    sus min_result meal = math_min(3.0, 7.0);
    vibez.spill(min_result);
    
    vibez.spill("Testing max(3, 7):");
    sus max_result meal = math_max(3.0, 7.0);
    vibez.spill(max_result);
    
    vibez.spill("=== Math Functions Test Complete ===");
}
