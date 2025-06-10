// Very simple test to prove linking works

#[test]
fn test_basic_math() {
    assert_eq!(2 + 2, 4)
}

#[test]
fn test_string_creation() {
    let s = "hello world".to_string();
    assert_eq!(s.len(), 11);
}
