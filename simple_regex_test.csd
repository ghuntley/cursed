fr fr SIMPLE REGEX ENGINE TEST
fr fr Basic test to verify the new regex engine functionality

yeet "regexz"
yeet "vibez"

vibez.spill("Testing new regex engine implementation...")

fr fr Test 1: Basic literal matching
vibez.spill("Test 1: Literal matching")
ready (regex_test("hello", "hello world")) {
    vibez.spill("✓ Literal match test passed")
} otherwise {
    vibez.spill("✗ Literal match test failed")
}

fr fr Test 2: Any character matching
vibez.spill("Test 2: Any character matching")
ready (regex_test("h.llo", "hello")) {
    vibez.spill("✓ Any character test passed")
} otherwise {
    vibez.spill("✗ Any character test failed")
}

fr fr Test 3: Find operation
vibez.spill("Test 3: Find operation")
sus found tea = regex_find("world", "hello world")
vibez.spill("Found: '" + found + "'")
ready (found == "world") {
    vibez.spill("✓ Find operation test passed")
} otherwise {
    vibez.spill("✗ Find operation test failed")
}

fr fr Test 4: Replace operation
vibez.spill("Test 4: Replace operation")
sus replaced tea = regex_replace("world", "hello world", "universe")
vibez.spill("Replaced: '" + replaced + "'")
ready (replaced == "hello universe") {
    vibez.spill("✓ Replace operation test passed")
} otherwise {
    vibez.spill("✗ Replace operation test failed")
}

fr fr Test 5: Compilation validation
vibez.spill("Test 5: Compilation validation")
sus regex RegexPattern = regex_compile("test.*pattern", "")
ready (regex.is_compiled) {
    vibez.spill("✓ Regex compilation test passed")
} otherwise {
    vibez.spill("✗ Regex compilation test failed: " + regex.error_message)
}

vibez.spill("Simple regex engine tests completed!")
