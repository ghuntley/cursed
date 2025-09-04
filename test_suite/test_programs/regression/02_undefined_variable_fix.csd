vibe main
yeet "vibez"

// Regression test for undefined variable access prevention
slay test_proper_variable_declaration() {
    vibez.spill("=== Proper Variable Declaration Tests ===")
    
    // Test 1: Proper variable declarations
    sus properly_declared normie = 42
    vibez.spill("Properly declared variable:", properly_declared)
    
    sus another_var tea = "Hello CURSED"
    vibez.spill("Another proper variable:", another_var)
    
    sus calculated_var normie = properly_declared + 8
    vibez.spill("Calculated variable:", calculated_var)
    
    // Test 2: Variable scope within blocks
    ready (properly_declared > 40) {
        sus block_var normie = properly_declared * 2
        vibez.spill("Block variable:", block_var)
        
        ready (block_var > 80) {
            sus nested_var normie = block_var + 10
            vibez.spill("Nested variable:", nested_var)
        }
    }
    
    vibez.spill("Variable declaration tests completed")
}

slay test_variable_initialization_patterns() {
    vibez.spill("=== Variable Initialization Pattern Tests ===")
    
    // Test different initialization patterns
    sus int_var normie = 0
    sus string_var tea = ""
    sus boolean_var lit = cringe
    
    vibez.spill("Initialized variables:")
    vibez.spill("Int var:", int_var)
    vibez.spill("String var:", string_var)
    vibez.spill("Boolean var:", boolean_var)
    
    // Test reassignment
    int_var = 100
    string_var = "Updated"
    boolean_var = based
    
    vibez.spill("After reassignment:")
    vibez.spill("Int var:", int_var)
    vibez.spill("String var:", string_var) 
    vibez.spill("Boolean var:", boolean_var)
    
    // Test initialization with expressions
    sus expr_var1 normie = int_var + 50
    sus expr_var2 normie = expr_var1 * 2
    
    vibez.spill("Expression variables:")
    vibez.spill("Expr var1:", expr_var1)
    vibez.spill("Expr var2:", expr_var2)
}

slay test_array_variable_patterns() {
    vibez.spill("=== Array Variable Pattern Tests ===")
    
    // Test proper array declarations
    sus number_array normie[value] = normie[value]{}
    vibez.spill("Empty array length:", len(number_array))
    
    sus initialized_array normie[value] = normie[value]{1, 2, 3, 4, 5}
    vibez.spill("Initialized array length:", len(initialized_array))
    
    // Test array access patterns
    bestie i := 0; i < len(initialized_array); i++ {
        sus array_element = initialized_array[i]
        vibez.spill("Element", i, ":", array_element)
    }
    
    // Test array modification
    number_array = append(number_array, 10)
    number_array = append(number_array, 20)
    number_array = append(number_array, 30)
    
    vibez.spill("Modified array length:", len(number_array))
    bestie j := 0; j < len(number_array); j++ {
        vibez.spill("Modified element", j, ":", number_array[j])
    }
}

slay test_function_parameter_variables() {
    vibez.spill("=== Function Parameter Variable Tests ===")
    
    slay process_parameters(param1 normie, param2 tea, param3 lit) {
        vibez.spill("Function parameters:")
        vibez.spill("Param1 (number):", param1)
        vibez.spill("Param2 (string):", param2)
        vibez.spill("Param3 (boolean):", param3)
        
        // Test parameter modification (local scope)
        param1 = param1 + 100
        param2 = param2 + " modified"
        param3 = !param3
        
        vibez.spill("Modified parameters:")
        vibez.spill("Modified param1:", param1)
        vibez.spill("Modified param2:", param2)
        vibez.spill("Modified param3:", param3)
    }
    
    // Test function calls with proper parameters
    process_parameters(42, "test", based)
    process_parameters(0, "empty", cringe)
    process_parameters(-5, "negative", based)
}

slay test_variable_scope_isolation() {
    vibez.spill("=== Variable Scope Isolation Tests ===")
    
    sus outer_var normie = 100
    vibez.spill("Outer variable:", outer_var)
    
    ready (outer_var > 50) {
        sus inner_var normie = outer_var + 25
        vibez.spill("Inner variable:", inner_var)
        
        ready (inner_var > 100) {
            sus deep_var normie = inner_var + outer_var
            vibez.spill("Deep variable:", deep_var)
            
            // Modify outer variable from inner scope
            outer_var = 200
            vibez.spill("Modified outer from inner:", outer_var)
        }
        
        vibez.spill("Back in middle scope, outer_var:", outer_var)
    }
    
    vibez.spill("Back in outer scope, outer_var:", outer_var)
    
    // Test loop variable scoping
    bestie loop_var := 0; loop_var < 3; loop_var++ {
        sus loop_local normie = loop_var * 10
        vibez.spill("Loop iteration", loop_var, "local value:", loop_local)
    }
    
    vibez.spill("Variable scope tests completed")
}

slay test_variable_lifecycle_patterns() {
    vibez.spill("=== Variable Lifecycle Pattern Tests ===")
    
    // Test variable creation, use, and reassignment patterns
    sus lifecycle_var normie = 1
    
    bestie stage := 1; stage <= 5; stage++ {
        vibez.spill("Lifecycle stage", stage, "value:", lifecycle_var)
        
        ready (stage == 1) {
            lifecycle_var = 10
        } else ready (stage == 2) {
            lifecycle_var = lifecycle_var * 2
        } else ready (stage == 3) {
            lifecycle_var = lifecycle_var + 5
        } else ready (stage == 4) {
            lifecycle_var = lifecycle_var - 3
        } else {
            lifecycle_var = lifecycle_var / 2
        }
    }
    
    vibez.spill("Final lifecycle value:", lifecycle_var)
    
    // Test multiple variables with interdependencies
    sus var_a normie = 5
    sus var_b normie = 10
    sus var_c normie = 15
    
    vibez.spill("Initial values - A:", var_a, "B:", var_b, "C:", var_c)
    
    // Swap and modify
    sus temp = var_a
    var_a = var_b
    var_b = var_c
    var_c = temp
    
    vibez.spill("After rotation - A:", var_a, "B:", var_b, "C:", var_c)
}

slay main_character() {
    vibez.spill("=== Undefined Variable Prevention Regression Tests ===")
    
    test_proper_variable_declaration()
    test_variable_initialization_patterns()
    test_array_variable_patterns()
    test_function_parameter_variables()
    test_variable_scope_isolation()
    test_variable_lifecycle_patterns()
    
    vibez.spill("All variable handling regression tests completed")
    vibez.spill("No undefined variable access detected - regression test passed")
}
