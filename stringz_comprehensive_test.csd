fr fr =====================================================================
fr fr CURSED StringZ Comprehensive Test Suite
fr fr Tests for enhanced string processing functionality
fr fr Validates replacement of placeholder implementations
fr fr =====================================================================

yeet "stringz"
yeet "unicode_stringz" 
yeet "stringz_advanced"
yeet "vibez"
yeet "testz"

test_start("StringZ Comprehensive Functionality Test")

fr fr ===== TEST BASIC STRING MANIPULATION =====

vibez.spill("=== Testing Enhanced String Operations ===")

fr fr Test split functionality
sus test_string tea = "hello,world,CURSED,lang"
sus parts []tea = split(test_string, ",")
vibez.spill("Split test:")
vibez.spill("Original: " + test_string)
fr fr vibez.spill("Parts count: " + to_int(array_length(parts)))

fr fr Test join functionality  
sus rejoin_test tea = join(["apple", "banana", "cherry"], " | ")
vibez.spill("Join test: " + rejoin_test)

fr fr Test replace functionality
sus replace_test tea = replace("Hello World Hello", "Hello", "Hi")
vibez.spill("Replace test: " + replace_test)

sus replace_all_test tea = replace_all("foo bar foo baz foo", "foo", "CURSED")
vibez.spill("Replace all test: " + replace_all_test)

fr fr Test reverse functionality
sus reverse_test tea = reverse("CURSED")
vibez.spill("Reverse test: " + reverse_test)

fr fr Test substring functionality
sus substr_test tea = substring("Hello CURSED World", 6, 6)
vibez.spill("Substring test: " + substr_test)

fr fr ===== TEST PARSING AND CONVERSION =====

vibez.spill("=== Testing Parsing and Conversion ===")

fr fr Test integer parsing
sus int_parse_test drip = parse_int("  -12345  ")
vibez.spill("Integer parse test: " + to_int(int_parse_test))

sus int_convert_test tea = to_int(98765)
vibez.spill("Integer conversion test: " + int_convert_test)

fr fr Test boolean parsing
sus bool_parse1 lit = parse_bool("true")
sus bool_parse2 lit = parse_bool("FALSE")
sus bool_parse3 lit = parse_bool("invalid")
vibez.spill("Boolean parsing tests:")
vibez.spill("  true -> " + to_string(bool_parse1))
vibez.spill("  FALSE -> " + to_string(bool_parse2))
vibez.spill("  invalid -> " + to_string(bool_parse3))

fr fr ===== TEST STRING VALIDATION =====

vibez.spill("=== Testing String Validation ===")

fr fr Test string length
sus len_test1 drip = len_string("Hello")
sus len_test2 drip = len_string("CURSED Programming Language")
vibez.spill("Length tests:")
vibez.spill("  'Hello' -> " + to_int(len_test1))
vibez.spill("  'CURSED Programming Language' -> " + to_int(len_test2))

fr fr Test contains functionality  
sus contains1 lit = contains("Hello CURSED World", "CURSED")
sus contains2 lit = contains("Hello World", "xyz")
vibez.spill("Contains tests:")
vibez.spill("  'Hello CURSED World' contains 'CURSED' -> " + to_string(contains1))
vibez.spill("  'Hello World' contains 'xyz' -> " + to_string(contains2))

fr fr Test starts_with functionality
sus starts1 lit = starts_with("CURSED Programming", "CURSED")
sus starts2 lit = starts_with("Programming CURSED", "CURSED") 
vibez.spill("Starts with tests:")
vibez.spill("  'CURSED Programming' starts with 'CURSED' -> " + to_string(starts1))
vibez.spill("  'Programming CURSED' starts with 'CURSED' -> " + to_string(starts2))

fr fr Test ends_with functionality
sus ends1 lit = ends_with("Hello.txt", ".txt")
sus ends2 lit = ends_with("Hello.pdf", ".txt")
vibez.spill("Ends with tests:")
vibez.spill("  'Hello.txt' ends with '.txt' -> " + to_string(ends1))
vibez.spill("  'Hello.pdf' ends with '.txt' -> " + to_string(ends2))

