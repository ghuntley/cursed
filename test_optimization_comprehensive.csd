// Comprehensive optimization test program
// This tests various optimization scenarios

vibez.spill("Testing CURSED optimization passes")

// Test constant folding
sus constant_test normie = 5 + 3 * 2 - 1
vibez.spill("Constant result: " + constant_test)

// Test dead code elimination
sus dead_var normie = 42
sus used_var normie = 100
// dead_var is not used, should be eliminated
vibez.spill("Used variable: " + used_var)

// Test function inlining
slay simple_function(x normie) normie {
    damn x + 1
}

sus inline_test normie = simple_function(5)
vibez.spill("Inlined function result: " + inline_test)

// Test loop optimizations
sus loop_result normie = 0
bestie i := 0; i < 10; i++ {
    loop_result = loop_result + i
}
vibez.spill("Loop result: " + loop_result)

// Test jump threading
sus conditional_test normie = 5
bestie conditional_test > 0 {
    vibez.spill("Positive number")
} else {
    vibez.spill("Not positive")
}

// Test memory optimization
sus array_test [5]normie = [1, 2, 3, 4, 5]
vibez.spill("Array element: " + array_test[2])

vibez.spill("Optimization test completed successfully!")
