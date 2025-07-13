# Simple test without testz framework to check basic functionality
sus test_passed lit = based
sus test_count normie = 0

# Test 1: Basic integer equality
sus result1 normie = 42
if result1 == 42 {
    vibez.spill("✅ Test 1 passed: integer equality")
    test_count = test_count + 1
} else {
    vibez.spill("❌ Test 1 failed: integer equality")
    test_passed = cap
}

# Test 2: String operations
sus result2 tea = "hello"
if result2 == "hello" {
    vibez.spill("✅ Test 2 passed: string equality")
    test_count = test_count + 1
} else {
    vibez.spill("❌ Test 2 failed: string equality")
    test_passed = cap
}

# Test 3: Boolean logic
if test_passed == based {
    vibez.spill("✅ Test 3 passed: boolean logic")
    test_count = test_count + 1
} else {
    vibez.spill("❌ Test 3 failed: boolean logic")
}

vibez.spill("═══════════════════════════")
vibez.spill("Test Summary")
vibez.spill("Total tests: " + test_count)
if test_passed == based {
    vibez.spill("🎉 ALL TESTS PASSED!")
} else {
    vibez.spill("⚠️ SOME TESTS FAILED")
}
vibez.spill("═══════════════════════════")
