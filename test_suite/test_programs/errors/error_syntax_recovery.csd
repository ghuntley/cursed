vibe main

yeet "vibez"

fr fr Test: Syntax error recovery
fr fr Purpose: Test parser error handling with various syntax errors
fr fr Expected: Should provide clear error messages for syntax issues

damn main() {
fr fr Missing semicolon (if required)
    sus x: i32 = 10
    
fr fr Unmatched parentheses
    sus y: i32 = (5 + 3;
    
fr fr Missing variable type
    sus z = 42;
    
fr fr Invalid operator usage
    sus result: i32 = x +* y;
    
fr fr Unclosed string literal
    vibez.spill("This string is not closed properly;
    
fr fr Invalid function call
    non_existent_function();
    
fr fr Missing return type in function that should return something
    sus value: i32 = broken_function();
    
    return 0;
}

fr fr Function with missing return statement
damn broken_function() -> i32 {
    sus temp: i32 = 5;
fr fr Missing return statement
}
