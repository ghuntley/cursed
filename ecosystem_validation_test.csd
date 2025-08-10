fr Comprehensive CURSED Ecosystem Validation Test

yeet "testz"

fr Test basic variables and types
slay test_variables() {
    sus name tea = "Alice"
    sus age drip = 25  
    sus active lit = based
    sus pi meal = 3.14159
    
    testz.assert_eq_string(name, "Alice")
    testz.assert_eq_int(age, 25)
    testz.assert_true(active)
    testz.assert_true(pi > 3.0)
    
    vibez.spill("✅ Variables test passed")
}

fr Test functions
slay add(a drip, b drip) drip {
    damn a + b
}

slay test_functions() {
    sus result drip = add(5, 3)
    testz.assert_eq_int(result, 8)
    
    vibez.spill("✅ Functions test passed")
}

fr Test control flow
slay test_control_flow() {
    sus x drip = 10
    sus result tea = ""
    
    ready (x > 5) {
        result = "greater"
    } otherwise {
        result = "less_or_equal"
    }
    
    testz.assert_eq_string(result, "greater")
    
    fr Test loops
    sus count drip = 0
    sus i drip = 0
    bestie (i < 3) {
        count = count + 1
        i = i + 1
    }
    testz.assert_eq_int(count, 3)
    
    vibez.spill("✅ Control flow test passed")
}

fr Test error handling
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

slay test_error_handling() {
    fr Test successful division
    sus result drip = divide(10, 2) fam {
        when _ -> damn 0
    }
    testz.assert_eq_int(result, 5)
    
    fr Test error case
    sus error_result drip = divide(10, 0) fam {
        when "division by zero" -> damn -1
        when _ -> damn 0
    }
    testz.assert_eq_int(error_result, -1)
    
    vibez.spill("✅ Error handling test passed")
}

fr Test arrays
slay test_arrays() {
    sus numbers []drip = [1, 2, 3, 4, 5]
    testz.assert_eq_int(len(numbers), 5)
    testz.assert_eq_int(numbers[0], 1)
    testz.assert_eq_int(numbers[4], 5)
    
    vibez.spill("✅ Arrays test passed")
}

fr Main test runner
slay main() {
    vibez.spill("🚀 Starting CURSED Ecosystem Validation Tests")
    vibez.spill("")
    
    testz.test_start("CURSED Ecosystem Tests")
    
    test_variables()
    test_functions()
    test_control_flow() 
    test_error_handling()
    test_arrays()
    
    vibez.spill("")
    vibez.spill("🎉 All ecosystem tests completed successfully!")
    testz.print_test_summary()
}
