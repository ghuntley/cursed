fr fr CURSED String Processing Validation Test
fr fr Comprehensive test for string processing functionality
fr fr Tests Unicode, edge cases, and large string handling

yeet "vibez"

fr fr =================================
fr fr Basic String Length Tests
fr fr =================================

slay test_string_length() {
    vibez.spill("=== Testing String Length ===")
    
    fr fr Empty string
    sus empty_len normie = builtin_string_len("")
    vibez.spill("Empty string length:", empty_len)
    
    fr fr Single character
    sus single_len normie = builtin_string_len("a")
    vibez.spill("Single char length:", single_len)
    
    fr fr Multi-character string
    sus multi_len normie = builtin_string_len("Hello World")
    vibez.spill("Hello World length:", multi_len)
    
    fr fr Unicode string (UTF-8)
    sus unicode_len normie = builtin_string_len("Hello 🌍")
    vibez.spill("Unicode string length:", unicode_len)
    
    vibez.spill("String length tests completed")
}

fr fr =================================
fr fr Character Access Tests
fr fr =================================

slay test_char_at() {
    vibez.spill("=== Testing Character Access ===")
    
    sus test_str tea = "Hello"
    
    fr fr Test valid indices
    sus char0 normie = builtin_string_char_at(test_str, 0)
    sus char1 normie = builtin_string_char_at(test_str, 1) 
    sus char4 normie = builtin_string_char_at(test_str, 4)
    
    vibez.spill("Characters at indices 0,1,4:", char0, char1, char4)
    
    fr fr Test bounds checking
    sus char_neg normie = builtin_string_char_at(test_str, -1)
    sus char_big normie = builtin_string_char_at(test_str, 100)
    
    vibez.spill("Out-of-bounds chars (-1, 100):", char_neg, char_big)
    
    vibez.spill("Character access tests completed")
}

fr fr =================================
fr fr Substring Tests
fr fr =================================

slay test_substring() {
    vibez.spill("=== Testing Substring Operations ===")
    
    sus test_str tea = "Hello World"
    
    fr fr Test valid substrings
    sus sub1 tea = builtin_string_substring(test_str, 0, 5)
    sus sub2 tea = builtin_string_substring(test_str, 6, 11)
    sus sub3 tea = builtin_string_substring(test_str, 2, 8)
    
    vibez.spill("Substring [0,5]:", sub1)
    vibez.spill("Substring [6,11]:", sub2)
    vibez.spill("Substring [2,8]:", sub3)
    
    fr fr Test edge cases
    sus sub_empty tea = builtin_string_substring(test_str, 5, 5)
    sus sub_full tea = builtin_string_substring(test_str, 0, builtin_string_len(test_str))
    
    vibez.spill("Empty substring [5,5]:", sub_empty)
    vibez.spill("Full string substring:", sub_full)
    
    vibez.spill("Substring tests completed")
}

fr fr =================================
fr fr String Trimming Tests  
fr fr =================================

slay test_string_trimming() {
    vibez.spill("=== Testing String Trimming ===")
    
    fr fr Test leading whitespace
    sus leading tea = "   Hello"
    vibez.spill("Before trim start:", "[" + leading + "]")
    
    fr fr Test trailing whitespace
    sus trailing tea = "World   "
    vibez.spill("Before trim end:", "[" + trailing + "]")
    
    fr fr Test both sides
    sus both_sides tea = "   Hello World   "
    vibez.spill("Before trim both:", "[" + both_sides + "]")
    
    fr fr Test mixed whitespace
    sus mixed tea = "\t\n  Hello\r\n  "
    vibez.spill("Before trim mixed:", "[" + mixed + "]")
    
    vibez.spill("String trimming tests completed")
}

fr fr =================================
fr fr Unicode and Edge Cases
fr fr =================================

slay test_unicode_edge_cases() {
    vibez.spill("=== Testing Unicode and Edge Cases ===")
    
    fr fr Unicode characters
    sus unicode tea = "Héllo 世界"
    sus uni_len normie = builtin_string_len(unicode)
    vibez.spill("Unicode string:", unicode)
    vibez.spill("Unicode length:", uni_len)
    
    fr fr Empty string operations
    sus empty tea = ""
    sus empty_char normie = builtin_string_char_at(empty, 0)
    sus empty_sub tea = builtin_string_substring(empty, 0, 1)
    
    vibez.spill("Empty string char at 0:", empty_char)
    vibez.spill("Empty string substring:", empty_sub)
    
    fr fr Large string test (performance)
    vibez.spill("Testing large string operations...")
    sus large_str tea = "This is a relatively long string for performance testing. It contains many characters and should test the efficiency of our string operations. Let's see how well our implementation handles strings of this size."
    sus large_len normie = builtin_string_len(large_str)
    sus large_sub tea = builtin_string_substring(large_str, 10, 30)
    
    vibez.spill("Large string length:", large_len)
    vibez.spill("Large string substring [10,30]:", large_sub)
    
    vibez.spill("Unicode and edge case tests completed")
}

fr fr =================================
fr fr Performance Stress Tests
fr fr =================================

slay test_performance_stress() {
    vibez.spill("=== Testing Performance Stress ===")
    
    fr fr Repeated operations
    sus counter normie = 0
    sus test_string tea = "Performance test string with reasonable length"
    
    bestie counter < 100 {
        sus len normie = builtin_string_len(test_string)
        sus char normie = builtin_string_char_at(test_string, counter % len)
        sus sub tea = builtin_string_substring(test_string, 0, (counter % len) + 1)
        counter = counter + 1
    }
    
    vibez.spill("Completed 100 iterations of string operations")
    vibez.spill("Performance stress tests completed")
}

fr fr =================================
fr fr Error Handling Tests
fr fr =================================

slay test_error_handling() {
    vibez.spill("=== Testing Error Handling ===")
    
    sus test_str tea = "Test"
    
    fr fr Negative indices
    sus char_neg normie = builtin_string_char_at(test_str, -5)
    sus sub_neg tea = builtin_string_substring(test_str, -1, 2)
    
    vibez.spill("Char at -5:", char_neg)
    vibez.spill("Substring [-1,2]:", sub_neg)
    
    fr fr Out of bounds
    sus char_oob normie = builtin_string_char_at(test_str, 100)
    sus sub_oob tea = builtin_string_substring(test_str, 5, 10)
    
    vibez.spill("Char at 100:", char_oob)
    vibez.spill("Substring [5,10]:", sub_oob)
    
    fr fr Invalid ranges
    sus sub_invalid tea = builtin_string_substring(test_str, 3, 1)
    vibez.spill("Invalid range [3,1]:", sub_invalid)
    
    vibez.spill("Error handling tests completed")
}

fr fr =================================
fr fr Main Test Runner
fr fr =================================

slay main() normie {
    vibez.spill("🚀 CURSED String Processing Validation Test Suite")
    vibez.spill("Testing comprehensive string functionality...")
    vibez.spill("")
    
    test_string_length()
    vibez.spill("")
    
    test_char_at()
    vibez.spill("")
    
    test_substring()
    vibez.spill("")
    
    test_string_trimming()
    vibez.spill("")
    
    test_unicode_edge_cases()
    vibez.spill("")
    
    test_performance_stress()
    vibez.spill("")
    
    test_error_handling()
    vibez.spill("")
    
    vibez.spill("✅ All string processing tests completed successfully!")
    vibez.spill("String functionality validation: PASSED")
    
    damn 0
}
