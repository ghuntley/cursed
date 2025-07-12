// CURSED Stringz Module - Comprehensive Test Suite
// Tests all advanced string operations in pure CURSED

yeet "testz"
yeet "stringz"

// ================================
// Test Core String Operations
// ================================

slay test_contains() {
    test_start("Contains function")
    
    assert_true(stringz.Contains("hello world", "world"))
    assert_true(stringz.Contains("hello world", "hello"))
    assert_true(stringz.Contains("hello world", ""))
    assert_false(stringz.Contains("hello world", "WORLD"))
    assert_false(stringz.Contains("hello world", "xyz"))
    assert_false(stringz.Contains("", "test"))
}

slay test_starts_ends_with() {
    test_start("StartsWith and EndsWith functions")
    
    assert_true(stringz.StartsWith("hello world", "hello"))
    assert_true(stringz.StartsWith("hello world", ""))
    assert_false(stringz.StartsWith("hello world", "world"))
    assert_false(stringz.StartsWith("hello world", "Hello"))
    
    assert_true(stringz.EndsWith("hello world", "world"))
    assert_true(stringz.EndsWith("hello world", ""))
    assert_false(stringz.EndsWith("hello world", "hello"))
    assert_false(stringz.EndsWith("hello world", "World"))
}

slay test_indexof() {
    test_start("IndexOf and LastIndexOf functions")
    
    assert_eq_int(stringz.IndexOf("hello world", "world"), 6)
    assert_eq_int(stringz.IndexOf("hello world", "hello"), 0)
    assert_eq_int(stringz.IndexOf("hello world", ""), 0)
    assert_eq_int(stringz.IndexOf("hello world", "xyz"), -1)
    
    assert_eq_int(stringz.LastIndexOf("hello world world", "world"), 12)
    assert_eq_int(stringz.LastIndexOf("hello world", "hello"), 0)
    assert_eq_int(stringz.LastIndexOf("hello world", "xyz"), -1)
}

// ================================
// Test String Splitting and Joining
// ================================

slay test_split() {
    test_start("Split function")
    
    // Basic split test - simplified for parser
    sus test_str tea = "hello,world,test"
    sus parts [tea] = stringz.Split(test_str, ",")
    
    // Test individual elements 
    assert_eq_string(parts[0], "hello")
    assert_eq_string(parts[1], "world")
    assert_eq_string(parts[2], "test")
    
    sus single [tea] = stringz.Split("hello", ",")
    assert_eq_string(single[0], "hello")
    
    sus empty [tea] = stringz.Split("", ",")
    assert_eq_string(empty[0], "")
}

slay test_join() {
    test_start("Join function")
    
    sus parts [tea] = ["hello", "world", "test"]
    sus joined tea = stringz.Join(parts, ",")
    assert_eq_string(joined, "hello,world,test")
    
    sus single [tea] = ["hello"]
    sus single_joined tea = stringz.Join(single, ",")
    assert_eq_string(single_joined, "hello")
    
    sus empty [tea] = []
    sus empty_joined tea = stringz.Join(empty, ",")
    assert_eq_string(empty_joined, "")
}

// ================================
// Test String Replacement
// ================================

slay test_replace() {
    test_start("Replace function")
    
    sus result1 tea = stringz.Replace("hello world world", "world", "CURSED", 1)
    assert_eq_string(result1, "hello CURSED world")
    
    sus result2 tea = stringz.Replace("hello world world", "world", "CURSED", 2)
    assert_eq_string(result2, "hello CURSED CURSED")
    
    sus result_all tea = stringz.Replace("hello world world", "world", "CURSED", -1)
    assert_eq_string(result_all, "hello CURSED CURSED")
    
    sus no_match tea = stringz.Replace("hello world", "xyz", "CURSED", 1)
    assert_eq_string(no_match, "hello world")
}

slay test_replace_all() {
    test_start("ReplaceAll function")
    
    sus result tea = stringz.ReplaceAll("hello world world", "world", "CURSED")
    assert_eq_string(result, "hello CURSED CURSED")
    
    sus no_match tea = stringz.ReplaceAll("hello world", "xyz", "CURSED")
    assert_eq_string(no_match, "hello world")
}

// ================================
// Test String Trimming
// ================================

slay test_trim() {
    test_start("Trim functions")
    
    sus trimmed tea = stringz.Trim("  hello world  ", "")
    assert_eq_string(trimmed, "hello world")
    
    sus custom_trim tea = stringz.Trim("***hello world***", "*")
    assert_eq_string(custom_trim, "hello world")
    
    sus left_trim tea = stringz.TrimLeft("  hello world  ", "")
    assert_eq_string(left_trim, "hello world  ")
    
    sus right_trim tea = stringz.TrimRight("  hello world  ", "")
    assert_eq_string(right_trim, "  hello world")
}

// ================================
// Test String Case Conversion
// ================================

slay test_case_conversion() {
    test_start("Case conversion functions")
    
    assert_eq_string(stringz.ToLower("HELLO WORLD"), "hello world")
    assert_eq_string(stringz.ToLower("Hello World"), "hello world")
    assert_eq_string(stringz.ToLower("hello world"), "hello world")
    
    assert_eq_string(stringz.ToUpper("hello world"), "HELLO WORLD")
    assert_eq_string(stringz.ToUpper("Hello World"), "HELLO WORLD")
    assert_eq_string(stringz.ToUpper("HELLO WORLD"), "HELLO WORLD")
    
    assert_eq_string(stringz.Capitalize("hello world"), "Hello world")
    assert_eq_string(stringz.Capitalize("HELLO WORLD"), "Hello world")
}

