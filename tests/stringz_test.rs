/// Tests for StringZ module - Tea manipulation functions with Gen Z flair
/// 
/// This test suite validates the comprehensive string manipulation functionality
/// provided by the StringZ module, ensuring all Gen Z naming conventions work
/// correctly with CURSED language semantics.

#[cfg(test)]
mod tests {
    use cursed::stdlib::stringz::*;

    #[test]
    fn test_core_tea_operations() {
        // Test basic string operations
        assert_eq!(string_length("hello"), 5);
        assert_eq!(string_length("🔥💯"), 2); // Unicode support
        
        assert!(is_empty_tea(""));
        assert!(!is_empty_tea("vibes"));
        
        assert_eq!(concat_tea("hello", " world"), "hello world");
        assert_eq!(repeat_tea("yo ", 3), "yo yo yo ");
        assert_eq!(reverse_tea("hello"), "olleh");
        
        assert_eq!(char_at_index("hello", 1), Some('e'));
        assert_eq!(char_at_index("hello", 10), None);
    }

    #[test]
    fn test_search_and_replace_tea() {
        // Test search functions
        assert!(contains_tea("hello world", "world"));
        assert!(!contains_tea("hello world", "foo"));
        
        assert!(starts_with_tea("hello", "he"));
        assert!(!starts_with_tea("hello", "lo"));
        
        assert!(ends_with_tea("hello", "lo"));
        assert!(!ends_with_tea("hello", "he"));
        
        assert_eq!(find_tea("hello world", "world"), Some(6));
        assert_eq!(find_tea("hello world", "foo"), None);
        
        assert_eq!(find_last_tea("hello hello", "hello"), Some(6));
        
        // Test replace functions
        assert_eq!(replace_all_tea("hello world", "l", "x"), "hexxo worxd");
        assert_eq!(replace_first_tea("hello hello", "hello", "hi"), "hi hello");
        
        assert_eq!(count_tea("hello hello hello", "hello"), 3);
        assert_eq!(count_tea("hello world", "foo"), 0);
    }

    #[test]
    fn test_tea_transformations() {
        // Test substring extraction
        assert_eq!(substring_tea("hello world", 0, 5).unwrap(), "hello");
        assert_eq!(substring_tea("hello world", 6, 11).unwrap(), "world");
        assert!(substring_tea("hello", 0, 10).is_err()); // Out of bounds
        
        // Test trimming
        assert_eq!(trim_tea("  hello  "), "hello");
        assert_eq!(trim_start_tea("  hello  "), "hello  ");
        assert_eq!(trim_end_tea("  hello  "), "  hello");
        
        // Test case transformations
        assert_eq!(to_lowercase_tea("HELLO"), "hello");
        assert_eq!(to_uppercase_tea("hello"), "HELLO");
        assert_eq!(to_title_case_tea("hello world"), "Hello World");
        assert_eq!(to_camel_case_tea("hello world"), "helloWorld");
        assert_eq!(to_pascal_case_tea("hello world"), "HelloWorld");
        assert_eq!(to_snake_case_tea("Hello World"), "hello_world");
        assert_eq!(to_kebab_case_tea("Hello World"), "hello-world");
        assert_eq!(capitalize_tea("hello"), "Hello");
    }

