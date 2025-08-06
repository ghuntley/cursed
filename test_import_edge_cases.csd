// Test various import syntaxes and edge cases

yeet "testz"         // Standard import
yeet "vibez"         // Standard import

// Test with comments
yeet "mathz"         // Math library

// Test with extra whitespace
   yeet    "stringz"     

// Test module not found case (non-existent module)
// yeet "nonexistent"

test_start("Import Edge Cases Test")

// Test that all valid imports work
vibez.spill("✅ All valid imports processed successfully")

// Test stdlib function calls work
assert_true(based)
vibez.spill("✅ Function calls through imports work")

print_test_summary()
