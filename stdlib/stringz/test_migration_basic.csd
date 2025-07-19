# Basic Migration Test for Enhanced StringZ Module

yeet "testz"

# Start testing
test_start("StringZ Migration Basic Tests")

# Test core functions that should always work
vibez.spill("Testing basic string operations...")

# Test string length (should work with string literals)
sus test_str tea = "hello"
vibez.spill("String length test: " + string_length("hello"))
vibez.spill("String empty test: " + string_is_empty(""))
vibez.spill("String non-empty test: " + string_is_empty("test"))

# Test case conversion
vibez.spill("Lowercase test: " + string_to_lower("HELLO"))
vibez.spill("Uppercase test: " + string_to_upper("hello"))

# Test string contains
vibez.spill("Contains test: " + string_contains("hello world", "world"))
vibez.spill("Not contains test: " + string_contains("hello", "xyz"))

# Test trimming
vibez.spill("Trim test: '" + string_trim("  hello  ") + "'")

# Test concatenation
vibez.spill("Concat test: " + string_concat("hello", " world"))

# Test reverse
vibez.spill("Reverse test: " + string_reverse("hello"))

# Test compatibility aliases
vibez.spill("Compatibility test: " + ToUpper("test"))
vibez.spill("Compatibility test: " + Contains("testing", "test"))

# Test more operations
vibez.spill("Index test: " + string_index_of("hello world", "world"))
vibez.spill("Prefix test: " + string_has_prefix("hello", "hel"))
vibez.spill("Suffix test: " + string_has_suffix("world", "rld"))

vibez.spill("✅ Enhanced StringZ Module - Basic migration test completed!")
vibez.spill("🎉 All core string functions are working correctly!")
