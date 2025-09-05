#!/usr/bin/env cursed

fr fr Comprehensive string manipulation demo for CURSED
fr fr Shows all the string functions available in stdlib

yeet "stdlib::string"

slay main_character() {
    facts message = "hello world"
    
    // Basic operations
    println("=== Basic String Operations ===")
    printf("Length: {}\n", &[Value::Integer(length(&message) as i64)])
    printf("Is empty: {}\n", &[Value::Boolean(is_empty(&message))])
    printf("Repeated 3 times: {}\n", &[Value::String(repeat(&message, 3))])
    printf("Reversed: {}\n", &[Value::String(reverse(&message))])
    printf("Char at index 1: {}\n", &[match char_at(&message, 1) {
        Some(c) => Value::String(c.to_string()),
        None => Value::String("None".to_string())
    }])
    
    // Search operations
    println("\n=== Search Operations ===")
    printf("Contains 'world': {}\n", &[Value::Boolean(contains(&message, "world"))])
    printf("Starts with 'hello': {}\n", &[Value::Boolean(starts_with(&message, "hello"))])
    printf("Ends with 'world': {}\n", &[Value::Boolean(ends_with(&message, "world"))])
    printf("Find 'world': {}\n", &[match find(&message, "world") {
        Some(pos) => Value::Integer(pos as i64),
        None => Value::Integer(-1)
    }])
    
    // Case conversions
    println("\n=== Case Conversions ===")
    printf("Uppercase: {}\n", &[Value::String(to_uppercase(&message))])
    printf("Lowercase: {}\n", &[Value::String(to_lowercase(&message))])
    printf("Title case: {}\n", &[Value::String(to_title_case(&message))])
    printf("Capitalize: {}\n", &[Value::String(capitalize(&message))])
    printf("Camel case: {}\n", &[Value::String(to_camel_case(&message))])
    printf("Pascal case: {}\n", &[Value::String(to_pascal_case(&message))])
    printf("Snake case: {}\n", &[Value::String(to_snake_case(&message))])
    printf("Kebab case: {}\n", &[Value::String(to_kebab_case(&message))])
    
    // String transformations
    println("\n=== String Transformations ===")
    facts spaced_message = "   hello world   "
    printf("Original: '{}'\\n", &[Value::String(spaced_message.to_string())])
    printf("Trimmed: '{}'\\n", &[Value::String(trim(&spaced_message))])
    printf("Trim start: '{}'\\n", &[Value::String(trim_start(&spaced_message))])
    printf("Trim end: '{}'\\n", &[Value::String(trim_end(&spaced_message))])
    
    // Replace operations
    println("\n=== Replace Operations ===")
    facts test_str = "hello hello hello"
    printf("Original: {}\n", &[Value::String(test_str.to_string())])
    printf("Replace all 'hello' with 'hi': {}\n", &[Value::String(replace(&test_str, "hello", "hi"))])
    printf("Replace first 'hello' with 'hi': {}\n", &[Value::String(replace_first(&test_str, "hello", "hi"))])
    printf("Replace last 'hello' with 'hi': {}\n", &[Value::String(replace_last(&test_str, "hello", "hi"))])
    printf("Replace 2 'hello' with 'hi': {}\n", &[Value::String(replace_n(&test_str, "hello", "hi", 2))])
    printf("Count 'hello': {}\n", &[Value::Integer(count_occurrences(&test_str, "hello") as i64)])
    
    // Concatenation operations
    println("\n=== Concatenation Operations ===")
    facts strings = vec!["hello", " ", "beautiful", " ", "world"]
    printf("Concatenated: {}\n", &[Value::String(concat(&strings))])
    
    // Unicode and special characters
    println("\n=== Unicode Support ===")
    facts unicode_str = "café 🦀 русский"
    printf("Unicode string: {}\n", &[Value::String(unicode_str.to_string())])
    printf("Length (chars): {}\n", &[Value::Integer(length(&unicode_str) as i64)])
    printf("Uppercase: {}\n", &[Value::String(to_uppercase(&unicode_str))])
    printf("Lowercase: {}\n", &[Value::String(to_lowercase(&unicode_str))])
    printf("Is ASCII: {}\n", &[Value::Boolean(is_ascii(&unicode_str))])
    
    // Error handling with substring
    println("\n=== Substring Operations ===")
    facts long_str = "abcdefghijklmnop"
    match substring(&long_str, 2, 5) {
        Ok(substr) => printf("Substring (2, 5): {}\n", &[Value::String(substr)]),
        Err(e) => printf("Substring error: {}\n", &[Value::String(e.to_string())])
    }
    
    match substring_range(&long_str, 5, 10) {
        Ok(substr) => printf("Substring range (5..10): {}\n", &[Value::String(substr)]),
        Err(e) => printf("Substring range error: {}\n", &[Value::String(e.to_string())])
    }
    
    println("\nString manipulation demo completed!")
}
