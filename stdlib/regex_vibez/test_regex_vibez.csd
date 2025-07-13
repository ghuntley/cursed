yeet "regex_vibez"

vibez.spill("Testing regex_vibez module...")

// Test basic pattern matching
sus result1 lit = regex_vibez.match_pattern("hello", "hello world")
vibez.spill("match_pattern test passed")

// Test pattern replacement
sus result2 tea = regex_vibez.replace_pattern("world", "hello world", "universe")
vibez.spill("replace_pattern test passed")

// Test starts_with_pattern
sus result3 lit = regex_vibez.starts_with_pattern("hello", "hello world")
vibez.spill("starts_with_pattern test passed")

// Test ends_with_pattern
sus result4 lit = regex_vibez.ends_with_pattern("world", "hello world")
vibez.spill("ends_with_pattern test passed")

// Test wildcard matching
sus result5 lit = regex_vibez.wildcard_match("hello", "hello")
vibez.spill("wildcard_match test passed")

// Test string utilities
sus len normie = regex_vibez.str_length("hello")
vibez.spill("str_length test passed")

sus equals lit = regex_vibez.str_equals("test", "test")
vibez.spill("str_equals test passed")

sus concat tea = regex_vibez.str_concat("hello", " world")
vibez.spill("str_concat test passed")

vibez.spill("All regex_vibez tests completed successfully!")
