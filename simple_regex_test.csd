yeet "regex_vibez"
yeet "vibez"

vibez.spill("Simple Regex Engine Test")

fr fr Test basic functionality
sus regex Regex = compile_pattern("test")
sus result lit = match_pattern(regex, "test this")

vibez.spill("Pattern: 'test'")
vibez.spill("Text: 'test this'")
vibez.spill("Match result:", result)

fr fr Test digit class
sus digit_regex Regex = compile_pattern("\\d")
sus digit_result lit = match_pattern(digit_regex, "abc123")

vibez.spill("Pattern: '\\d'")
vibez.spill("Text: 'abc123'")
vibez.spill("Match result:", digit_result)

fr fr Test wildcard
sus wildcard_regex Regex = compile_pattern("h.llo")
sus wildcard_result lit = match_pattern(wildcard_regex, "hello world")

vibez.spill("Pattern: 'h.llo'")
vibez.spill("Text: 'hello world'")
vibez.spill("Match result:", wildcard_result)
