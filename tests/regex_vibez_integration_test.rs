/// Integration tests for the RegexVibez module
/// Testing complete workflows and real-world scenarios

#[cfg(test)]
mod tests {
    use cursed::stdlib::regex_vibez::*;
    use cursed::stdlib::regex_vibez::common::CommonPatterns;
    use std::collections::HashMap;

    #[test]
    fn test_email_validation_workflow() {
        let email_pattern = CommonPatterns::email();
        
        // Valid emails
        let valid_emails = vec![
            "user@example.com",
            "test.email+tag@domain.org",
            "user.name123@sub.domain.co.uk",
            "first.last@company-name.com",
        ];

        for email in valid_emails {
            assert!(email_pattern.match_string(email), "Should match valid email: {}", email);
        }

        // Invalid emails
        let invalid_emails = vec![
            "invalid-email",
            "@domain.com",
            "user@",
            "user.domain.com",
            "user @domain.com",
            "user@domain",
        ];

        for email in invalid_emails {
            assert!(!email_pattern.match_string(email), "Should not match invalid email: {}", email);
        }
    }

    #[test]
    fn test_log_parsing_workflow() {
        let log_pattern = VibePattern::compile(
            r"(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)"
        ).expect("Should compile log pattern");

        let log_lines = vec![
            "2023-12-25 14:30:45 [INFO] Application started successfully",
            "2023-12-25 14:30:46 [ERROR] Database connection failed",
            "2023-12-25 14:30:47 [WARN] High memory usage detected",
            "2023-12-25 14:30:48 [DEBUG] Processing user request",
        ];

        let groups = log_pattern.vibe_groups();
        
        for line in log_lines {
            assert!(log_pattern.match_string(line), "Should match log line: {}", line);
            
            let parsed = groups.find_groups_string(line);
            assert!(parsed.contains_key("timestamp"), "Should extract timestamp from: {}", line);
            assert!(parsed.contains_key("level"), "Should extract level from: {}", line);
            assert!(parsed.contains_key("message"), "Should extract message from: {}", line);
            
            let timestamp = parsed.get("timestamp").unwrap();
            let level = parsed.get("level").unwrap();
            let message = parsed.get("message").unwrap();
            
            assert!(timestamp.len() == 19, "Timestamp should be 19 chars: {}", timestamp);
            assert!(["INFO", "ERROR", "WARN", "DEBUG"].contains(&level.as_str()), "Invalid level: {}", level);
            assert!(!message.is_empty(), "Message should not be empty");
        }
    }

