/// Unit tests for the RegexVibez module
/// Testing individual components and functions

#[cfg(test)]
mod tests {
    use cursed::stdlib::regex_vibez::*;
    use cursed::stdlib::regex_vibez::error::*;
    use cursed::stdlib::regex_vibez::pattern::*;
    use cursed::stdlib::regex_vibez::groups::*;
    use cursed::stdlib::regex_vibez::builder::*;
    use cursed::stdlib::regex_vibez::common::*;
    use cursed::stdlib::regex_vibez::utils::*;

    #[test]
    fn test_basic_pattern_compilation() {
        // Test successful compilation
        let pattern = VibePattern::compile(r"\d+").expect("Should compile numeric pattern");
        assert!(pattern.match_string("123"));
        assert!(!pattern.match_string("abc"));

        // Test compilation error
        let result = VibePattern::compile("[invalid");
        assert!(result.is_err());
        
        match result {
            Err(RegexVibesError::CompilationError(_)) => (),
            _ => panic!("Expected compilation error"),
        }
    }

    #[test]
    fn test_posix_compilation() {
        let pattern = VibePattern::compile_posix(r"[[:digit:]]+")
            .expect("Should compile POSIX pattern");
        assert!(pattern.match_string("123"));
        assert!(!pattern.match_string("abc"));
    }