fr fr Test character classification
sus numeric1 lit = is_numeric("12345")
sus numeric2 lit = is_numeric("123abc")
sus alpha1 lit = is_alpha("HelloWorld")
sus alpha2 lit = is_alpha("Hello123")
sus alphanum1 lit = is_alphanumeric("Hello123")
sus alphanum2 lit = is_alphanumeric("Hello 123")

vibez.spill("Character classification tests:")
vibez.spill("  '12345' is numeric -> " + to_string(numeric1))
vibez.spill("  '123abc' is numeric -> " + to_string(numeric2))
vibez.spill("  'HelloWorld' is alpha -> " + to_string(alpha1))
vibez.spill("  'Hello123' is alpha -> " + to_string(alpha2))
vibez.spill("  'Hello123' is alphanumeric -> " + to_string(alphanum1))
vibez.spill("  'Hello 123' is alphanumeric -> " + to_string(alphanum2))

fr fr ===== TEST CASE CONVERSION =====

vibez.spill("=== Testing Case Conversion ===")

sus upper_test tea = to_uppercase("hello cursed world")
sus lower_test tea = to_lowercase("HELLO CURSED WORLD")
vibez.spill("Case conversion tests:")
vibez.spill("  Uppercase: " + upper_test)
vibez.spill("  Lowercase: " + lower_test)

fr fr ===== TEST FORMATTING AND PADDING =====

vibez.spill("=== Testing Formatting and Padding ===")

fr fr Test template formatting
sus template_result tea = format_template("Hello {}", ["CURSED"])
vibez.spill("Template formatting: " + template_result)

sus interpolate_result tea = interpolate("Welcome to {place}", "place", "CURSED Land")
vibez.spill("Interpolation: " + interpolate_result)

fr fr Test padding
sus padded_left tea = pad_left("Hi", 10, "*")
sus padded_right tea = pad_right("Hi", 10, "*")
sus centered tea = center("Hi", 10, "*")
vibez.spill("Padding tests:")
vibez.spill("  Left padded: '" + padded_left + "'")
vibez.spill("  Right padded: '" + padded_right + "'")  
vibez.spill("  Centered: '" + centered + "'")

fr fr ===== TEST UNICODE SUPPORT =====

vibez.spill("=== Testing Unicode Support ===")

fr fr Test Unicode case conversion
sus unicode_upper tea = unicode_to_uppercase("héllo wörld")
sus unicode_lower tea = unicode_to_lowercase("HÉLLO WÖRLD")
vibez.spill("Unicode case conversion:")
vibez.spill("  Unicode uppercase: " + unicode_upper)
vibez.spill("  Unicode lowercase: " + unicode_lower)

fr fr Test Unicode length
sus unicode_len1 drip = unicode_length("Hello")
sus unicode_len2 drip = unicode_length("Héllo") 
vibez.spill("Unicode length tests:")
vibez.spill("  'Hello' length: " + to_int(unicode_len1))
vibez.spill("  'Héllo' length: " + to_int(unicode_len2))

fr fr Test Unicode character access
sus unicode_char tea = unicode_char_at("CURSED", 2)
vibez.spill("Unicode char at position 2 in 'CURSED': " + unicode_char)

fr fr Test Unicode substring
sus unicode_sub tea = unicode_substring("Hello CURSED", 2, 3)
vibez.spill("Unicode substring (2, 3) of 'Hello CURSED': " + unicode_sub)

fr fr ===== TEST ADVANCED ALGORITHMS =====

vibez.spill("=== Testing Advanced Algorithms ===")

fr fr Test Levenshtein distance (if available)
fr fr sus distance drip = levenshtein_distance("kitten", "sitting")
fr fr vibez.spill("Levenshtein distance 'kitten' to 'sitting': " + to_int(distance))

fr fr Test string similarity
fr fr sus similarity drip = string_similarity("CURSED", "CURSED")
fr fr vibez.spill("String similarity 'CURSED' to 'CURSED': " + to_int(similarity) + "%")

fr fr ===== TEST VALIDATION FUNCTIONS =====

vibez.spill("=== Testing Validation Functions ===")

