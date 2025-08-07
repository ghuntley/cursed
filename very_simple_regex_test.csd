yeet "regex_vibez"

fr fr Test basic pattern compilation
sus regex Regex = compile_pattern("test")

fr fr Test basic matching
sus result1 lit = match_pattern(regex, "test this")
sus result2 lit = match_pattern(regex, "goodbye")

fr fr Test digit matching
sus digit_regex Regex = compile_pattern("\\d")
sus digit_result lit = match_pattern(digit_regex, "abc123")

fr fr Test wildcard
sus wildcard_regex Regex = compile_pattern("h.llo")
sus wildcard_result lit = match_pattern(wildcard_regex, "hello world")

fr fr Manual check
vibes result1 {
    vibez.spill("SUCCESS: 'test' found in 'test this'")
} otherwise {
    vibez.spill("FAIL: 'test' not found in 'test this'")
}

vibes digit_result {
    vibez.spill("SUCCESS: digit found in 'abc123'")
} otherwise {
    vibez.spill("FAIL: digit not found in 'abc123'")
}

vibes wildcard_result {
    vibez.spill("SUCCESS: 'h.llo' matched 'hello world'")
} otherwise {
    vibez.spill("FAIL: 'h.llo' did not match 'hello world'")
}
