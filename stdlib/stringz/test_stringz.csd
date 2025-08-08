yeet "testz"
yeet "stringz"

fr fr Comprehensive String Processing Test Suite - Production Quality

test_start("String Length and Basic Operations")

fr fr Test string length functions
assert_eq_int(stringz.len_str("hello"), 5)
assert_eq_int(stringz.length("hello"), 5)
assert_eq_int(stringz.len_str(""), 0)
assert_eq_int(stringz.len_str("a"), 1)
assert_eq_int(stringz.len_str("Hello, World!"), 13)

fr fr Test string concatenation
assert_eq_string(stringz.concat("hello", "world"), "helloworld")
assert_eq_string(stringz.concat("", "test"), "test")
assert_eq_string(stringz.concat("test", ""), "test")
assert_eq_string(stringz.concat("", ""), "")

fr fr Test character access
assert_true(stringz.char_at("hello", 0) == 'h')
assert_true(stringz.char_at("hello", 4) == 'o')
assert_true(stringz.char_at("hello", 2) == 'l')
assert_true(stringz.char_at("", 0) == '\0')
assert_true(stringz.char_at("test", 10) == '\0')

test_start("Substring and String Manipulation")

fr fr Test substring extraction
assert_eq_string(stringz.substring("hello", 1, 3), "ell")
assert_eq_string(stringz.substring("test", 0, 2), "te")
assert_eq_string(stringz.substring("abc", 2, 1), "c")
assert_eq_string(stringz.substring("hello", 0, 5), "hello")
assert_eq_string(stringz.substring("hello", 1, 10), "ello")

fr fr Edge cases for substring
assert_eq_string(stringz.substring("test", -1, 2), "")
assert_eq_string(stringz.substring("test", 10, 2), "")
assert_eq_string(stringz.substring("test", 2, 0), "")
assert_eq_string(stringz.substring("test", 2, -1), "")

fr fr Test left, right, mid functions
assert_eq_string(stringz.left("hello", 3), "hel")
assert_eq_string(stringz.right("hello", 3), "llo")
assert_eq_string(stringz.mid("hello", 1, 3), "ell")
assert_eq_string(stringz.left("hi", 10), "hi")
assert_eq_string(stringz.right("hi", 10), "hi")

test_start("String Equality and Comparison")

fr fr Test string equality
assert_true(stringz.equals("test", "test"))
assert_false(stringz.equals("test", "other"))
assert_true(stringz.equals("", ""))
assert_false(stringz.equals("a", ""))

fr fr Test string comparison
assert_eq_int(stringz.compare("abc", "abc"), 0)
assert_eq_int(stringz.compare("abc", "def"), -1)
assert_eq_int(stringz.compare("def", "abc"), 1)
assert_eq_int(stringz.compare("a", "ab"), -1)
assert_eq_int(stringz.compare("ab", "a"), 1)

fr fr Test comparison predicates
assert_true(stringz.less_than("abc", "def"))
assert_false(stringz.less_than("def", "abc"))
assert_true(stringz.greater_than("def", "abc"))
assert_false(stringz.greater_than("abc", "def"))

fr fr Test case-insensitive comparison
assert_eq_int(stringz.compare_ignore_case("Hello", "HELLO"), 0)
assert_eq_int(stringz.compare_ignore_case("ABC", "def"), -1)

fr fr Test empty string checks
assert_true(stringz.is_empty(""))
assert_false(stringz.is_empty("test"))
assert_false(stringz.is_empty(" "))

test_start("String Searching Operations")

fr fr Test index_of and find functions
assert_eq_int(stringz.index_of("hello world", "world"), 6)
assert_eq_int(stringz.find("hello world", "world"), 6)
assert_eq_int(stringz.index_of("hello", "xyz"), -1)
assert_eq_int(stringz.index_of("hello", "hello"), 0)
assert_eq_int(stringz.index_of("hello", ""), 0)
assert_eq_int(stringz.index_of("", "test"), -1)

fr fr Test last_index_of
assert_eq_int(stringz.last_index_of("hello hello", "hello"), 6)
assert_eq_int(stringz.last_index_of("abc abc abc", "abc"), 8)
assert_eq_int(stringz.last_index_of("test", "xyz"), -1)

fr fr Test count_occurrences
assert_eq_int(stringz.count_occurrences("hello hello hello", "hello"), 3)
assert_eq_int(stringz.count_occurrences("aaa", "aa"), 2)
assert_eq_int(stringz.count_occurrences("test", "xyz"), 0)

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

