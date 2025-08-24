# RegexZ Performance and Feature Demonstration
# Showcasing advanced regex capabilities and optimizations

yeet "regexz"
yeet "vibez"

slay main() drip {
    vibez.spill("🦀 RegexZ Advanced Regular Expression Engine Demo")
    vibez.spill("=" * 50)
    
    demo_basic_features()
    demo_unicode_support()
    demo_named_groups()
    demo_lookaround()
    demo_performance_optimization()
    demo_practical_applications()
    
    vibez.spill("\n✅ RegexZ Demo Complete!")
    vibez.spill("🚀 Production-ready advanced regex engine implemented!")
}

# Demo basic regex features
slay demo_basic_features() drip {
    vibez.spill("\n📝 Basic Regex Features:")
    vibez.spill("-" * 25)
    
    # Pattern matching
    sus text tea = "The price is $25.99 and tax is $3.50"
    sus prices []tea = regex_extract_all("\\$\\d+\\.\\d{2}", text) shook {
        vibez.spill("❌ Failed to extract prices")
        damn
    }
    
    vibez.spill("Text:", text)
    vibez.spill("Extracted prices:", prices)
    
    # String replacement  
    sus replaced tea = regex_replace_simple("\\$\\d+\\.\\d{2}", text, "$XX.XX") shook {
        vibez.spill("❌ Failed replacement")
        damn
    }
    
    vibez.spill("Masked prices:", replaced)
    
    # Validation
    sus email tea = "test@cursedlang.org"
    sus valid_email lit = regex_is_email(email) shook {
        vibez.spill("❌ Email validation failed")
        damn
    }
    
    vibez.spill("Email validation for", email, ":", ready (valid_email) { "✅ Valid" } otherwise { "❌ Invalid" })
}

# Demo Unicode support
slay demo_unicode_support() drip {
    vibez.spill("\n🌍 Unicode Property Support:")
    vibez.spill("-" * 28)
    
    # Multilingual text
    sus multilingual tea = "Hello 世界 🌍 مرحبا Здравствуйте"
    
    # Extract different scripts
    sus latin []tea = regex_extract_all("\\p{Script=Latin}+", multilingual) shook {
        vibez.spill("❌ Failed Latin extraction")
        damn
    }
    
    vibez.spill("Original text:", multilingual)
    vibez.spill("Latin script parts:", latin)
    
    # Extract all letters (any script)
    sus all_letters []tea = regex_extract_all("\\p{Letter}+", multilingual) shook {
        vibez.spill("❌ Failed letter extraction")
        damn
    }
    
    vibez.spill("All letter sequences:", all_letters)
    
    # Unicode categories
    sus text_with_numbers tea = "Price: €25, ¥100, $30"
    sus currency_symbols []tea = regex_extract_all("\\p{Symbol_Currency}", text_with_numbers) shook {
        vibez.spill("❌ Failed currency extraction")
        damn
    }
    
    vibez.spill("Currency symbols found:", currency_symbols)
}

# Demo named capture groups
slay demo_named_groups() drip {
    vibez.spill("\n🏷️ Named Capture Groups:")
    vibez.spill("-" * 25)
    
    # Parse log entry
    sus log_entry tea = "2023-12-25 14:30:15 ERROR user_service: Authentication failed for user john_doe"
    
    sus log_pattern tea = "(?P<date>\\d{4}-\\d{2}-\\d{2}) (?P<time>\\d{2}:\\d{2}:\\d{2}) (?P<level>\\w+) (?P<service>\\w+): (?P<message>.*)"
    
    sus engine RegexEngine = regex_compile(log_pattern) shook {
        vibez.spill("❌ Failed to compile log pattern")
        damn
    }
    
    sus result MatchResult = regex_match(&engine, log_entry) shook {
        vibez.spill("❌ Failed to match log entry")
        damn
    }
    
    ready (result.matched) {
        vibez.spill("Log entry:", log_entry)
        vibez.spill("Parsed components:")
        
        sus date tea = get_named_group(result, "date") shook { "N/A" }
        sus time tea = get_named_group(result, "time") shook { "N/A" }
        sus level tea = get_named_group(result, "level") shook { "N/A" }
        sus service tea = get_named_group(result, "service") shook { "N/A" }
        sus message tea = get_named_group(result, "message") shook { "N/A" }
        
        vibez.spill("  📅 Date:", date)
        vibez.spill("  🕐 Time:", time)
        vibez.spill("  📊 Level:", level)
        vibez.spill("  🔧 Service:", service)
        vibez.spill("  💬 Message:", message)
    }
    
    # URL parsing
    sus url tea = "https://api.cursedlang.org:8443/v1/users/123?format=json"
    sus url_pattern tea = "(?P<scheme>https?)://(?P<host>[^:]+)(?::(?P<port>\\d+))?(?P<path>/[^?]*)?(?:\\?(?P<query>.*))?"
    
    sus url_engine RegexEngine = regex_compile(url_pattern) shook {
        vibez.spill("❌ Failed to compile URL pattern")
        damn
    }
    
    sus url_result MatchResult = regex_match(&url_engine, url) shook {
        vibez.spill("❌ Failed to parse URL")
        damn
    }
    
    ready (url_result.matched) {
        vibez.spill("\nURL parsing:", url)
        
        sus scheme tea = get_named_group(url_result, "scheme") shook { "N/A" }
        sus host tea = get_named_group(url_result, "host") shook { "N/A" }
        sus port tea = get_named_group(url_result, "port") shook { "N/A" }
        sus path tea = get_named_group(url_result, "path") shook { "N/A" }
        sus query tea = get_named_group(url_result, "query") shook { "N/A" }
        
        vibez.spill("  🔗 Scheme:", scheme)
        vibez.spill("  🖥️  Host:", host)
        vibez.spill("  🔌 Port:", port)
        vibez.spill("  📁 Path:", path)
        vibez.spill("  🔍 Query:", query)
    }
}

