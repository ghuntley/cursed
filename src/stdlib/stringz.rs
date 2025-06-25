/// StringZ - Tea manipulation functions with Gen Z flair 🔥
/// 
/// This module provides essential string manipulation functions using CURSED language
/// conventions and Gen Z naming. All functions work with `tea` (strings) and provide
/// comprehensive text processing capabilities for the modern developer.
/// 
/// # Why StringZ matters:
/// - Essential for text processing in any application
/// - Bridges the gap between system strings and CURSED tea types
/// - Provides Unicode-aware operations for global applications
/// - Optimized for performance while maintaining ease of use

// use crate::stdlib::string::{self, StringError, StringResult};
use crate::error::CursedError;
use std::collections::HashMap;

/// CursedError type for StringZ operations
pub type StringzError = StringError;

/// Result type for StringZ operations  
pub type StringzResult<T> = StringResult<T>;

// ================================
// CORE TEA MANIPULATION FUNCTIONS
// ================================

/// Returns the length of a tea (string) in Unicode code points
/// 
/// # Examples
/// ```cursed
/// facts len = string_length("hello") // 5
/// facts emoji_len = string_length("🔥💯") // 2
/// ```
pub fn string_length(s: &str) -> usize {
    string::length(s)
/// Checks if a tea is empty (no cap)
/// 
/// # Examples
/// ```cursed
/// facts empty = is_empty_tea("") // true  
/// facts not_empty = is_empty_tea("vibes") // false
/// ```
pub fn is_empty_tea(s: &str) -> bool {
    string::is_empty(s)
/// Concatenates two teas together (merge the vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = concat_tea("hello", " world") // "hello world"
/// ```
pub fn concat_tea(s1: &str, s2: &str) -> String {
    string::concat(s1, s2)
/// Repeats a tea n times (spam those vibes)
/// 
/// # Examples
/// ```cursed
/// facts spam = repeat_tea("yo ", 3) // "yo yo yo "
/// ```
pub fn repeat_tea(s: &str, n: usize) -> String {
    string::repeat(s, n)
/// Reverses a tea (flip the script)
/// 
/// # Examples
/// ```cursed
/// facts flipped = reverse_tea("hello") // "olleh"
/// ```
pub fn reverse_tea(s: &str) -> String {
    string::reverse(s)
/// Gets character at specific index (peek the char)
/// 
/// # Examples
/// ```cursed
/// facts ch = char_at_index("hello", 1) // Some('e')
/// ```
pub fn char_at_index(s: &str, index: usize) -> Option<char> {
    string::char_at(s, index)
// ================================
// SEARCH AND REPLACE FUNCTIONS
// ================================

/// Checks if tea contains substring (search the vibes)
/// 
/// # Examples
/// ```cursed
/// facts found = contains_tea("hello world", "world") // true
/// ```
pub fn contains_tea(s: &str, substr: &str) -> bool {
    string::contains(s, substr)
/// Checks if tea starts with prefix (check the opening)
/// 
/// # Examples
/// ```cursed
/// facts starts = starts_with_tea("hello", "he") // true
/// ```
pub fn starts_with_tea(s: &str, prefix: &str) -> bool {
    string::starts_with(s, prefix)
/// Checks if tea ends with suffix (check the closing)
/// 
/// # Examples
/// ```cursed
/// facts ends = ends_with_tea("hello", "lo") // true
/// ```
pub fn ends_with_tea(s: &str, suffix: &str) -> bool {
    string::ends_with(s, suffix)
/// Finds first occurrence of substring (locate the vibes)
/// 
/// # Examples
/// ```cursed
/// facts pos = find_tea("hello world", "world") // Some(6)
/// ```
pub fn find_tea(s: &str, substr: &str) -> Option<usize> {
    string::find(s, substr)
/// Finds last occurrence of substring (locate from the back)
/// 
/// # Examples
/// ```cursed
/// facts pos = find_last_tea("hello hello", "hello") // Some(6)
/// ```
pub fn find_last_tea(s: &str, substr: &str) -> Option<usize> {
    string::find_last(s, substr)
/// Replaces all occurrences of substring (swap the vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = replace_all_tea("hello world", "l", "x") // "hexxo worxd"
/// ```
pub fn replace_all_tea(s: &str, old: &str, new: &str) -> String {
    string::replace(s, old, new)
/// Replaces first occurrence of substring (swap once)
/// 
/// # Examples
/// ```cursed
/// facts result = replace_first_tea("hello hello", "hello", "hi") // "hi hello"
/// ```
pub fn replace_first_tea(s: &str, old: &str, new: &str) -> String {
    string::replace_first(s, old, new)
/// Counts occurrences of substring (count the vibes)
/// 
/// # Examples
/// ```cursed
/// facts count = count_tea("hello hello hello", "hello") // 3
/// ```
pub fn count_tea(s: &str, substr: &str) -> usize {
    string::count_occurrences(s, substr)
// ================================
// TRANSFORMATION FUNCTIONS
// ================================

/// Extracts substring (slice the tea)
/// 
/// # Examples
/// ```cursed
/// facts slice = substring_tea("hello world", 0, 5) // "hello"
/// ```
pub fn substring_tea(s: &str, start: usize, end: usize) -> StringzResult<String> {
    string::substring(s, start, end)
/// Trims whitespace from both ends (clean the edges)
/// 
/// # Examples
/// ```cursed
/// facts clean = trim_tea("  hello  ") // "hello"
/// ```
pub fn trim_tea(s: &str) -> String {
    string::trim(s)
/// Trims whitespace from start (clean the front)
/// 
/// # Examples
/// ```cursed
/// facts clean = trim_start_tea("  hello  ") // "hello  "
/// ```
pub fn trim_start_tea(s: &str) -> String {
    string::trim_start(s)
/// Trims whitespace from end (clean the back)
/// 
/// # Examples
/// ```cursed
/// facts clean = trim_end_tea("  hello  ") // "  hello"
/// ```
pub fn trim_end_tea(s: &str) -> String {
    string::trim_end(s)
/// Converts tea to lowercase (make it chill)
/// 
/// # Examples
/// ```cursed
/// facts lower = to_lowercase_tea("HELLO") // "hello"
/// ```
pub fn to_lowercase_tea(s: &str) -> String {
    string::to_lowercase(s)
/// Converts tea to uppercase (make it loud)
/// 
/// # Examples
/// ```cursed
/// facts upper = to_uppercase_tea("hello") // "HELLO"
/// ```
pub fn to_uppercase_tea(s: &str) -> String {
    string::to_uppercase(s)
/// Converts tea to title case (make it proper)
/// 
/// # Examples
/// ```cursed
/// facts title = to_title_case_tea("hello world") // "Hello World"
/// ```
pub fn to_title_case_tea(s: &str) -> String {
    string::to_title_case(s)
/// Converts tea to camelCase (make it camel)
/// 
/// # Examples
/// ```cursed
/// facts camel = to_camel_case_tea("hello world") // "helloWorld"
/// ```
pub fn to_camel_case_tea(s: &str) -> String {
    string::to_camel_case(s)
/// Converts tea to PascalCase (make it pascal)
/// 
/// # Examples
/// ```cursed
/// facts pascal = to_pascal_case_tea("hello world") // "HelloWorld"
/// ```
pub fn to_pascal_case_tea(s: &str) -> String {
    string::to_pascal_case(s)
/// Converts tea to snake_case (make it snake)
/// 
/// # Examples
/// ```cursed
/// facts snake = to_snake_case_tea("Hello World") // "hello_world"
/// ```
pub fn to_snake_case_tea(s: &str) -> String {
    string::to_snake_case(s)
/// Converts tea to kebab-case (make it kebab)
/// 
/// # Examples
/// ```cursed
/// facts kebab = to_kebab_case_tea("Hello World") // "hello-world"
/// ```
pub fn to_kebab_case_tea(s: &str) -> String {
    string::to_kebab_case(s)
/// Capitalizes first letter (make it proper start)
/// 
/// # Examples
/// ```cursed
/// facts cap = capitalize_tea("hello") // "Hello"
/// ```
pub fn capitalize_tea(s: &str) -> String {
    string::capitalize(s)
// ================================
// SPLITTING AND JOINING FUNCTIONS
// ================================

/// Splits tea by delimiter (break it down)
/// 
/// # Examples
/// ```cursed
/// facts parts = split_tea("a,b,c", ",") // ["a", "b", "c"]
/// ```
pub fn split_tea(s: &str, delimiter: &str) -> Vec<String> {
    string::split(s, delimiter)
/// Splits tea by delimiter with limit (controlled break)
/// 
/// # Examples
/// ```cursed
/// facts parts = split_tea_n("a,b,c,d", ",", 2) // ["a", "b,c,d"]
/// ```
pub fn split_tea_n(s: &str, delimiter: &str, n: usize) -> Vec<String> {
    string::split_n(s, delimiter, n)
/// Splits tea by lines (break by lines)
/// 
/// # Examples
/// ```cursed
/// facts lines = split_lines_tea("line1\nline2") // ["line1", "line2"]
/// ```
pub fn split_lines_tea(s: &str) -> Vec<String> {
    string::split_lines(s)
/// Splits tea by whitespace (break by spaces)
/// 
/// # Examples
/// ```cursed
/// facts words = split_whitespace_tea("hello  world") // ["hello", "world"]
/// ```
pub fn split_whitespace_tea(s: &str) -> Vec<String> {
    string::split_whitespace(s)
/// Joins tea slices with separator (merge the vibes)
/// 
/// # Examples
/// ```cursed
/// facts joined = join_tea(&["a", "b", "c"], ",") // "a,b,c"
/// ```
pub fn join_tea(parts: &[&str], separator: &str) -> String {
    string::join(parts, separator)
/// Joins owned tea slices with separator (merge owned vibes)
/// 
/// # Examples
/// ```cursed
/// facts owned_parts = vec!["a".to_string(), "b".to_string()];
/// facts joined = join_owned_tea(&owned_parts, ",") // "a,b"
/// ```
pub fn join_owned_tea(parts: &[String], separator: &str) -> String {
    string::join_owned(parts, separator)
// ================================
// VALIDATION FUNCTIONS
// ================================

/// Checks if tea is numeric (all digits vibes)
/// 
/// # Examples
/// ```cursed
/// facts numeric = is_numeric_tea("123") // true
/// facts not_numeric = is_numeric_tea("abc") // false
/// ```
pub fn is_numeric_tea(s: &str) -> bool {
    string::is_numeric(s)
/// Checks if tea is integer (whole number vibes)
/// 
/// # Examples
/// ```cursed
/// facts integer = is_integer_tea("123") // true
/// facts not_integer = is_integer_tea("12.3") // false
/// ```
pub fn is_integer_tea(s: &str) -> bool {
    string::is_integer(s)
/// Checks if tea is alphabetic (letters only vibes)
/// 
/// # Examples
/// ```cursed
/// facts alpha = is_alphabetic_tea("hello") // true
/// facts not_alpha = is_alphabetic_tea("hello123") // false
/// ```
pub fn is_alphabetic_tea(s: &str) -> bool {
    string::is_alphabetic(s)
/// Checks if tea is alphanumeric (letters and digits vibes)
/// 
/// # Examples
/// ```cursed
/// facts alphanum = is_alphanumeric_tea("hello123") // true
/// facts not_alphanum = is_alphanumeric_tea("hello@123") // false
/// ```
pub fn is_alphanumeric_tea(s: &str) -> bool {
    string::is_alphanumeric(s)
/// Checks if tea is whitespace only (empty vibes)
/// 
/// # Examples
/// ```cursed
/// facts whitespace = is_whitespace_tea("   ") // true
/// facts not_whitespace = is_whitespace_tea("hello") // false
/// ```
pub fn is_whitespace_tea(s: &str) -> bool {
    string::is_whitespace(s)
/// Checks if tea is uppercase (loud vibes)
/// 
/// # Examples
/// ```cursed
/// facts upper = is_uppercase_tea("HELLO") // true
/// facts not_upper = is_uppercase_tea("Hello") // false
/// ```
pub fn is_uppercase_tea(s: &str) -> bool {
    string::is_uppercase(s)
/// Checks if tea is lowercase (quiet vibes)
/// 
/// # Examples
/// ```cursed
/// facts lower = is_lowercase_tea("hello") // true  
/// facts not_lower = is_lowercase_tea("Hello") // false
/// ```
pub fn is_lowercase_tea(s: &str) -> bool {
    string::is_lowercase(s)
/// Checks if tea is valid email (email vibes)
/// 
/// # Examples
/// ```cursed
/// facts email = is_email_tea("user@example.com") // true
/// facts not_email = is_email_tea("invalid-email") // false
/// ```
pub fn is_email_tea(s: &str) -> bool {
    string::is_email(s)
/// Checks if tea is valid URL (link vibes)
/// 
/// # Examples
/// ```cursed
/// facts url = is_url_tea("https://example.com") // true
/// facts not_url = is_url_tea("not-a-url") // false
/// ```
pub fn is_url_tea(s: &str) -> bool {
    string::is_url(s)
/// Checks if tea is palindrome (same forward and backward vibes)
/// 
/// # Examples
/// ```cursed
/// facts palindrome = is_palindrome_tea("racecar") // true
/// facts not_palindrome = is_palindrome_tea("hello") // false
/// ```
pub fn is_palindrome_tea(s: &str) -> bool {
    string::is_palindrome(s)
// ================================
// FORMATTING FUNCTIONS
// ================================

/// Pads tea to left with spaces (add left vibes)
/// 
/// # Examples
/// ```cursed
/// facts padded = pad_left_tea("hello", 10) // "     hello"
/// ```
pub fn pad_left_tea(s: &str, width: usize) -> String {
    string::pad_left(s, width)
/// Pads tea to right with spaces (add right vibes)
/// 
/// # Examples
/// ```cursed
/// facts padded = pad_right_tea("hello", 10) // "hello     "
/// ```
pub fn pad_right_tea(s: &str, width: usize) -> String {
    string::pad_right(s, width)
/// Centers tea with spaces (center the vibes)
/// 
/// # Examples
/// ```cursed
/// facts centered = center_tea("hello", 10) // "  hello   "
/// ```
pub fn center_tea(s: &str, width: usize) -> String {
    string::center(s, width)
/// Truncates tea to max length (cut the vibes)
/// 
/// # Examples
/// ```cursed
/// facts truncated = truncate_tea("hello world", 5) // "hello"
/// ```
pub fn truncate_tea(s: &str, max_len: usize) -> String {
    string::truncate(s, max_len)
/// Wraps tea to specified width (wrap the vibes)
/// 
/// # Examples
/// ```cursed
/// facts wrapped = wrap_text_tea("hello world", 5) // "hello\nworld"
/// ```
pub fn wrap_text_tea(s: &str, width: usize) -> String {
    string::wrap_text(s, width)
/// Escapes HTML characters (make it web safe)
/// 
/// # Examples
/// ```cursed
/// facts escaped = escape_html_tea("<p>hello</p>") // "&lt;p&gt;hello&lt;/p&gt;"
/// ```
pub fn escape_html_tea(s: &str) -> String {
    string::escape_html(s)
/// Escapes JSON characters (make it JSON safe)
/// 
/// # Examples
/// ```cursed
/// facts escaped = escape_json_tea("hello \"world\"") // "hello \\\"world\\\""
/// ```
pub fn escape_json_tea(s: &str) -> String {
    string::escape_json(s)
// ================================
// ENHANCED TEA FUNCTIONS
// ================================

/// Converts tea to bytes (byte vibes)
/// 
/// # Examples
/// ```cursed
/// facts bytes = tea_to_bytes("hello") // [104, 101, 108, 108, 111]
/// ```
pub fn tea_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
/// Converts bytes to tea (string vibes)
/// 
/// # Examples
/// ```cursed
/// facts tea = bytes_to_tea(&[104, 101, 108, 108, 111]) // Ok("hello")
/// ```
pub fn bytes_to_tea(bytes: &[u8]) -> StringzResult<String> {
    string::from_utf8(bytes)
/// Gets tea characters as vec (char collection vibes)
/// 
/// # Examples
/// ```cursed
/// facts chars = tea_chars("hello") // ['h', 'e', 'l', 'l', 'o']
/// ```
pub fn tea_chars(s: &str) -> Vec<char> {
    string::chars(s)
/// Checks if tea is ASCII only (basic vibes)
/// 
/// # Examples
/// ```cursed
/// facts ascii = is_ascii_tea("hello") // true
/// facts not_ascii = is_ascii_tea("héllo") // false
/// ```
pub fn is_ascii_tea(s: &str) -> bool {
    string::is_ascii(s)
/// Inserts tea at position (inject vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = insert_at_tea("hello", 2, "XX") // Ok("heXXllo")
/// ```
pub fn insert_at_tea(s: &str, pos: usize, text: &str) -> StringzResult<String> {
    string::insert_at(s, pos, text)
/// Removes range from tea (cut out vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = remove_range_tea("hello", 1, 3) // Ok("hlo")
/// ```
pub fn remove_range_tea(s: &str, start: usize, end: usize) -> StringzResult<String> {
    string::remove_range(s, start, end)
// ================================
// UTILITY FUNCTIONS
// ================================

/// Partitions tea by first delimiter (split once)
/// 
/// # Examples
/// ```cursed
/// facts (before, after) = partition_tea("a=b=c", "=") // ("a", "b=c")
/// ```
pub fn partition_tea(s: &str, delimiter: &str) -> (String, String) {
    string::partition(s, delimiter)
/// Partitions tea by last delimiter (split from end)
/// 
/// # Examples
/// ```cursed
/// facts (before, after) = rpartition_tea("a=b=c", "=") // ("a=b", "c")
/// ```
pub fn rpartition_tea(s: &str, delimiter: &str) -> (String, String) {
    string::rpartition(s, delimiter)
/// Chunks tea into pieces (break into chunks)
/// 
/// # Examples
/// ```cursed
/// facts chunks = chunk_tea("hello", 2) // Ok(["he", "ll", "o"])
/// ```
pub fn chunk_tea(s: &str, size: usize) -> StringzResult<Vec<String>> {
    string::chunk(s, size)
/// Adds line numbers to tea (number the lines)
/// 
/// # Examples
/// ```cursed
/// facts numbered = add_line_numbers_tea("hello\nworld") // "1: hello\n2: world"
/// ```
pub fn add_line_numbers_tea(s: &str) -> String {
    string::add_line_numbers(s)
/// Indents all lines in tea (add spacing)
/// 
/// # Examples
/// ```cursed
/// facts indented = indent_lines_tea("hello\nworld", "  ") // "  hello\n  world"
/// ```
pub fn indent_lines_tea(s: &str, indent: &str) -> String {
    string::indent_lines(s, indent)
/// Checks if parentheses are balanced (valid structure)
/// 
/// # Examples
/// ```cursed
/// facts balanced = has_balanced_parentheses_tea("(hello)") // true
/// facts not_balanced = has_balanced_parentheses_tea("(hello") // false
/// ```
pub fn has_balanced_parentheses_tea(s: &str) -> bool {
    string::has_balanced_parentheses(s)
/// Checks if brackets are balanced (valid structure)
/// 
/// # Examples
/// ```cursed
/// facts balanced = has_balanced_brackets_tea("[hello]") // true
/// facts not_balanced = has_balanced_brackets_tea("[hello") // false
/// ```
pub fn has_balanced_brackets_tea(s: &str) -> bool {
    string::has_balanced_brackets(s)
/// Module initialization function
pub fn init_stringz() -> StringzResult<()> {
    // Initialize any global state for StringZ module
    Ok(())
/// Get module statistics and information
pub fn get_stringz_stats() -> HashMap<String, String> {
    let mut stats = HashMap::new();
    stats.insert("version".to_string(), "1.0.0".to_string());
    stats.insert("functions".to_string(), "50+".to_string());
    stats.insert("features".to_string(), "Unicode-aware, Gen Z naming".to_string());
    stats
