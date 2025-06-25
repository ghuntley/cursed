// RegexVibez Demo - Comprehensive demonstration of regex functionality in CURSED
// This example shows all the major features of the RegexVibez module

import "stdlib::regex_vibez";
import "stdlib::io";
import "stdlib::string";

squad VibeDemo {
    slay demo_basic_matching() -> () {
        println("=== Basic Pattern Matching ===");
        
        // Simple pattern matching
        sus pattern = regex_vibez.compile(r"f[a-z]+");
        lowkey (pattern.is_err()) {
            println("Failed to compile pattern");
            yolo;
        }
        
        facts compiled_pattern = pattern.unwrap();
        
        // Test various strings
        facts test_strings = ["frfr", "facts", "FACTS", "flex", "nomatch"];
        
        tea string : test_strings {
            sus matches = compiled_pattern.match_string(string);
            println(format!("'{}' matches pattern: {}", string, matches));
        }
        
        println("");
    }

    slay demo_email_validation() -> () {
        println("=== Email Validation ===");
        
        // Use pre-compiled email pattern
        facts email_pattern = regex_vibez.EMAIL_PATTERN;
        
        facts test_emails = [
            "user@example.com",
            "test.email+tag@domain.org", 
            "invalid-email",
            "@domain.com",
            "user@domain",
            "admin.user@company-name.co.uk"
        ];
        
        tea email : test_emails {
            sus is_valid = email_pattern.match_string(email);
            sus status = lowkey is_valid { "✓ VALID" } bestie { "✗ INVALID" };
            println(format!("{} - {}", email, status));
        }
        
        println("");
    }

    slay demo_named_groups() -> () {
        println("=== Named Capture Groups ===");
        
        // Email pattern with named groups
        sus email_pattern = regex_vibez.compile(r"(?P<user>[a-zA-Z0-9._%+-]+)@(?P<domain>[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})");
        lowkey (email_pattern.is_err()) {
            println("Failed to compile email pattern");
            yolo;
        }
        
        facts pattern = email_pattern.unwrap();
        facts groups = pattern.vibe_groups();
        
        facts test_email = "admin@example.com";
        sus matches = groups.find_groups_string(test_email);
        
        println(format!("Analyzing email: {}", test_email));
        lowkey (matches.contains_key("user")) {
            println(format!("Username: {}", matches.get("user").unwrap()));
        }
        lowkey (matches.contains_key("domain")) {
            println(format!("Domain: {}", matches.get("domain").unwrap()));
        }
        
        println("");
    }

    slay demo_pattern_builder() -> () {
        println("=== Pattern Builder ===");
        
        // Build a complex pattern using the fluent interface
        sus builder_result = regex_vibez.new_pattern_builder()
            .starts_with("")
            .named_group("protocol", "https?")
            .literal("://")
            .named_group("domain", r"[a-zA-Z0-9.-]+")
            .optional("/")
            .named_group("path", r"[^\s]*")
            .ends_with("")
            .build();
            
        lowkey (builder_result.is_err()) {
            println("Failed to build URL pattern");
            yolo;
        }
        
        facts url_pattern = builder_result.unwrap();
        facts groups = url_pattern.vibe_groups();
        
        facts test_urls = [
            "https://example.com",
            "http://blog.example.org/posts/123",
            "https://api.company.com/v1/users",
            "not-a-url"
        ];
        
        tea url : test_urls {
            lowkey (url_pattern.match_string(url)) {
                println(format!("✓ Valid URL: {}", url));
                sus parsed = groups.find_groups_string(url);
                println(format!("  Protocol: {}", parsed.get("protocol").unwrap_or(&"".to_string())));
                println(format!("  Domain: {}", parsed.get("domain").unwrap_or(&"".to_string())));
                println(format!("  Path: {}", parsed.get("path").unwrap_or(&"".to_string())));
            } bestie {
                println(format!("✗ Invalid URL: {}", url));
            }
        }
        
        println("");
    }

    slay demo_search_replace() -> () {
        println("=== Search and Replace ===");
        
        sus pattern = regex_vibez.compile(r"cap");
        lowkey (pattern.is_err()) {
            println("Failed to compile pattern");
            yolo;
        }
        
        facts cap_pattern = pattern.unwrap();
        
        // Simple replacement
        facts text1 = "no cap, that's cap";
        facts replaced1 = cap_pattern.replace_all_string(text1, "lies");
        println(format!("Original: {}", text1));
        println(format!("Replaced: {}", replaced1));
        
        // Function-based replacement
        sus counter = 0;
        facts replaced2 = cap_pattern.replace_all_string_func(text1, |_| {
            counter += 1;
            format!("truth{}", counter)
        });
        println(format!("Function replacement: {}", replaced2));
        
        println("");
    }

    slay demo_text_splitting() -> () {
        println("=== Text Splitting ===");
        
        sus pattern = regex_vibez.compile(r",\s*");
        lowkey (pattern.is_err()) {
            println("Failed to compile split pattern");
            yolo;
        }
        
        facts comma_pattern = pattern.unwrap();
        
        facts csv_data = "apple, banana, cherry, date, elderberry";
        facts items = comma_pattern.split(csv_data, -1);
        
        println(format!("Original: {}", csv_data));
        println("Split items:");
        tea item : items {
            println(format!("  - {}", item));
        }
        
        // Limited split
        facts limited_items = comma_pattern.split(csv_data, 3);
        println("Limited split (3 parts):");
        tea item : limited_items {
            println(format!("  - {}", item));
        }
        
        println("");
    }

    slay demo_log_parsing() -> () {
        println("=== Log File Parsing ===");
        
        sus log_pattern = regex_vibez.compile(
            r"(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)"
        );
        lowkey (log_pattern.is_err()) {
            println("Failed to compile log pattern");
            yolo;
        }
        
        facts pattern = log_pattern.unwrap();
        facts groups = pattern.vibe_groups();
        
        facts log_lines = [
            "2023-12-25 14:30:45 [INFO] Application started successfully",
            "2023-12-25 14:30:46 [ERROR] Database connection failed", 
            "2023-12-25 14:30:47 [WARN] High memory usage detected",
            "2023-12-25 14:30:48 [DEBUG] Processing user request"
        ];
        
        tea line : log_lines {
            lowkey (pattern.match_string(line)) {
                sus parsed = groups.find_groups_string(line);
                sus timestamp = parsed.get("timestamp").unwrap_or(&"".to_string());
                sus level = parsed.get("level").unwrap_or(&"".to_string());
                sus message = parsed.get("message").unwrap_or(&"".to_string());
                
                println(format!("[{}] {} - {}", level, timestamp, message));
            } bestie {
                println(format!("Invalid log line: {}", line));
            }
        }
        
        println("");
    }

    slay demo_phone_number_normalization() -> () {
        println("=== Phone Number Normalization ===");
        
        facts phone_numbers = [
            "(555) 123-4567",
            "555-123-4567", 
            "555.123.4567",
            "5551234567",
            "1-555-123-4567"
        ];
        
        // Pattern to remove non-digits
        sus digit_only = regex_vibez.compile(r"[^\d]");
        lowkey (digit_only.is_err()) {
            println("Failed to compile digit pattern");
            yolo;
        }
        
        facts normalize_pattern = digit_only.unwrap();
        
        tea phone : phone_numbers {
            // First check if it matches the phone pattern
            sus is_valid = regex_vibez.PHONE_PATTERN.match_string(phone);
            sus status = lowkey is_valid { "✓" } bestie { "✗" };
            
            // Normalize by removing non-digits
            sus normalized = normalize_pattern.replace_all_string(phone, "");
            
            println(format!("{} {} -> {}", status, phone, normalized));
        }
        
        println("");
    }

    slay demo_common_patterns() -> () {
        println("=== Common Pattern Library ===");
        
        facts test_data = [
            ("Email", "user@example.com", regex_vibez.EMAIL_PATTERN),
            ("URL", "https://example.com", regex_vibez.URL_PATTERN),
            ("Phone", "(555) 123-4567", regex_vibez.PHONE_PATTERN),
            ("Date", "2023-12-25", regex_vibez.DATE_PATTERN),
            ("Time", "14:30:45", regex_vibez.TIME_PATTERN),
            ("UUID", "123e4567-e89b-41d4-a456-426614174000", regex_vibez.UUID_PATTERN)
        ];
        
        tea (type_name, test_value, pattern) : test_data {
            sus matches = pattern.match_string(test_value);
            sus status = lowkey matches { "✓ MATCH" } bestie { "✗ NO MATCH" };
            println(format!("{}: {} - {}", type_name, test_value, status));
        }
        
        println("");
    }

    slay demo_performance_testing() -> () {
        println("=== Performance Testing ===");
        
        facts test_texts = [
            "user123@example.com",
            "contact@company.org", 
            "admin@website.net",
            "notanemail",
            "support@domain.co.uk"
        ];
        
        // Benchmark email pattern
        sus benchmark_result = regex_vibez.benchmark_pattern(
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
            &test_texts.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            1000
        );
        
        lowkey (benchmark_result.is_ok()) {
            facts result = benchmark_result.unwrap();
            println(format!("Benchmarked {} operations", result.total_operations));
            println(format!("Found {} matches", result.total_matches));
            println(format!("Performance: {} ops/second", result.operations_per_second));
            println(format!("Match rate: {:.1}%", result.match_rate()));
        } bestie {
            println("Benchmark failed");
        }
        
        println("");
    }

    slay demo_data_extraction() -> () {
        println("=== Data Extraction ===");
        
        facts document = r#"
        Contact Information:
        Email: support@company.com
        Phone: (555) 123-4567
        Website: https://www.company.com
        Address: 123 Main St, ZIP 12345
        Social: #CompanyNews #TechUpdates
        "#;
        
        println("Extracting data from document:");
        println(document);
        
        // Extract emails
        facts emails = regex_vibez.EMAIL_PATTERN.find_all_string(document, -1);
        println("Emails found:");
        tea email : emails {
            println(format!("  - {}", email));
        }
        
        // Extract phone numbers
        facts phones = regex_vibez.PHONE_PATTERN.find_all_string(document, -1);
        println("Phone numbers found:");
        tea phone : phones {
            println(format!("  - {}", phone));
        }
        
        // Extract URLs
        facts urls = regex_vibez.URL_PATTERN.find_all_string(document, -1);
        println("URLs found:");
        tea url : urls {
            println(format!("  - {}", url));
        }
        
        // Extract hashtags
        facts hashtags = regex_vibez.HASHTAG_PATTERN.find_all_string(document, -1);
        println("Hashtags found:");
        tea hashtag : hashtags {
            println(format!("  - {}", hashtag));
        }
        
        println("");
    }

    slay demo_error_handling() -> () {
        println("=== Error Handling ===");
        
        // Test invalid pattern
        sus invalid_result = regex_vibez.compile("[invalid");
        lowkey (invalid_result.is_err()) {
            println("✓ Properly caught invalid pattern error");
            println(format!("Error: {}", invalid_result.err().unwrap()));
        } bestie {
            println("✗ Failed to catch invalid pattern");
        }
        
        // Test must_compile (would panic on invalid pattern)
        sus valid_pattern = regex_vibez.must_compile(r"\d+");
        println("✓ MustCompile succeeded with valid pattern");
        
        // Test helper functions
        sus (matched, err) = regex_vibez.match_string(r"test", "testing");
        lowkey (err.is_none()) {
            println(format!("✓ match_string succeeded: {}", matched));
        } bestie {
            println(format!("✗ match_string failed: {}", err.unwrap()));
        }
        
        sus (no_match, err2) = regex_vibez.match_string("[invalid", "test");
        lowkey (err2.is_some()) {
            println("✓ match_string properly handled invalid pattern");
        } bestie {
            println("✗ match_string should have failed with invalid pattern");
        }
        
        println("");
    }
}

slay main() -> () {
    println("RegexVibez Comprehensive Demo");
    println("============================");
    println("");
    
    facts demo = VibeDemo{};
    
    demo.demo_basic_matching();
    demo.demo_email_validation();
    demo.demo_named_groups();
    demo.demo_pattern_builder();
    demo.demo_search_replace();
    demo.demo_text_splitting();
    demo.demo_log_parsing();
    demo.demo_phone_number_normalization();
    demo.demo_common_patterns();
    demo.demo_performance_testing();
    demo.demo_data_extraction();
    demo.demo_error_handling();
    
    println("Demo completed successfully! 🎉");
}
