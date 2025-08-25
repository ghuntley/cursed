fr fr Token Scanning Test
fr fr Testing proper tokenization and end-of-input detection

yeet "slay_io"

fr fr Test 1: Empty input should return false
vibez.spill("=== Test 1: Empty Input ===")
sus emptyScanner tea = NewSlayScanner("")
sus hasToken1 lit = ScanNext(emptyScanner)
vibez.spill("Empty input ScanNext result: " + tea(hasToken1))
ready (hasToken1 == cap) {
    vibez.spill("✅ PASS: Empty input correctly returns false")
} otherwise {
    vibez.spill("❌ FAIL: Empty input should return false")
}

fr fr Test 2: Single number token
vibez.spill("\n=== Test 2: Single Number Token ===")
sus numberScanner tea = NewSlayScanner("123")
sus hasToken2 lit = ScanNext(numberScanner)
vibez.spill("Number token ScanNext result: " + tea(hasToken2))
ready (hasToken2 == based) {
    vibez.spill("✅ PASS: Number token correctly returns true")
} otherwise {
    vibez.spill("❌ FAIL: Number token should return true")
}

fr fr Test 3: Multiple tokens with whitespace
vibez.spill("\n=== Test 3: Multiple Tokens ===")
sus multiScanner tea = NewSlayScanner("hello 123 world")
sus hasToken3a lit = ScanNext(multiScanner)
vibez.spill("First token result: " + tea(hasToken3a))
sus hasToken3b lit = ScanNext(multiScanner)
vibez.spill("Second token result: " + tea(hasToken3b))
sus hasToken3c lit = ScanNext(multiScanner)
vibez.spill("Third token result: " + tea(hasToken3c))

fr fr Test 4: String tokens
vibez.spill("\n=== Test 4: String Tokens ===")
sus stringScanner tea = NewSlayScanner("\"hello world\" 42")
sus hasToken4a lit = ScanNext(stringScanner)
vibez.spill("String token result: " + tea(hasToken4a))
sus hasToken4b lit = ScanNext(stringScanner)
vibez.spill("Number after string result: " + tea(hasToken4b))

fr fr Test 5: Identifier tokens
vibez.spill("\n=== Test 5: Identifier Tokens ===")
sus identScanner tea = NewSlayScanner("variable_name function_call")
sus hasToken5a lit = ScanNext(identScanner)
vibez.spill("First identifier result: " + tea(hasToken5a))
sus hasToken5b lit = ScanNext(identScanner)
vibez.spill("Second identifier result: " + tea(hasToken5b))

fr fr Test 6: Operator tokens
vibez.spill("\n=== Test 6: Operator Tokens ===")
sus opScanner tea = NewSlayScanner("+ - * /")
sus hasToken6a lit = ScanNext(opScanner)
vibez.spill("Plus operator result: " + tea(hasToken6a))
sus hasToken6b lit = ScanNext(opScanner)
vibez.spill("Minus operator result: " + tea(hasToken6b))

fr fr Test 7: End of input detection
vibez.spill("\n=== Test 7: End of Input Detection ===")
sus endScanner tea = NewSlayScanner("a")
sus hasToken7a lit = ScanNext(endScanner)
vibez.spill("Single char token result: " + tea(hasToken7a))
sus hasToken7b lit = ScanNext(endScanner)
vibez.spill("End of input result: " + tea(hasToken7b))
ready (hasToken7a == based && hasToken7b == cap) {
    vibez.spill("✅ PASS: End of input correctly detected")
} otherwise {
    vibez.spill("❌ FAIL: End of input detection failed")
}

fr fr Test 8: Whitespace-only input
vibez.spill("\n=== Test 8: Whitespace-Only Input ===")
sus wsScanner tea = NewSlayScanner("   \t\n  ")
sus hasToken8 lit = ScanNext(wsScanner)
vibez.spill("Whitespace-only result: " + tea(hasToken8))
ready (hasToken8 == cap) {
    vibez.spill("✅ PASS: Whitespace-only correctly returns false")
} otherwise {
    vibez.spill("❌ FAIL: Whitespace-only should return false")
}

vibez.spill("\n=== Token Scanning Test Complete ===")
vibez.spill("All tests verify proper tokenization and end-of-input detection")