    #[test]
    fn test_url_extraction_workflow() {
        let text = r#"
        Visit our website at https://example.com for more info.
        You can also check out http://blog.example.org/posts/123
        or our API docs at https://api.example.com/v1/docs.
        Invalid URLs like ftp://example.com won't match.
        "#;

        let url_pattern = CommonPatterns::url();
        let urls = url_pattern.find_all_string(text, -1);
        
        assert_eq!(urls.len(), 3, "Should find 3 valid URLs");
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"http://blog.example.org/posts/123".to_string()));
        assert!(urls.contains(&"https://api.example.com/v1/docs".to_string()));
        assert!(!urls.iter().any(|url| url.contains("ftp://")));
    }

    #[test]
    fn test_phone_number_normalization_workflow() {
        let phone_pattern = CommonPatterns::phone();
        
        let phone_numbers = vec![
            "(555) 123-4567",
            "555-123-4567",
            "555.123.4567",
            "5551234567",
        ];

        // All should match the pattern
        for phone in &phone_numbers {
            assert!(phone_pattern.match_string(phone), "Should match phone: {}", phone);
        }

        // Create a normalization pattern
        let normalize_pattern = VibePattern::compile(r"[^\d]").expect("Should compile");
        
        let normalized: Vec<String> = phone_numbers.iter()
            .map(|phone| normalize_pattern.replace_all_string(phone, ""))
            .collect();

        // All normalized numbers should be the same
        for normalized_phone in &normalized {
            assert_eq!(normalized_phone, "5551234567", "Normalized phone should be 5551234567");
        }
    }

    #[test]
    fn test_data_extraction_workflow() {
        let csv_line = "John,Doe,30,john.doe@email.com,(555) 123-4567";
        
        // Build a pattern to extract CSV data
        let csv_pattern = PatternBuilder::new()
            .named_group("first", r"[^,]+")
            .literal(",")
            .named_group("last", r"[^,]+")
            .literal(",")
            .named_group("age", r"\d+")
            .literal(",")
            .named_group("email", r"[^,]+")
            .literal(",")
            .named_group("phone", r"[^,]+")
            .build()
            .expect("Should build CSV pattern");

        let groups = csv_pattern.vibe_groups();
        let data = groups.find_groups_string(csv_line);

        assert_eq!(data.get("first"), Some(&"John".to_string()));
        assert_eq!(data.get("last"), Some(&"Doe".to_string()));
        assert_eq!(data.get("age"), Some(&"30".to_string()));
        assert_eq!(data.get("email"), Some(&"john.doe@email.com".to_string()));
        assert_eq!(data.get("phone"), Some(&"(555) 123-4567".to_string()));

        // Validate extracted email and phone
        let email_valid = CommonPatterns::email().match_string(data.get("email").unwrap());
        let phone_valid = CommonPatterns::phone().match_string(data.get("phone").unwrap());
        
        assert!(email_valid, "Extracted email should be valid");
        assert!(phone_valid, "Extracted phone should be valid");
    }

    #[test]
    fn test_template_replacement_workflow() {
        let user_data = vec![
            ("john.doe@example.com", "John Doe"),
            ("jane.smith@company.org", "Jane Smith"),
            ("admin@website.net", "Admin User"),
        ];

        let email_pattern = VibePattern::compile(r"(\w+)\.(\w+)@(\w+\.\w+)")
            .expect("Should compile");

        let results: Vec<String> = user_data.iter()
            .map(|(email, _)| {
                if email_pattern.match_string(email) {
                    email_pattern.replace_all_string(email, "First: $1, Last: $2, Domain: $3")
                } else {
                    format!("No match for: {}", email)
                }
            })
            .collect();

        assert_eq!(results[0], "First: john, Last: doe, Domain: example.com");
        assert_eq!(results[1], "First: jane, Last: smith, Domain: company.org");
        assert!(results[2].starts_with("No match for:")); // admin@website.net doesn't match the pattern
    }

    #[test]
    fn test_complex_search_and_replace_workflow() {
        let text = r#"
        Error occurred at 2023-12-25 14:30:45 in module user_auth
        Warning at 2023-12-25 14:31:02 in module database_conn
        Info at 2023-12-25 14:31:15 in module request_handler
        "#;

        // Find all timestamps and modules
        let log_pattern = VibePattern::compile(
            r"(\w+) at (\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) in module (\w+)"
        ).expect("Should compile");

        let matches = log_pattern.find_all_string_submatch(text, -1);
        assert_eq!(matches.len(), 3, "Should find 3 log entries");

        // Replace with formatted output
        let formatted = log_pattern.replace_all_string_func(text, |match_str| {
            // Extract level, timestamp, and module from the match
            let parts: Vec<&str> = match_str.split_whitespace().collect();
            if parts.len() >= 6 {
                let level = parts[0];
                let date = parts[2];
                let time = parts[3];
                let module = parts[6];
                format!("[{}] {} {} - {}", level.to_uppercase(), date, time, module.to_uppercase())
            } else {
                match_str.to_string()
            }
        });

        assert!(formatted.contains("[ERROR] 2023-12-25 14:30:45 - USER_AUTH"));
        assert!(formatted.contains("[WARNING] 2023-12-25 14:31:02 - DATABASE_CONN"));
        assert!(formatted.contains("[INFO] 2023-12-25 14:31:15 - REQUEST_HANDLER"));
    }

    #[test]
    fn test_pattern_validation_and_optimization_workflow() {
        let test_patterns = vec![
            r"\d+",
            r"[a-zA-Z]+",
            r"[invalid",  // Invalid pattern
            r"(?P<name>\w+)",
            r".*",
        ];

        let mut valid_patterns = Vec::new();
        let mut invalid_patterns = Vec::new();

        // Validate patterns
        for pattern in test_patterns {
            if is_valid_pattern(pattern) {
                valid_patterns.push(pattern);
            } else {
                invalid_patterns.push(pattern);
            }
        }

        assert_eq!(valid_patterns.len(), 4, "Should have 4 valid patterns");
        assert_eq!(invalid_patterns.len(), 1, "Should have 1 invalid pattern");
        assert!(invalid_patterns.contains(&"[invalid"));

        // Test each valid pattern
        let test_text = "Hello123 World456";
        for pattern in valid_patterns {
            let compiled = VibePattern::compile(pattern).expect("Should compile valid pattern");
            let matches = compiled.match_string(test_text);
            println!("Pattern '{}' matches '{}': {}", pattern, test_text, matches);
        }
    }

    #[test]
    fn test_multi_pattern_text_analysis_workflow() {
        let document = r#"
        Contact us at support@company.com or call (555) 123-4567.
        Visit our website https://www.company.com for more information.
        Our office is located at 123 Main St, ZIP 12345.
        Follow us on social media #CompanyNews #TechUpdates
        Document ID: 550e8400-e29b-41d4-a716-446655440000
        "#;

        // Test against multiple common patterns
        let pattern_tests = vec![
            ("email", CommonPatterns::email()),
            ("phone", CommonPatterns::phone()),
            ("url", CommonPatterns::url()),
            ("zip_code", CommonPatterns::zip_code()),
            ("hashtag", CommonPatterns::hashtag()),
            ("uuid", CommonPatterns::uuid()),
        ];

        let mut found_patterns = HashMap::new();

        for (name, pattern) in pattern_tests {
            let matches = pattern.find_all_string(&document, -1);
            if !matches.is_empty() {
                found_patterns.insert(name, matches);
            }
        }

        // Verify expected patterns were found
        assert!(found_patterns.contains_key("email"));
        assert!(found_patterns.contains_key("phone"));
        assert!(found_patterns.contains_key("url"));
        assert!(found_patterns.contains_key("zip_code"));
        assert!(found_patterns.contains_key("hashtag"));
        assert!(found_patterns.contains_key("uuid"));

        // Verify specific extractions
        let emails = found_patterns.get("email").unwrap();
        assert!(emails.contains(&"support@company.com".to_string()));

        let phones = found_patterns.get("phone").unwrap();
        assert!(phones.contains(&"(555) 123-4567".to_string()));

        let hashtags = found_patterns.get("hashtag").unwrap();
        assert!(hashtags.len() >= 2); // Should find #CompanyNews and #TechUpdates
    }

    #[test]
    fn test_performance_comparison_workflow() {
        let test_texts = vec![
            "user123@example.com",
            "admin456@company.org",
            "support789@website.net",
            "notanemail",
            "another@test.co.uk",
        ];

        // Compare different email patterns
        let simple_email = r"\w+@\w+\.\w+";
        let complex_email = r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";

        let simple_result = benchmark_pattern(simple_email, &test_texts.iter().map(|s| s.as_str()).collect::<Vec<_>>(), 1000)
            .expect("Should benchmark simple pattern");

        let complex_result = benchmark_pattern(complex_email, &test_texts.iter().map(|s| s.as_str()).collect::<Vec<_>>(), 1000)
            .expect("Should benchmark complex pattern");

        assert!(simple_result.operations_per_second > 0);
        assert!(complex_result.operations_per_second > 0);
        assert_eq!(simple_result.total_operations, complex_result.total_operations);

        // Both should find the same number of matches for these test cases
        println!("Simple pattern: {} ops/sec, {} matches", 
                simple_result.operations_per_second, simple_result.total_matches);
        println!("Complex pattern: {} ops/sec, {} matches", 
                complex_result.operations_per_second, complex_result.total_matches);
    }

    #[test]
    fn test_text_cleaning_workflow() {
        let dirty_text = "Hello!!!   This is a test... with lots of    spaces and!!!punctuation???";
        
        // Clean up multiple spaces
        let space_pattern = VibePattern::compile(r"\s+").expect("Should compile");
        let cleaned_spaces = space_pattern.replace_all_string(&dirty_text, " ");

        // Clean up multiple punctuation
        let punct_pattern = VibePattern::compile(r"[!.?]{2,}").expect("Should compile");
        let cleaned_punct = punct_pattern.replace_all_string(&cleaned_spaces, ".");

        let expected = "Hello. This is a test. with lots of spaces and.punctuation.";
        assert_eq!(cleaned_punct, expected);

        // Extract meaningful words
        let word_pattern = VibePattern::compile(r"\b[a-zA-Z]{3,}\b").expect("Should compile");
        let words = word_pattern.find_all_string(&cleaned_punct, -1);
        
        let expected_words = vec!["Hello", "This", "test", "with", "lots", "spaces", "and", "punctuation"];
        assert_eq!(words, expected_words);
    }

    #[test]
    fn test_configuration_parsing_workflow() {
        let config_text = r#"
        server.host=localhost
        server.port=8080
        database.url=postgresql://user:pass@localhost/db
        cache.enabled=true
        log.level=INFO
        "#;

        // Parse configuration key-value pairs
        let config_pattern = VibePattern::compile(r"(?P<key>\w+\.\w+)=(?P<value>.+)")
            .expect("Should compile config pattern");

        let groups = config_pattern.vibe_groups();
        let all_configs = groups.find_all_groups_string(config_text, -1);

        assert_eq!(all_configs.len(), 5, "Should find 5 config entries");

        // Verify specific configurations
        let server_host = all_configs.iter()
            .find(|config| config.get("key") == Some(&"server.host".to_string()))
            .expect("Should find server.host config");
        assert_eq!(server_host.get("value"), Some(&"localhost".to_string()));

        let server_port = all_configs.iter()
            .find(|config| config.get("key") == Some(&"server.port".to_string()))
            .expect("Should find server.port config");
        assert_eq!(server_port.get("value"), Some(&"8080".to_string()));

        // Validate URL format
        let db_config = all_configs.iter()
            .find(|config| config.get("key") == Some(&"database.url".to_string()))
            .expect("Should find database.url config");
        let db_url = db_config.get("value").unwrap();
        assert!(db_url.starts_with("postgresql://"));
    }

    #[test]
    fn test_code_syntax_highlighting_workflow() {
        let code = r#"
        function calculateSum(a, b) {
            return a + b;
        }
        const result = calculateSum(10, 20);
        console.log("Result: " + result);
        "#;

        // Extract different code elements
        let function_pattern = VibePattern::compile(r"function\s+(\w+)").expect("Should compile");
        let variable_pattern = VibePattern::compile(r"const\s+(\w+)").expect("Should compile");
        let string_pattern = VibePattern::compile(r#""([^"]*)""#).expect("Should compile");
        let number_pattern = VibePattern::compile(r"\b\d+\b").expect("Should compile");

        // Extract functions
        let functions = function_pattern.find_all_string_submatch(code, -1);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0][1], "calculateSum");

        // Extract variables
        let variables = variable_pattern.find_all_string_submatch(code, -1);
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0][1], "result");

        // Extract strings
        let strings = string_pattern.find_all_string_submatch(code, -1);
        assert_eq!(strings.len(), 1);
        assert_eq!(strings[0][1], "Result: ");

        // Extract numbers
        let numbers = number_pattern.find_all_string(code, -1);
        assert_eq!(numbers, vec!["10", "20"]);
    }
}
