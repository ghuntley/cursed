fr fr RegexVibez Practical Examples - Real-world usage scenarios
fr fr This example demonstrates practical applications of regex in CURSED

yeet "stdlib::regex_vibez"
yeet "stdlib::io"
yeet "stdlib::string"
yeet "stdlib::collections"

squad LogAnalyzer {
    sus error_pattern: VibePattern;
    sus warning_pattern: VibePattern;
    sus timestamp_pattern: VibePattern;
    
    slay new() -> LogAnalyzer {
        facts error_pat = regex_vibez.compile(r"\[ERROR\].*").unwrap();
        facts warning_pat = regex_vibez.compile(r"\[WARN\].*").unwrap();
        facts timestamp_pat = regex_vibez.compile(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
        
        LogAnalyzer {
            error_pattern: error_pat,
            warning_pattern: warning_pat,
            timestamp_pattern: timestamp_pat
        }
    }
    
    slay analyze_log_line(tea line) -> tea {
        lowkey (self.error_pattern.match_string(line)) {
            yolo format!("🔴 {}", line);
        } bestie lowkey (self.warning_pattern.match_string(line)) {
            yolo format!("🟡 {}", line);
        } bestie {
            yolo format!("🟢 {}", line);
        }
    }
    
    slay extract_timestamps(tea text) -> []tea {
        yolo self.timestamp_pattern.find_all_string(text, -1);
    }
}

squad DataValidator {
    slay validate_email(tea email) -> (lit, tea) {
        lowkey (regex_vibez.EMAIL_PATTERN.match_string(email)) {
            yolo (based, "");
        } bestie {
            yolo (nocap, "Invalid email format");
        }
    }
    
    slay validate_phone(tea phone) -> (lit, tea) {
        lowkey (regex_vibez.PHONE_PATTERN.match_string(phone)) {
            yolo (based, "");
        } bestie {
            yolo (nocap, "Invalid phone format");
        }
    }
    
    slay validate_url(tea url) -> (lit, tea) {
        lowkey (regex_vibez.URL_PATTERN.match_string(url)) {
            yolo (based, "");
        } bestie {
            yolo (nocap, "Invalid URL format");
        }
    }
    
    slay sanitize_input(tea input) -> tea {
        // Remove potentially dangerous characters
        facts dangerous_pattern = regex_vibez.compile(r"[<>\"'&]").unwrap();
        yolo dangerous_pattern.replace_all_string(input, "");
    }
}

squad TextProcessor {
    slay extract_hashtags(tea text) -> []tea {
        yolo regex_vibez.HASHTAG_PATTERN.find_all_string(text, -1);
    }
    
    slay extract_mentions(tea text) -> []tea {
        facts mention_pattern = regex_vibez.compile(r"@([a-zA-Z0-9_]+)").unwrap();
        facts matches = mention_pattern.find_all_string_submatch(text, -1);
        facts mentions = Vec::new();
        
        tea match_group : matches {
            lowkey (match_group.len() > 1) {
                mentions.push(match_group[1].clone());
            }
        }
        
        yolo mentions;
    }
    
    slay format_currency(normie cents) -> tea {
        facts dollars = cents / 100;
        facts remaining_cents = cents % 100;
        yolo format!("${}.{:02}", dollars, remaining_cents);
    }
    
    slay parse_currency(tea text) -> Option<normie> {
        facts currency_pattern = regex_vibez.compile(r"\$(\d+)\.(\d{2})").unwrap();
        facts groups = currency_pattern.vibe_groups();
        facts matches = groups.find_groups_string(text);
        
        lowkey (matches.contains_key("1") && matches.contains_key("2")) {
            facts dollars: normie = matches.get("1").unwrap().parse().unwrap_or(0);
            facts cents: normie = matches.get("2").unwrap().parse().unwrap_or(0);
            yolo Some(dollars * 100 + cents);
        } bestie {
            yolo None;
        }
    }
}

squad ConfigParser {
    slay parse_config_file(tea content) -> HashMap<tea, tea> {
        facts config_pattern = regex_vibez.compile(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.*)$").unwrap();
        facts groups = config_pattern.vibe_groups();
        facts result = HashMap::new();
        
        facts lines = content.lines();
        tea line : lines {
            facts trimmed = line.trim();
            
            // Skip comments and empty lines
            lowkey (trimmed.starts_with("#") || trimmed.is_empty()) {
                persist;
            }
            
            facts matches = groups.find_groups_string(trimmed);
            lowkey (matches.len() >= 2) {
                facts key = matches.get("1").unwrap_or(&"".to_string()).clone();
                facts value = matches.get("2").unwrap_or(&"".to_string()).clone();
                
                // Remove quotes from value if present
                facts clean_value = self.remove_quotes(value);
                result.insert(key, clean_value);
            }
        }
        
        yolo result;
    }
    
    slay remove_quotes(tea value) -> tea {
        facts quote_pattern = regex_vibez.compile(r#"^["'](.*)["']$"#).unwrap();
        facts groups = quote_pattern.vibe_groups();
        facts matches = groups.find_groups_string(value);
        
        lowkey (matches.contains_key("1")) {
            yolo matches.get("1").unwrap().clone();
        } bestie {
            yolo value;
        }
    }
}

squad URLProcessor {
    slay extract_domain(tea url) -> Option<tea> {
        facts domain_pattern = regex_vibez.compile(r"https?://([^/]+)").unwrap();
        facts groups = domain_pattern.vibe_groups();
        facts matches = groups.find_groups_string(url);
        
        yolo matches.get("1").cloned();
    }
    
    slay extract_query_params(tea url) -> HashMap<tea, tea> {
        facts param_pattern = regex_vibez.compile(r"[?&]([^=]+)=([^&]*)").unwrap();
        facts groups = param_pattern.vibe_groups();
        facts all_matches = groups.find_all_groups_string(url, -1);
        facts params = HashMap::new();
        
        tea match_group : all_matches {
            lowkey (match_group.contains_key("1") && match_group.contains_key("2")) {
                facts key = match_group.get("1").unwrap().clone();
                facts value = match_group.get("2").unwrap().clone();
                params.insert(key, value);
            }
        }
        
        yolo params;
    }
    
    slay is_secure_url(tea url) -> lit {
        facts secure_pattern = regex_vibez.compile(r"^https://").unwrap();
        yolo secure_pattern.match_string(url);
    }
}

squad PasswordValidator {
    slay validate_strength(tea password) -> (lit, []tea) {
        facts issues = Vec::new();
        
        // Check length
        lowkey (password.len() < 8) {
            issues.push("Password must be at least 8 characters long".to_string());
        }
        
        // Check for lowercase
        facts has_lower = regex_vibez.compile(r"[a-z]").unwrap();
        lowkey (!has_lower.match_string(password)) {
            issues.push("Password must contain lowercase letters".to_string());
        }
        
        // Check for uppercase  
        facts has_upper = regex_vibez.compile(r"[A-Z]").unwrap();
        lowkey (!has_upper.match_string(password)) {
            issues.push("Password must contain uppercase letters".to_string());
        }
        
        // Check for digits
        facts has_digit = regex_vibez.compile(r"\d").unwrap();
        lowkey (!has_digit.match_string(password)) {
            issues.push("Password must contain numbers".to_string());
        }
        
        // Check for special characters
        facts has_special = regex_vibez.compile(r"[!@#$%^&*(),.?\":{}|<>]").unwrap();
        lowkey (!has_special.match_string(password)) {
            issues.push("Password must contain special characters".to_string());
        }
        
        // Check for common patterns
        facts has_common = regex_vibez.compile(r"(123|abc|password|qwerty)").unwrap();
        lowkey (has_common.match_string(&password.to_lowercase())) {
            issues.push("Password contains common patterns".to_string());
        }
        
        yolo (issues.is_empty(), issues);
    }
    
    slay generate_strength_score(tea password) -> normie {
        facts score = 0;
        
        // Length score
        score += std::cmp::min(password.len() / 2, 25);
        
        // Character variety score
        lowkey (regex_vibez.compile(r"[a-z]").unwrap().match_string(password)) { score += 5; }
        lowkey (regex_vibez.compile(r"[A-Z]").unwrap().match_string(password)) { score += 5; }
        lowkey (regex_vibez.compile(r"\d").unwrap().match_string(password)) { score += 5; }
        lowkey (regex_vibez.compile(r"[!@#$%^&*()]").unwrap().match_string(password)) { score += 10; }
        
        // Complexity bonus
        facts complexity_pattern = regex_vibez.compile(r"(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*()])").unwrap();
        lowkey (complexity_pattern.match_string(password)) { score += 25; }
        
        // Penalty for common patterns
        facts common_pattern = regex_vibez.compile(r"(123|abc|password)").unwrap();
        lowkey (common_pattern.match_string(&password.to_lowercase())) { score -= 20; }
        
        yolo std::cmp::max(0, std::cmp::min(100, score));
    }
}

slay demo_log_analysis() -> () {
    println("=== Log Analysis Demo ===");
    
    facts analyzer = LogAnalyzer::new();
    
    facts log_lines = [
        "2023-12-25 14:30:45 [INFO] Application started",
        "2023-12-25 14:30:46 [ERROR] Database connection failed", 
        "2023-12-25 14:30:47 [WARN] High memory usage detected",
        "2023-12-25 14:30:48 [INFO] User logged in successfully"
    ];
    
    tea line : log_lines {
        facts analyzed = analyzer.analyze_log_line(line);
        println(analyzed);
    }
    
    // Extract all timestamps
    facts all_logs = log_lines.join("\n");
    facts timestamps = analyzer.extract_timestamps(all_logs);
    println(format!("Found {} timestamps", timestamps.len()));
    
    println("");
}

slay demo_data_validation() -> () {
    println("=== Data Validation Demo ===");
    
    facts validator = DataValidator{};
    
    facts test_data = [
        ("email", "user@example.com"),
        ("email", "invalid-email"),
        ("phone", "(555) 123-4567"),
        ("phone", "not-a-phone"),
        ("url", "https://example.com"),
        ("url", "not-a-url")
    ];
    
    tea (data_type, value) : test_data {
        facts (is_valid, error) = lowkey data_type == "email" {
            validator.validate_email(value)
        } bestie lowkey data_type == "phone" {
            validator.validate_phone(value)  
        } bestie lowkey data_type == "url" {
            validator.validate_url(value)
        } bestie {
            (nocap, "Unknown type")
        };
        
        facts status = lowkey is_valid { "✓" } bestie { "✗" };
        println(format!("{} {} {}: {}", status, data_type, value, error));
    }
    
    println("");
}

slay demo_text_processing() -> () {
    println("=== Text Processing Demo ===");
    
    facts processor = TextProcessor{};
    
    facts social_text = "Hello @john! Check out #RegexVibez and #CURSED for awesome regex features! @admin please review.";
    
    facts hashtags = processor.extract_hashtags(social_text);
    facts mentions = processor.extract_mentions(social_text);
    
    println(format!("Text: {}", social_text));
    println("Hashtags found:");
    tea hashtag : hashtags {
        println(format!("  {}", hashtag));
    }
    println("Mentions found:");
    tea mention : mentions {
        println(format!("  @{}", mention));
    }
    
    // Currency processing
    facts price_text = "The item costs $29.99 and shipping is $5.50";
    facts currency_pattern = regex_vibez.compile(r"\$(\d+)\.(\d{2})").unwrap();
    facts prices = currency_pattern.find_all_string(price_text, -1);
    
    println(format!("Price text: {}", price_text));
    println("Prices found:");
    tea price : prices {
        println(format!("  {}", price));
    }
    
    println("");
}

slay demo_config_parsing() -> () {
    println("=== Configuration Parsing Demo ===");
    
    facts config_text = r#"
# Database configuration
database_host=localhost
database_port=5432
database_name="myapp_db"
database_user='dbuser'

# Server configuration  
server_port=8080
server_host=0.0.0.0
debug_mode=based

# Cache settings
cache_enabled=cap
cache_ttl=3600
"#;

    facts parser = ConfigParser{};
    facts config = parser.parse_config_file(config_text);
    
    println("Parsed configuration:");
    tea (key, value) : config {
        println(format!("  {} = {}", key, value));
    }
    
    println("");
}

slay demo_url_processing() -> () {
    println("=== URL Processing Demo ===");
    
    facts processor = URLProcessor{};
    
    facts test_url = "https://api.example.com/users?page=1&limit=10&sort=name";
    
    facts domain = processor.extract_domain(test_url);
    facts params = processor.extract_query_params(test_url);
    facts is_secure = processor.is_secure_url(test_url);
    
    println(format!("URL: {}", test_url));
    println(format!("Domain: {}", domain.unwrap_or("None".to_string())));
    println(format!("Secure: {}", is_secure));
    println("Query parameters:");
    tea (key, value) : params {
        println(format!("  {} = {}", key, value));
    }
    
    println("");
}

slay demo_password_validation() -> () {
    println("=== Password Validation Demo ===");
    
    facts validator = PasswordValidator{};
    
    facts test_passwords = [
        "weak",
        "Password123",
        "P@ssw0rd!",
        "VerySecureP@ssw0rd2023!",
        "password123"
    ];
    
    tea password : test_passwords {
        facts (is_strong, issues) = validator.validate_strength(password);
        facts score = validator.generate_strength_score(password);
        
        facts status = lowkey is_strong { "✓ STRONG" } bestie { "✗ WEAK" };
        println(format!("{} '{}' (Score: {})", status, password, score));
        
        lowkey (!issues.is_empty()) {
            tea issue : issues {
                println(format!("    - {}", issue));
            }
        }
    }
    
    println("");
}

slay main() -> () {
    println("RegexVibez Practical Examples");
    println("============================");
    println("");
    
    demo_log_analysis();
    demo_data_validation();
    demo_text_processing();
    demo_config_parsing();
    demo_url_processing();
    demo_password_validation();
    
    println("All practical demos completed! 🚀");
}
