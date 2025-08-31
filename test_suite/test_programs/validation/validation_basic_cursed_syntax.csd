vibe main

yeet "vibez"

fr fr Test: Basic CURSED language syntax validation
fr fr Purpose: Ensure fundamental language constructs parse correctly
fr fr Expected: All basic syntax elements should be recognized properly

damn main() {
fr fr Variable declarations with different types
    sus integer_var: i32 = 100;
    sus float_var: f32 = 3.14;
    sus string_var: string = "CURSED is fire";
    sus bool_var: bool = true;
    
    vibez.spill("Integer: {}", integer_var);
    vibez.spill("Float: {}", float_var);
    vibez.spill("String: {}", string_var);
    vibez.spill("Boolean: {}", bool_var);
    
fr fr Basic arithmetic
    sus sum: i32 = integer_var + 50;
    sus product: i32 = integer_var * 2;
    sus quotient: f32 = float_var / 2.0;
    
    vibez.spill("Sum: {}", sum);
    vibez.spill("Product: {}", product);
    vibez.spill("Quotient: {}", quotient);
    
fr fr Conditional statements
    if integer_var > 50 {
        vibez.spill("Integer is greater than 50");
    } else {
        vibez.spill("Integer is 50 or less");
    }
    
fr fr Boolean logic
    if bool_var && integer_var > 0 {
        vibez.spill("Both conditions are true");
    }
    
    if bool_var || integer_var < 0 {
        vibez.spill("At least one condition is true");
    }
    
fr fr String concatenation or comparison
    if string_var == "CURSED is fire" {
        vibez.spill("String comparison works");
    }
    
    return 0;
}
