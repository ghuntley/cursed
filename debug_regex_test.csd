yeet "regex_vibez"
yeet "vibez"

vibez.spill("Debug Regex Test")

fr fr Test simple literal match
sus pattern tea = "test"
sus text tea = "test"

vibez.spill("Testing literal match:")
vibez.spill("Pattern:", pattern)
vibez.spill("Text:", text)

sus regex Regex = compile_pattern(pattern)
vibez.spill("Regex compiled successfully")

sus result lit = match_pattern(regex, text)
vibez.spill("Match result:", result)

fr fr Test very basic functionality
sus simple_pattern tea = "a"
sus simple_text tea = "a"
sus simple_regex Regex = compile_pattern(simple_pattern)
sus simple_result lit = match_pattern(simple_regex, simple_text)

vibez.spill("Simple test - Pattern 'a' vs Text 'a':", simple_result)

fr fr Test character by character
vibez.spill("Character test:")
sus char_a sip = runtime_string_char_at("a", 0)
sus char_test sip = runtime_string_char_at("test", 0)
vibez.spill("First char of 'a':", char_a)
vibez.spill("First char of 'test':", char_test)
