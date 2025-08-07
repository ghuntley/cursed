fr fr Comprehensive CURSED debug test program
fr fr Testing function definitions, variables, and stack traces

sus global_counter drip = 42

slay debug_test_function(x drip) drip {
    sus local_variable drip = x * 2
    sus another_local tea = "debug test string"
    vibez.spill("Function executing:", local_variable)
    damn local_variable + global_counter
}

slay recursive_function(depth drip) drip {
    ready (depth <= 0) {
        damn 1
    }
    sus result drip = recursive_function(depth - 1)
    damn result * depth
}

slay main() {
    vibez.spill("Starting debug test program")
    
    sus test_var drip = debug_test_function(21)
    vibez.spill("Test result:", test_var)
    
    sus recursive_result drip = recursive_function(5)
    vibez.spill("Recursive result:", recursive_result)
    
    vibez.spill("Debug test complete")
}

main()
