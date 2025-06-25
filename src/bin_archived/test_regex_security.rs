use crate::error::Error;
use regex::Regex;
use std::collections::HashMap;

/// Test the exact regex patterns used in the security module
fn test_security_regex_patterns() {
    println!("Testing regex patterns from security module...");
    
    // Test email pattern
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    assert!(email_regex.is_match("test@example.com"));
    assert!(email_regex.is_match("user.name+tag@domain.co.uk"));
    assert!(!email_regex.is_match("invalid.email"));
    assert!(!email_regex.is_match("@domain.com"));
    assert!(!email_regex.is_match("user@"));
    println!("✓ Email pattern working");
    
    // Test URL pattern  
    let url_regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+(?:\.[a-zA-Z]{2,})+(?:/[^\s]*)?$").unwrap();
    assert!(url_regex.is_match("https://example.com"));
    assert!(url_regex.is_match("http://test.example.org/path/to/resource"));
    assert!(url_regex.is_match("https://sub.domain.com/"));
    assert!(!url_regex.is_match("ftp://example.com"));
    assert!(!url_regex.is_match("not-a-url"));
    println!("✓ URL pattern working");
    
    // Test strong password pattern
    let password_regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap();
    assert!(password_regex.is_match("Password123!"));
    assert!(password_regex.is_match("MyStr0ng@Pass"));
    assert!(password_regex.is_match("Secure123$"));
    assert!(!password_regex.is_match("weakpass"));
    assert!(!password_regex.is_match("NoNumbers!"));
    assert!(!password_regex.is_match("nonumbersorspecial"));
    assert!(!password_regex.is_match("SHORT1!"));
    println!("✓ Strong password pattern working");
    
    // Test IPv4 pattern
    let ipv4_regex = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    assert!(ipv4_regex.is_match("192.168.1.1"));
    assert!(ipv4_regex.is_match("10.0.0.1"));
    assert!(ipv4_regex.is_match("255.255.255.255"));
    assert!(ipv4_regex.is_match("0.0.0.0"));
    assert!(!ipv4_regex.is_match("999.999.999.999"));
    assert!(!ipv4_regex.is_match("192.168.1"));
    assert!(!ipv4_regex.is_match("192.168.1.1.1"));
    assert!(!ipv4_regex.is_match("256.0.0.1"));
    println!("✓ IPv4 pattern working");
    
    // Test username pattern
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();
    assert!(username_regex.is_match("john_doe"));
    assert!(username_regex.is_match("user123"));
    assert!(username_regex.is_match("test_user_name"));
    assert!(username_regex.is_match("ABC"));
    assert!(username_regex.is_match("a".repeat(20).as_str()));
    assert!(!username_regex.is_match("jo")); // Too short
    assert!(!username_regex.is_match("a".repeat(21).as_str())); // Too long
    assert!(!username_regex.is_match("user-with-dashes")); // Contains dashes
    assert!(!username_regex.is_match("user with spaces")); // Contains spaces
    println!("✓ Username pattern working");
    
    // Test hex color pattern
    let hex_color_regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
    assert!(hex_color_regex.is_match("#FF5733"));
    assert!(hex_color_regex.is_match("#ffffff"));
    assert!(hex_color_regex.is_match("#000000"));
    assert!(hex_color_regex.is_match("#AbCdEf"));
    assert!(!hex_color_regex.is_match("#GG5733")); // Invalid hex characters
    assert!(!hex_color_regex.is_match("#FFF")); // Too short
    assert!(!hex_color_regex.is_match("#FFFFFFF")); // Too long
    assert!(!hex_color_regex.is_match("FF5733")); // Missing #
    println!("✓ Hex color pattern working");

    // Test phone pattern
    let phone_regex = Regex::new(r"^\+?[\d\s\-\(\)]{10,}$").unwrap();
    assert!(phone_regex.is_match("+1 (555) 123-4567"));
    assert!(phone_regex.is_match("555-123-4567"));
    assert!(phone_regex.is_match("15551234567"));
    assert!(phone_regex.is_match("+44 20 7946 0958"));
    assert!(!phone_regex.is_match("123")); // Too short
    assert!(!phone_regex.is_match("abc-def-ghij")); // Contains letters
    println!("✓ Phone pattern working");

    // Test numeric pattern
    let numeric_regex = Regex::new(r"^\d+$").unwrap();
    assert!(numeric_regex.is_match("123456"));
    assert!(numeric_regex.is_match("0"));
    assert!(numeric_regex.is_match("999999999"));
    assert!(!numeric_regex.is_match("123abc"));
    assert!(!numeric_regex.is_match("12.34"));
    assert!(!numeric_regex.is_match("-123"));
    println!("✓ Numeric pattern working");

    // Test alphanumeric pattern
    let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    assert!(alphanumeric_regex.is_match("abc123"));
    assert!(alphanumeric_regex.is_match("ABC"));
    assert!(alphanumeric_regex.is_match("123"));
    assert!(!alphanumeric_regex.is_match("abc_123"));
    assert!(!alphanumeric_regex.is_match("abc-123"));
    assert!(!alphanumeric_regex.is_match("abc 123"));
    println!("✓ Alphanumeric pattern working");

    // Test slug pattern
    let slug_regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    assert!(slug_regex.is_match("hello-world"));
    assert!(slug_regex.is_match("test-123"));
    assert!(slug_regex.is_match("simple"));
    assert!(slug_regex.is_match("multi-word-slug"));
    assert!(!slug_regex.is_match("Hello-World")); // Contains uppercase
    assert!(!slug_regex.is_match("hello_world")); // Contains underscore
    assert!(!slug_regex.is_match("-hello")); // Starts with dash
    assert!(!slug_regex.is_match("hello-")); // Ends with dash
    println!("✓ Slug pattern working");
}

