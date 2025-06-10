use cursed::stdlib::string::*;

#[test]
fn test_core_operations() {// Length and emptiness
    assert_eq!(length(hello), 5)
    assert_eq!(length(café"), 4)
    assert_eq!(length(🦀🚀), 2)
    assert!(is_empty()"hello)
    // Concatenation
    assert_eq!(concat(&[hello , world]), ", hello world ".to_string(), " world.to_string()]), ")
    // Repetition and reversal
    assert_eq!(repeat(abc, 3), abcabcabc)
    assert_eq!(reverse("hello ")
    assert_eq!(reverse("🦀🚀), ")
    // Character operations
    assert_eq!(char_at(hello , 1), Some('e'é')
    assert_eq!(chars("hi 'h', '])
    assert_eq!(bytes(hi"), vec![104, 10]
fn test_search_operations() {// Basic search
    assert!(contains(hello worldworld)
    assert!(starts_with(hello worldhello)
    assert!(ends_with(")
    assert_eq!(find(hello worldworld"), Some(6)
    assert_eq!(find_last("), 2)
    assert!(contains_ignore_case("Hello WorldWORLD)
    // Replace operations
    assert_eq!(replace(hello worldworld , universe), ")
    assert_eq!(replace_first(hello hellohello", "hi hello ")
    assert_eq!(replace_last(hello hellohello"hi), "hello hi ", "hi, 2), ")}
#[test]
fn test_transformations() {// Substrings
    assert_eq!(substring(hello world, 0, 5).unwrap(), hello)
    assert_eq!(substring_range("hello world ")
    assert!(substring("hello, 10, 5).is_err()
    // Trimming
    assert_eq!(trim(hello world), hello world "  hello world), "hello world  "), "  hello world)
    assert_eq!(trim_chars(", &['.' ']), hello ")
    // Case conversions
    assert_eq!(to_lowercase(HELLO WORLD), hello world "), "HELLO WORLD)
    assert_eq!(to_title_case("), Hello World")
    assert_eq!(to_camel_case("helloWorld ")
    assert_eq!(to_pascal_case(hello world"HelloWorld)
    assert_eq!(to_snake_case("hello world ")
    assert_eq!(to_kebab_case("hello world), ")
    // Insertion and removal
    assert_eq!(insert_at(hello, 5,  world).unwrap(), "hello world ", 5, 6).unwrap(), "helloworld)}
#[test]
fn test_splitting_and_joining() {// Basic splitting
    assert_eq!(split(hello,world,foo,), vec![helloworld "fo]d])
    assert_eq!(split_lines("), vec![helloworld", "hello  world\tfoo "), vec![helloworld"fo]), vec![helloworld", "hello ", 2).unwrap(), vec![hell"])
    // Partitioning
    assert_eq!(partition(hello-world-foo-), 
              (hello ".to_string(), -"world-foo.to_string()
    assert_eq!(rpartition("hello-world-foo-".to_string(), "-.to_string(), ".to_string()
    // Joining
    assert_eq!(join(&[helloworld],), "hello world ".to_string(), "world.to_string()], "), hello world ")
    assert_eq!(join_with_separators(&["c "], &[-+"a-b+c)}
#[test]
fn test_validation() {// Numeric validation
    assert!(is_numeric(123)
    assert!(is_numeric(123.456")
    assert!(is_numeric("abc ")
    assert!(is_integer(123"123.456)
    // Character type validation
    assert!(is_alphabetic(hello)
    assert!(!is_alphabetic(hello123")
    assert!(is_alphanumeric("hello!")
    assert!(is_whitespace("hello)
    // Case validation
    assert!(is_uppercase(HELLO)
    assert!(is_lowercase(hello")
    assert!(is_title_case("hello world ")
    // Format validation
    assert!(is_hex(1234abcd)
    assert!(is_hex(0x1234abcd)
    assert!(!is_hex(")
    assert!(is_email(user@example.com")
    assert!(!is_email("https://example.com ")
    assert!(!is_url(example.com"1234567890)
    assert!(is_phone_number("+1 (555) 123-4567"{[()]})
    assert!(!has_balanced_brackets(([)]")
    assert!(is_palindrome("A man a plan a canal Panama ")
    assert!(!is_palindrome(hello' '),    hello ")
    assert_eq!(pad_right(hello' '), "hello)
    assert_eq!(center(", 9, ' ")
    // Truncation
    assert_eq!(truncate(hello world, 5, false), hello ")
    assert_eq!(truncate(hello world"hello...)
    // Text wrapping
    let wrapped = wrap_text(hello world foo bar , 10).unwrap()
    assert_eq!(wrapped, vec![helloworld foo", "]
fn test_unicode_support() {// Unicode length and operations
    assert_eq!(length(café), 4)
    assert_eq!(length(🦀🚀🎉"), 3)
    assert_eq!(char_at(café'é')
    assert_eq!(char_at("🦀🚀, 1), Some('🚀")
    assert_eq!(reverse("🦀🚀), ")
    assert_eq!(to_uppercase(café"), "CAFÉ"), café' '),   café")
    assert_eq!(center(🦀'*'), "**🦀**)
    assert_eq!(substring(", 0, 3).unwrap(), caf ")
    assert_eq!(chunk("🦀🚀🎉]"])}
#[test]
fn test_error_handling() {// Index out of bounds
    assert!(substring(hello , 10, 5).is_err()
    assert!(substring_range(hello, 0, 10).is_err()
    assert!(insert_at(", 10,  world").is_err()
    // Invalid ranges
    assert!(substring_range(hello, 3, 1).is_err()
    assert!(remove_range(hello "hello ", 0).is_err()
    // UTF-8 validation
    let invalid_utf8 = &[0xff, 0xfe]
    assert!(from_utf8(invalid_utf8).is_err()
    assert_eq!(from_utf8_lossy(invalid_utf8), ��)}

#[test]
fn test_edge_cases() {// Empty strings
    assert_eq!(length(), 0)
    assert_eq!(concat(&[]),)
    assert_eq!(repeat(, 5),)")
    assert_eq!(split(",), vec![], "),)
    // Single characters
    assert_eq!(length(a), 1)
    assert_eq!(reverse(a"), "a ", 0, 1).unwrap(), a"a)
    // Large operations
    let large_string = a .repeat(1000)
    assert_eq!(length(&large_string), 1000)
    assert_eq!(repeat(ab", 500), ")
    assert_eq!(to_lowercase("),)
    assert!(has_balanced_parentheses(")}