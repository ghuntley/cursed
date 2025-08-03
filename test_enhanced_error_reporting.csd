fr fr Test file for enhanced error reporting demonstration

fr fr This file contains various syntax errors to test the error reporting system

slay main() {
    fr fr Test 1: Unterminated string
    vibez.spill("Hello world;
    
    fr fr Test 2: Missing variable type
    sus x = 42;
    
    fr fr Test 3: Invalid function call
    undefinedFunction();
    
    fr fr Test 4: Type mismatch
    sus name normie = "John";
    
    fr fr Test 5: Unbalanced braces
    lowkey (based) {
        vibez.spill("missing closing brace");
    
    fr fr Test 6: Invalid escape sequence
    sus badString tea = "invalid \z escape";
    
    fr fr Test 7: Return outside function
}
damn 42;

fr fr Test 8: Duplicate function definition
slay main() {
    vibez.spill("duplicate main function");
}

fr fr Test 9: Invalid number format
sus badNumber normie = 123abc;

fr fr Test 10: Missing semicolon and proper structure
slay testFunction(param
    vibez.spill("incomplete parameter list")
}