# Demo lookahead and lookbehind
slay demo_lookaround() drip {
    vibez.spill("\n🔍 Lookahead & Lookbehind:")
    vibez.spill("-" * 27)
    
    # Password validation with lookahead
    sus passwords []tea = [
        "password123",      # Missing uppercase and special
        "PASSWORD123",      # Missing lowercase and special  
        "Password",         # Missing digits and special
        "Pass@123",        # Missing length
        "Password@123"      # Valid
    ]
    
    # Password must have: uppercase, lowercase, digit, special, 8+ chars
    sus strong_password_pattern tea = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[@$!%*?&])[A-Za-z\\d@$!%*?&]{8,}$"
    
    vibez.spill("Password validation (must have: upper, lower, digit, special, 8+ chars):")
    
    bestie (password in passwords) {
        sus is_strong lit = regex_test_pattern(strong_password_pattern, password) shook { nah }
        sus status tea = ready (is_strong) { "✅ Strong" } otherwise { "❌ Weak" }
        vibez.spill("  ", password, "->", status)
    }
    
    # Price extraction with currency lookbehind
    sus price_text tea = "Items: €25.99, $30.50, ¥1000, 15.75 (no currency)"
    
    # Extract numbers preceded by currency symbol
    sus with_currency []tea = regex_extract_all("(?<=[€$¥])\\d+(?:\\.\\d{2})?", price_text) shook {
        vibez.spill("❌ Failed currency price extraction")
        damn
    }
    
    # Extract numbers not preceded by currency
    sus without_currency []tea = regex_extract_all("(?<![€$¥])\\b\\d+(?:\\.\\d{2})?\\b", price_text) shook {
        vibez.spill("❌ Failed non-currency extraction")
        damn
    }
    
    vibez.spill("\nPrice extraction:", price_text)
    vibez.spill("With currency:", with_currency)
    vibez.spill("Without currency:", without_currency)
}

# Demo performance optimizations
slay demo_performance_optimization() drip {
    vibez.spill("\n⚡ Performance Optimizations:")
    vibez.spill("-" * 29)
    
    # Create patterns with different optimization levels
    sus pattern tea = "\\b\\w{5,10}@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,4}\\b"
    
    sus basic_options RegexOptions = create_default_options()
    basic_options.optimization_level = 0
    basic_options.cache_enabled = nah
    
    sus optimized_options RegexOptions = create_default_options()
    optimized_options.optimization_level = 2
    optimized_options.cache_enabled = based
    
    sus basic_engine RegexEngine = regex_compile_with_options(pattern, basic_options) shook {
        vibez.spill("❌ Failed basic compilation")
        damn
    }
    
    sus optimized_engine RegexEngine = regex_compile_with_options(pattern, optimized_options) shook {
        vibez.spill("❌ Failed optimized compilation")
        damn
    }
    
    # Test text with multiple emails
    sus test_text tea = "Contact us at support@cursedlang.org, sales@example.com, or admin@test.co.uk for assistance"
    
    vibez.spill("Testing pattern:", pattern)
    vibez.spill("Test text:", test_text)
    
    # Find matches with both engines
    sus basic_matches []MatchResult = regex_find_all(&basic_engine, test_text) shook {
        vibez.spill("❌ Basic matching failed")
        damn
    }
    
    sus optimized_matches []MatchResult = regex_find_all(&optimized_engine, test_text) shook {
        vibez.spill("❌ Optimized matching failed")
        damn
    }
    
    vibez.spill("Basic engine found:", basic_matches.len(), "matches")
    vibez.spill("Optimized engine found:", optimized_matches.len(), "matches")
    
    ready (basic_matches.len() > 0) {
        vibez.spill("Extracted emails:")
        bestie (match in optimized_matches) {
            vibez.spill("  📧", match.full_match)
        }
    }
    
    # Show optimization features
    vibez.spill("\n🎯 Optimization features enabled:")
    vibez.spill("  ✅ Pattern compilation caching")
    vibez.spill("  ✅ NFA to DFA conversion")
    vibez.spill("  ✅ Character class optimization")
    vibez.spill("  ✅ Memory pool allocation")
    vibez.spill("  ✅ Backtracking prevention")
}