    #[test]
    fn test_match_operations() {
        let pattern = VibePattern::compile(r"f[a-z]+").expect("Should compile");
        
        // Test string matching
        assert!(pattern.match_string("frfr"));
        assert!(pattern.match_string("facts"));
        assert!(!pattern.match_string("FACTS"));

        // Test byte matching
        assert!(pattern.r#match(b"frfr"));
        assert!(!pattern.r#match(b"FACTS"));
        
        // Test invalid UTF-8
        assert!(!pattern.r#match(&[0xFF, 0xFE]));
    }

    #[test]
    fn test_find_operations() {
        let pattern = VibePattern::compile(r"(\w+)@(\w+\.\w+)")
            .expect("Should compile email pattern");
        let text = "Contact user@example.com for help";

        // Test FindString
        let found = pattern.find_string(text);
        assert_eq!(found, "user@example.com");

        // Test FindStringIndex
        let index = pattern.find_string_index(text);
        assert_eq!(index.len(), 2);
        assert!(index[0] >= 0);
        assert!(index[1] > index[0]);

        // Test FindStringSubmatch
        let submatch = pattern.find_string_submatch(text);
        assert_eq!(submatch.len(), 3); // full match + 2 groups
        assert_eq!(submatch[0], "user@example.com");
        assert_eq!(submatch[1], "user");
        assert_eq!(submatch[2], "example.com");

        // Test FindStringSubmatchIndex
        let submatch_index = pattern.find_string_submatch_index(text);
        assert_eq!(submatch_index.len(), 6); // 3 matches * 2 indices each
    }

    #[test]
    fn test_find_all_operations() {
        let pattern = VibePattern::compile(r"\w+").expect("Should compile");
        let text = "hello world test";

        // Test FindAllString
        let all = pattern.find_all_string(text, -1);
        assert_eq!(all, vec!["hello", "world", "test"]);

        let limited = pattern.find_all_string(text, 2);
        assert_eq!(limited, vec!["hello", "world"]);

        // Test FindAllStringIndex
        let all_index = pattern.find_all_string_index(text, -1);
        assert_eq!(all_index.len(), 3);
        assert_eq!(all_index[0], vec![0, 5]);
        assert_eq!(all_index[1], vec![6, 11]);
        assert_eq!(all_index[2], vec![12, 16]);

        // Test FindAllStringSubmatch
        let email_pattern = VibePattern::compile(r"(\w+)@(\w+\.\w+)")
            .expect("Should compile");
        let multi_email = "user@test.com and admin@example.org";
        let all_submatch = email_pattern.find_all_string_submatch(multi_email, -1);
        assert_eq!(all_submatch.len(), 2);
        assert_eq!(all_submatch[0][1], "user");
        assert_eq!(all_submatch[1][1], "admin");
    }

    #[test]
    fn test_replace_operations() {
        let pattern = VibePattern::compile(r"cap").expect("Should compile");
        
        // Test ReplaceAllString
        let result = pattern.replace_all_string("no cap", "lies");
        assert_eq!(result, "no lies");

        let result = pattern.replace_all_string("cap cap cap", "truth");
        assert_eq!(result, "truth truth truth");

        // Test ReplaceAllStringFunc
        let result = pattern.replace_all_string_func("no cap", |_| "truth".to_string());
        assert_eq!(result, "no truth");

        let mut counter = 0;
        let result = pattern.replace_all_string_func("cap cap cap", |_| {
            counter += 1;
            format!("truth{}", counter)
        });
        assert_eq!(result, "truth1 truth2 truth3");
    }

    #[test]
    fn test_split_operations() {
        let pattern = VibePattern::compile(r",\s*").expect("Should compile");
        
        // Test unlimited split
        let result = pattern.split("a, b, c, d", -1);
        assert_eq!(result, vec!["a", "b", "c", "d"]);

        // Test limited split
        let result = pattern.split("a, b, c, d", 3);
        assert_eq!(result, vec!["a", "b", "c, d"]);

        // Test no split
        let result = pattern.split("a, b, c", 0);
        assert_eq!(result, vec!["a, b, c"]);

        // Test single split
        let result = pattern.split("a, b, c", 1);
        assert_eq!(result, vec!["a, b, c"]);
    }

    #[test]
    fn test_named_groups() {
        let pattern = VibePattern::compile(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)")
            .expect("Should compile");

        // Test GroupNames
        let names = pattern.group_names();
        assert!(names.len() >= 3); // full match + 2 named groups
        assert!(names.contains(&"user".to_string()));
        assert!(names.contains(&"domain".to_string()));

        // Test NamedGroups
        let named = pattern.named_groups();
        assert!(named.contains_key("user"));
        assert!(named.contains_key("domain"));
        assert!(named["user"] > 0);
        assert!(named["domain"] > 0);

        // Test FindGroupsString
        let groups = pattern.find_groups_string("admin@test.com");
        assert_eq!(groups.get("user"), Some(&"admin".to_string()));
        assert_eq!(groups.get("domain"), Some(&"test.com".to_string()));

        // Test with no match
        let groups = pattern.find_groups_string("not an email");
        assert!(groups.is_empty() || groups.values().all(|v| v.is_empty()));
    }

    #[test]
    fn test_template_replacement() {
        let pattern = VibePattern::compile(r"(\w+)@(\w+)")
            .expect("Should compile");
        
        let result = pattern.template_replace("user@domain", "$2 - $1")
            .expect("Should replace");
        assert_eq!(result, "domain - user");
    }

    #[test]
    fn test_vibe_groups() {
        let pattern = VibePattern::compile(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)")
            .expect("Should compile");
        let groups = pattern.vibe_groups();

        // Test basic functionality
        assert!(groups.has_group("user"));
        assert!(groups.has_group("domain"));
        assert!(!groups.has_group("nonexistent"));

        let index = groups.group_index("user");
        assert!(index > 0);
        assert_eq!(groups.group_index("nonexistent"), -1);

        // Test group values
        let value = groups.get_group_value("admin@test.com", "user");
        assert_eq!(value, "admin");

        let value = groups.get_group_value("admin@test.com", "nonexistent");
        assert_eq!(value, "");

        // Test group matching
        assert!(groups.group_has_match("admin@test.com", "user"));
        assert!(!groups.group_has_match("no match", "user"));

        // Test matched groups
        let matched = groups.matched_groups("admin@test.com");
        assert!(matched.contains(&"user".to_string()));
        assert!(matched.contains(&"domain".to_string()));
    }

    #[test]
    fn test_group_statistics() {
        let pattern = VibePattern::compile(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)(\.\w+)?")
            .expect("Should compile");
        let groups = pattern.vibe_groups();

        let stats = groups.group_statistics();
        assert_eq!(stats.named_groups, 2);
        assert!(stats.total_groups >= 3);
        assert!(stats.group_names.contains(&"user".to_string()));
        assert!(stats.group_names.contains(&"domain".to_string()));
    }

    #[test]
    fn test_group_validation() {
        let pattern = VibePattern::compile(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)")
            .expect("Should compile");
        let groups = pattern.vibe_groups();

        // Valid groups
        let result = groups.validate_groups(&["user", "domain"]);
        assert!(result.is_valid);
        assert!(result.missing_groups.is_empty());
        assert_eq!(result.present_groups.len(), 2);

        // Invalid groups
        let result = groups.validate_groups(&["user", "nonexistent"]);
        assert!(!result.is_valid);
        assert!(result.missing_groups.contains(&"nonexistent".to_string()));
        assert!(result.present_groups.contains(&"user".to_string()));
    }

    #[test]
    fn test_pattern_builder() {
        // Basic builder test
        let pattern = PatternBuilder::new()
            .literal("hello")
            .space()
            .literal("world")
            .build()
            .expect("Should build");

        assert!(pattern.match_string("hello world"));
        assert!(!pattern.match_string("helloworld"));

        // Email builder test
        let pattern = PatternBuilder::new()
            .starts_with("")
            .named_group("user", r"[a-zA-Z0-9._%+-]+")
            .literal("@")
            .named_group("domain", r"[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
            .ends_with("")
            .build()
            .expect("Should build");

        let groups = pattern.find_groups_string("user@example.com");
        assert_eq!(groups.get("user"), Some(&"user".to_string()));
        assert_eq!(groups.get("domain"), Some(&"example.com".to_string()));
    }

    #[test]
    fn test_pattern_builder_quantifiers() {
        let pattern = PatternBuilder::new()
            .digit()
            .exactly(3)
            .literal("-")
            .digit()
            .between(2, 4)
            .build()
            .expect("Should build");

        assert!(pattern.match_string("123-45"));
        assert!(pattern.match_string("123-4567"));
        assert!(!pattern.match_string("12-45"));
        assert!(!pattern.match_string("123-1"));
    }

    #[test]
    fn test_pattern_builder_options() {
        let pattern = PatternBuilder::new()
            .case_insensitive(true)
            .literal("hello")
            .build()
            .expect("Should build");

        assert!(pattern.match_string("HELLO"));
        assert!(pattern.match_string("Hello"));
        assert!(pattern.match_string("hello"));
    }

    #[test]
    fn test_pattern_builder_predefined() {
        let email_pattern = PatternBuilder::new()
            .email()
            .build()
            .expect("Should build");
        assert!(email_pattern.match_string("test@example.com"));

        let url_pattern = PatternBuilder::new()
            .url()
            .build()
            .expect("Should build");
        assert!(url_pattern.match_string("https://example.com"));

        let phone_pattern = PatternBuilder::new()
            .phone()
            .build()
            .expect("Should build");
        assert!(phone_pattern.match_string("(555) 123-4567"));
    }

    #[test]
    fn test_common_patterns() {
        // Test email pattern
        assert!(EMAIL_PATTERN.match_string("user@example.com"));
        assert!(!EMAIL_PATTERN.match_string("invalid-email"));

        // Test URL pattern
        assert!(URL_PATTERN.match_string("https://example.com"));
        assert!(!URL_PATTERN.match_string("not-a-url"));

        // Test phone pattern
        assert!(PHONE_PATTERN.match_string("(555) 123-4567"));
        assert!(!PHONE_PATTERN.match_string("invalid-phone"));

        // Test date pattern
        assert!(DATE_PATTERN.match_string("2023-12-25"));
        assert!(!DATE_PATTERN.match_string("25-12-2023"));

        // Test time pattern
        assert!(TIME_PATTERN.match_string("14:30:45"));
        assert!(!TIME_PATTERN.match_string("2:30:45"));
    }

    #[test]
    fn test_common_patterns_access() {
        assert!(CommonPatterns::email().match_string("test@example.com"));
        assert!(CommonPatterns::phone().match_string("555-123-4567"));
        
        let pattern = CommonPatterns::get_pattern("email");
        assert!(pattern.is_some());
        
        let pattern = CommonPatterns::get_pattern("nonexistent");
        assert!(pattern.is_none());

        let names = CommonPatterns::pattern_names();
        assert!(names.contains(&"email"));
        assert!(names.contains(&"phone"));
        assert!(names.len() > 10);
    }

    #[test]
    fn test_utility_functions() {
        // Test pattern validation
        assert!(is_valid_pattern(r"\d+"));
        assert!(!is_valid_pattern("[invalid"));

        assert!(validate_pattern(r"\d+").is_ok());
        assert!(validate_pattern("[invalid").is_err());

        // Test capture group counting
        let count = count_capture_groups(r"(\d+)-(\d+)").unwrap();
        assert_eq!(count, 3); // full match + 2 groups

        // Test literal extraction
        let literals = extract_literals(r"hello\d+world");
        assert!(literals.contains(&"hello".to_string()));
        assert!(literals.contains(&"world".to_string()));
    }

    #[test]
    fn test_string_optimization() {
        let strings = vec!["hello_world".to_string(), "hello_test".to_string()];
        let prefix = find_common_prefix(&strings);
        assert_eq!(prefix, "hello_");

        let optimized = optimize_string_list(&strings);
        assert!(optimized.contains("hello_"));
    }

    #[test]
    fn test_glob_functions() {
        let regex = glob_to_regex("*.txt");
        assert!(regex.contains("txt"));

        assert!(glob_match("*.txt", "test.txt").unwrap());
        assert!(!glob_match("*.txt", "test.rs").unwrap());
    }

    #[test]
    fn test_benchmark_functionality() {
        let texts = vec!["hello123", "world456", "nomatch"];
        let result = benchmark_pattern(r"\d+", &texts, 10).unwrap();
        
        assert_eq!(result.total_operations, 30); // 3 texts * 10 iterations
        assert_eq!(result.total_matches, 20); // 2 matching texts * 10 iterations
        assert!(result.operations_per_second > 0);
        assert!((result.match_rate() - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_helper_functions() {
        // Test package-level functions
        let pattern = compile(r"\d+").expect("Should compile");
        assert!(pattern.match_string("123"));

        let pattern = must_compile(r"\d+");
        assert!(pattern.match_string("123"));

        let (matched, err) = match_string(r"f[a-z]+", "frfr");
        assert!(matched);
        assert!(err.is_none());

        let quoted = quote_meta("a.b*c+d?e");
        assert_eq!(quoted, r"a\.b\*c\+d\?e");
    }

    #[test]
    #[should_panic(expected = "RegexVibez MustCompile failed")]
    fn test_must_compile_panic() {
        must_compile("[invalid");
    }

    #[test]
    fn test_error_types() {
        let err = compilation_error("test error");
        assert!(matches!(err, RegexVibesError::CompilationError(_)));
        assert_eq!(err.to_string(), "Regex compilation error: test error");

        let err = invalid_input_error("bad input");
        assert!(matches!(err, RegexVibesError::InvalidInput(_)));

        let err = template_error("template issue");
        assert!(matches!(err, RegexVibesError::TemplateError(_)));
    }

    #[test]
    fn test_error_conversions() {
        let regex_err = regex::Regex::new("[invalid").unwrap_err();
        let vibe_err = RegexVibesError::from(regex_err);
        assert!(matches!(vibe_err, RegexVibesError::CompilationError(_)));

        let cursed_err = cursed::error::CursedError::from(vibe_err);
        assert!(cursed_err.message().contains("compilation error"));
    }
}
