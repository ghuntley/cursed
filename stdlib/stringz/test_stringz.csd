yeet "testz"
yeet "stringz"

fr fr Comprehensive String Processing Test Suite

test_start("String Length and Basic Operations")

fr fr Test string length
assert_eq_int(stringz.length("hello"), 5)
assert_eq_int(stringz.length(""), 0)
assert_eq_int(stringz.length("a"), 1)

fr fr Test string concatenation
assert_eq_string(stringz.concat("hello", "world"), "helloworld")
assert_eq_string(stringz.concat("", "test"), "test")
assert_eq_string(stringz.concat("test", ""), "test")

fr fr Test character access
assert_true(stringz.char_at("hello", 0) == 'h')
assert_true(stringz.char_at("hello", 4) == 'o')

fr fr Test substring extraction
assert_eq_string(stringz.substring("hello", 1, 3), "ell")
assert_eq_string(stringz.substring("test", 0, 2), "te")
assert_eq_string(stringz.substring("abc", 2, 1), "c")

test_start("String Equality and Empty Checks")

fr fr Test string equality
assert_true(stringz.equals("test", "test"))
assert_false(stringz.equals("test", "other"))
assert_true(stringz.equals("", ""))

fr fr Test empty string checks
assert_true(stringz.is_empty(""))
assert_false(stringz.is_empty("test"))
assert_false(stringz.is_empty(" "))

test_start("String Searching Operations")

fr fr Test find function
assert_eq_int(stringz.find("hello world", "world"), 6)
assert_eq_int(stringz.find("hello", "xyz"), -1)
assert_eq_int(stringz.find("hello", "hello"), 0)
assert_eq_int(stringz.find("hello", ""), 0)

fr fr Test contains function
assert_true(stringz.contains("hello world", "world"))
assert_false(stringz.contains("hello", "xyz"))
assert_true(stringz.contains("test", ""))
assert_true(stringz.contains("programming", "gram"))

fr fr Test starts_with function
assert_true(stringz.starts_with("hello world", "hello"))
assert_false(stringz.starts_with("hello", "world"))
assert_true(stringz.starts_with("test", ""))
assert_true(stringz.starts_with("abc", "a"))

fr fr Test ends_with function
assert_true(stringz.ends_with("hello world", "world"))
assert_false(stringz.ends_with("hello", "world"))
assert_true(stringz.ends_with("test", ""))
assert_true(stringz.ends_with("abc", "c"))

test_start("String Manipulation Operations")

fr fr Test replace function
assert_eq_string(stringz.replace("hello world", "world", "universe"), "hello universe")
assert_eq_string(stringz.replace("test test", "test", "exam"), "exam exam")
assert_eq_string(stringz.replace("hello", "xyz", "abc"), "hello")

fr fr Test trim function
assert_eq_string(stringz.trim("  hello  "), "hello")
assert_eq_string(stringz.trim("hello"), "hello")
assert_eq_string(stringz.trim("   "), "")

fr fr Test padding functions
assert_eq_string(stringz.pad_left("hi", 5, ' '), "   hi")
assert_eq_string(stringz.pad_right("hi", 5, ' '), "hi   ")
assert_eq_string(stringz.pad_left("hello", 3, ' '), "hello")

fr fr Test reverse function
assert_eq_string(stringz.reverse("hello"), "olleh")
assert_eq_string(stringz.reverse("a"), "a")
assert_eq_string(stringz.reverse(""), "")

test_start("Case Conversion Operations")

fr fr Test uppercase conversion
assert_eq_string(stringz.to_upper("hello"), "HELLO")
assert_eq_string(stringz.to_upper("HELLO"), "HELLO")
assert_eq_string(stringz.to_upper("Hello123"), "HELLO123")

fr fr Test lowercase conversion
assert_eq_string(stringz.to_lower("HELLO"), "hello")
assert_eq_string(stringz.to_lower("hello"), "hello")
assert_eq_string(stringz.to_lower("Hello123"), "hello123")

fr fr Test title case conversion
assert_eq_string(stringz.to_title("hello world"), "Hello World")
assert_eq_string(stringz.to_title("HELLO WORLD"), "Hello World")
assert_eq_string(stringz.to_title("hello"), "Hello")

test_start("String Splitting and Joining")

fr fr Test split function
sus split_result [tea] = stringz.split("a,b,c", ",")
assert_eq_int(len(split_result), 3)
assert_eq_string(split_result[0], "a")
assert_eq_string(split_result[1], "b")
assert_eq_string(split_result[2], "c")

sus split_empty [tea] = stringz.split("hello", "")
assert_eq_int(len(split_empty), 1)
assert_eq_string(split_empty[0], "hello")

