yeet "stringz"
yeet "vibez"

fr fr Quick test of key stringz functionality
vibez.spill("=== CURSED String Module Quick Test ===")

fr fr Test basic operations
vibez.spill("Length of 'hello':", stringz.len_str("hello"))
vibez.spill("Concatenation:", stringz.concat("hello", "world"))
vibez.spill("Substring 'hello'[1:3]:", stringz.substring("hello", 1, 3))

fr fr Test case conversion
vibez.spill("Uppercase 'hello':", stringz.to_upper("hello"))
vibez.spill("Lowercase 'WORLD':", stringz.to_lower("WORLD"))

fr fr Test search operations
vibez.spill("Index of 'world' in 'hello world':", stringz.index_of("hello world", "world"))
vibez.spill("Contains 'gram' in 'programming':", stringz.contains("programming", "gram"))

fr fr Test trimming
vibez.spill("Trimmed '  hello  ':", "'" + stringz.trim("  hello  ") + "'")

fr fr Test splitting and joining
sus parts []tea
parts = append(parts, "apple")
parts = append(parts, "banana")
parts = append(parts, "cherry")
vibez.spill("Joined with comma:", stringz.join(parts, ","))

fr fr Test validation
vibez.spill("Is 'hello' alpha?", stringz.is_alpha("hello"))
vibez.spill("Is '123' numeric?", stringz.is_numeric("123"))

vibez.spill("=== All basic tests completed ===")
