// Test character functions - simple working version

#[cfg(test)]
mod tests {
    use cursed::stdlib;

    #[test]
    fn test_is_uppercase() {
        // Test basic uppercase validation
        assert!(stdlib::is_uppercase("A"));
        assert!(!stdlib::is_uppercase("a"));
        assert!(!stdlib::is_uppercase("9"));
    }

    #[test]
    fn test_is_lowercase() {
        // Test basic lowercase validation
        assert!(!stdlib::is_lowercase("A"));
        assert!(stdlib::is_lowercase("a"));
        assert!(!stdlib::is_lowercase("9"));
    }

    #[test]
    fn test_to_uppercase() {
        // Test uppercase transformation
        assert_eq!(stdlib::to_uppercase("a"), "A");
        assert_eq!(stdlib::to_uppercase("hello"), "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        // Test lowercase transformation
        assert_eq!(stdlib::to_lowercase("A"), "a");
        assert_eq!(stdlib::to_lowercase("HELLO"), "hello");
    }

    #[test]
    fn dummy_character_functions_test() {
        assert!(true);
    }
}
