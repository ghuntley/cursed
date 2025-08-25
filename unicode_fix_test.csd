yeet "vibez"

# Test the fixed Unicode functions directly from regex_real_engine
yeet "stdlib/regex_real_engine"

vibez.spill("Testing Unicode fixes...")

# Test 1: string_to_codepoint with ASCII
sus ascii_cp drip = string_to_codepoint("A")
vibez.spill("ASCII 'A' codepoint:", ascii_cp)

# Test 2: string_to_codepoint with UTF-8
sus utf8_cp drip = string_to_codepoint("é")
vibez.spill("UTF-8 'é' codepoint:", utf8_cp)

# Test 3: text_to_codepoints_real
sus codepoints []drip = text_to_codepoints_real("Hé!")
vibez.spill("Text 'Hé!' codepoints:", codepoints)

# Test 4: substring_by_codepoints
sus substr tea = substring_by_codepoints("Hé€🚀", 1, 3)
vibez.spill("Substring 'Hé€🚀'[1:3]:", substr)

vibez.spill("Unicode fix testing complete!")