test_start("String Replacement Operations")

fr fr Test replace function
assert_eq_string(stringz.replace("hello world", "world", "universe"), "hello universe")
assert_eq_string(stringz.replace("test test", "test", "exam"), "exam exam")
assert_eq_string(stringz.replace("hello", "xyz", "abc"), "hello")
assert_eq_string(stringz.replace("", "a", "b"), "")
assert_eq_string(stringz.replace("test", "", "x"), "test")

fr fr Test replace_first
assert_eq_string(stringz.replace_first("hello hello", "hello", "hi"), "hi hello")
assert_eq_string(stringz.replace_first("abc", "xyz", "def"), "abc")

fr fr Test replace_all (same as replace in this implementation)
assert_eq_string(stringz.replace_all("test test test", "test", "exam"), "exam exam exam")

test_start("String Trimming and Padding")

fr fr Test trim functions
assert_eq_string(stringz.trim("  hello  "), "hello")
assert_eq_string(stringz.trim("hello"), "hello")
assert_eq_string(stringz.trim("   "), "")
assert_eq_string(stringz.trim("\t\nhello\r\n"), "hello")

assert_eq_string(stringz.trim_left("  hello  "), "hello  ")
assert_eq_string(stringz.trim_right("  hello  "), "  hello")

fr fr Test padding functions
assert_eq_string(stringz.pad_left("hi", 5, ' '), "   hi")
assert_eq_string(stringz.pad_right("hi", 5, ' '), "hi   ")
assert_eq_string(stringz.pad_left("hello", 3, ' '), "hello")
assert_eq_string(stringz.pad_right("hello", 3, ' '), "hello")

fr fr Test center function
assert_eq_string(stringz.center("hi", 6, ' '), "  hi  ")
assert_eq_string(stringz.center("test", 8, '-'), "--test--")
assert_eq_string(stringz.center("hello", 3, ' '), "hello")

fr fr Test reverse function
assert_eq_string(stringz.reverse("hello"), "olleh")
assert_eq_string(stringz.reverse("a"), "a")
assert_eq_string(stringz.reverse(""), "")
assert_eq_string(stringz.reverse("12345"), "54321")

test_start("Case Conversion Operations")

fr fr Test uppercase conversion
assert_eq_string(stringz.to_upper("hello"), "HELLO")
assert_eq_string(stringz.to_upper("HELLO"), "HELLO")
assert_eq_string(stringz.to_upper("Hello123"), "HELLO123")
assert_eq_string(stringz.to_upper(""), "")

fr fr Test lowercase conversion
assert_eq_string(stringz.to_lower("HELLO"), "hello")
assert_eq_string(stringz.to_lower("hello"), "hello")
assert_eq_string(stringz.to_lower("Hello123"), "hello123")
assert_eq_string(stringz.to_lower(""), "")

fr fr Test title case conversion
assert_eq_string(stringz.to_title_case("hello world"), "Hello World")
assert_eq_string(stringz.to_title_case("HELLO WORLD"), "Hello World")
assert_eq_string(stringz.to_title_case("hello"), "Hello")
assert_eq_string(stringz.to_title_case(""), "")

fr fr Test capitalize function
assert_eq_string(stringz.capitalize("hello"), "Hello")
assert_eq_string(stringz.capitalize("HELLO"), "HELLO")
assert_eq_string(stringz.capitalize(""), "")

fr fr Test swap_case function
assert_eq_string(stringz.swap_case("Hello World"), "hELLO wORLD")
assert_eq_string(stringz.swap_case("abc123"), "ABC123")

test_start("String Splitting and Joining")

fr fr Test split function
sus split_result []tea = stringz.split("a,b,c", ",")
assert_eq_int(len(split_result), 3)
assert_eq_string(split_result[0], "a")
assert_eq_string(split_result[1], "b")
assert_eq_string(split_result[2], "c")

sus split_empty []tea = stringz.split("hello", "")
assert_eq_int(len(split_empty), 1)
assert_eq_string(split_empty[0], "hello")

sus split_multi []tea = stringz.split("a,,b,c", ",")
assert_eq_int(len(split_multi), 4)
assert_eq_string(split_multi[0], "a")
assert_eq_string(split_multi[1], "")
assert_eq_string(split_multi[2], "b")