    #[test]
    fn test_tea_splitting_and_joining() {
        // Test splitting
        assert_eq!(split_tea("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(split_tea_n("a,b,c,d", ",", 2), vec!["a", "b,c,d"]);
        assert_eq!(split_lines_tea("line1\nline2\nline3"), vec!["line1", "line2", "line3"]);
        assert_eq!(split_whitespace_tea("hello  world\ttest"), vec!["hello", "world", "test"]);
        
        // Test joining
        assert_eq!(join_tea(&["a", "b", "c"], ","), "a,b,c");
        assert_eq!(join_tea(&["hello", "world"], " "), "hello world");
        
        let owned_parts = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(join_owned_tea(&owned_parts, "-"), "a-b-c");
    }

    #[test]
    fn test_tea_validation() {
        // Test numeric validation
        assert!(is_numeric_tea("123"));
        assert!(is_numeric_tea("123.45"));
        assert!(!is_numeric_tea("abc"));
        assert!(!is_numeric_tea("12a3"));
        
        assert!(is_integer_tea("123"));
        assert!(!is_integer_tea("12.3"));
        assert!(!is_integer_tea("abc"));
        
        // Test character type validation
        assert!(is_alphabetic_tea("hello"));
        assert!(is_alphabetic_tea("HELLO"));
        assert!(!is_alphabetic_tea("hello123"));
        assert!(!is_alphabetic_tea("hello!"));
        
        assert!(is_alphanumeric_tea("hello123"));
        assert!(is_alphanumeric_tea("ABC123"));
        assert!(!is_alphanumeric_tea("hello@123"));
        
        assert!(is_whitespace_tea("   "));
        assert!(is_whitespace_tea("\t\n "));
        assert!(!is_whitespace_tea("hello"));
        
        assert!(is_uppercase_tea("HELLO"));
        assert!(!is_uppercase_tea("Hello"));
        assert!(!is_uppercase_tea("hello"));
        
        assert!(is_lowercase_tea("hello"));
        assert!(!is_lowercase_tea("Hello"));
        assert!(!is_lowercase_tea("HELLO"));
        
        // Test format validation
        assert!(is_email_tea("user@example.com"));
        assert!(is_email_tea("test.email+tag@domain.co.uk"));
        assert!(!is_email_tea("invalid-email"));
        assert!(!is_email_tea("@domain.com"));
        
        assert!(is_url_tea("https://example.com"));
        assert!(is_url_tea("http://localhost:8080/path"));
        assert!(!is_url_tea("not-a-url"));
        assert!(!is_url_tea("ftp://"));
        
        assert!(is_palindrome_tea("racecar"));
        assert!(is_palindrome_tea("A man a plan a canal Panama"));
        assert!(!is_palindrome_tea("hello"));
    }

    #[test]
    fn test_tea_formatting() {
        // Test padding
        assert_eq!(pad_left_tea("hello", 10), "     hello");
        assert_eq!(pad_left_tea("hello", 3), "hello"); // No padding needed
        
        assert_eq!(pad_right_tea("hello", 10), "hello     ");
        assert_eq!(pad_right_tea("hello", 3), "hello"); // No padding needed
        
        assert_eq!(center_tea("hello", 11), "   hello   ");
        assert_eq!(center_tea("hello", 10), "  hello   "); // Uneven padding
        
        // Test truncation
        assert_eq!(truncate_tea("hello world", 5), "hello");
        assert_eq!(truncate_tea("hello", 10), "hello"); // No truncation needed
        
        // Test wrapping
        assert_eq!(wrap_text_tea("hello world", 5), "hello\nworld");
        assert_eq!(wrap_text_tea("verylongword", 5), "veryl\nongwo\nrd");
        
        // Test escaping
        assert_eq!(escape_html_tea("<p>hello</p>"), "&lt;p&gt;hello&lt;/p&gt;");
        assert_eq!(escape_html_tea("hello & goodbye"), "hello &amp; goodbye");
        
        assert_eq!(escape_json_tea("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_json_tea("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_enhanced_tea_operations() {
        // Test byte conversion
        let bytes = tea_to_bytes("hello");
        assert_eq!(bytes, vec![104, 101, 108, 108, 111]);
        
        let tea = bytes_to_tea(&[104, 101, 108, 108, 111]).unwrap();
        assert_eq!(tea, "hello");
        
        // Test invalid UTF-8
        assert!(bytes_to_tea(&[0xFF, 0xFE]).is_err());
        
        // Test character extraction
        let chars = tea_chars("hello");
        assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
        
        let unicode_chars = tea_chars("🔥💯");
        assert_eq!(unicode_chars, vec!['🔥', '💯']);
        
        // Test ASCII checking
        assert!(is_ascii_tea("hello"));
        assert!(is_ascii_tea("123!@#"));
        assert!(!is_ascii_tea("héllo"));
        assert!(!is_ascii_tea("🔥"));
        
        // Test insertion and removal
        assert_eq!(insert_at_tea("hello", 2, "XX").unwrap(), "heXXllo");
        assert_eq!(remove_range_tea("hello", 1, 3).unwrap(), "hlo");
        
        // Test bounds checking
        assert!(insert_at_tea("hello", 10, "XX").is_err());
        assert!(remove_range_tea("hello", 10, 15).is_err());
    }

    #[test]
    fn test_tea_utility_functions() {
        // Test partitioning
        let (before, after) = partition_tea("a=b=c", "=");
        assert_eq!(before, "a");
        assert_eq!(after, "b=c");
        
        let (before, after) = rpartition_tea("a=b=c", "=");
        assert_eq!(before, "a=b");
        assert_eq!(after, "c");
        
        // Test chunking
        let chunks = chunk_tea("hello", 2).unwrap();
        assert_eq!(chunks, vec!["he", "ll", "o"]);
        
        let chunks = chunk_tea("hello", 10).unwrap();
        assert_eq!(chunks, vec!["hello"]);
        
        assert!(chunk_tea("hello", 0).is_err()); // Invalid chunk size
        
        // Test line numbers
        let numbered = add_line_numbers_tea("hello\nworld\ntest");
        assert_eq!(numbered, "1: hello\n2: world\n3: test");
        
        // Test indentation
        let indented = indent_lines_tea("hello\nworld", "  ");
        assert_eq!(indented, "  hello\n  world");
        
        // Test balanced checking
        assert!(has_balanced_parentheses_tea("(hello (world))"));
        assert!(!has_balanced_parentheses_tea("(hello (world)"));
        assert!(!has_balanced_parentheses_tea("(hello) world)"));
        
        assert!(has_balanced_brackets_tea("[hello [world]]"));
        assert!(!has_balanced_brackets_tea("[hello [world]"));
        assert!(!has_balanced_brackets_tea("[hello] world]"));
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_stringz().is_ok());
        
        // Test module statistics
        let stats = get_stringz_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("functions"));
        assert!(stats.contains_key("features"));
        
        assert_eq!(stats.get("version").unwrap(), "1.0.0");
        assert!(stats.get("functions").unwrap().contains("50+"));
        assert!(stats.get("features").unwrap().contains("Gen Z"));
    }

    #[test]
    fn test_unicode_support() {
        // Test with various Unicode characters
        let emoji_tea = "🔥💯⚡";
        assert_eq!(string_length(emoji_tea), 3);
        assert!(!is_ascii_tea(emoji_tea));
        
        let unicode_tea = "héllö wörld";
        assert!(contains_tea(unicode_tea, "ö"));
        assert_eq!(to_uppercase_tea(unicode_tea), "HÉLLÖ WÖRLD");
        
        // Test with mixed Unicode and ASCII
        let mixed = "Hello 🌍 World";
        assert_eq!(string_length(mixed), 13);
        assert!(contains_tea(mixed, "🌍"));
        
        let chars = tea_chars(mixed);
        assert_eq!(chars[6], '🌍');
    }

    #[test]
    fn test_edge_cases() {
        // Test empty strings
        assert_eq!(string_length(""), 0);
        assert!(is_empty_tea(""));
        assert_eq!(reverse_tea(""), "");
        assert_eq!(split_tea("", ","), vec![""]);
        
        // Test single character
        assert_eq!(string_length("a"), 1);
        assert_eq!(reverse_tea("a"), "a");
        assert_eq!(char_at_index("a", 0), Some('a'));
        
        // Test very long strings
        let long_string = "a".repeat(10000);
        assert_eq!(string_length(&long_string), 10000);
        assert!(contains_tea(&long_string, "aa"));
        
        // Test special characters
        let special = "!@#$%^&*()_+-={}[]|\\:;\"'<>?,./";
        assert!(!is_alphabetic_tea(special));
        assert!(!is_alphanumeric_tea(special));
        assert!(!is_numeric_tea(special));
    }
}
