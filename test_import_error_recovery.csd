# Test error recovery and edge cases

# Test 1: Valid comma-separated imports (should work)
yeet "mathz", "stringz"

# Test 2: Missing quotes (should trigger error recovery)
yeet mathz, "stringz"

# Test 3: Missing comma (should handle gracefully)
yeet "mathz" "stringz"

# Test 4: Empty import in middle
yeet "mathz", "", "stringz"

# Test 5: Trailing comma (should handle gracefully)
yeet "mathz", "stringz",

# Test 6: Multiple commas
yeet "mathz",, "stringz"

vibez.spill("Testing error recovery")
vibez.spill("Math test:", abs_normie(-5))
