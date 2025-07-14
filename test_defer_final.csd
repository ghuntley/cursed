yeet "testz"

slay test_defer_simple() {
    vibez.spill("Test started")
    later vibez.spill("Defer 1")
    later vibez.spill("Defer 2")
    later vibez.spill("Defer 3")
    vibez.spill("Test ending")
    // Defers will execute in reverse order: 3, 2, 1
}

slay cleanup_resource() {
    vibez.spill("Resource cleaned up")
}

slay test_defer_with_function() {
    vibez.spill("Acquiring resource")
    later cleanup_resource()
    vibez.spill("Using resource")
    vibez.spill("Done with resource")
}

slay test_defer_in_loop() {
    bestie i := 0; i < 2; i++ {
        vibez.spill("Loop iteration")
        later vibez.spill("Loop defer cleanup")
    }
    vibez.spill("Loop complete")
}

vibez.spill("=== Testing defer functionality ===")

vibez.spill("1. Basic defer order test:")
test_defer_simple()

vibez.spill("2. Defer with function calls:")
test_defer_with_function()

vibez.spill("3. Defer in loops:")
test_defer_in_loop()

vibez.spill("=== Defer tests complete ===")
