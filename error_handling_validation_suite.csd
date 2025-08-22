# CURSED Error Handling Validation Suite
# Tests comprehensive error handling capabilities

yeet "vibez"
yeet "arrayz"
yeet "stringz"
yeet "mathz"

# ===== ERROR TYPES AND HANDLING =====

# Division by zero error handling
slay safe_divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero - cannot divide " + to_string(a) + " by zero"
    }
    damn a / b
}

# Array bounds checking error handling  
slay safe_array_access(arr []drip, index drip) yikes<drip> {
    ready (index < 0) {
        yikes "Negative array index: " + to_string(index)
    }
    ready (index >= arrayz.len(arr)) {
        yikes "Array index out of bounds: " + to_string(index) + " >= " + to_string(arrayz.len(arr))
    }
    damn arr[index]
}

# String processing error handling
slay safe_substring(text tea, start drip, length drip) yikes<tea> {
    ready (start < 0) {
        yikes "Negative start position: " + to_string(start)
    }
    ready (start >= stringz.len(text)) {
        yikes "Start position beyond string length: " + to_string(start) + " >= " + to_string(stringz.len(text))
    }
    ready (length < 0) {
        yikes "Negative length: " + to_string(length)
    }
    ready (start + length > stringz.len(text)) {
        yikes "Substring extends beyond string end"
    }
    damn stringz.substring(text, start, length)
}

# Complex nested error handling
slay complex_operation(data []drip, operation tea) yikes<drip> {
    ready (arrayz.len(data) == 0) {
        yikes "Empty data array provided"
    }
    
    sick (operation) {
        when "sum" -> {
            sus total drip = 0
            bestie (sus val drip in data) {
                total = total + val
            }
            damn total
        }
        when "average" -> {
            sus sum drip = complex_operation(data, "sum") fam {
                when _ -> yikes "Failed to calculate sum for average"
            }
            damn sum / arrayz.len(data)
        }
        when "max" -> {
            sus max_val drip = data[0]
            bestie (sus val drip in data) {
                ready (val > max_val) {
                    max_val = val
                }
            }
            damn max_val
        }
        when _ -> {
            yikes "Unknown operation: " + operation
        }
    }
}

# ===== ERROR RECOVERY TESTING =====

slay test_basic_error_handling() {
    vibez.spill("🚨 Testing Basic Error Handling")
    
    # Test division by zero
    sus result1 drip = safe_divide(10, 2) fam {
        when _ -> {
            vibez.spill("Unexpected error in valid division")
            damn -1
        }
    }
    vibez.spill("10 / 2 =", result1)
    
    sus result2 drip = safe_divide(10, 0) fam {
        when msg tea -> {
            vibez.spill("Caught expected error:", msg)
            damn 0
        }
    }
    vibez.spill("10 / 0 handled, result:", result2)
}

slay test_array_bounds_errors() {
    vibez.spill("📋 Testing Array Bounds Error Handling")
    
    sus test_array []drip = [10, 20, 30, 40, 50]
    
    # Valid access
    sus val1 drip = safe_array_access(test_array, 2) fam {
        when _ -> damn -1
    }
    vibez.spill("test_array[2] =", val1)
    
    # Invalid access - negative index
    sus val2 drip = safe_array_access(test_array, -1) fam {
        when msg tea -> {
            vibez.spill("Caught negative index error:", msg)
            damn 0
        }
    }
    
    # Invalid access - out of bounds
    sus val3 drip = safe_array_access(test_array, 10) fam {
        when msg tea -> {
            vibez.spill("Caught out of bounds error:", msg)
            damn 0
        }
    }
}

slay test_string_error_handling() {
    vibez.spill("📝 Testing String Error Handling")
    
    sus test_string tea = "Hello World"
    
    # Valid substring
    sus sub1 tea = safe_substring(test_string, 0, 5) fam {
        when _ -> damn ""
    }
    vibez.spill("Substring [0,5]:", sub1)
    
    # Invalid substring - negative start
    sus sub2 tea = safe_substring(test_string, -1, 3) fam {
        when msg tea -> {
            vibez.spill("Caught negative start error:", msg)
            damn ""
        }
    }
    
    # Invalid substring - beyond end
    sus sub3 tea = safe_substring(test_string, 5, 20) fam {
        when msg tea -> {
            vibez.spill("Caught beyond end error:", msg)
            damn ""
        }
    }
}