// ================================
// Test String Validation
// ================================

slay test_validation() {
    test_start("String validation functions")
    
    assert_true(stringz.IsEmpty(""))
    assert_false(stringz.IsEmpty("hello"))
    assert_false(stringz.IsEmpty(" "))
    
    assert_true(stringz.IsNumeric("12345"))
    assert_true(stringz.IsNumeric("0"))
    assert_false(stringz.IsNumeric("123a"))
    assert_false(stringz.IsNumeric(""))
    
    assert_true(stringz.IsAlpha("hello"))
    assert_true(stringz.IsAlpha("HELLO"))
    assert_false(stringz.IsAlpha("hello123"))
    assert_false(stringz.IsAlpha(""))
    
    assert_true(stringz.IsAlphanumeric("hello123"))
    assert_true(stringz.IsAlphanumeric("HELLO"))
    assert_true(stringz.IsAlphanumeric("12345"))
    assert_false(stringz.IsAlphanumeric("hello@world"))
}

// ================================
// Test String Utilities
// ================================

slay test_utilities() {
    test_start("String utility functions")
    
    assert_eq_int(stringz.Len("hello world"), 11)
    assert_eq_int(stringz.Len(""), 0)
    assert_eq_int(stringz.Len("🚀"), 1)
    
    assert_eq_string(stringz.Repeat("ho", 3), "hohoho")
    assert_eq_string(stringz.Repeat("test", 0), "")
    assert_eq_string(stringz.Repeat("a", 1), "a")
    
    assert_eq_string(stringz.Reverse("hello"), "olleh")
    assert_eq_string(stringz.Reverse(""), "")
    assert_eq_string(stringz.Reverse("a"), "a")
    
    assert_eq_string(stringz.Substring("hello world", 0, 5), "hello")
    assert_eq_string(stringz.Substring("hello world", 6, 5), "world")
    assert_eq_string(stringz.Substring("hello world", 0, 0), "")
    
    assert_eq_string(stringz.CharAt("hello", 0), "h")
    assert_eq_string(stringz.CharAt("hello", 4), "o")
    
    assert_eq_int(stringz.Count("hello world world", "world"), 2)
    assert_eq_int(stringz.Count("hello world", "xyz"), 0)
    assert_eq_int(stringz.Count("aaaa", "aa"), 2)
}

// ================================
// Test String Comparison
// ================================

slay test_comparison() {
    test_start("String comparison functions")
    
    assert_eq_int(stringz.Compare("apple", "banana"), -1)
    assert_eq_int(stringz.Compare("banana", "apple"), 1)
    assert_eq_int(stringz.Compare("apple", "apple"), 0)
    
    assert_true(stringz.Equals("hello", "hello"))
    assert_false(stringz.Equals("hello", "Hello"))
    assert_false(stringz.Equals("hello", "world"))
    
    assert_true(stringz.EqualsIgnoreCase("hello", "HELLO"))
    assert_true(stringz.EqualsIgnoreCase("Hello", "hello"))
    assert_false(stringz.EqualsIgnoreCase("hello", "world"))
}

// ================================
// Test String Conversion
// ================================

slay test_conversion() {
    test_start("String conversion functions")
    
    assert_eq_int(stringz.ToInt("123"), 123)
    assert_eq_int(stringz.ToInt("-456"), -456)
    assert_eq_int(stringz.ToInt("0"), 0)
    
    assert_eq_string(stringz.FromInt(123), "123")
    assert_eq_string(stringz.FromInt(-456), "-456")
    assert_eq_string(stringz.FromInt(0), "0")
    
    assert_eq_string(stringz.FromBool(based), "based")
    assert_eq_string(stringz.FromBool(cap), "cap")
    
    assert_true(stringz.ToBool("based"))
    assert_false(stringz.ToBool("cap"))
}

// ================================
// Test Advanced Operations
// ================================

slay test_advanced() {
    test_start("Advanced string operations")
    
    assert_eq_string(stringz.PadLeft("hello", 10, " "), "     hello")
    assert_eq_string(stringz.PadLeft("hello", 3, " "), "hello")
    
    assert_eq_string(stringz.PadRight("hello", 10, " "), "hello     ")
    assert_eq_string(stringz.PadRight("hello", 3, " "), "hello")
    
    // Test hash consistency
    sus hash1 normie = stringz.Hash("hello")
    sus hash2 normie = stringz.Hash("hello")
    assert_eq_int(hash1, hash2)
    
    // Test Levenshtein distance
    assert_eq_int(stringz.LevenshteinDistance("hello", "hello"), 0)
    assert_eq_int(stringz.LevenshteinDistance("hello", "hallo"), 1)
    assert_eq_int(stringz.LevenshteinDistance("", "hello"), 5)
}

// ================================
// Run All Tests
// ================================

slay main() {
    vibez.spill("Running CURSED Stringz Module Tests...")
    vibez.spill("=====================================")
    
    test_contains()
    test_starts_ends_with()
    test_indexof()
    test_split()
    test_join()
    test_replace()
    test_replace_all()
    test_trim()
    test_case_conversion()
    test_validation()
    test_utilities()
    test_comparison()
    test_conversion()
    test_advanced()
    
    print_test_summary()
}

// Execute main function
main()
