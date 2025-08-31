vibe main

yeet "vibez"
yeet "mathz"

fr fr Test: Boundary value edge cases
fr fr Purpose: Test handling of edge values and boundary conditions
fr fr Expected: All boundary cases should be handled correctly

damn main() {
    vibez.spill("=== Testing Boundary Values ===");
    
fr fr Test zero values
    sus zero_int: i32 = 0;
    sus zero_float: f32 = 0.0;
    
    vibez.spill("Zero integer: {}", zero_int);
    vibez.spill("Zero float: {}", zero_float);
    
fr fr Test negative values
    sus neg_int: i32 = -42;
    sus neg_float: f32 = -3.14;
    
    vibez.spill("Negative integer: {}", neg_int);
    vibez.spill("Negative float: {}", neg_float);
    
fr fr Test division by zero prevention (if handled)
    sus dividend: i32 = 10;
    sus divisor: i32 = 0;
    
    if divisor != 0 {
        sus safe_division: i32 = dividend / divisor;
        vibez.spill("Safe division: {}", safe_division);
    } else {
        vibez.spill("Division by zero avoided");
    }
    
fr fr Test maximum/minimum comparisons
    sus large_num: i32 = 1000000;
    sus small_num: i32 = -1000000;
    
    sus max_val: i32 = mathz.max(large_num, small_num);
    sus min_val: i32 = mathz.min(large_num, small_num);
    
    vibez.spill("Max of {} and {}: {}", large_num, small_num, max_val);
    vibez.spill("Min of {} and {}: {}", large_num, small_num, min_val);
    
fr fr Test boolean edge cases
    sus always_true: bool = true || false;
    sus always_false: bool = false && true;
    
    vibez.spill("Always true: {}", always_true);
    vibez.spill("Always false: {}", always_false);
    
fr fr Test empty string
    sus empty_string: string = "";
    vibez.spill("Empty string: '{}'", empty_string);
    
fr fr Test single character operations
    sus single_digit: i32 = 1;
    sus result: i32 = mathz.abs(single_digit);
    vibez.spill("Abs of single digit: {}", result);
    
    return 0;
}
