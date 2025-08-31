vibe main

yeet "vibez"
yeet "mathz"

fr fr Test: Complex expression parsing edge cases
fr fr Purpose: Test parser handles deeply nested and complex expressions
fr fr Expected: All complex expressions should parse and evaluate correctly

damn main() {
fr fr Deeply nested arithmetic expressions
    sus complex1: i32 = ((10 + 5) * (8 - 3)) / (2 + 1);
    vibez.spill("Complex arithmetic 1: {}", complex1);
    
fr fr Mixed operations with precedence
    sus complex2: i32 = 2 + 3 * 4 - 1 * (5 + 2) / 7;
    vibez.spill("Complex arithmetic 2: {}", complex2);
    
fr fr Boolean expressions with multiple operators
    sus a: i32 = 10;
    sus b: i32 = 20;
    sus c: i32 = 15;
    
    sus complex_bool: bool = (a < b) && (b > c) || (a + c == b + 5);
    vibez.spill("Complex boolean: {}", complex_bool);
    
fr fr Nested function calls
    sus nested_result: i32 = mathz.abs(mathz.max(-5, mathz.min(10, -3)));
    vibez.spill("Nested function calls: {}", nested_result);
    
fr fr Complex conditional with nested expressions
    if ((a * 2) > (b - 5)) && (mathz.abs(c - 10) < 6) {
        vibez.spill("Complex condition passed");
    } else {
        vibez.spill("Complex condition failed");
    }
    
fr fr Expression with multiple mathematical operations
    sus x: f32 = 2.5;
    sus y: f32 = 1.5;
    sus complex_float: f32 = (x + y) * (x - y) + (x * y) / (x + 1.0);
    vibez.spill("Complex float expression: {}", complex_float);
    
fr fr Chain of comparisons
    sus chain_result: bool = a < b && b < (c + 10) && (c + 10) < 30;
    vibez.spill("Chained comparisons: {}", chain_result);
    
    return 0;
}
