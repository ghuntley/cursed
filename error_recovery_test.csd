# Test file for error recovery - contains intentional syntax errors

# Valid function
slay good_function() drip {
    damn 42
}

# Syntax error: missing parameter type
slay bad_function(param) {
    vibez.spill("This should be parsed after error recovery")
}

# Another valid statement after error
sus valid_var drip = 100;

# Syntax error: malformed expression
sus broken_expr drip = 1 + + 2;

# This should still be parsed
vibez.spill("Recovery test successful");

# Syntax error: unclosed string
sus broken_string tea = "unclosed string

# Another statement that should be recovered
sus another_var drip = 42;

# Syntax error: invalid function syntax  
slay function(
    # Missing closing paren and body

# Final valid statement
vibez.spill("End of test");
