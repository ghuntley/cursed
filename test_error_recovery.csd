# Test program with various types of errors for error recovery testing

# Syntax error - missing semicolon
sus x normie = 42

# Type mismatch error
sus y tea = 123

# Undefined variable error
vibez.spill(unknown_var)

# Function call with missing parameter
vibez.spill()

# Unterminated string
sus message tea = "Hello world

# Missing closing brace
slay test_function() {
    vibez.spill("In function")
    # Missing closing brace

# Typo in keyword
su wrong_var normie = 10

# Wrong type annotation
sus another_var wrong_type = "string"

# Valid statement after errors
sus valid_var normie = 100
vibez.spill("This should still work")
