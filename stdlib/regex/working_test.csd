# Working regex test

vibez.spill("Starting regex tests...")

# Test 1: Basic pattern matching
vibez.spill("Test 1: Basic pattern matching")
sus match1 lit = match_pattern("hello", "hello")
vibez.spill("Exact match (hello == hello):")
vibez.spill(match1)

sus match2 lit = match_pattern("hello", "world")
vibez.spill("Different strings (hello == world):")
vibez.spill(match2)

# Test 2: Wildcard matching
vibez.spill("Test 2: Wildcard matching")
sus wild1 lit = match_wildcard("hello", "*")
vibez.spill("Universal wildcard (* matches hello):")
vibez.spill(wild1)

sus wild2 lit = match_wildcard("hello", "h?llo")
vibez.spill("Question mark wildcard (h?llo matches hello):")
vibez.spill(wild2)

# Test 3: Pattern replacement
vibez.spill("Test 3: Pattern replacement")
sus replaced tea = replace_pattern("hello world", "hello", "hi")
vibez.spill("Replaced 'hello' with 'hi':")
vibez.spill(replaced)

# Test 4: Regex compilation
vibez.spill("Test 4: Regex compilation")
sus engine RegexEngine = regex_compile_pcre("test", 0)
vibez.spill("Compiled pattern:")
vibez.spill(engine.pattern)
vibez.spill("Unicode enabled:")
vibez.spill(engine.unicode_enabled)

vibez.spill("All tests completed successfully!")
