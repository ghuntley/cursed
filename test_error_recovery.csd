# Test file for error recovery system
# This file contains intentional syntax and semantic errors to test recovery

# Syntax error: missing semicolon
sus x normie = 42

# Syntax error: unmatched parenthesis  
vibez.spill("hello world"

# Type error: undefined variable
vibez.spill(undefined_var)

# Syntax error: malformed function
slay broken_function( {
    damn 42
}

# Good code that should still work
sus y normie = 10
vibez.spill("This should work")

# Type error: type mismatch
sus z tea = 42

# Syntax error: missing brace
lowkey y > 5 {
    vibez.spill("condition true")
# Missing closing brace

# More good code
sus result normie = y + 5
vibez.spill(result)
