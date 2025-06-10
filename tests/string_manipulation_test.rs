use cursed::stdlib::string::*;

#[test]
fn test_core_operations() {
    // Length tests
    assert_eq!(length("hello"), 5);
    assert_eq!(length(""), 0);
    assert_eq!(length("café"), 4); // Unicode
    assert_eq!(length("🦀🚀🎉"), 3); // Emojis

    // Empty check
    assert!(is_empty(""));
    assert!(!is_empty("hello"));
    assert!(!is_empty(" ")); // Space is not empty

    // Concatenation
    assert_eq!(concat(&["hello", " ", "world"]), "hello world");
    assert_eq!(concat(&[]), "");
    assert_eq!(concat_owned(vec!["hello".to_string(), " world".to_string()]), "hello world");

    // Reverse
    assert_eq!(reverse("hello"), "olleh");
    assert_eq!(reverse("🦀🚀"), "🚀🦀");

    // Character operations
    assert_eq!(char_at("hello", 1), Some('e'));
    assert_eq!(char_at("café", 3), Some('é'));
    assert_eq!(char_at("🦀🚀", 1), Some('🚀'));
    assert_eq!(char_at("hello", 10), None);

    // Character and byte conversion
    assert_eq!(chars("hi"), vec!['h', 'i']);
    assert_eq!(bytes("hi"), vec![104, 105]);

    // ASCII check
    assert!(is_ascii("hello"));
    assert!(!is_ascii("café"));
}

#[test]
fn test_search_operations() {
    // Contains
    assert!(contains("hello world", "world"));
    assert!(!contains("hello", "xyz"));

    // Find operations
    assert_eq!(find("hello world", "world"), Some(6));
    assert_eq!(find("hello world", "xyz"), None);
    assert_eq!(find_last("hello hello", "hello"), Some(6));

    // Case insensitive operations
    assert!(contains_ignore_case("Hello", "HELLO"));
    assert_eq!(find_ignore_case("Hello World", "WORLD"), Some(6));

    // Replace operations
    assert_eq!(replace("hello world", "world", "universe"), "hello universe");
    assert_eq!(replace_first("hello hello", "hello", "hi"), "hi hello");
}

#[test]
fn test_string_transformations() {
    // Substring operations
    let result = substring("hello world", 0, 5).unwrap();
    assert_eq!(result, "hello");
    let result = substring_range("hello world", 6, 11).unwrap();
    assert_eq!(result, "world");
    
    // Error cases
    assert!(substring("hello", 10, 5).is_err());

    // Trim operations
    assert_eq!(trim("  hello world  "), "hello world");
    assert_eq!(trim_start("  hello world  "), "hello world  ");
    assert_eq!(trim_end("  hello world  "), "  hello world");
    assert_eq!(trim_chars("...hello...", &['.']), "hello");

    // Case conversions
    assert_eq!(to_lowercase("HELLO WORLD"), "hello world");
    assert_eq!(to_uppercase("hello world"), "HELLO WORLD");
    assert_eq!(capitalize("hello world"), "Hello world");
    assert_eq!(to_title_case("hello world"), "Hello World");

    // Case style conversions
    assert_eq!(to_camel_case("hello world"), "helloWorld");
    assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    assert_eq!(to_snake_case("hello world"), "hello_world");
    assert_eq!(to_kebab_case("hello world"), "hello-world");

    // Insert and remove
    assert_eq!(insert_at("hello", 5, " world").unwrap(), "hello world");
    assert_eq!(remove_range("hello world", 5, 6).unwrap(), "helloworld");
}

#[test]
fn test_split_and_join() {
    // Split operations
    assert_eq!(split("hello,world,foo", ","), vec!["hello", "world", "foo"]);
    assert_eq!(split_lines("hello\nworld\r\nfoo"), vec!["hello", "world", "foo"]);
    assert_eq!(split_whitespace("hello   world\t\nfoo"), vec!["hello", "world", "foo"]);

    // Partition operations
    let (before, delimiter, after) = partition("hello-world-foo", "-");
    assert_eq!(before, "hello");
    assert_eq!(delimiter, "-");
    assert_eq!(after, "world-foo");
    
    let (before, delimiter, after) = rpartition("hello-world-foo", "-");
    assert_eq!(before, "hello-world");
    assert_eq!(delimiter, "-");
    assert_eq!(after, "foo");

    // Join operations
    assert_eq!(join(&["hello", "world"], " "), "hello world");
    let owned_strings = vec!["hello".to_string(), "world".to_string()];
    assert_eq!(join_owned(&owned_strings, " "), "hello world");
    assert_eq!(join_with_separators(&["a", "b", "c"], &["-", "+"]).unwrap(), "a-b+c");

    // Chunk operations
    assert_eq!(chunk("hello", 2).unwrap(), vec!["he", "ll", "o"]);
}

