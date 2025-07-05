fr fr This is a test file to demonstrate the new error reporting system

vibe test_errors

slay broken_function(x normie) {
    fr fr Missing closing quote in string
    sus message tea = "This string is missing a quote
    
    fr fr Using undefined variable
    sus result = undefined_variable + 5
    
    fr fr Type mismatch
    sus number normie = "this is a string"
    
    fr fr Function not found
    sus value = nonexistent_function()
    
    fr fr Invalid escape sequence
    sus bad_escape = "Invalid \q escape"
    
    fr fr Missing semicolon and other syntax errors
    lowkey x > 5 {
        vibez.spill("test"
    } fr fr missing closing parenthesis
    
    fr fr Unterminated block comment
    no cap
    This comment is never closed
    
    yolo result
}

slay main() {
    broken_function(42)
}