fr fr Test split_limit function
sus split_limited []tea = stringz.split_limit("a,b,c,d,e", ",", 3)
assert_eq_int(len(split_limited), 3)
assert_eq_string(split_limited[0], "a")
assert_eq_string(split_limited[1], "b")
assert_eq_string(split_limited[2], "c,d,e")

fr fr Test join function
sus parts []tea
parts = append(parts, "hello")
parts = append(parts, "world")
parts = append(parts, "test")
assert_eq_string(stringz.join(parts, " "), "hello world test")
assert_eq_string(stringz.join(parts, ""), "helloworldtest")
assert_eq_string(stringz.join(parts, "-"), "hello-world-test")

sus empty_parts []tea
assert_eq_string(stringz.join(empty_parts, ","), "")

fr fr Test lines function
sus line_result []tea = stringz.lines("line1\nline2\nline3")
assert_eq_int(len(line_result), 3)
assert_eq_string(line_result[0], "line1")
assert_eq_string(line_result[1], "line2")
assert_eq_string(line_result[2], "line3")

fr fr Test words and split_whitespace
sus word_result []tea = stringz.words("hello world test")
assert_eq_int(len(word_result), 3)
assert_eq_string(word_result[0], "hello")
assert_eq_string(word_result[1], "world")
assert_eq_string(word_result[2], "test")

sus whitespace_result []tea = stringz.split_whitespace("  hello\tworld\ntest  ")
assert_eq_int(len(whitespace_result), 3)
assert_eq_string(whitespace_result[0], "hello")
assert_eq_string(whitespace_result[1], "world")
assert_eq_string(whitespace_result[2], "test")

test_start("String Validation Functions")

fr fr Test is_alpha function
assert_true(stringz.is_alpha("hello"))
assert_true(stringz.is_alpha("ABC"))
assert_true(stringz.is_alpha("AbCdEf"))
assert_false(stringz.is_alpha("hello123"))
assert_false(stringz.is_alpha("hello world"))
assert_false(stringz.is_alpha(""))
assert_false(stringz.is_alpha("hello!"))

fr fr Test is_numeric/is_digit function
assert_true(stringz.is_numeric("123"))
assert_true(stringz.is_digit("123"))
assert_true(stringz.is_numeric("0"))
assert_true(stringz.is_numeric("999"))
assert_false(stringz.is_numeric("123a"))
assert_false(stringz.is_numeric("hello"))
assert_false(stringz.is_numeric(""))
assert_false(stringz.is_numeric("12.3"))

fr fr Test is_alphanumeric/is_alnum function
assert_true(stringz.is_alphanumeric("hello123"))
assert_true(stringz.is_alnum("hello123"))
assert_true(stringz.is_alphanumeric("ABC"))
assert_true(stringz.is_alphanumeric("123"))
assert_true(stringz.is_alphanumeric("Test123"))
assert_false(stringz.is_alphanumeric("hello world"))
assert_false(stringz.is_alphanumeric("hello!"))
assert_false(stringz.is_alphanumeric(""))

fr fr Test is_whitespace/is_space function
assert_true(stringz.is_whitespace("   "))
assert_true(stringz.is_space("   "))
assert_true(stringz.is_whitespace("\t"))
assert_true(stringz.is_whitespace("\n"))
assert_true(stringz.is_whitespace(" \t\n"))
assert_false(stringz.is_whitespace("hello"))
assert_false(stringz.is_whitespace(" a "))
assert_false(stringz.is_whitespace(""))

fr fr Test is_ascii function
assert_true(stringz.is_ascii("hello"))
assert_true(stringz.is_ascii("ABC123"))
assert_true(stringz.is_ascii("!@#$%"))
assert_true(stringz.is_ascii(""))

fr fr Test is_printable function
assert_true(stringz.is_printable("hello"))
assert_true(stringz.is_printable("ABC123"))
assert_true(stringz.is_printable("!@#$%"))
assert_true(stringz.is_printable(" "))
assert_true(stringz.is_printable(""))

test_start("String Encoding and Escaping")

fr fr Test UTF-8 conversion
sus utf8_bytes []normie = stringz.to_utf8("ABC")
assert_eq_int(len(utf8_bytes), 3)
assert_eq_int(utf8_bytes[0], 65) fr fr 'A'
assert_eq_int(utf8_bytes[1], 66) fr fr 'B'
assert_eq_int(utf8_bytes[2], 67) fr fr 'C'

sus bytes_to_convert []normie
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