fr fr Test join function
sus parts [tea]
parts = append(parts, "hello")
parts = append(parts, "world")
parts = append(parts, "test")
assert_eq_string(stringz.join(parts, " "), "hello world test")
assert_eq_string(stringz.join(parts, ""), "helloworldtest")

sus empty_parts [tea]
assert_eq_string(stringz.join(empty_parts, ","), "")

fr fr Test lines function
sus line_result [tea] = stringz.lines("line1\nline2\nline3")
assert_eq_int(len(line_result), 3)
assert_eq_string(line_result[0], "line1")
assert_eq_string(line_result[1], "line2")
assert_eq_string(line_result[2], "line3")

test_start("String Validation Functions")

fr fr Test is_alpha function
assert_true(stringz.is_alpha("hello"))
assert_true(stringz.is_alpha("ABC"))
assert_false(stringz.is_alpha("hello123"))
assert_false(stringz.is_alpha("hello world"))
assert_false(stringz.is_alpha(""))

fr fr Test is_digit function
assert_true(stringz.is_digit("123"))
assert_true(stringz.is_digit("0"))
assert_false(stringz.is_digit("123a"))
assert_false(stringz.is_digit("hello"))
assert_false(stringz.is_digit(""))

fr fr Test is_alnum function
assert_true(stringz.is_alnum("hello123"))
assert_true(stringz.is_alnum("ABC"))
assert_true(stringz.is_alnum("123"))
assert_false(stringz.is_alnum("hello world"))
assert_false(stringz.is_alnum("hello!"))
assert_false(stringz.is_alnum(""))

fr fr Test is_space function
assert_true(stringz.is_space("   "))
assert_true(stringz.is_space("\t"))
assert_true(stringz.is_space("\n"))
assert_false(stringz.is_space("hello"))
assert_false(stringz.is_space(" a "))
assert_false(stringz.is_space(""))

test_start("String Encoding Functions")

fr fr Test UTF-8 conversion
sus utf8_bytes [normie] = stringz.to_utf8("ABC")
assert_eq_int(len(utf8_bytes), 3)
assert_eq_int(utf8_bytes[0], 65) fr fr 'A'
assert_eq_int(utf8_bytes[1], 66) fr fr 'B'
assert_eq_int(utf8_bytes[2], 67) fr fr 'C'

sus bytes_to_convert [normie]
bytes_to_convert = append(bytes_to_convert, 72) fr fr 'H'
bytes_to_convert = append(bytes_to_convert, 73) fr fr 'I'
assert_eq_string(stringz.from_utf8(bytes_to_convert), "HI")

fr fr Test URL encoding
assert_eq_string(stringz.url_encode("hello world"), "hello%20world")
assert_eq_string(stringz.url_encode("test"), "test")
assert_eq_string(stringz.url_encode("a-b_c.d~e"), "a-b_c.d~e")

fr fr Test URL decoding
assert_eq_string(stringz.url_decode("hello%20world"), "hello world")
assert_eq_string(stringz.url_decode("test"), "test")

test_start("Helper Function Tests")

fr fr Test character validation helpers
assert_true(stringz.is_alpha_char('a'))
assert_true(stringz.is_alpha_char('Z'))
assert_false(stringz.is_alpha_char('1'))
assert_false(stringz.is_alpha_char(' '))

assert_true(stringz.is_digit_char('0'))
assert_true(stringz.is_digit_char('9'))
assert_false(stringz.is_digit_char('a'))
assert_false(stringz.is_digit_char(' '))

assert_true(stringz.is_space_char(' '))
assert_true(stringz.is_space_char('\t'))
assert_true(stringz.is_space_char('\n'))
assert_false(stringz.is_space_char('a'))

test_start("Legacy Compatibility Tests")

fr fr Test legacy function aliases
assert_eq_int(stringz.string_length("hello"), 5)
assert_eq_string(stringz.string_concat("hello", "world"), "helloworld")

test_start("Edge Cases and Error Conditions")

fr fr Test edge cases for substring
assert_eq_string(stringz.substring("test", -1, 2), "")
assert_eq_string(stringz.substring("test", 10, 2), "")
assert_eq_string(stringz.substring("test", 2, 0), "")
assert_eq_string(stringz.substring("test", 2, 10), "st")

fr fr Test edge cases for find
assert_eq_int(stringz.find("", ""), 0)
assert_eq_int(stringz.find("test", "testing"), -1)
assert_eq_int(stringz.find("aaaa", "aa"), 0)

fr fr Test edge cases for replace
assert_eq_string(stringz.replace("", "a", "b"), "")
assert_eq_string(stringz.replace("test", "", "x"), "test")

fr fr Test edge cases for split
sus single_split [tea] = stringz.split("hello", "hello")
assert_eq_int(len(single_split), 2)
assert_eq_string(single_split[0], "")
assert_eq_string(single_split[1], "")

print_test_summary()