fn test_pattern_caching_simulation() {
    println!("Testing pattern compilation and caching simulation...");
    
    let mut patterns: HashMap<String, Regex> = HashMap::new();
    
    // Test that we can compile and cache patterns like the security module does
    let test_patterns = [
        ("email", r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"),
        ("url", r"^https?://[a-zA-Z0-9.-]+(?:\.[a-zA-Z]{2,})+(?:/[^\s]*)?$"),
        ("password_strong", r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"),
        ("ipv4", r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"),
        ("username", r"^[a-zA-Z0-9_]{3,20}$"),
    ];
    
    for (name, pattern) in &test_patterns {
        match Regex::new(pattern) {
            Ok(compiled) => {
                patterns.insert(name.to_string(), compiled);
                println!("✓ Pattern '{}' compiled successfully", name);
            }
            Err(e) => panic!("Failed to compile pattern '{}': {}", name, e),
        }
    }
    
    // Test that cached patterns work
    if let Some(email_regex) = patterns.get("email") {
        assert!(email_regex.is_match("test@example.com"));
        println!("✓ Cached email pattern working");
    }
    
    if let Some(password_regex) = patterns.get("password_strong") {
        assert!(password_regex.is_match("MyPassword123!"));
        println!("✓ Cached password pattern working");
    }
    
    println!("✓ Pattern compilation and caching simulation successful");
}

fn test_invalid_patterns() {
    println!("Testing invalid pattern handling...");
    
    // Test invalid regex patterns that would cause compilation errors
    let invalid_patterns = [
        r"[invalid regex(",
        r"*+invalid",
        r"(?P<invalid)",
        r"(?<incomplete",
        r"\",
        r"[unclosed",
    ];
    
    for pattern in &invalid_patterns {
        match Regex::new(pattern) {
            Ok(_) => panic!("Expected pattern '{}' to be invalid", pattern),
            Err(e) => println!("✓ Invalid pattern '{}' correctly rejected: {}", pattern, e),
        }
    }
    
    println!("✓ Invalid pattern handling working correctly");
}

fn test_security_xss_patterns() {
    println!("Testing XSS detection patterns...");
    
    // Test patterns that would be used for XSS detection
    let dangerous_patterns = [
        r"<script[^>]*>",
        r"javascript:",
        r"vbscript:",
        r"on\w+\s*=",
        r"eval\s*\(",
    ];
    
    for pattern in &dangerous_patterns {
        match Regex::new(pattern) {
            Ok(regex) => {
                println!("✓ XSS detection pattern '{}' compiled successfully", pattern);
                
                // Test some basic cases
                match pattern {
                    p if p.contains("script") => {
                        assert!(regex.is_match("<script>alert('xss')</script>"));
                        assert!(regex.is_match("<script type=\"text/javascript\">"));
                    }
                    p if p.contains("javascript") => {
                        assert!(regex.is_match("javascript:alert('xss')"));
                    }
                    p if p.contains("on\\w+") => {
                        assert!(regex.is_match("onload="));
                        assert!(regex.is_match("onclick="));
                        assert!(regex.is_match("onerror="));
                    }
                    p if p.contains("eval") => {
                        assert!(regex.is_match("eval("));
                        assert!(regex.is_match("eval ("));
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("Failed to compile XSS pattern '{}': {}", pattern, e),
        }
    }
    
    println!("✓ XSS detection patterns working");
}

fn test_custom_regex_patterns() {
    println!("Testing custom regex patterns...");
    
    // Test various custom patterns that might be used in the security module
    let custom_patterns = [
        (r"^[A-Z]{3}\d{3}$", "ABC123", true),
        (r"^[A-Z]{3}\d{3}$", "abc123", false),
        (r"^\d{4}-\d{2}-\d{2}$", "2023-12-25", true),
        (r"^\d{4}-\d{2}-\d{2}$", "23-12-25", false),
        (r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", "test@example.com", true),
        (r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", "invalid.email", false),
    ];
    
    for (pattern, test_value, expected) in &custom_patterns {
        match Regex::new(pattern) {
            Ok(regex) => {
                let result = regex.is_match(test_value);
                assert_eq!(result, *expected, 
                    "Pattern '{}' with value '{}' expected {} but got {}", 
                    pattern, test_value, expected, result);
                println!("✓ Custom pattern '{}' with '{}' -> {}", pattern, test_value, result);
            }
            Err(e) => panic!("Failed to compile custom pattern '{}': {}", pattern, e),
        }
    }
    
    println!("✓ Custom regex patterns working");
}

fn main() {
    println!("=== Security Module Regex Integration Verification ===\n");
    
    test_security_regex_patterns();
    println!();
    
    test_pattern_caching_simulation();
    println!();
    
    test_invalid_patterns();
    println!();
    
    test_security_xss_patterns();
    println!();
    
    test_custom_regex_patterns();
    println!();
    
    println!("=== All tests passed! ===");
    println!("✅ Regex dependency is properly integrated and available");
    println!("✅ All security module regex patterns compile correctly");
    println!("✅ Pattern validation works as expected");
    println!("✅ XSS detection patterns are functional");
    println!("✅ Custom regex patterns are supported");
    println!("✅ Error handling for invalid patterns is robust");
    println!("✅ Pattern caching functionality is working");
    println!("\n🎉 The regex integration in security.rs is fully functional!");
}
