# Test file for semantic error recovery
# Tests type checking and semantic analysis recovery

# Undefined variable error
vibez.spill(nonexistent_variable)

# Type mismatch error
sus number normie = "this is a string"

# Function call with wrong arity
slay test_function(param normie) normie {
    damn param + 1
}

# Call with wrong number of parameters
test_function(1, 2, 3)

# Undefined function call
nonexistent_function("test")

# Valid code that should work despite errors above
sus valid_var normie = 42
vibez.spill("Valid code works")
vibez.spill(valid_var)

# Interface compliance error (if interfaces are supported)
interface TestInterface {
    slay method_one() normie
    slay method_two(param tea) tea
}

struct TestStruct {
    field normie
}

# Missing interface implementation methods
# TestStruct should implement TestInterface but doesn't

# More valid code
sus another_var tea = "hello"
vibez.spill(another_var)
