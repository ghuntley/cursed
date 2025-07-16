# Comprehensive Error Recovery Test for CURSED Compiler
# This file tests the error recovery system with various types of errors

# Test 1: Syntax Errors with Recovery
vibez.spill("=== Testing Syntax Error Recovery ===")

# Missing semicolon
sus x normie = 42

# Unmatched parenthesis  
vibez.spill("hello world"

# Missing closing brace
lowkey x > 0 {
    vibez.spill("x is positive")
# Brace intentionally missing

# This should still parse despite errors above
sus y normie = 10
vibez.spill("Basic variable declaration works")

# Test 2: Type Errors with Recovery
vibez.spill("=== Testing Type Error Recovery ===")

# Type mismatch - string assigned to integer
sus number normie = "this should be a number"

# Undefined variable usage
vibez.spill(some_undefined_variable)

# Function call with wrong parameters
slay test_func(param normie) normie {
    damn param + 1
}

# Call with wrong arity
test_func(1, 2, 3)

# This should still work despite type errors above
sus valid_string tea = "this is valid"
vibez.spill(valid_string)

# Test 3: Complex Expression Errors
vibez.spill("=== Testing Complex Expression Error Recovery ===")

# Malformed function definition
slay broken_func( {  # Missing parameter list closing
    damn 42
}

# Complex nested expression that might break parsing
sus result normie = ((x + y) * (z -  # Incomplete expression

# Array access with potential issues
sus arr [5]normie
arr[undefined_index] = 42

# This should still compile correctly
sus simple normie = 5
vibez.spill(simple)

# Test 4: Control Flow Errors
vibez.spill("=== Testing Control Flow Error Recovery ===")

# Malformed if statement
lowkey  {  # Missing condition
    vibez.spill("no condition")
}

# Malformed loop
bestie ; ; {  # Missing loop components
    vibez.spill("broken loop")
}

# This should work despite errors above
lowkey simple > 0 {
    vibez.spill("Simple condition works")
}

# Test 5: Valid Code Mixed with Errors
vibez.spill("=== Testing Valid Code Execution ===")

# These should all work correctly
sus a normie = 100
sus b normie = 200
sus c normie = a + b

vibez.spill("Mathematical operations:")
vibez.spill(a)
vibez.spill(b) 
vibez.spill(c)

# Function definition that should work
slay working_function(input normie) normie {
    damn input * 2
}

sus doubled normie = working_function(a)
vibez.spill("Function result:")
vibez.spill(doubled)

# Loop that should work
bestie i := 0; i < 3; i++ {
    vibez.spill("Loop iteration:")
    vibez.spill(i)
}

vibez.spill("=== Error Recovery Test Complete ===")
vibez.spill("This message should appear if error recovery works correctly")
