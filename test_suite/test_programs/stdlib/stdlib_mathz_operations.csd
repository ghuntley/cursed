vibe main_character

yeet "vibez"
yeet "mathz"

fr fr Test: mathz module mathematical operations
fr fr Purpose: Validate mathz functions work correctly
fr fr Expected: All mathematical operations should produce correct results

slay main_character() {
fr fr Test basic mathz functions
    sus x: normie = -42;
    sus y: normie = 15;
    sus z: meal = -7.5;
    
fr fr Test absolute value
    sus abs_x: normie = mathz.abs_normie(x);
    vibez.spill("abs({}) = {}", x, abs_x);
    
fr fr Test max function
    sus max_val: normie = mathz.max(x, y);
    vibez.spill("max({}, {}) = {}", x, y, max_val);
    
fr fr Test min function
    sus min_val: normie = mathz.min(x, y);
    vibez.spill("min({}, {}) = {}", x, y, min_val);
    
fr fr Test power function (if available)
    sus base: normie = 2;
    sus exponent: normie = 3;
    sus power_result: normie = mathz.pow(base, exponent);
    vibez.spill("{}^{} = {}", base, exponent, power_result);
    
fr fr Test square root (if available)
    sus sqrt_val: meal = mathz.sqrt(16.0);
    vibez.spill("sqrt(16.0) = {}", sqrt_val);
    
fr fr Test multiple operations in sequence
    sus a: normie = 5;
    sus b: normie = -3;
    sus c: normie = mathz.max(mathz.abs_normie(a), mathz.abs_normie(b));
    vibez.spill("max(abs({}), abs({})) = {}", a, b, c);
    
fr fr Test with expressions
    sus expr_result: normie = mathz.abs_normie(a - b * 2);
    vibez.spill("abs({} - {} * 2) = {}", a, b, expr_result);
    
    damn 0;
}