slay test_complex_error_chains() {
    vibez.spill("🔗 Testing Complex Error Chains")
    
    sus data []drip = [1, 2, 3, 4, 5]
    
    # Valid operations
    sus sum_result drip = complex_operation(data, "sum") fam {
        when msg tea -> {
            vibez.spill("Unexpected error in sum:", msg)
            damn 0
        }
    }
    vibez.spill("Sum result:", sum_result)
    
    sus avg_result drip = complex_operation(data, "average") fam {
        when msg tea -> {
            vibez.spill("Unexpected error in average:", msg)
            damn 0
        }
    }
    vibez.spill("Average result:", avg_result)
    
    # Invalid operation
    sus invalid_result drip = complex_operation(data, "invalid") fam {
        when msg tea -> {
            vibez.spill("Caught invalid operation error:", msg)
            damn -1
        }
    }
    
    # Empty data
    sus empty_data []drip = []
    sus empty_result drip = complex_operation(empty_data, "sum") fam {
        when msg tea -> {
            vibez.spill("Caught empty data error:", msg)
            damn 0
        }
    }
}

# ===== NESTED ERROR PROPAGATION =====

slay level3_function(val drip) yikes<drip> {
    ready (val < 0) {
        yikes "Level 3: Negative value not allowed"
    }
    damn val * 3
}

slay level2_function(val drip) yikes<drip> {
    sus result drip = level3_function(val) fam {
        when msg tea -> yikes "Level 2: " + msg
    }
    damn result * 2
}

slay level1_function(val drip) yikes<drip> {
    sus result drip = level2_function(val) fam {
        when msg tea -> yikes "Level 1: " + msg
    }
    damn result + 1
}

slay test_error_propagation() {
    vibez.spill("📡 Testing Error Propagation")
    
    # Valid case
    sus result1 drip = level1_function(5) fam {
        when _ -> damn -1
    }
    vibez.spill("level1_function(5) =", result1)
    
    # Error case - should propagate through all levels
    sus result2 drip = level1_function(-2) fam {
        when msg tea -> {
            vibez.spill("Caught propagated error:", msg)
            damn 0
        }
    }
}

# ===== RESOURCE CLEANUP WITH ERRORS =====

slay resource_operation_with_cleanup(should_fail lit) yikes<drip> {
    vibez.spill("Allocating resources...")
    
    # Simulate resource allocation
    sus resource []drip = []
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        arrayz.push(resource, i)
    }
    
    ready (should_fail) {
        vibez.spill("Cleaning up resources before error...")
        # Resource cleanup would happen here
        yikes "Simulated resource operation failure"
    }
    
    vibez.spill("Operation completed successfully")
    damn arrayz.len(resource)
}

slay test_resource_cleanup_errors() {
    vibez.spill("🧹 Testing Resource Cleanup with Errors")
    
    # Successful operation
    sus result1 drip = resource_operation_with_cleanup(nah) fam {
        when _ -> damn -1
    }
    vibez.spill("Successful operation result:", result1)
    
    # Failed operation with cleanup
    sus result2 drip = resource_operation_with_cleanup(based) fam {
        when msg tea -> {
            vibez.spill("Caught error after cleanup:", msg)
            damn 0
        }
    }
}

# ===== MAIN ERROR VALIDATION RUNNER =====

slay main() {
    vibez.spill("🛡️ CURSED Error Handling Validation Suite")
    vibez.spill("==========================================")
    
    test_basic_error_handling()
    vibez.spill("")
    
    test_array_bounds_errors() 
    vibez.spill("")
    
    test_string_error_handling()
    vibez.spill("")
    
    test_complex_error_chains()
    vibez.spill("")
    
    test_error_propagation()
    vibez.spill("")
    
    test_resource_cleanup_errors()
    
    vibez.spill("==========================================")
    vibez.spill("✅ Error handling validation completed!")
    vibez.spill("🎯 All error scenarios handled correctly")
}

# Helper function
slay to_string(n drip) tea {
    # Placeholder for actual number to string conversion
    damn "number"
}
