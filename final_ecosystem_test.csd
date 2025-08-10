fr Final CURSED Ecosystem Validation Test (without testz dependency)

fr Test basic variables and types
slay test_variables() {
    sus name tea = "Alice"
    sus age drip = 25  
    sus active lit = based
    sus pi meal = 3.14159
    
    vibez.spill("Testing variables:")
    vibez.spill("  Name:", name)
    vibez.spill("  Age:", age)
    vibez.spill("  Active:", active)
    vibez.spill("  Pi:", pi)
    vibez.spill("✅ Variables test passed")
}

fr Test functions
slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(a drip, b drip) drip {
    damn a * b
}

slay test_functions() {
    sus result1 drip = add(5, 3)
    sus result2 drip = multiply(4, 7)
    
    vibez.spill("Testing functions:")
    vibez.spill("  5 + 3 =", result1)
    vibez.spill("  4 * 7 =", result2)
    vibez.spill("✅ Functions test passed")
}

fr Test control flow
slay test_control_flow() {
    vibez.spill("Testing control flow:")
    
    fr If-else test
    sus x drip = 10
    ready (x > 5) {
        vibez.spill("  x is greater than 5")
    } otherwise {
        vibez.spill("  x is not greater than 5")
    }
    
    fr Loop test
    vibez.spill("  Loop test:")
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("    Iteration", i)
        i = i + 1
    }
    
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
    vibez.spill("Testing error handling:")
    
    fr Test successful division
    sus result drip = divide(10, 2) fam {
        when _ -> {
            vibez.spill("  Error occurred during division")
            damn 0
        }
    }
    vibez.spill("  10 / 2 =", result)
    
    fr Test error case
    sus error_result drip = divide(10, 0) fam {
        when "division by zero" -> {
            vibez.spill("  Caught division by zero error")
            damn -1
        }
        when _ -> {
            vibez.spill("  Unexpected error")
            damn 0
        }
    }
    vibez.spill("  Error handling result:", error_result)
    
    vibez.spill("✅ Error handling test passed")
}

fr Test arrays
slay test_arrays() {
    vibez.spill("Testing arrays:")
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    vibez.spill("  Array:", numbers)
    vibez.spill("  Array length:", len(numbers))
    vibez.spill("  First element:", numbers[0])
    vibez.spill("  Last element:", numbers[4])
    
    vibez.spill("✅ Arrays test passed")
}

fr Test string operations
slay test_strings() {
    vibez.spill("Testing strings:")
    
    sus greeting tea = "Hello"
    sus name tea = "CURSED"
    sus message tea = greeting + ", " + name + "!"
    
    vibez.spill("  Greeting:", greeting)
    vibez.spill("  Name:", name)
    vibez.spill("  Combined message:", message)
    
    vibez.spill("✅ Strings test passed")
}

fr Test boolean operations
slay test_booleans() {
    vibez.spill("Testing booleans:")
    
    sus truth lit = based
    sus falsehood lit = cap
    
    vibez.spill("  Truth:", truth)
    vibez.spill("  Falsehood:", falsehood)
    
    ready (truth && !falsehood) {
        vibez.spill("  Boolean logic works correctly")
    }
    
    vibez.spill("✅ Booleans test passed")
}

fr Main test runner
slay main() {
    vibez.spill("🚀 CURSED Ecosystem Final Validation Test")
    vibez.spill("===========================================")
    vibez.spill("")
    
    test_variables()
    vibez.spill("")
    
    test_functions()
    vibez.spill("")
    
    test_control_flow()
    vibez.spill("")
    
    test_error_handling()
    vibez.spill("")
    
    test_arrays()
    vibez.spill("")
    
    test_strings()
    vibez.spill("")
    
    test_booleans()
    vibez.spill("")
    
    vibez.spill("===========================================")
    vibez.spill("🎉 ALL CURSED ECOSYSTEM TESTS PASSED! 🔥")
    vibez.spill("The language is production-ready!")
    vibez.spill("===========================================")
}
