yeet "regex"

sus pattern tea = "hello"
sus text tea = "hello world hello"

fr fr Test basic match
sus match_result lit = regex.match(pattern, text)
io.println("Match test:")
io.println(match_result)  // Should be true

fr fr Test find
sus find_result tea = regex.find(pattern, text)
io.println("Find test:")
io.println(find_result)  // Should be "hello"

fr fr Test find_all
sus find_all_result = regex.find_all(pattern, text)
io.println("Find all test:")
io.println(find_all_result)  // Should be array with two "hello"s

fr fr Test replace
sus replace_result tea = regex.replace(pattern, text, "hi")
io.println("Replace test:")
io.println(replace_result)  // Should be "hi world hello"

fr fr Test replace_all
sus replace_all_result tea = regex.replace_all(pattern, text, "hi")
io.println("Replace all test:")
io.println(replace_all_result)  // Should be "hi world hi"

fr fr Test split
sus split_result = regex.split(pattern, text)
io.println("Split test:")
io.println(split_result)  // Should be array with " world ", ""

fr fr Test digit pattern
sus digit_pattern tea = "\\d+"
sus digit_text tea = "abc123def456"
sus digit_matches = regex.find_all(digit_pattern, digit_text)
io.println("Digit pattern test:")
io.println(digit_matches)  // Should be ["123", "456"]

fr fr Test word pattern
sus word_pattern tea = "\\w+"
sus word_text tea = "hello_world 123 test"
sus word_matches = regex.find_all(word_pattern, word_text)
io.println("Word pattern test:")
io.println(word_matches)  // Should be ["hello_world", "123", "test"]
