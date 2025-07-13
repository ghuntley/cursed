# Simple regex functionality test
# Direct function calls without module system

vibez.spill("Testing basic regex pattern matching...")

# Test basic pattern matching
sus match1 lit = match_pattern("hello", "hello")
vibez.spill("Exact match test: ")
vibez.spill(match1)

sus match2 lit = match_pattern("hello", "world") 
vibez.spill("Different pattern test: ")
vibez.spill(match2)

# Test wildcard matching
sus wildcard1 lit = match_wildcard("hello", "*")
vibez.spill("Wildcard * test: ")
vibez.spill(wildcard1)

sus wildcard2 lit = match_wildcard("hello", "h?llo")
vibez.spill("Wildcard ? test: ")
vibez.spill(wildcard2)

# Test find matches
sus matches [tea] = find_matches("test test test", "test")
vibez.spill("Find matches count: ")
vibez.spill(len(matches))

# Test replace pattern
sus replaced tea = replace_pattern("hello world", "hello", "hi")
vibez.spill("Replace result: ")
vibez.spill(replaced)

# Test regex compilation
sus engine RegexEngine = regex_compile_pcre("hello", 0)
vibez.spill("Compiled pattern: ")
vibez.spill(engine.pattern)

vibez.spill("All regex tests completed!")
