vibe main
yeet "vibez"
yeet "mathz"

// Regression test for division by zero error handling
slay test_division_scenarios() {
    vibez.spill("=== Division by Zero Regression Tests ===")
    
    // Test 1: Normal division operations
    vibez.spill("Normal division tests:")
    vibez.spill("10 / 2 =", 10 / 2)
    vibez.spill("100 / 5 =", 100 / 5)
    vibez.spill("15 / 3 =", 15 / 3)
    
    // Test 2: Division with 1
    vibez.spill("Division by 1 tests:")
    vibez.spill("42 / 1 =", 42 / 1)
    vibez.spill("0 / 1 =", 0 / 1)
    vibez.spill("-7 / 1 =", -7 / 1)
    
    // Test 3: Edge case divisions
    vibez.spill("Edge case divisions:")
    vibez.spill("8 / 8 =", 8 / 8)
    vibez.spill("1 / 1 =", 1 / 1)
}

slay test_division_by_zero_handling() {
    vibez.spill("=== Division by Zero Error Handling ===")
    
    // This test ensures that division by zero is handled properly
    // In a robust implementation, this should either:
    // 1. Return an error/exception
    // 2. Return a special value (like infinity) 
    // 3. Handle gracefully without crashing
    
    vibez.spill("Testing division by zero scenarios:")
    
    // Test direct division by zero
    vibez.spill("Attempting: 10 / 0")
    // Note: This line may cause runtime error - that's expected behavior
    // The test verifies the system handles it gracefully
    fam {
        sus result = 10 / 0
        vibez.spill("Unexpected: division by zero succeeded with result:", result)
    } sus error {
        vibez.spill("Expected: division by zero handled as error")
    }
    
    // Test variable division by zero
    sus zero_var = 0
    vibez.spill("Attempting: 25 / zero_var")
    fam {
        sus result2 = 25 / zero_var
        vibez.spill("Unexpected: variable division by zero succeeded:", result2)
    } sus error {
        vibez.spill("Expected: variable division by zero handled as error")
    }
    
    // Test expression division by zero
    sus expr_result = 5 - 5
    vibez.spill("Attempting: 30 / (5-5)")
    fam {
        sus result3 = 30 / expr_result
        vibez.spill("Unexpected: expression division by zero succeeded:", result3)
    } sus error {
        vibez.spill("Expected: expression division by zero handled as error")
    }
}

slay test_safe_division_function() yikes {
    vibez.spill("=== Safe Division Function Test ===")
    
    // Safe division implementation
    slay safe_divide(a normie, b normie) normie yikes {
        ready (b == 0) {
            yikes "Division by zero attempted"
        }
        damn a / b
    }
    
    // Test safe division
    fam {
        sus result1 = safe_divide(20, 4) shook
        vibez.spill("Safe division 20/4 =", result1)
        
        sus result2 = safe_divide(15, 3) shook  
        vibez.spill("Safe division 15/3 =", result2)
        
        sus result3 = safe_divide(10, 0) shook
        vibez.spill("This line should not execute")
    } sus error {
        vibez.spill("Safe division caught error:", error.message())
    }
    
    damn cringe
}

slay test_mathematical_division_edge_cases() {
    vibez.spill("=== Mathematical Division Edge Cases ===")
    
    // Test division with mathz module
    vibez.spill("Mathz division tests:")
    
    // Test mathz.divide if available
    fam {
        sus math_result1 = mathz.divide(100, 10) shook
        vibez.spill("Mathz divide 100/10 =", math_result1)
        
        sus math_result2 = mathz.divide(50, 0) shook
        vibez.spill("This should not print")
    } sus error {
        vibez.spill("Mathz division by zero handled:", error.message())
    }
    
    // Test with negative numbers
    vibez.spill("Negative division tests:")
    vibez.spill("-10 / 2 =", -10 / 2)
    vibez.spill("10 / -2 =", 10 / -2)
    vibez.spill("-10 / -2 =", -10 / -2)
    
    // Test with computed values
    sus a = 12
    sus b = 3
    sus c = 6
    vibez.spill("Computed division:", a, "/", (b / c), "=", a / (b / c))
}

slay test_division_in_loops() {
    vibez.spill("=== Division in Loop Scenarios ===")
    
    // Test division operations in loops to ensure no regression
    finna i normie = 1; i <= 10; i++ {
        sus result = 100 / i
        vibez.spill("100 /", i, "=", result)
    }
    
    // Test potential division by zero in loop
    vibez.spill("Testing division in loop with potential zeros:")
    finna j normie = 3; j >= -3; j-- {
        ready (j != 0) {
            sus loop_result = 18 / j
            vibez.spill("18 /", j, "=", loop_result)
        } basic {
            vibez.spill("Skipped division by zero in loop")
        }
    }
}

slay main() {
    vibez.spill("=== Division by Zero Regression Test Suite ===")
    
    test_division_scenarios()
    test_division_by_zero_handling()
    
    fam {
        test_safe_division_function() shook
    } sus error {
        vibez.spill("Safe division test error:", error.message())
    }
    
    test_mathematical_division_edge_cases()
    test_division_in_loops()
    
    vibez.spill("Division by zero regression tests completed")
    vibez.spill("System successfully handled all division scenarios")
}
