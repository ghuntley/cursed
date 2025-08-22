fr fr Enhanced stack trace test with nested function calls

yeet "vibez"
yeet "errorz"

fr fr Deep nested function chain to test stack trace
slay level_3() {
    vibez.spill("Level 3: Creating error with stack trace")
    sus err = errorz.create_error("Deep nested error test")
    errorz.print_error_with_stack(err)
}

slay level_2() {
    vibez.spill("Level 2: Calling level 3")
    level_3()
}

slay level_1() {
    vibez.spill("Level 1: Calling level 2")
    level_2()
}

fr fr Test the enhanced error creation with context
slay test_detailed_error() {
    vibez.spill("Testing detailed error with stack trace")
    sus detailed_err = errorz.create_detailed_error("Detailed error test", 500, "Additional context info", 2)
    errorz.print_error_with_stack(detailed_err)
}

slay main() {
    vibez.spill("=== Enhanced Stack Trace Test ===")
    vibez.spill("")
    
    vibez.spill("Test 1: Deep nested call stack")
    level_1()
    
    vibez.spill("")
    vibez.spill("Test 2: Detailed error information")
    test_detailed_error()
    
    vibez.spill("")
    vibez.spill("=== Test Complete ===")
}

main()
