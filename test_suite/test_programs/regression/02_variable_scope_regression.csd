vibe main
yeet "vibez"

fr fr Variable Scope Regression Test
fr fr Tests: Ensures proper variable scoping doesn't regress
fr fr Expected: Variables are properly scoped and accessible

slay main() {
    vibez.spill("=== Variable Scope Regression Test ===")
    
    vibez.spill("Testing outer scope variables...")
    sus outer_var = 100
    vibez.spill("Outer variable:", outer_var)
    
    vibez.spill("Testing inner scope...")
    ready (outer_var > 50) {
        sus inner_var = outer_var + 25
        vibez.spill("Inner variable:", inner_var)
        
        outer_var = 200
        vibez.spill("Modified outer from inner:", outer_var)
    }
    
    vibez.spill("Back in outer scope:", outer_var)
    
    vibez.spill("Testing loop scope...")
    bestie i := 0; i < 3; i++ {
        sus loop_var = i * 10
        vibez.spill("Loop iteration", i, "var:", loop_var)
    }
    
    vibez.spill("Testing function scope...")
    sus param_test = 42
    
    slay test_function(param normie) {
        vibez.spill("Function parameter:", param)
        param = param + 100
        vibez.spill("Modified parameter:", param)
    }
    
    test_function(param_test)
    vibez.spill("Original parameter unchanged:", param_test)
    
    vibez.spill("Variable scope regression test passed")
}
