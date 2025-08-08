// CURSED Error Handling LLVM Compilation Test
// Tests enhanced try/catch exception handling with LLVM backend

// Basic yikes (error creation) test
slay test_yikes() drip {
    sus error_obj = yikes("Basic error message", 42)
    vibez.spill("Created error:", error_obj)
    damn 0
}

// Basic shook (error propagation) test
slay risky_operation() (drip, yikes) {
    ready (based) {
        yikes("Operation failed", 500)
    }
    damn 42, cringe
}

slay test_shook() drip {
    sus result, err = risky_operation()
    ready (err != cringe) {
        vibez.spill("Caught error:", err)
        damn -1
    }
    vibez.spill("Success:", result)
    damn 0
}

// Advanced shook with immediate catch
slay test_shook_catch() drip {
    sus result = risky_operation() shook {
        vibez.spill("Immediate catch handled error")
        damn 999  // Default value on error
    }
    vibez.spill("Result:", result)
    damn 0
}

// Basic fam (try/catch/finally) test
slay test_fam_basic() drip {
    fam {
        vibez.spill("Executing try block")
        sus value drip = 10 / 2
        vibez.spill("Try block result:", value)
    } fam err {
        vibez.spill("Caught error in fam block:", err)
    }
    damn 0
}

// Advanced fam with finally block
slay test_fam_finally() drip {
    sus resource drip = 100
    
    fam {
        vibez.spill("Acquiring resource:", resource)
        ready (resource > 50) {
            yikes("Resource value too high", 600)
        }
        vibez.spill("Using resource successfully")
    } fam err {
        vibez.spill("Error occurred:", err)
        resource = 0  // Reset on error
    } finally {
        vibez.spill("Cleaning up resource:", resource)
        resource = -1  // Always cleanup
    }
    
    damn resource
}

// Nested error handling test
slay test_nested_error_handling() drip {
    fam {
        vibez.spill("Outer try block")
        
        fam {
            vibez.spill("Inner try block")
            sus inner_result = risky_operation() shook {
                vibez.spill("Inner shook catch")
                yikes("Inner operation failed", 700)
            }
            vibez.spill("Inner success:", inner_result)
        } fam inner_err {
            vibez.spill("Inner catch:", inner_err)
            yikes("Re-throwing from inner", 800)
        }
        
        vibez.spill("Outer try completed")
    } fam outer_err {
        vibez.spill("Outer catch:", outer_err)
    }
    
    damn 0
}

// Error handling with function calls
slay divide_safe(a drip, b drip) (drip, yikes) {
    ready (b == 0) {
        yikes("Division by zero", 100)
    }
    damn a / b, cringe
}

slay test_function_error_handling() drip {
    sus result1 = divide_safe(10, 2) shook {
        vibez.spill("Division failed")
        damn -1
    }
    vibez.spill("Division result 1:", result1)
    
    sus result2 = divide_safe(10, 0) shook {
        vibez.spill("Division by zero caught")
        damn 0
    }
    vibez.spill("Division result 2:", result2)
    
    damn result1 + result2
}

// Main test function
slay main() drip {
    vibez.spill("=== CURSED Error Handling LLVM Tests ===")
    
    vibez.spill("Testing yikes (error creation)...")
    test_yikes()
    
    vibez.spill("Testing shook (error propagation)...")
    test_shook()
    
    vibez.spill("Testing shook with immediate catch...")
    test_shook_catch()
    
    vibez.spill("Testing fam (basic try/catch)...")
    test_fam_basic()
    
    vibez.spill("Testing fam with finally...")
    test_fam_finally()
    
    vibez.spill("Testing nested error handling...")
    test_nested_error_handling()
    
    vibez.spill("Testing function error handling...")
    test_function_error_handling()
    
    vibez.spill("=== All error handling tests completed ===")
    damn 0
}
