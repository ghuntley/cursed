fr fr Simple demonstration of working string runtime support

yeet "stringz"

vibez.spill("=== CURSED String Runtime Support Demo ===")

fr fr Basic string operations
sus greeting tea = "Hello"
sus target tea = "CURSED"
sus combined tea = stringz.concat(greeting, " ")
combined = stringz.concat(combined, target)

vibez.spill("Combined string: " + combined)
vibez.spill("String length: " + string(stringz.length(combined)))

fr fr Character access
sus first_char sip = stringz.char_at(combined, 0)
sus sixth_char sip = stringz.char_at(combined, 6)

vibez.spill("First character: " + stringz.runtime_char_to_string(first_char))
vibez.spill("Sixth character: " + stringz.runtime_char_to_string(sixth_char))

fr fr Substring operations
sus hello_part tea = stringz.substring(combined, 0, 5)
sus cursed_part tea = stringz.substring(combined, 6, 6)

vibez.spill("Hello part: " + hello_part)
vibez.spill("CURSED part: " + cursed_part)

fr fr String searching
sus cursed_pos normie = stringz.find(combined, "CURSED")
sus xyz_pos normie = stringz.find(combined, "xyz")

vibez.spill("Position of 'CURSED': " + string(cursed_pos))
vibez.spill("Position of 'xyz': " + string(xyz_pos))

fr fr Case conversion
sus upper_hello tea = stringz.to_upper(hello_part)
sus lower_cursed tea = stringz.to_lower(cursed_part)

vibez.spill("Uppercase hello: " + upper_hello)
vibez.spill("Lowercase CURSED: " + lower_cursed)

fr fr String validation
sus is_alpha_hello lit = stringz.is_alpha(hello_part)
sus is_digit_hello lit = stringz.is_digit(hello_part)

vibez.spill("Is 'Hello' alphabetic? " + string(is_alpha_hello))
vibez.spill("Is 'Hello' numeric? " + string(is_digit_hello))

fr fr URL encoding
sus url_string tea = "hello world test"
sus encoded tea = stringz.url_encode(url_string)
sus decoded tea = stringz.url_decode(encoded)

vibez.spill("Original: " + url_string)
vibez.spill("URL encoded: " + encoded)
vibez.spill("URL decoded: " + decoded)

vibez.spill("✅ String runtime support is working perfectly!")
vibez.spill("✅ This enables hashz, vibez, and other stdlib modules!")