fr fr Test HTML escaping
assert_eq_string(stringz.escape_html("<script>"), "&lt;script&gt;")
assert_eq_string(stringz.escape_html("a & b"), "a &amp; b")
assert_eq_string(stringz.escape_html("\"quoted\""), "&quot;quoted&quot;")

fr fr Test HTML unescaping
assert_eq_string(stringz.unescape_html("&lt;script&gt;"), "<script>")
assert_eq_string(stringz.unescape_html("a &amp; b"), "a & b")

fr fr Test quote escaping
assert_eq_string(stringz.escape_quotes("He said \"Hello\""), "He said \\\"Hello\\\"")
assert_eq_string(stringz.escape_quotes("It's fine"), "It\\'s fine")

test_start("String Formatting and Utilities")

fr fr Test repeat function
assert_eq_string(stringz.repeat("abc", 3), "abcabcabc")
assert_eq_string(stringz.repeat("x", 5), "xxxxx")
assert_eq_string(stringz.repeat("test", 0), "")
assert_eq_string(stringz.repeat("", 5), "")

fr fr Test format function
sus format_values []tea
format_values = append(format_values, "world")
format_values = append(format_values, "test")
assert_eq_string(stringz.format("Hello {0}! This is a {1}.", format_values), "Hello world! This is a test.")

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

assert_true(stringz.is_whitespace_char(' '))
assert_true(stringz.is_whitespace_char('\t'))
assert_true(stringz.is_whitespace_char('\n'))
assert_false(stringz.is_whitespace_char('a'))

assert_true(stringz.is_punctuation_char('!'))
assert_true(stringz.is_punctuation_char('.'))
assert_true(stringz.is_punctuation_char('?'))
assert_false(stringz.is_punctuation_char('a'))
assert_false(stringz.is_punctuation_char('1'))

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

fr fr Test edge cases for char_at
assert_true(stringz.char_at("test", -1) == '\0')
assert_true(stringz.char_at("test", 100) == '\0')

fr fr Test edge cases for find/index_of
assert_eq_int(stringz.index_of("", ""), 0)
assert_eq_int(stringz.index_of("test", "testing"), -1)
assert_eq_int(stringz.index_of("aaaa", "aa"), 0)

fr fr Test edge cases for replace
assert_eq_string(stringz.replace("", "a", "b"), "")
assert_eq_string(stringz.replace("test", "", "x"), "test")

fr fr Test edge cases for split
sus single_split []tea = stringz.split("hello", "hello")
assert_eq_int(len(single_split), 2)
assert_eq_string(single_split[0], "")
assert_eq_string(single_split[1], "")

sus empty_split []tea = stringz.split("", ",")
assert_eq_int(len(empty_split), 1)
assert_eq_string(empty_split[0], "")

fr fr Test edge cases for padding
assert_eq_string(stringz.pad_left("hello", 3, ' '), "hello")
assert_eq_string(stringz.pad_right("hello", 3, ' '), "hello")
assert_eq_string(stringz.center("hello", 3, ' '), "hello")

fr fr Test edge cases for comparison
assert_eq_int(stringz.compare("", ""), 0)
assert_eq_int(stringz.compare("", "a"), -1)
assert_eq_int(stringz.compare("a", ""), 1)

fr fr Test edge cases for validation functions
assert_false(stringz.is_alpha(""))
assert_false(stringz.is_numeric(""))
assert_false(stringz.is_alphanumeric(""))
assert_false(stringz.is_whitespace(""))

test_start("Performance and Stress Tests")

fr fr Test with longer strings
sus long_string tea = stringz.repeat("Hello World! ", 100)
assert_eq_int(stringz.len_str(long_string), 1300)
assert_true(stringz.contains(long_string, "Hello"))
assert_true(stringz.starts_with(long_string, "Hello"))
assert_true(stringz.ends_with(long_string, " "))

fr fr Test multiple operations on same string
sus test_string tea = "  Hello, Beautiful World!  "
sus trimmed tea = stringz.trim(test_string)
sus upper tea = stringz.to_upper(trimmed)
sus replaced tea = stringz.replace(upper, "BEAUTIFUL", "AMAZING")
assert_eq_string(replaced, "HELLO, AMAZING WORLD!")

fr fr Test complex splitting and joining
sus complex_string tea = "apple,banana;cherry:date|elderberry"
sus parts1 []tea = stringz.split(complex_string, ",")
sus parts2 []tea = stringz.split(parts1[1], ";")
assert_eq_string(parts2[0], "banana")
assert_eq_string(parts2[1], "cherry:date|elderberry")

print_test_summary()