#[test]
fn test_validation() {
    // Numeric validation
    assert!(is_numeric("123"));
    assert!(is_numeric("123.456"));
    assert!(!is_numeric("hello"));
    assert!(is_integer("123"));
    assert!(!is_integer("123.456"));

    // Character type validation
    assert!(is_alphabetic("hello"));
    assert!(!is_alphabetic("hello123"));
    assert!(is_alphanumeric("hello123"));
    assert!(!is_alphanumeric("hello-123"));

    // Format validation
    assert!(is_email("test@example.com"));
    assert!(!is_email("invalid-email"));
    assert!(is_url("https://example.com"));
    assert!(!is_url("not-a-url"));
    assert!(is_phone_number("(555) 123-4567"));
    assert!(!is_phone_number("not-a-phone"));

    // Balance validation
    assert!(has_balanced_parentheses("(hello (world))"));
    assert!(!has_balanced_parentheses("(hello (world)"));
    assert!(has_balanced_brackets("[hello [world]]"));
    assert!(!has_balanced_brackets("[hello [world]"));

    // Palindrome check
    assert!(is_palindrome("racecar"));
    assert!(!is_palindrome("hello"));
}

#[test]
fn test_formatting() {
    // Padding operations
    assert_eq!(pad_left("hello", 10, ' '), "     hello");
    assert_eq!(pad_right("hello", 10, ' '), "hello     ");
    assert_eq!(center("hello", 11, '*'), "***hello***");

    // Truncation
    assert_eq!(truncate("hello world", 5, false), "hello");
    assert_eq!(truncate("hello world", 8, true), "hello...");

    // Text wrapping
    let wrapped = wrap_text("hello world foo bar", 10).unwrap();
    assert_eq!(wrapped, vec!["hello", "world foo", "bar"]);

    // Repetition
    assert_eq!(repeat("abc", 3), "abcabcabc");
    assert_eq!(repeat("", 5), "");
}

#[test]
fn test_unicode_support() {
    // Unicode character counting
    assert_eq!(length("🦀🚀🎉"), 3);
    assert_eq!(char_at("café", 3), Some('é'));
    assert_eq!(char_at("🦀🚀", 1), Some('🚀'));

    // Unicode transformations
    assert_eq!(reverse("🦀🚀"), "🚀🦀");
    assert_eq!(to_uppercase("café"), "CAFÉ");
    assert_eq!(center("🦀", 5, '*'), "**🦀**");

    // Unicode substring
    assert_eq!(substring("café", 0, 3).unwrap(), "caf");
    assert_eq!(substring("🦀🚀🎉", 1, 2).unwrap(), "🚀🎉");
}

#[test]
fn test_edge_cases() {
    // Empty strings
    assert_eq!(length(""), 0);
    assert!(is_empty(""));
    assert_eq!(reverse(""), "");
    assert_eq!(trim(""), "");

    // Large repetitions
    let result = repeat("a", 1000);
    assert_eq!(result.len(), 1000);
    assert!(result.chars().all(|c| c == 'a'));

    // Complex splitting
    assert_eq!(split("", ","), vec![""]);
    assert_eq!(split("a,,b", ","), vec!["a", "", "b"]);

    // Index boundary tests
    assert!(char_at("hello", 100).is_none());
    assert!(substring("hello", 10, 5).is_err());
}

#[test]
fn test_performance_basic() {
    // Test with reasonably sized strings to ensure basic performance
    let large_string = "a".repeat(10000);
    
    // These should complete quickly
    assert_eq!(length(&large_string), 10000);
    assert!(contains(&large_string, "aaa"));
    assert!(!is_empty(&large_string));
    
    let reversed = reverse(&large_string);
    assert_eq!(length(&reversed), 10000);
    
    // Substring operations
    let sub = substring(&large_string, 0, 100).unwrap();
    assert_eq!(length(&sub), 100);
}

#[test]
fn test_error_handling() {
    // Index out of bounds
    match substring("hello", 10, 5) {
        Err(StringError::IndexOutOfBounds { index, length }) => {
            assert_eq!(index, 10);
            assert_eq!(length, 5);
        }
        _ => panic!("Expected IndexOutOfBounds error"),
    }

    // Invalid range
    match substring_range("hello", 3, 1) {
        Err(StringError::InvalidRange { start, end, length }) => {
            assert_eq!(start, 3);
            assert_eq!(end, 1);
            assert_eq!(length, 5);
        }
        _ => panic!("Expected InvalidRange error"),
    }

    // Invalid UTF-8 testing
    let invalid_bytes = vec![0xff, 0xfe];
    match from_utf8(&invalid_bytes) {
        Err(StringError::InvalidUtf8 { position: _ }) => {
            // Expected
        }
        _ => panic!("Expected InvalidUtf8 error"),
    }
}
