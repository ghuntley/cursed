fr fr Comprehensive test suite for STRINGZ string processing module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "stringz"

slay main() {
    testz.test_start("STRINGZ Comprehensive Test Suite")
    
    fr fr ===== BASIC STRING OPERATIONS TESTS =====
    testz.test_group("Basic String Operations")
    
    fr fr Test concat_strings
    sus concat_result tea = stringz.concat_strings("Hello", "World")
    testz.assert_eq_string(concat_result, "HelloWorld", "concat_strings should join two strings")
    
    sus concat_empty1 tea = stringz.concat_strings("", "World")
    testz.assert_eq_string(concat_empty1, "World", "concat_strings should handle empty first string")
    
    sus concat_empty2 tea = stringz.concat_strings("Hello", "")
    testz.assert_eq_string(concat_empty2, "Hello", "concat_strings should handle empty second string")
    
    fr fr Test concat_three
    sus concat3_result tea = stringz.concat_three("A", "B", "C")
    testz.assert_eq_string(concat3_result, "ABC", "concat_three should join three strings")
    
    fr fr Test repeat_string
    sus repeat_basic tea = stringz.repeat_string("X", 3.0)
    testz.assert_eq_string(repeat_basic, "XXX", "repeat_string should repeat string n times")
    
    sus repeat_zero tea = stringz.repeat_string("Y", 0.0)
    testz.assert_eq_string(repeat_zero, "", "repeat_string should return empty for 0 repetitions")
    
    sus repeat_one tea = stringz.repeat_string("Z", 1.0)
    testz.assert_eq_string(repeat_one, "Z", "repeat_string should return original for 1 repetition")
    
    fr fr ===== STRING VALIDATION TESTS =====
    testz.test_group("String Validation Functions")
    
    fr fr Test is_empty_string
    sus empty_true lit = stringz.is_empty_string("")
    testz.assert_true(empty_true, "is_empty_string should return true for empty string")
    
    sus empty_false lit = stringz.is_empty_string("not empty")
    testz.assert_false(empty_false, "is_empty_string should return false for non-empty string")
    
    fr fr Test is_not_empty
    sus not_empty_true lit = stringz.is_not_empty("text")
    testz.assert_true(not_empty_true, "is_not_empty should return true for non-empty string")
    
    sus not_empty_false lit = stringz.is_not_empty("")
    testz.assert_false(not_empty_false, "is_not_empty should return false for empty string")
    
    fr fr Test strings_equal
    sus equal_true lit = stringz.strings_equal("test", "test")
    testz.assert_true(equal_true, "strings_equal should return true for identical strings")
    
    sus equal_false lit = stringz.strings_equal("test", "different")
    testz.assert_false(equal_false, "strings_equal should return false for different strings")
    
    fr fr Test strings_not_equal
    sus not_equal_true lit = stringz.strings_not_equal("alpha", "beta")
    testz.assert_true(not_equal_true, "strings_not_equal should return true for different strings")
    
    sus not_equal_false lit = stringz.strings_not_equal("same", "same")
    testz.assert_false(not_equal_false, "strings_not_equal should return false for identical strings")
    
    fr fr ===== STRING BUILDING TESTS =====
    testz.test_group("String Building Functions")
    
    fr fr Test build_string_two
    sus build2_result tea = stringz.build_string_two("part1", "part2")
    testz.assert_eq_string(build2_result, "part1part2", "build_string_two should combine two parts")
    
    fr fr Test build_string_three
    sus build3_result tea = stringz.build_string_three("A", "B", "C")
    testz.assert_eq_string(build3_result, "ABC", "build_string_three should combine three parts")
    
    fr fr Test build_string_four
    sus build4_result tea = stringz.build_string_four("1", "2", "3", "4")
    testz.assert_eq_string(build4_result, "1234", "build_string_four should combine four parts")
    
    fr fr Test surround functions
    sus quotes_result tea = stringz.surround_with_quotes("text")
    testz.assert_eq_string(quotes_result, "\"text\"", "surround_with_quotes should add quotes")
    
    sus parens_result tea = stringz.surround_with_parens("content")
    testz.assert_eq_string(parens_result, "(content)", "surround_with_parens should add parentheses")
    
    sus brackets_result tea = stringz.surround_with_brackets("item")
    testz.assert_eq_string(brackets_result, "[item]", "surround_with_brackets should add brackets")
    
    fr fr ===== FORMATTING HELPERS TESTS =====
    testz.test_group("Formatting Helper Functions")
    
    fr fr Test format_as_title
    sus title_result tea = stringz.format_as_title("My Title")
    testz.assert_eq_string(title_result, "=== My Title ===", "format_as_title should format with equals signs")
    
    fr fr Test format_as_bullet
    sus bullet_result tea = stringz.format_as_bullet("First item")
    testz.assert_eq_string(bullet_result, "• First item", "format_as_bullet should add bullet point")
    
    fr fr Test format_as_numbered
    sus numbered_result tea = stringz.format_as_numbered(1.0, "First item")
    testz.assert_eq_string(numbered_result, "1. First item", "format_as_numbered should add number prefix")
    
    fr fr Test format_key_value
    sus kv_result tea = stringz.format_key_value("name", "value")
    testz.assert_eq_string(kv_result, "name: value", "format_key_value should format as key: value")
    
    fr fr ===== STRING CHECKING TESTS =====
    testz.test_group("String Checking Functions")
    
    fr fr Test starts_with_char (simplified implementation)
    sus starts_true lit = stringz.starts_with_char("A", "A")
    testz.assert_true(starts_true, "starts_with_char should return true for matching single char")
    
    sus starts_false lit = stringz.starts_with_char("A", "B")
    testz.assert_false(starts_false, "starts_with_char should return false for non-matching char")
    
    sus starts_empty1 lit = stringz.starts_with_char("", "A")
    testz.assert_false(starts_empty1, "starts_with_char should return false for empty string")
    
    sus starts_empty2 lit = stringz.starts_with_char("A", "")
    testz.assert_false(starts_empty2, "starts_with_char should return false for empty char")
    
    fr fr Test ends_with_char (simplified implementation)
    sus ends_true lit = stringz.ends_with_char("Z", "Z")
    testz.assert_true(ends_true, "ends_with_char should return true for matching single char")
    
    sus ends_false lit = stringz.ends_with_char("Z", "Y")
    testz.assert_false(ends_false, "ends_with_char should return false for non-matching char")
    
    fr fr ===== STRING GENERATION TESTS =====
    testz.test_group("String Generation Functions")
    
    fr fr Test make_separator
    sus separator_result tea = stringz.make_separator("-", 5.0)
    testz.assert_eq_string(separator_result, "-----", "make_separator should create separator line")
    
    fr fr Test make_line
    sus line_result tea = stringz.make_line(3.0)
    testz.assert_eq_string(line_result, "---", "make_line should create dash line")
    
    fr fr Test make_equals_line
    sus equals_result tea = stringz.make_equals_line(4.0)
    testz.assert_eq_string(equals_result, "====", "make_equals_line should create equals line")
    
    fr fr Test make_space_padding
    sus padding_result tea = stringz.make_space_padding(3.0)
    testz.assert_eq_string(padding_result, "   ", "make_space_padding should create spaces")
    
    fr fr ===== SIMPLE TRANSFORMATIONS TESTS =====
    testz.test_group("Simple Transformation Functions")
    
    fr fr Test wrap_in_spaces
    sus wrapped_result tea = stringz.wrap_in_spaces("text")
    testz.assert_eq_string(wrapped_result, " text ", "wrap_in_spaces should add spaces around text")
    
    fr fr Test prepend_prefix
    sus prefix_result tea = stringz.prepend_prefix("pre_", "text")
    testz.assert_eq_string(prefix_result, "pre_text", "prepend_prefix should add prefix")
    
    fr fr Test append_suffix
    sus suffix_result tea = stringz.append_suffix("text", "_suf")
    testz.assert_eq_string(suffix_result, "text_suf", "append_suffix should add suffix")
    
    fr fr Test sandwich_string
    sus sandwich_result tea = stringz.sandwich_string("(", "middle", ")")
    testz.assert_eq_string(sandwich_result, "(middle)", "sandwich_string should wrap middle with left and right")
    
    fr fr ===== EDGE CASE TESTS =====
    testz.test_group("Edge Case Handling")
    
    fr fr Test operations with empty strings
    sus empty_concat tea = stringz.concat_strings("", "")
    testz.assert_eq_string(empty_concat, "", "concat_strings should handle both empty strings")
    
    sus empty_repeat tea = stringz.repeat_string("", 5.0)
    testz.assert_eq_string(empty_repeat, "", "repeat_string should handle empty input string")
    
    fr fr Test with very long repetition
    sus long_repeat tea = stringz.repeat_string("X", 10.0)
    testz.assert_eq_string(long_repeat, "XXXXXXXXXX", "repeat_string should handle longer repetitions")
    
    fr fr Test formatting with empty inputs
    sus empty_title tea = stringz.format_as_title("")
    testz.assert_eq_string(empty_title, "===  ===", "format_as_title should handle empty title")
    
    sus empty_bullet tea = stringz.format_as_bullet("")
    testz.assert_eq_string(empty_bullet, "• ", "format_as_bullet should handle empty item")
    
    fr fr ===== PERFORMANCE TESTS =====
    testz.test_group("Performance Validation")
    
    fr fr Test repeated string operations (stress test)
    sus performance_result tea = ""
    bestie i := 0; i < 10; i++ {
        performance_result = stringz.concat_strings(performance_result, "X")
    }
    testz.assert_eq_string(performance_result, "XXXXXXXXXX", "Performance test should build string correctly")
    
    fr fr Test complex string building
    sus complex_result tea = stringz.build_string_four(
        stringz.surround_with_parens("A"),
        " + ",
        stringz.surround_with_brackets("B"),
        " = C"
    )
    testz.assert_eq_string(complex_result, "(A) + [B] = C", "Complex string building should work")
    
    fr fr ===== CONSISTENCY TESTS =====
    testz.test_group("Consistency Validation")
    
    fr fr Test that equivalent operations produce same results
    sus method1 tea = stringz.concat_strings("Hello", "World")
    sus method2 tea = stringz.build_string_two("Hello", "World")
    testz.assert_eq_string(method1, method2, "Different concat methods should produce same result")
    
    sus method3 tea = stringz.concat_three("A", "B", "C")
    sus method4 tea = stringz.build_string_three("A", "B", "C")
    testz.assert_eq_string(method3, method4, "Different three-part methods should produce same result")
    
    fr fr Test bidirectional consistency
    sus wrap_test tea = stringz.wrap_in_spaces("test")
    sus manual_wrap tea = stringz.sandwich_string(" ", "test", " ")
    testz.assert_eq_string(wrap_test, manual_wrap, "wrap_in_spaces and sandwich_string should be equivalent")
    
    fr fr ===== BOUNDARY TESTS =====
    testz.test_group("Boundary Condition Tests")
    
    fr fr Test with very small inputs
    sus single_char tea = stringz.repeat_string("A", 1.0)
    testz.assert_eq_string(single_char, "A", "repeat_string should handle single repetition")
    
    sus zero_padding tea = stringz.make_space_padding(0.0)
    testz.assert_eq_string(zero_padding, "", "make_space_padding should handle zero padding")
    
    fr fr Test string equality edge cases
    sus self_equal lit = stringz.strings_equal("test", "test")
    testz.assert_true(self_equal, "String should equal itself")
    
    sus self_not_equal lit = stringz.strings_not_equal("test", "test")
    testz.assert_false(self_not_equal, "String should not be not-equal to itself")
    
    fr fr ===== SPECIAL CHARACTER TESTS =====
    testz.test_group("Special Character Handling")
    
    fr fr Test with special characters
    sus special_concat tea = stringz.concat_strings("Hello\n", "World!")
    testz.assert_ne_string(special_concat, "", "Should handle special characters in concatenation")
    
    sus special_quotes tea = stringz.surround_with_quotes("He said \"Hi\"")
    testz.assert_ne_string(special_quotes, "", "Should handle quotes within quotes")
    
    fr fr Test with numeric-looking strings
    sus numeric_concat tea = stringz.concat_strings("123", "456")
    testz.assert_eq_string(numeric_concat, "123456", "Should handle numeric strings as text")
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test building a complex formatted string
    sus title tea = stringz.format_as_title("Report")
    sus separator tea = stringz.make_equals_line(20.0)
    sus item1 tea = stringz.format_as_bullet("First item")
    sus item2 tea = stringz.format_as_numbered(1.0, "Numbered item")
    sus footer tea = stringz.surround_with_parens("End of report")
    
    testz.assert_ne_string(title, "", "Title formatting should work")
    testz.assert_ne_string(separator, "", "Separator generation should work")
    testz.assert_ne_string(item1, "", "Bullet formatting should work")
    testz.assert_ne_string(item2, "", "Numbered formatting should work")
    testz.assert_ne_string(footer, "", "Footer formatting should work")
    
    fr fr Test building structured output
    sus header tea = stringz.build_string_three(
        stringz.make_line(10.0),
        " HEADER ",
        stringz.make_line(10.0)
    )
    testz.assert_ne_string(header, "", "Complex header building should work")
    
    fr fr ===== VALIDATION FUNCTIONS TESTING =====
    testz.test_group("Validation Function Testing")
    
    fr fr Test that validation functions are consistent
    sus test_str tea = "validation test"
    sus is_empty lit = stringz.is_empty_string(test_str)
    sus is_not_empty lit = stringz.is_not_empty(test_str)
    
    testz.assert_false(is_empty, "Non-empty string should not be empty")
    testz.assert_true(is_not_empty, "Non-empty string should be not-empty")
    testz.assert_ne(is_empty, is_not_empty, "is_empty and is_not_empty should be opposites")
    
    fr fr ===== FINAL COMPREHENSIVE TEST =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Test a real-world string building scenario
    sus document_title tea = stringz.format_as_title("User Manual")
    sus section_header tea = stringz.format_as_bullet("Getting Started")
    sus numbered_step tea = stringz.format_as_numbered(1.0, "Install the software")
    sus code_example tea = stringz.surround_with_brackets("example code")
    sus note tea = stringz.surround_with_parens("Important note")
    
    sus full_document tea = stringz.build_string_four(
        stringz.concat_strings(document_title, "\n"),
        stringz.concat_strings(section_header, "\n"),
        stringz.concat_strings(numbered_step, "\n"),
        stringz.concat_three(code_example, " ", note)
    )
    
    testz.assert_ne_string(full_document, "", "Full document building should work")
    testz.assert_true(stringz.is_not_empty(full_document), "Full document should not be empty")
    
    fr fr Test string operation chaining
    sus chained_result tea = stringz.prepend_prefix("PREFIX_", 
        stringz.append_suffix(
            stringz.wrap_in_spaces("MIDDLE"),
            "_SUFFIX"
        )
    )
    testz.assert_ne_string(chained_result, "", "Chained string operations should work")
    
    fr fr Validate all core functions work together
    sus validation_passed lit = based
    ready stringz.is_empty_string("") != based { validation_passed = cap }
    ready stringz.is_not_empty("text") != based { validation_passed = cap }
    ready stringz.strings_equal("same", "same") != based { validation_passed = cap }
    ready stringz.concat_strings("A", "B") != "AB" { validation_passed = cap }
    ready stringz.repeat_string("X", 3.0) != "XXX" { validation_passed = cap }
    
    testz.assert_true(validation_passed, "All core string functions should work correctly")
    
    testz.print_test_summary()
}
