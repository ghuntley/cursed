// Test function value execution system
yeet "testz"

// Test functions with different arities
slay test_function_zero_args() normie {
    vibez.spill("Function with zero arguments")
    damn 42
}

slay test_function_one_arg(x normie) normie {
    vibez.spill("Function with one argument:")
    vibez.spill(x)
    damn x * 2
}

slay test_function_two_args(x normie, y normie) normie {
    vibez.spill("Function with two arguments:")
    vibez.spill(x)
    vibez.spill(y)
    damn x + y
}

slay test_function_three_args(x normie, y normie, z normie) normie {
    vibez.spill("Function with three arguments:")
    vibez.spill(x)
    vibez.spill(y)
    vibez.spill(z)
    damn x + y + z
}

// Test function value system
slay test_function_values() {
    vibez.spill("Testing function value execution system")
    
    // These would test the real function value system
    // In a real implementation, these would call execute_function_value
    
    vibez.spill("Testing zero-argument function")
    sus result0 normie = test_function_zero_args()
    vibez.spill("Result:")
    vibez.spill(result0)
    
    vibez.spill("Testing one-argument function")
    sus result1 normie = test_function_one_arg(10)
    vibez.spill("Result:")
    vibez.spill(result1)
    
    vibez.spill("Testing two-argument function")
    sus result2 normie = test_function_two_args(5, 7)
    vibez.spill("Result:")
    vibez.spill(result2)
    
    vibez.spill("Testing three-argument function")
    sus result3 normie = test_function_three_args(1, 2, 3)
    vibez.spill("Result:")
    vibez.spill(result3)
    
    vibez.spill("Function value tests completed")
}

// Test function pointers and executable functions
slay test_executable_functions() {
    vibez.spill("Testing executable function system")
    
    // In a real implementation, this would:
    // 1. Register functions in the executable function registry
    // 2. Get function pointers for native LLVM functions
    // 3. Execute functions through the function value system
    // 4. Test both native and interpreted function execution
    
    vibez.spill("Note: This test demonstrates the concept")
    vibez.spill("Real implementation would integrate with LLVM JIT")
    vibez.spill("and provide actual function pointer execution")
    
    vibez.spill("Executable function tests completed")
}

// Test higher-order functions
slay apply_function(f_name tea, arg normie) normie {
    vibez.spill("Applying function:")
    vibez.spill(f_name)
    vibez.spill("with argument:")
    vibez.spill(arg)
    
    // In a real implementation, this would:
    // 1. Look up the function by name in the registry
    // 2. Execute it with the provided argument
    // For now, just return a placeholder
    damn arg * 2
}

slay test_higher_order_functions() {
    vibez.spill("Testing higher-order function system")
    
    sus result normie = apply_function("test_function_one_arg", 15)
    vibez.spill("Higher-order function result:")
    vibez.spill(result)
    
    vibez.spill("Higher-order function tests completed")
}

slay main() {
    vibez.spill("Testing function value execution system")
    
    test_function_values()
    test_executable_functions()
    test_higher_order_functions()
    
    vibez.spill("All function value tests completed")
}

main()