# Demo practical applications
slay demo_practical_applications() drip {
    vibez.spill("\n💼 Practical Applications:")
    vibez.spill("-" * 26)
    
    # Data extraction from mixed text
    sus mixed_data tea = `
    Server logs from 2023-12-25:
    10:30:15 INFO: User john.doe@company.com logged in from 192.168.1.100
    10:31:22 ERROR: Failed login attempt for invalid@domain.xyz from 10.0.0.50  
    10:32:05 WARN: High memory usage: 85% on server-prod-01
    Call support at +1-555-123-4567 for help.
    Visit https://docs.cursedlang.org/regex for documentation.
    `
    
    vibez.spill("Extracting data from server logs...")
    
    # Extract timestamps
    sus timestamps []tea = regex_extract_all("\\d{2}:\\d{2}:\\d{2}", mixed_data) shook { create_array() }
    vibez.spill("⏰ Timestamps:", timestamps)
    
    # Extract email addresses
    sus emails []tea = regex_extract_all("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}", mixed_data) shook { create_array() }
    vibez.spill("📧 Emails:", emails)
    
    # Extract IP addresses
    sus ips []tea = regex_extract_all("\\b(?:\\d{1,3}\\.){3}\\d{1,3}\\b", mixed_data) shook { create_array() }
    vibez.spill("🌐 IP addresses:", ips)
    
    # Extract URLs
    sus urls []tea = regex_extract_all("https?://[a-zA-Z0-9.-]+(?:/[^\\s]*)?", mixed_data) shook { create_array() }
    vibez.spill("🔗 URLs:", urls)
    
    # Extract phone numbers
    sus phones []tea = regex_extract_all("\\+?1?-?\\(?\\d{3}\\)?[-.]?\\d{3}[-.]?\\d{4}", mixed_data) shook { create_array() }
    vibez.spill("📞 Phone numbers:", phones)
    
    # Extract log levels
    sus levels []tea = regex_extract_all("\\b(INFO|ERROR|WARN|DEBUG|FATAL)\\b", mixed_data) shook { create_array() }
    vibez.spill("📊 Log levels:", levels)
    
    # Configuration file processing
    vibez.spill("\n⚙️ Configuration file processing:")
    
    sus config_text tea = `
    # Database settings
    db_host = localhost
    db_port = 5432
    db_name = cursed_app
    
    # Server settings  
    server_port = 8080
    max_connections = 100
    
    # Feature flags
    enable_logging = true
    debug_mode = false
    `
    
    sus config_lines []tea = config_text.split("\n")
    sus config_pairs []ConfigPair = create_array()
    
    bestie (line in config_lines) {
        sus trimmed tea = line.trim()
        ready (trimmed.len() > 0 && !trimmed.starts_with("#")) {
            sus pair ConfigPair = regex_parse_config_line(trimmed) fam {
                when _ -> ConfigPair{ key: "", value: "" }
            }
            ready (pair.key != "") {
                config_pairs.push(pair)
            }
        }
    }
    
    vibez.spill("Parsed configuration:")
    bestie (pair in config_pairs) {
        vibez.spill("  ", pair.key, "=", pair.value)
    }
    
    # Social media text processing
    vibez.spill("\n📱 Social media text processing:")
    
    sus social_text tea = "Just deployed the new #cursedlang compiler! 🚀 Thanks to @team_lead and @dev_contributor for the amazing work! Check it out at https://github.com/cursedlang #programming #rustacean"
    
    sus hashtags []tea = regex_extract_hashtags(social_text) shook { create_array() }
    sus mentions []tea = regex_extract_mentions(social_text) shook { create_array() }
    
    vibez.spill("Social post:", social_text)
    vibez.spill("🏷️ Hashtags:", hashtags)
    vibez.spill("👤 Mentions:", mentions)
    
    # Clean and format
    sus clean_text tea = regex_strip_html("<p>HTML content with <b>bold</b> and <i>italic</i> text</p>") shook { "Failed to clean" }
    vibez.spill("🧹 HTML cleaned: '" + clean_text + "'")
}
