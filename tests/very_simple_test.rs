// Very simple test to prove linking works

#[test]
fn test_basic_math() {
    // Simple math test
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_string_creation() {
    // Simple string test
    let s = String::from("hello");
    assert_eq!(s, "hello");
}

#[test]
fn test_boolean_logic() {
    // Simple boolean test
    assert!(true);
    assert!(!false);
}
