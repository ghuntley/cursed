yeet "vibez"
yeet "stringz"

fr fr Comprehensive String Manipulation Demo

vibez.spill("=== CURSED String Manipulation Demo ===")
vibez.spill("")

fr fr Basic Operations
vibez.spill("1. Basic String Operations:")
vibez.spill("   Length of 'hello': " + stringz.length("hello"))
vibez.spill("   Concatenation: " + stringz.concat("Hello", " World"))
vibez.spill("   Character at position 1 in 'hello': " + stringz.char_at("hello", 1))
vibez.spill("   Substring (1,3) of 'hello': " + stringz.substring("hello", 1, 3))
vibez.spill("")

fr fr Searching Operations
vibez.spill("2. String Searching:")
vibez.spill("   Find 'wor' in 'hello world': " + stringz.find("hello world", "wor"))
vibez.spill("   Contains 'world' in 'hello world': " + stringz.contains("hello world", "world"))
vibez.spill("   Starts with 'hello': " + stringz.starts_with("hello world", "hello"))
vibez.spill("   Ends with 'world': " + stringz.ends_with("hello world", "world"))
vibez.spill("")

fr fr String Manipulation
vibez.spill("3. String Manipulation:")
vibez.spill("   Replace 'world' with 'CURSED': " + stringz.replace("hello world", "world", "CURSED"))
vibez.spill("   Trim '  hello  ': '" + stringz.trim("  hello  ") + "'")
vibez.spill("   Reverse 'hello': " + stringz.reverse("hello"))
vibez.spill("   Pad left 'hi' to width 8: '" + stringz.pad_left("hi", 8, '-') + "'")
vibez.spill("   Pad right 'hi' to width 8: '" + stringz.pad_right("hi", 8, '-') + "'")
vibez.spill("")

fr fr Case Conversion
vibez.spill("4. Case Conversion:")
vibez.spill("   Uppercase 'hello': " + stringz.to_upper("hello"))
vibez.spill("   Lowercase 'WORLD': " + stringz.to_lower("WORLD"))
vibez.spill("   Title case 'hello world': " + stringz.to_title("hello world"))
vibez.spill("")

fr fr String Validation
vibez.spill("5. String Validation:")
vibez.spill("   Is 'hello' alphabetic: " + stringz.is_alpha("hello"))
vibez.spill("   Is '12345' numeric: " + stringz.is_digit("12345"))
vibez.spill("   Is 'hello123' alphanumeric: " + stringz.is_alnum("hello123"))
vibez.spill("   Is '   ' whitespace: " + stringz.is_space("   "))
vibez.spill("")

fr fr Character Testing
vibez.spill("6. Character Testing:")
vibez.spill("   Is 'a' alphabetic: " + stringz.is_alpha_char('a'))
vibez.spill("   Is '5' digit: " + stringz.is_digit_char('5'))
vibez.spill("   Is ' ' space: " + stringz.is_space_char(' '))
vibez.spill("")

fr fr Splitting and Joining
vibez.spill("7. String Splitting and Joining:")
sus words [tea] = stringz.split("apple,banana,cherry", ",")
vibez.spill("   Split 'apple,banana,cherry' by comma:")
vibez.spill("     First word: " + words[0])
vibez.spill("     Second word: " + words[1])
vibez.spill("     Third word: " + words[2])

sus fruits [tea]
fruits = append(fruits, "apple")
fruits = append(fruits, "banana")
fruits = append(fruits, "cherry")
vibez.spill("   Join with ' | ': " + stringz.join(fruits, " | "))
vibez.spill("")

fr fr URL Encoding
vibez.spill("8. URL Encoding/Decoding:")
vibez.spill("   URL encode 'hello world': " + stringz.url_encode("hello world"))
vibez.spill("   URL decode 'hello%20world': " + stringz.url_decode("hello%20world"))
vibez.spill("")

fr fr UTF-8 Conversion
vibez.spill("9. UTF-8 Byte Conversion:")
sus utf8_bytes [normie] = stringz.to_utf8("ABC")
vibez.spill("   UTF-8 bytes of 'ABC': [" + utf8_bytes[0] + ", " + utf8_bytes[1] + ", " + utf8_bytes[2] + "]")

sus test_bytes [normie]
test_bytes = append(test_bytes, 72) fr fr 'H'
test_bytes = append(test_bytes, 105) fr fr 'i'
vibez.spill("   Bytes [72, 105] to string: " + stringz.from_utf8(test_bytes))
vibez.spill("")

vibez.spill("=== String manipulation capabilities completed! ===")
