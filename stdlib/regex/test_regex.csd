yeet "testz"
yeet "regex"

fr fr ========================================
fr fr Regex Module Comprehensive Tests
fr fr ========================================

test_start("Pattern Compilation Tests")

fr fr Test basic pattern compilation
sus pattern Pattern = compile("hello")
assert_eq_string(pattern.raw, "hello")
assert_eq_string(pattern.compiled, "hello")

fr fr Test pattern with flags
sus flagged_pattern Pattern = compile_with_flags("test", "i")
assert_eq_string(flagged_pattern.flags, "i")

test_start("Literal Matching Tests")

fr fr Test literal string matching
sus literal_pattern Pattern = compile("world")
sus match Match = literal_pattern.find("hello world test")
assert_eq_string(match.text, "world")
assert_eq_int(match.start, 6)
assert_eq_int(match.end, 11)

fr fr Test no match case
sus no_match Match = literal_pattern.find("hello test")
assert_eq_int(no_match.start, -1)

test_start("Special Character Pattern Tests")

fr fr Test digit pattern
sus digit_pattern Pattern = compile("\\d+")
sus digit_match Match = digit_pattern.find("abc123def")
assert_eq_string(digit_match.text, "123")
assert_eq_int(digit_match.start, 3)

fr fr Test word character pattern
sus word_pattern Pattern = compile("\\w+")
sus word_match Match = word_pattern.find("  hello  ")
assert_eq_string(word_match.text, "hello")

fr fr Test whitespace pattern
sus space_pattern Pattern = compile("\\s+")
sus space_match Match = space_pattern.find("hello   world")
assert_eq_string(space_match.text, "   ")

test_start("Character Class Tests")

fr fr Test digit character class
sus digits_class Pattern = compile("[0-9]+")
sus digits_match Match = digits_class.find("test456")
assert_eq_string(digits_match.text, "456")

fr fr Test letter character class
sus letters_class Pattern = compile("[a-zA-Z]+")
sus letters_match Match = letters_class.find("123abc456")
assert_eq_string(letters_match.text, "abc")

test_start("Pattern Testing")

fr fr Test pattern matching
sus test_pattern Pattern = compile("test")
assert_true(test_pattern.test("testing"))
assert_false(test_pattern.test("hello world"))

test_start("Find All Matches Tests")

fr fr Test finding all digit sequences
sus all_digits Pattern = compile("\\d+")
sus text_with_numbers tea = "abc123def456ghi789"
sus all_matches Match[value] = all_digits.find_all(text_with_numbers)
assert_eq_int(all_matches.length(), 3)
assert_eq_string(all_matches[0].text, "123")
assert_eq_string(all_matches[1].text, "456")
assert_eq_string(all_matches[2].text, "789")

test_start("Replace Tests")

fr fr Test single replacement
sus replace_pattern Pattern = compile("cat")
sus replaced tea = replace_pattern.replace("the cat sat", "dog")
assert_eq_string(replaced, "the dog sat")

fr fr Test replace all
sus all_replaced tea = replace_pattern.replace_all("cat cat cat", "dog")
assert_eq_string(all_replaced, "dog dog dog")

test_start("Split Tests")

fr fr Test split by pattern
sus split_pattern Pattern = compile("\\s+")
sus split_text tea = "hello    world    test"
sus parts tea[value] = split_pattern.split(split_text)
assert_eq_int(parts.length(), 3)
assert_eq_string(parts[0], "hello")
assert_eq_string(parts[1], "world")
assert_eq_string(parts[2], "test")

test_start("Convenience Function Tests")

fr fr Test email matching
assert_true(match_email("test@example.com"))
assert_false(match_email("not-an-email"))

fr fr Test URL matching
assert_true(match_url("https://www.example.com"))
assert_true(match_url("http://example.com/path"))
assert_false(match_url("not-a-url"))

fr fr Test IP address matching
assert_true(match_ip_address("192.168.1.1"))
assert_false(match_ip_address("not.an.ip.address"))

test_start("Extract Functions Tests")

fr fr Test number extraction
sus text_with_nums tea = "I have 5 apples and 10 oranges"
sus extracted_numbers tea[value] = extract_numbers(text_with_nums)
assert_eq_int(extracted_numbers.length(), 2)
assert_eq_string(extracted_numbers[0], "5")
assert_eq_string(extracted_numbers[1], "10")

fr fr Test word extraction
sus text_with_words tea = "hello, world! test123"
sus extracted_words tea[value] = extract_words(text_with_words)
assert_eq_int(extracted_words.length(), 3)
assert_eq_string(extracted_words[0], "hello")
assert_eq_string(extracted_words[1], "world")
assert_eq_string(extracted_words[2], "test123")

test_start("Character Classification Tests")

fr fr Test digit classification
assert_true(is_digit("5"))
assert_false(is_digit("a"))

fr fr Test letter classification
assert_true(is_letter("A"))
assert_true(is_letter("z"))
assert_false(is_letter("5"))

fr fr Test word character classification
assert_true(is_word_char("_"))
assert_true(is_word_char("a"))
assert_true(is_word_char("1"))
assert_false(is_word_char(" "))

fr fr Test whitespace classification
assert_true(is_whitespace(" "))
assert_true(is_whitespace("\t"))
assert_false(is_whitespace("a"))

test_start("Utility Function Tests")

fr fr Test quote metacharacters
sus quoted tea = quote("hello.world*")
assert_eq_string(quoted, "hello\\.world\\*")

fr fr Test alternation building
sus words tea[value] = ["cat", "dog", "bird"]
sus alternation tea = build_alternation(words)
sus alt_pattern Pattern = compile(alternation)
assert_true(alt_pattern.test("I have a cat"))
assert_true(alt_pattern.test("The dog barks"))
assert_false(alt_pattern.test("Fish swim"))

test_start("ASCII Conversion Tests")

fr fr Test character to ASCII conversion
assert_eq_int(char_to_ascii("A"), 65)
assert_eq_int(char_to_ascii("z"), 122)
assert_eq_int(char_to_ascii("0"), 48)
assert_eq_int(char_to_ascii("9"), 57)
assert_eq_int(char_to_ascii(" "), 32)

test_start("Group Counting Tests")

fr fr Test capture group counting
sus simple_group normie = count_groups("(hello)")
assert_eq_int(simple_group, 1)

sus multiple_groups normie = count_groups("(hello) (world)")
assert_eq_int(multiple_groups, 2)

sus no_groups normie = count_groups("hello world")
assert_eq_int(no_groups, 0)

test_start("Complex Pattern Tests")

fr fr Test dotall pattern
sus dotall Pattern = compile(".*")
sus dotall_match Match = dotall.find("hello\nworld")
assert_true(dotall_match.start >= 0)

fr fr Test alternation pattern
sus alt_pattern Pattern = compile("cat|dog")
assert_true(alt_pattern.test("I have a cat"))
assert_true(alt_pattern.test("The dog runs"))
assert_false(alt_pattern.test("Birds fly"))

print_test_summary()
