# Test file for parser error recovery - lexically valid but syntactically problematic

# Valid function
slay good_function() drip {
    damn 42
}

# Missing parameter type (syntax error)
slay bad_function(param) drip {
    vibez.spill("This should be parsed after error recovery")
}

# Another valid statement after error
sus valid_var drip = 100;

# Malformed expression (missing operand)
sus broken_expr drip = 1 +;

# This should still be parsed due to semicolon recovery
vibez.spill("Recovery test successful");

# Missing function body
slay incomplete_function() drip 

# Another statement that should be recovered
sus another_var drip = 42;

# Final valid statement
vibez.spill("End of test");
