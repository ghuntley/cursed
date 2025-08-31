vibe main

yeet "vibez"

fr fr Test: vibez module output functionality
fr fr Purpose: Validate vibez.spill works correctly with various data types
fr fr Expected: All output should display properly formatted strings

damn main() {
fr fr Test basic string output
    vibez.spill("Hello, CURSED world!");
    
fr fr Test string formatting with integers
    sus age: i32 = 25;
    vibez.spill("I am {} years old", age);
    
fr fr Test multiple format arguments
    sus name: string = "Alice";
    sus score: i32 = 95;
    vibez.spill("{} scored {} points", name, score);
    
fr fr Test boolean formatting
    sus is_valid: bool = true;
    sus is_complete: bool = false;
    vibez.spill("Valid: {}, Complete: {}", is_valid, is_complete);
    
fr fr Test numeric formatting
    sus pi: f32 = 3.14159;
    vibez.spill("Pi value: {}", pi);
    
fr fr Test without formatting
    vibez.spill("No formatting needed");
    
fr fr Test empty string
    vibez.spill("");
    
fr fr Test complex expression formatting
    sus a: i32 = 10;
    sus b: i32 = 20;
    vibez.spill("Result: {} + {} = {}", a, b, a + b);
    
    return 0;
}
