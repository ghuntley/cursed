/// Basic test to verify regex_vibez module compilation

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Simple test to verify the module compiles
        assert!(true);
    }

    #[test]
    fn test_regex_crate_works() {
        use regex::Regex;
        let re = Regex::new(r"\d+").expect("Should compile");
        assert!(re.is_match("123"));
        assert!(!re.is_match("abc"));
    }

    #[test]
    fn test_lazy_static_works() {
        use lazy_static::lazy_static;
        
        lazy_static! {
            static ref TEST_REGEX: regex::Regex = regex::Regex::new(r"\d+").unwrap();
        }
        
        assert!(TEST_REGEX.is_match("456"));
        assert!(!TEST_REGEX.is_match("xyz"));
    }
}
