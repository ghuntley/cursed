yeet "vibez"
yeet "testz"

slay test_yikes_error() {
    vibez.spill("Testing YIKES error creation...")
    
    # This should create an error and propagate it
    sus result tea = yikes "This is a test error" fam {
        when "This is a test error" -> {
            vibez.spill("Caught error successfully!")
            damn "Error handled"
        }
        otherwise -> {
            damn "Unexpected error"
        }
    }
    
    vibez.spill("Result:", result)
    damn result
}

slay test_error_propagation() {
    vibez.spill("Testing SHOOK error propagation...")
    
    sus computation sus = slay() {
        ready (2 == 2) {
            yikes "Computation failed with division by zero"
        }
        damn 42
    }
    
    # Use shook to propagate the error
    sus final_result tea = computation() shook fam {
        when "Computation failed with division by zero" -> {
            vibez.spill("Propagated error caught!")
            damn "Recovered from computation error"  
        }
    }
    
    vibez.spill("Final result:", final_result)
    damn final_result
}

slay test_defer_cleanup() {
    vibez.spill("Testing defer cleanup...")
    
    sus resource sus = "allocated_resource"
    
    defer {
        vibez.spill("Cleaning up resource:", resource)
    }
    
    yikes "Error with cleanup" fam {
        when "Error with cleanup" -> {
            vibez.spill("Error handled, cleanup should execute")
        }
    }
    
    damn "cleanup_test_complete"
}

slay test_nested_error_handling() {
    vibez.spill("Testing nested error handling...")
    
    sus outer_result tea = fam {
        try {
            sus inner_result tea = fam {
                try {
                    yikes "Inner error"
                }
                when "Inner error" -> {
                    vibez.spill("Inner error caught")
                    yikes "Outer error from inner catch"
                }
            }
            damn inner_result
        }
        when "Outer error from inner catch" -> {
            vibez.spill("Outer error caught") 
            damn "nested_error_handled"
        }
    }
    
    vibez.spill("Nested result:", outer_result)
    damn outer_result
}

slay test_multiple_error_types() {
    vibez.spill("Testing multiple error types...")
    
    # Test different error types
    sus errors []tea = []
    
    fam {
        try {
            yikes "Memory error" 
        }
        when "Memory error" -> {
            errors.append("memory_error_caught")
        }
    }
    
    fam {
        try {
            yikes "IO error"
        }  
        when "IO error" -> {
            errors.append("io_error_caught")
        }
    }
    
    vibez.spill("Error handling results:", errors)
    damn errors
}

slay main() {
    vibez.spill("=== CURSED Error Handling Runtime Test Suite ===")
    
    sus test1 tea = test_yikes_error()
    vibez.spill("Test 1 (yikes):", test1)
    
    sus test2 tea = test_error_propagation()  
    vibez.spill("Test 2 (shook):", test2)
    
    sus test3 tea = test_defer_cleanup()
    vibez.spill("Test 3 (defer):", test3)
    
    sus test4 tea = test_nested_error_handling()
    vibez.spill("Test 4 (nested):", test4)
    
    sus test5 []tea = test_multiple_error_types()
    vibez.spill("Test 5 (multiple):", test5)
    
    vibez.spill("=== Error Handling Runtime Tests Complete ===")
    damn "all_tests_passed"
}
