# This file contains intentional syntax errors for testing error reporting

# Missing semicolon
sus x drip = 42
sus y drip = 24

# Undefined variable
vibez.spill("Value: " + str(undefined_var))

# Type mismatch
sus number drip = "not a number"

# Invalid function call
sus result drip = non_existent_function(42)

# Malformed struct
squad BadStruct {
    spill x drip
    spill y # Missing type
}