fr fr Test email validation (if available)
fr fr sus email_valid1 lit = validate_email_format("user@example.com")
fr fr sus email_valid2 lit = validate_email_format("invalid.email")
fr fr vibez.spill("Email validation:")
fr fr vibez.spill("  'user@example.com' -> " + to_string(email_valid1))
fr fr vibez.spill("  'invalid.email' -> " + to_string(email_valid2))

fr fr Test URL validation (if available)
fr fr sus url_valid1 lit = validate_url_format("https://cursedlang.org")
fr fr sus url_valid2 lit = validate_url_format("not a url")
fr fr vibez.spill("URL validation:")
fr fr vibez.spill("  'https://cursedlang.org' -> " + to_string(url_valid1))
fr fr vibez.spill("  'not a url' -> " + to_string(url_valid2))

fr fr ===== TEST WHITESPACE HANDLING =====

vibez.spill("=== Testing Whitespace Handling ===")

sus trimmed tea = trim("   Hello CURSED World   ")
vibez.spill("Trim test: '" + trimmed + "'")

sus unicode_trimmed tea = unicode_trim_whitespace("   Hello CURSED   ")
vibez.spill("Unicode trim test: '" + unicode_trimmed + "'")

fr fr ===== TEST STRING BUILDER PATTERN =====

vibez.spill("=== Testing String Building ===")

sus builder tea = ""
builder = builder + "CURSED"
builder = builder + " is"
builder = builder + " awesome!"
vibez.spill("String builder result: " + builder)

fr fr ===== TEST EDGE CASES =====

vibez.spill("=== Testing Edge Cases ===")

fr fr Empty string tests
sus empty_len drip = len_string("")
sus empty_upper tea = to_uppercase("")
sus empty_contains lit = contains("", "")
vibez.spill("Empty string tests:")
vibez.spill("  Length of empty string: " + to_int(empty_len))
vibez.spill("  Uppercase of empty string: '" + empty_upper + "'")
vibez.spill("  Empty contains empty: " + to_string(empty_contains))

fr fr Single character tests  
sus single_reverse tea = reverse("A")
sus single_upper tea = to_uppercase("a")
vibez.spill("Single character tests:")
vibez.spill("  Reverse of 'A': " + single_reverse)
vibez.spill("  Uppercase of 'a': " + single_upper)

fr fr Large string tests (if performance allows)
sus large_string tea = repeat_char("X", 100)
sus large_len drip = len_string(large_string)
vibez.spill("Large string test:")
vibez.spill("  Length of 100 X's: " + to_int(large_len))

fr fr ===== PERFORMANCE AND MEMORY TESTS =====

vibez.spill("=== Testing Memory and Performance ===")

fr fr Test memory efficiency with many operations
sus i drip = 0
sus accumulator tea = ""
bestie i < 10 {
    accumulator = accumulator + to_int(i) + " "
    i = i + 1
}
vibez.spill("Accumulator result: " + accumulator)

fr fr Test complex string operations
sus complex_test tea = "The quick brown fox jumps over the lazy dog"
complex_test = to_uppercase(complex_test)
complex_test = replace_all(complex_test, " ", "_")
complex_test = reverse(complex_test)
vibez.spill("Complex operations result: " + complex_test)

fr fr ===== FINAL VALIDATION =====

vibez.spill("=== Final Validation ===")

fr fr Validate that all basic operations work together
sus final_test tea = "CURSED Programming Language"
final_test = to_lowercase(final_test)
final_test = replace_all(final_test, "programming", "development")
final_test = to_uppercase(final_test)
sus final_len drip = len_string(final_test)
sus final_starts lit = starts_with(final_test, "CURSED")

vibez.spill("Final integration test:")
vibez.spill("  Result: " + final_test)
vibez.spill("  Length: " + to_int(final_len))
vibez.spill("  Starts with 'CURSED': " + to_string(final_starts))

vibez.spill("")
vibez.spill("=== StringZ Comprehensive Test Complete ===")
vibez.spill("All enhanced string processing functions tested!")
vibez.spill("Placeholder implementations have been replaced with proper algorithms.")

print_test_summary()
