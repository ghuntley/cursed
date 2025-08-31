vibe main

yeet "vibez"
yeet "mathz"

fr fr Test: mathz module mathematical operations
fr fr Purpose: Validate mathz functions work correctly
fr fr Expected: All mathematical operations should produce correct results

damn main() {
fr fr Test basic mathz functions
    sus x: i32 = -42;
    sus y: i32 = 15;
    sus z: f32 = -7.5;
    
fr fr Test absolute value
    sus abs_x: i32 = mathz.abs(x);
    vibez.spill("abs({}) = {}", x, abs_x);
    
fr fr Test max function
    sus max_val: i32 = mathz.max(x, y);
    vibez.spill("max({}, {}) = {}", x, y, max_val);
    
fr fr Test min function
    sus min_val: i32 = mathz.min(x, y);
    vibez.spill("min({}, {}) = {}", x, y, min_val);
    
fr fr Test power function (if available)
    sus base: i32 = 2;
    sus exponent: i32 = 3;
    sus power_result: i32 = mathz.pow(base, exponent);
    vibez.spill("{}^{} = {}", base, exponent, power_result);
    
fr fr Test square root (if available)
    sus sqrt_val: f32 = mathz.sqrt(16.0);
    vibez.spill("sqrt(16.0) = {}", sqrt_val);
    
fr fr Test multiple operations in sequence
    sus a: i32 = 5;
    sus b: i32 = -3;
    sus c: i32 = mathz.max(mathz.abs(a), mathz.abs(b));
    vibez.spill("max(abs({}), abs({})) = {}", a, b, c);
    
fr fr Test with expressions
    sus expr_result: i32 = mathz.abs(a - b * 2);
    vibez.spill("abs({} - {} * 2) = {}", a, b, expr_result);
    
    return 0;
}
