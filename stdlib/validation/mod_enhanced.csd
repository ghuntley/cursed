// CURSED Enhanced Validation Module - Production Security Implementation
// Comprehensive validation with proper cryptographic security and performance optimizations

yeet "cryptz"
yeet "timez"
yeet "stringz"
yeet "mathz"
yeet "regexz"

// Enhanced validation result with detailed metadata
be_like ValidationResult squad {
    is_valid lit
    errors []tea
    warnings []tea  
    metadata map[tea]tea
    severity tea  // "low", "medium", "high", "critical"
    field_name tea
    validation_time drip // microseconds
}

// Security validation context for tracking validation attempts  
be_like ValidationContext squad {
    source_ip tea
    user_agent tea
    session_id tea
    rate_limit_key tea
    validation_count drip
    last_validation_time drip
}

// Global rate limiting for validation requests
sus validation_attempts map[tea]drip = make(map[tea]drip)
sus validation_last_reset drip = timez.now_unix()

// Enhanced string processing with Unicode support
be_like UnicodeString squad {
    data tea
    runes []rune
    byte_length drip
    rune_length drip
}

slay create_unicode_string(s tea) UnicodeString {
    sus runes []rune = stringz.to_runes(s)
    damn UnicodeString{
        data: s,
        runes: runes, 
        byte_length: len(s),
        rune_length: len(runes),
    }
}

// Secure string indexing with bounds checking
slay safe_char_at(str tea, index drip) yikes<rune> {
    ready index < 0 || index >= len(str) {
        yikes "string index out of bounds"
    }
    
    sus u_str UnicodeString = create_unicode_string(str)
    ready index >= u_str.rune_length {
        yikes "string index exceeds rune length"  
    }
    
    damn u_str.runes[index]
}

slay safe_substring(str tea, start drip, end drip) yikes<tea> {
    sus u_str UnicodeString = create_unicode_string(str)
    
    ready start < 0 || end < 0 || start > end {
        yikes "invalid substring range"
    }
    
    ready start >= u_str.rune_length || end > u_str.rune_length {
        yikes "substring range exceeds string length"
    }
    
    sus result_runes []rune = u_str.runes[start:end]
    damn stringz.from_runes(result_runes)
}

// Enhanced validation result creation with timing and metadata
slay create_enhanced_validation_result(field_name tea, context *ValidationContext) ValidationResult {
    sus start_time drip = timez.now_microseconds()
    
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        errors: []tea{},
        warnings: []tea{},
        metadata: make(map[tea]tea),
        severity: "low",
        field_name: field_name,
        validation_time: 0,
    }
    
    ready context != nil {
        result.metadata["source_ip"] = context.source_ip
        result.metadata["session_id"] = context.session_id
        result.metadata["validation_count"] = tea(context.validation_count)
    }
    
    result.metadata["created_at"] = tea(start_time)
    damn result
}

// Rate limiting for validation requests (prevent DoS)
slay check_validation_rate_limit(context *ValidationContext) yikes<tea> {
    ready context == nil || context.rate_limit_key == "" {
        damn ""  // No rate limiting
    }
    
    sus current_time drip = timez.now_unix()
    
    // Reset counters every minute
    ready current_time - validation_last_reset > 60 {
        validation_attempts = make(map[tea]drip)
        validation_last_reset = current_time
    }
    
    sus attempts drip = validation_attempts[context.rate_limit_key]
    ready attempts >= 100 { // Max 100 validations per minute per key
        yikes "Rate limit exceeded for validation requests"
    }
    
    validation_attempts[context.rate_limit_key] = attempts + 1
    damn ""
}

// Comprehensive input sanitization
slay sanitize_input(input tea, max_length drip) tea {
    ready len(input) > max_length {
        input = safe_substring(input, 0, max_length) fam {
            when _ -> input // Fallback to original if substring fails
        }
    }
    
    // Remove null bytes and control characters (security)
    sus sanitized tea = ""
    sus u_str UnicodeString = create_unicode_string(input)
    
    bestie (r := range u_str.runes) {
        ready r >= 32 && r != 127 { // Printable ASCII range
            sanitized += tea(r)
        } else ready r == 9 || r == 10 || r == 13 { // Allow tab, LF, CR
            sanitized += tea(r)
        }
        // Skip other control characters
    }
    
    damn sanitized
}

// Enhanced error and warning management
slay add_error_with_severity(result *ValidationResult, error tea, severity tea) {
    sus sanitized_error tea = sanitize_input(error, 1000)
    result.errors = append(result.errors, sanitized_error)
    result.is_valid = cap
    
    // Track highest severity
    sick severity {
        "critical" -> result.severity = "critical"
        "high" -> {
            ready result.severity != "critical" {
                result.severity = "high"
            }
        }
        "medium" -> {
            ready result.severity != "critical" && result.severity != "high" {
                result.severity = "medium"
            }
        }
        "low" -> {
            ready result.severity == "low" {
                result.severity = "low"
            }
        }
    }
}

slay add_warning_with_metadata(result *ValidationResult, warning tea, metadata map[tea]tea) {
    sus sanitized_warning tea = sanitize_input(warning, 1000)
    result.warnings = append(result.warnings, sanitized_warning)
    
    // Add metadata
    bestie (key, value := range metadata) {
        sus safe_key tea = sanitize_input(key, 100)
        sus safe_value tea = sanitize_input(value, 500)
        result.metadata[safe_key] = safe_value
    }
}

// ADVANCED STRING VALIDATION WITH REGEX SUPPORT

slay validate_regex_pattern(value tea, pattern tea, error_message tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("regex_validation", context)
    
    // Check rate limiting
    sus rate_check_err tea = check_validation_rate_limit(context) fam {
        when err -> {
            add_error_with_severity(&result, err, "high")
            damn result
        }
    }
    
    ready len(value) == 0 {
        add_error_with_severity(&result, "Value cannot be empty for regex validation", "medium")
        damn result
    }
    
    ready len(pattern) == 0 {
        add_error_with_severity(&result, "Regex pattern cannot be empty", "high")
        damn result  
    }
    
    // Compile regex with timeout protection
    sus compiled_regex regexz.Regex = regexz.compile(pattern) fam {
        when err -> {
            add_error_with_severity(&result, "Invalid regex pattern: " + err, "high")
            damn result
        }
    }
    
    // Execute regex with timeout (prevent ReDoS attacks)
    sus match_result lit = regexz.match_with_timeout(compiled_regex, value, 1000) fam { // 1 second timeout
        when err -> {
            add_error_with_severity(&result, "Regex execution timeout - possible ReDoS attack", "critical")
            damn result
        }
    }
    
    ready !match_result {
        sus safe_message tea = sanitize_input(error_message, 500)
        add_error_with_severity(&result, safe_message, "medium")
    }
    
    result.metadata["regex_pattern"] = pattern
    result.metadata["match_found"] = tea(match_result)
    damn result
}

// ENHANCED EMAIL VALIDATION WITH COMPREHENSIVE RFC COMPLIANCE

slay validate_email_comprehensive(email tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("email", context)
    
    // Rate limiting check
    check_validation_rate_limit(context) fam {
        when err -> {
            add_error_with_severity(&result, err, "high")
            damn result
        }
    }
    
    // Input sanitization
    sus clean_email tea = sanitize_input(email, 320) // RFC 5321 maximum length
    
    ready len(clean_email) == 0 {
        add_error_with_severity(&result, "Email address cannot be empty", "high")
        damn result
    }
    
    // Comprehensive RFC 5322 validation
    sus email_regex tea = "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    
    sus regex_result ValidationResult = validate_regex_pattern(clean_email, email_regex, "Invalid email format", context)
    ready !regex_result.is_valid {
        // Copy errors from regex validation
        bestie (error := range regex_result.errors) {
            add_error_with_severity(&result, error, "medium")
        }
        damn result
    }
    
    // Additional email-specific validations
    sus at_pos drip = stringz.index_of(clean_email, "@")
    ready at_pos == -1 {
        add_error_with_severity(&result, "Email must contain @ symbol", "high")
        damn result
    }
    
    sus local_part tea = safe_substring(clean_email, 0, at_pos) fam {
        when err -> {
            add_error_with_severity(&result, "Invalid email local part: " + err, "medium")
            damn result
        }
    }
    
    sus domain_part tea = safe_substring(clean_email, at_pos + 1, len(clean_email)) fam {
        when err -> {
            add_error_with_severity(&result, "Invalid email domain part: " + err, "medium")
            damn result
        }
    }
    
    // Local part validation (RFC 5321 - max 64 characters)
    ready len(local_part) > 64 {
        add_error_with_severity(&result, "Email local part too long (max 64 characters)", "medium")
    }
    
    // Domain part validation (RFC 5321 - max 253 characters)
    ready len(domain_part) > 253 {
        add_error_with_severity(&result, "Email domain part too long (max 253 characters)", "medium")
    }
    
    // Check for dangerous characters (security)
    sus dangerous_chars []tea = ["<", ">", "\"", "'", "&", ";", "|", "`"]
    bestie (char := range dangerous_chars) {
        ready stringz.contains(clean_email, char) {
            add_error_with_severity(&result, "Email contains potentially dangerous character: " + char, "high")
        }
    }
    
    // Domain validation - must have at least one dot and valid TLD
    ready !stringz.contains(domain_part, ".") {
        add_error_with_severity(&result, "Email domain must contain at least one dot", "medium")
    }
    
    // Check for valid TLD (basic check for common TLDs)
    sus domain_parts []tea = stringz.split(domain_part, ".")
    ready len(domain_parts) < 2 {
        add_error_with_severity(&result, "Email domain must have valid TLD", "medium")
    } else {
        sus tld tea = domain_parts[len(domain_parts) - 1]
        ready len(tld) < 2 || len(tld) > 6 {
            add_warning_with_metadata(&result, "Unusual TLD length detected", map[tea]tea{
                "tld": tld,
                "tld_length": tea(len(tld)),
            })
        }
    }
    
    // Add security metadata
    result.metadata["local_part_length"] = tea(len(local_part))
    result.metadata["domain_part_length"] = tea(len(domain_part))
    result.metadata["total_length"] = tea(len(clean_email))
    result.metadata["at_symbol_count"] = tea(stringz.count(clean_email, "@"))
    
    damn result
}

// SECURE PASSWORD VALIDATION WITH ENTROPY CALCULATION

slay validate_password_security(password tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("password", context)
    
    // Rate limiting (important for password validation to prevent brute force analysis)
    check_validation_rate_limit(context) fam {
        when err -> {
            add_error_with_severity(&result, err, "critical")
            damn result
        }
    }
    
    ready len(password) == 0 {
        add_error_with_severity(&result, "Password cannot be empty", "critical")
        damn result
    }
    
    ready len(password) < 12 {
        add_error_with_severity(&result, "Password must be at least 12 characters long", "high")
    }
    
    ready len(password) > 256 {
        add_error_with_severity(&result, "Password too long (max 256 characters)", "medium")
        damn result
    }
    
    // Character class analysis
    sus has_upper lit = cap
    sus has_lower lit = cap
    sus has_digit lit = cap
    sus has_special lit = cap
    sus has_unicode lit = cap
    sus char_counts map[rune]drip = make(map[rune]drip)
    
    sus u_str UnicodeString = create_unicode_string(password)
    
    bestie (r := range u_str.runes) {
        char_counts[r] = char_counts[r] + 1
        
        ready r >= 'A' && r <= 'Z' {
            has_upper = based
        } otherwise ready r >= 'a' && r <= 'z' {
            has_lower = based
        } otherwise ready r >= '0' && r <= '9' {
            has_digit = based
        } otherwise ready r > 127 {
            has_unicode = based
        } otherwise {
            has_special = based
        }
    }
    
    ready !has_upper {
        add_error_with_severity(&result, "Password must contain uppercase letters", "medium")
    }
    
    ready !has_lower {
        add_error_with_severity(&result, "Password must contain lowercase letters", "medium")  
    }
    
    ready !has_digit {
        add_error_with_severity(&result, "Password must contain digits", "medium")
    }
    
    ready !has_special {
        add_error_with_severity(&result, "Password must contain special characters", "medium")
    }
    
    // Calculate password entropy (Shannon entropy)
    sus entropy meal = calculate_shannon_entropy(char_counts, u_str.rune_length)
    
    ready entropy < 3.0 {
        add_error_with_severity(&result, "Password entropy too low - password is too predictable", "high")
    } else ready entropy < 4.0 {
        add_warning_with_metadata(&result, "Password entropy is low - consider a more complex password", map[tea]tea{
            "entropy": tea(entropy),
            "recommendation": "Add more character variety",
        })
    }
    
    // Check for common patterns
    ready check_common_patterns(password) {
        add_error_with_severity(&result, "Password contains common patterns or sequences", "high")
    }
    
    // Dictionary attack resistance (basic check)
    ready check_common_passwords(password) {
        add_error_with_severity(&result, "Password appears in common password dictionary", "critical")
    }
    
    // Add security metadata
    result.metadata["length"] = tea(u_str.rune_length)
    result.metadata["entropy"] = tea(entropy)
    result.metadata["has_upper"] = tea(has_upper)
    result.metadata["has_lower"] = tea(has_lower)
    result.metadata["has_digit"] = tea(has_digit)
    result.metadata["has_special"] = tea(has_special) 
    result.metadata["has_unicode"] = tea(has_unicode)
    result.metadata["unique_chars"] = tea(len(char_counts))
    
    damn result
}

// Shannon entropy calculation for password strength
slay calculate_shannon_entropy(char_counts map[rune]drip, total_chars drip) meal {
    ready total_chars == 0 {
        damn 0.0
    }
    
    sus entropy meal = 0.0
    
    bestie (_, count := range char_counts) {
        sus probability meal = meal(count) / meal(total_chars)
        ready probability > 0.0 {
            entropy -= probability * mathz.log2(probability)
        }
    }
    
    damn entropy
}

// Check for common password patterns
slay check_common_patterns(password tea) lit {
    sus lower_pass tea = stringz.to_lower(password)
    
    // Sequential patterns
    sus sequences []tea = [
        "123456", "abcdef", "qwerty", "asdf", "zxcv",
        "098765", "fedcba", "ytrewq", "fdsa", "vcxz",
    ]
    
    bestie (seq := range sequences) {
        ready stringz.contains(lower_pass, seq) {
            damn based
        }
    }
    
    // Keyboard patterns  
    sus keyboard_patterns []tea = [
        "qwertyuiop", "asdfghjkl", "zxcvbnm",
        "poiuytrewq", "lkjhgfdsa", "mnbvcxz",
    ]
    
    bestie (pattern := range keyboard_patterns) {
        ready stringz.contains(lower_pass, pattern) {
            damn based
        }
    }
    
    // Date patterns (YYYY, MM/DD/YYYY, etc.)
    sus date_regex tea = "\\b(19|20)\\d{2}\\b|\\b\\d{1,2}[/.-]\\d{1,2}[/.-]\\d{2,4}\\b"
    sus date_match lit = regexz.match_string(date_regex, password) fam {
        when _ -> cap
    }
    
    damn date_match
}

// Check against common password dictionary (simplified)
slay check_common_passwords(password tea) lit {
    sus lower_pass tea = stringz.to_lower(password)
    
    sus common_passwords []tea = [
        "password", "123456", "password123", "admin", "letmein",
        "welcome", "monkey", "dragon", "master", "hello",
        "freedom", "whatever", "qwerty", "trustno1", "jordan23",
        "harley", "robert", "matthew", "jordan", "michelle",
    ]
    
    bestie (common := range common_passwords) {
        ready lower_pass == common || stringz.contains(lower_pass, common) {
            damn based
        }
    }
    
    damn cap
}

// ENHANCED URL VALIDATION WITH SECURITY CHECKS

slay validate_url_comprehensive(url tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("url", context)
    
    check_validation_rate_limit(context) fam {
        when err -> {
            add_error_with_severity(&result, err, "high")
            damn result
        }
    }
    
    sus clean_url tea = sanitize_input(url, 2048) // Reasonable URL length limit
    
    ready len(clean_url) == 0 {
        add_error_with_severity(&result, "URL cannot be empty", "medium")
        damn result
    }
    
    // Comprehensive URL regex with protocol, domain, and path validation
    sus url_regex tea = "^(https?|ftp|ftps|file)://[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?([.][a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*([:]\\d{1,5})?([/?#].*)?$"
    
    sus regex_result ValidationResult = validate_regex_pattern(clean_url, url_regex, "Invalid URL format", context)
    ready !regex_result.is_valid {
        bestie (error := range regex_result.errors) {
            add_error_with_severity(&result, error, "medium")
        }
        damn result
    }
    
    // Extract URL components for detailed validation
    sus protocol tea = extract_url_protocol(clean_url)
    sus domain tea = extract_url_domain(clean_url)
    sus port tea = extract_url_port(clean_url)
    sus path tea = extract_url_path(clean_url)
    
    // Protocol security validation
    ready protocol != "https" {
        ready protocol == "http" {
            add_warning_with_metadata(&result, "HTTP URLs are not encrypted - consider HTTPS", map[tea]tea{
                "security_risk": "unencrypted_transmission",
                "recommendation": "Use HTTPS for sensitive data",
            })
        } otherwise ready protocol == "ftp" || protocol == "ftps" {
            add_warning_with_metadata(&result, "FTP protocols detected", map[tea]tea{
                "protocol": protocol,
                "security_note": "FTP should be used carefully",
            })
        } otherwise ready protocol == "file" {
            add_warning_with_metadata(&result, "File protocol detected - may have security implications", map[tea]tea{
                "protocol": protocol,
                "security_risk": "local_file_access",
            })
        }
    }
    
    // Domain validation
    ready len(domain) > 253 {
        add_error_with_severity(&result, "Domain name too long (max 253 characters)", "medium")
    }
    
    // Check for suspicious domains or IPs
    ready is_suspicious_domain(domain) {
        add_warning_with_metadata(&result, "Potentially suspicious domain detected", map[tea]tea{
            "domain": domain,
            "risk_type": "suspicious_domain",
        })
    }
    
    // Port validation 
    ready port != "" {
        sus port_num drip = stringz.to_int(port) fam {
            when _ -> -1
        }
        
        ready port_num <= 0 || port_num > 65535 {
            add_error_with_severity(&result, "Invalid port number", "medium")
        } otherwise ready is_suspicious_port(port_num) {
            add_warning_with_metadata(&result, "Unusual port detected", map[tea]tea{
                "port": port,
                "risk_note": "Non-standard port usage",
            })
        }
    }
    
    // Path validation for security
    ready stringz.contains(path, "../") {
        add_error_with_severity(&result, "URL contains path traversal sequences", "critical")
    }
    
    ready stringz.contains(path, "%2e%2e%2f") {
        add_error_with_severity(&result, "URL contains encoded path traversal sequences", "critical")
    }
    
    // Check for malicious query parameters
    ready stringz.contains(clean_url, "javascript:") || stringz.contains(clean_url, "data:") {
        add_error_with_severity(&result, "URL contains potentially malicious protocol", "critical")
    }
    
    // Add metadata
    result.metadata["protocol"] = protocol
    result.metadata["domain"] = domain
    result.metadata["port"] = port
    result.metadata["path_length"] = tea(len(path))
    result.metadata["total_length"] = tea(len(clean_url))
    
    damn result
}

// URL component extraction helpers
slay extract_url_protocol(url tea) tea {
    sus protocol_end drip = stringz.index_of(url, "://")
    ready protocol_end == -1 {
        damn ""
    }
    damn safe_substring(url, 0, protocol_end) fam {
        when _ -> ""
    }
}

slay extract_url_domain(url tea) tea {
    sus protocol_end drip = stringz.index_of(url, "://")
    ready protocol_end == -1 {
        damn ""
    }
    
    sus start drip = protocol_end + 3
    sus domain_with_path tea = safe_substring(url, start, len(url)) fam {
        when _ -> ""
    }
    
    // Find end of domain (before port, path, query, or fragment)
    sus domain_end drip = len(domain_with_path)
    
    bestie (char := range []tea{":", "/", "?", "#"}) {
        sus pos drip = stringz.index_of(domain_with_path, char)
        ready pos != -1 && pos < domain_end {
            domain_end = pos
        }
    }
    
    damn safe_substring(domain_with_path, 0, domain_end) fam {
        when _ -> ""
    }
}

slay extract_url_port(url tea) tea {
    sus domain tea = extract_url_domain(url)
    sus full_domain_part tea = domain
    
    // Find port after domain
    sus protocol_end drip = stringz.index_of(url, "://")
    ready protocol_end != -1 {
        sus start drip = protocol_end + 3
        sus after_protocol tea = safe_substring(url, start, len(url)) fam {
            when _ -> ""
        }
        
        sus colon_pos drip = stringz.index_of(after_protocol, ":")
        ready colon_pos != -1 {
            sus port_start drip = colon_pos + 1
            sus port_part tea = safe_substring(after_protocol, port_start, len(after_protocol)) fam {
                when _ -> ""
            }
            
            // Extract port until next delimiter
            sus port_end drip = len(port_part)
            bestie (char := range []tea{"/", "?", "#"}) {
                sus pos drip = stringz.index_of(port_part, char)
                ready pos != -1 && pos < port_end {
                    port_end = pos
                }
            }
            
            damn safe_substring(port_part, 0, port_end) fam {
                when _ -> ""
            }
        }
    }
    
    damn ""
}

slay extract_url_path(url tea) tea {
    sus domain_and_port tea = extract_url_domain(url) + ":" + extract_url_port(url)
    sus protocol_end drip = stringz.index_of(url, "://")
    ready protocol_end == -1 {
        damn ""
    }
    
    sus domain_start drip = protocol_end + 3
    sus path_start drip = domain_start + len(extract_url_domain(url))
    
    // Account for port if present
    sus port tea = extract_url_port(url)
    ready port != "" {
        path_start += len(":") + len(port)
    }
    
    ready path_start >= len(url) {
        damn ""
    }
    
    damn safe_substring(url, path_start, len(url)) fam {
        when _ -> ""
    }
}

// Security helper functions
slay is_suspicious_domain(domain tea) lit {
    sus suspicious_patterns []tea = [
        "bit.ly", "tinyurl", "t.co", "goo.gl", // URL shorteners
        "tempmail", "10minutemail", "guerrillamail", // Temp email services
        ".tk", ".ml", ".ga", ".cf", // Free TLD services
        "localhost", "127.0.0.1", "0.0.0.0", // Local addresses
    ]
    
    sus lower_domain tea = stringz.to_lower(domain)
    
    bestie (pattern := range suspicious_patterns) {
        ready stringz.contains(lower_domain, pattern) {
            damn based
        }
    }
    
    damn cap
}

slay is_suspicious_port(port drip) lit {
    // Common suspicious ports used for malware, backdoors, etc.
    sus suspicious_ports []drip = [
        1337, 4444, 5555, 6666, 7777, 8888, 9999, // Common backdoor ports
        31337, 12345, 54321, 65000, // Hacker ports
        4445, 5554, 9995, // Variations
    ]
    
    bestie (suspicious := range suspicious_ports) {
        ready port == suspicious {
            damn based
        }
    }
    
    damn cap
}

// ENHANCED IP ADDRESS VALIDATION WITH GEOLOCATION AND SECURITY CHECKS

slay validate_ip_comprehensive(ip tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("ip_address", context)
    
    check_validation_rate_limit(context) fam {
        when err -> {
            add_error_with_severity(&result, err, "high")
            damn result
        }
    }
    
    sus clean_ip tea = sanitize_input(ip, 45) // IPv6 max length is 39, IPv4 is 15
    
    ready len(clean_ip) == 0 {
        add_error_with_severity(&result, "IP address cannot be empty", "medium")
        damn result
    }
    
    // Determine IP version and validate accordingly
    ready stringz.contains(clean_ip, ":") {
        // IPv6 validation
        sus ipv6_result ValidationResult = validate_ipv6_address(clean_ip, context)
        // Copy validation results
        result.is_valid = ipv6_result.is_valid
        result.errors = ipv6_result.errors
        result.warnings = ipv6_result.warnings
        result.metadata["ip_version"] = "6"
    } else {
        // IPv4 validation
        sus ipv4_result ValidationResult = validate_ipv4_address(clean_ip, context)
        result.is_valid = ipv4_result.is_valid
        result.errors = ipv4_result.errors
        result.warnings = ipv4_result.warnings
        result.metadata["ip_version"] = "4"
    }
    
    ready result.is_valid {
        // Additional security checks for valid IPs
        sus security_analysis map[tea]tea = analyze_ip_security(clean_ip)
        
        bestie (key, value := range security_analysis) {
            result.metadata[key] = value
        }
        
        // Warn about private/reserved addresses if in public context
        ready context != nil && context.source_ip != "" && context.source_ip != clean_ip {
            ready is_private_ip(clean_ip) {
                add_warning_with_metadata(&result, "Private IP address detected", map[tea]tea{
                    "ip_class": "private",
                    "security_note": "Private IPs not routable on internet",
                })
            }
        }
    }
    
    result.metadata["original_input"] = ip
    result.metadata["sanitized_input"] = clean_ip
    
    damn result
}

slay validate_ipv4_address(ip tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("ipv4", context)
    
    // IPv4 regex: 4 octets, each 0-255
    sus ipv4_regex tea = "^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"
    
    sus regex_result ValidationResult = validate_regex_pattern(ip, ipv4_regex, "Invalid IPv4 address format", context)
    ready !regex_result.is_valid {
        result.is_valid = cap
        result.errors = regex_result.errors
        damn result
    }
    
    // Parse octets for additional validation
    sus octets []tea = stringz.split(ip, ".")
    ready len(octets) != 4 {
        add_error_with_severity(&result, "IPv4 address must have exactly 4 octets", "medium")
        damn result
    }
    
    sus parsed_octets []drip = []
    bestie (octet_str := range octets) {
        sus octet drip = stringz.to_int(octet_str) fam {
            when _ -> {
                add_error_with_severity(&result, "Invalid octet: " + octet_str, "medium")
                damn result
            }
        }
        
        ready octet < 0 || octet > 255 {
            add_error_with_severity(&result, "Octet out of range (0-255): " + tea(octet), "medium")
            damn result
        }
        
        // Check for leading zeros (not allowed except for single 0)
        ready len(octet_str) > 1 && octet_str[0] == '0' {
            add_error_with_severity(&result, "Octets cannot have leading zeros: " + octet_str, "medium")
            damn result
        }
        
        parsed_octets = append(parsed_octets, octet)
    }
    
    // Store parsed octets in metadata
    result.metadata["octet1"] = tea(parsed_octets[0])
    result.metadata["octet2"] = tea(parsed_octets[1])
    result.metadata["octet3"] = tea(parsed_octets[2])
    result.metadata["octet4"] = tea(parsed_octets[3])
    
    damn result
}

slay validate_ipv6_address(ip tea, context *ValidationContext) ValidationResult {
    sus result ValidationResult = create_enhanced_validation_result("ipv6", context)
    
    // IPv6 is complex - simplified validation for key patterns
    sus colon_count drip = stringz.count(ip, ":")
    sus double_colon_count drip = stringz.count(ip, "::")
    
    ready colon_count < 2 || colon_count > 7 {
        add_error_with_severity(&result, "IPv6 address must have 2-7 colons", "medium")
        damn result
    }
    
    ready double_colon_count > 1 {
        add_error_with_severity(&result, "IPv6 address can have at most one double colon (::)", "medium")  
        damn result
    }
    
    // Validate hex digits
    sus hex_chars tea = "0123456789abcdefABCDEF:"
    bestie (i := 0; i < len(ip); i += 1) {
        sus char tea = tea(ip[i])
        ready !stringz.contains(hex_chars, char) {
            add_error_with_severity(&result, "IPv6 address contains invalid character: " + char, "medium")
            damn result
        }
    }
    
    // Basic structure validation
    ready stringz.starts_with(ip, ":") && !stringz.starts_with(ip, "::") {
        add_error_with_severity(&result, "IPv6 address cannot start with single colon", "medium")
        damn result
    }
    
    ready stringz.ends_with(ip, ":") && !stringz.ends_with(ip, "::") {
        add_error_with_severity(&result, "IPv6 address cannot end with single colon", "medium")
        damn result
    }
    
    result.metadata["colon_count"] = tea(colon_count)
    result.metadata["has_double_colon"] = tea(double_colon_count > 0)
    
    damn result
}

// IP security analysis
slay analyze_ip_security(ip tea) map[tea]tea {
    sus analysis map[tea]tea = make(map[tea]tea)
    
    analysis["is_private"] = tea(is_private_ip(ip))
    analysis["is_reserved"] = tea(is_reserved_ip(ip))
    analysis["is_loopback"] = tea(is_loopback_ip(ip))
    analysis["is_multicast"] = tea(is_multicast_ip(ip))
    analysis["is_link_local"] = tea(is_link_local_ip(ip))
    
    // Geolocation analysis (simplified)
    analysis["estimated_region"] = estimate_ip_region(ip)
    
    damn analysis
}

slay is_private_ip(ip tea) lit {
    ready stringz.contains(ip, ":") {
        // IPv6 private ranges
        ready stringz.starts_with(ip, "fc") || stringz.starts_with(ip, "fd") {
            damn based // Unique local addresses
        }
        damn cap
    } else {
        // IPv4 private ranges: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        sus octets []tea = stringz.split(ip, ".")
        ready len(octets) == 4 {
            sus first drip = stringz.to_int(octets[0]) fam { when _ -> 0 }
            sus second drip = stringz.to_int(octets[1]) fam { when _ -> 0 }
            
            ready first == 10 {
                damn based
            }
            
            ready first == 172 && second >= 16 && second <= 31 {
                damn based
            }
            
            ready first == 192 && second == 168 {
                damn based
            }
        }
    }
    
    damn cap
}

slay is_reserved_ip(ip tea) lit {
    ready stringz.contains(ip, ":") {
        // IPv6 reserved ranges
        ready stringz.starts_with(ip, "::") || ip == "::1" {
            damn based
        }
        ready stringz.starts_with(ip, "2001:db8") {
            damn based // Documentation prefix
        }
        damn cap
    } else {
        // IPv4 reserved ranges
        sus octets []tea = stringz.split(ip, ".")
        ready len(octets) == 4 {
            sus first drip = stringz.to_int(octets[0]) fam { when _ -> 0 }
            
            ready first == 0 || first == 127 || first >= 224 {
                damn based
            }
            
            ready first == 169 {
                sus second drip = stringz.to_int(octets[1]) fam { when _ -> 0 }
                ready second == 254 {
                    damn based // Link-local
                }
            }
        }
    }
    
    damn cap
}

slay is_loopback_ip(ip tea) lit {
    ready ip == "::1" || stringz.starts_with(ip, "127.") {
        damn based
    }
    damn cap
}

slay is_multicast_ip(ip tea) lit {
    ready stringz.starts_with(ip, "ff") {
        damn based // IPv6 multicast
    }
    
    sus octets []tea = stringz.split(ip, ".")
    ready len(octets) == 4 {
        sus first drip = stringz.to_int(octets[0]) fam { when _ -> 0 }
        ready first >= 224 && first <= 239 {
            damn based // IPv4 multicast
        }
    }
    
    damn cap
}

slay is_link_local_ip(ip tea) lit {
    ready stringz.starts_with(ip, "fe8") || stringz.starts_with(ip, "fe9") || 
          stringz.starts_with(ip, "fea") || stringz.starts_with(ip, "feb") {
        damn based // IPv6 link-local
    }
    
    sus octets []tea = stringz.split(ip, ".")
    ready len(octets) == 4 {
        sus first drip = stringz.to_int(octets[0]) fam { when _ -> 0 }
        sus second drip = stringz.to_int(octets[1]) fam { when _ -> 0 }
        ready first == 169 && second == 254 {
            damn based // IPv4 link-local
        }
    }
    
    damn cap
}

slay estimate_ip_region(ip tea) tea {
    // Simplified IP geolocation - in production would use GeoIP database
    ready is_private_ip(ip) || is_reserved_ip(ip) || is_loopback_ip(ip) {
        damn "local"
    }
    
    ready stringz.contains(ip, ":") {
        damn "global_ipv6"
    }
    
    // Basic IPv4 regional estimation based on first octet
    sus octets []tea = stringz.split(ip, ".")
    ready len(octets) == 4 {
        sus first drip = stringz.to_int(octets[0]) fam { when _ -> 0 }
        
        ready first >= 1 && first <= 49 {
            damn "north_america"
        } otherwise ready first >= 50 && first <= 99 {
            damn "europe"  
        } otherwise ready first >= 100 && first <= 149 {
            damn "asia_pacific"
        } otherwise ready first >= 150 && first <= 199 {
            damn "mixed_region"
        } otherwise {
            damn "unknown_region"
        }
    }
    
    damn "unknown"
}

// FINALIZED VALIDATION EXECUTION WITH COMPREHENSIVE REPORTING

slay execute_validation_suite(validations []ValidationResult, context *ValidationContext) ValidationResult {
    sus start_time drip = timez.now_microseconds()
    sus suite_result ValidationResult = create_enhanced_validation_result("validation_suite", context)
    
    sus total_errors drip = 0
    sus total_warnings drip = 0
    sus highest_severity tea = "low"
    sus failed_validations drip = 0
    
    bestie (validation := range validations) {
        ready !validation.is_valid {
            failed_validations += 1
            suite_result.is_valid = cap
            
            // Aggregate errors
            bestie (error := range validation.errors) {
                suite_result.errors = append(suite_result.errors, 
                    "[" + validation.field_name + "] " + error)
                total_errors += 1
            }
        }
        
        // Aggregate warnings
        bestie (warning := range validation.warnings) {
            suite_result.warnings = append(suite_result.warnings,
                "[" + validation.field_name + "] " + warning)
            total_warnings += 1
        }
        
        // Track highest severity
        ready validation.severity == "critical" {
            highest_severity = "critical"
        } otherwise ready validation.severity == "high" && highest_severity != "critical" {
            highest_severity = "high"
        } otherwise ready validation.severity == "medium" && highest_severity != "critical" && highest_severity != "high" {
            highest_severity = "medium"
        }
        
        // Aggregate metadata
        bestie (key, value := range validation.metadata) {
            suite_result.metadata[validation.field_name + "_" + key] = value
        }
    }
    
    suite_result.severity = highest_severity
    
    // Add suite-level metadata
    sus end_time drip = timez.now_microseconds()
    suite_result.validation_time = end_time - start_time
    suite_result.metadata["total_validations"] = tea(len(validations))
    suite_result.metadata["failed_validations"] = tea(failed_validations) 
    suite_result.metadata["total_errors"] = tea(total_errors)
    suite_result.metadata["total_warnings"] = tea(total_warnings)
    suite_result.metadata["execution_time_microseconds"] = tea(suite_result.validation_time)
    suite_result.metadata["highest_severity"] = highest_severity
    
    ready context != nil {
        suite_result.metadata["context_source_ip"] = context.source_ip
        suite_result.metadata["context_session_id"] = context.session_id
    }
    
    damn suite_result
}

// Export comprehensive validation report
slay generate_validation_report(result ValidationResult) tea {
    sus report tea = "=== CURSED Validation Report ===\n\n"
    
    report += "Status: " + ready result.is_valid { "VALID" } else { "INVALID" } + "\n"
    report += "Severity: " + stringz.to_upper(result.severity) + "\n"
    report += "Field: " + result.field_name + "\n"
    report += "Execution Time: " + result.metadata["execution_time_microseconds"] + " μs\n"
    report += "Timestamp: " + timez.format_rfc3339(timez.now()) + "\n\n"
    
    ready len(result.errors) > 0 {
        report += "ERRORS (" + tea(len(result.errors)) + "):\n"
        bestie (i, error := range result.errors) {
            report += "  " + tea(i+1) + ". " + error + "\n"
        }
        report += "\n"
    }
    
    ready len(result.warnings) > 0 {
        report += "WARNINGS (" + tea(len(result.warnings)) + "):\n"
        bestie (i, warning := range result.warnings) {
            report += "  " + tea(i+1) + ". " + warning + "\n"
        }
        report += "\n"
    }
    
    ready len(result.metadata) > 0 {
        report += "METADATA:\n"
        bestie (key, value := range result.metadata) {
            report += "  " + key + ": " + value + "\n"
        }
        report += "\n"
    }
    
    report += "=== End Report ===\n"
    
    damn report
}

// Utility functions for quick validation
slay is_valid_email_secure(email tea, context *ValidationContext) lit {
    sus result ValidationResult = validate_email_comprehensive(email, context)
    damn result.is_valid
}

slay is_strong_password(password tea, context *ValidationContext) lit {
    sus result ValidationResult = validate_password_security(password, context)
    damn result.is_valid && result.severity != "critical" && result.severity != "high"
}

slay is_safe_url(url tea, context *ValidationContext) lit {
    sus result ValidationResult = validate_url_comprehensive(url, context)
    damn result.is_valid && result.severity != "critical"
}

slay is_valid_ip_secure(ip tea, context *ValidationContext) lit {
    sus result ValidationResult = validate_ip_comprehensive(ip, context)
    damn result.is_valid
}
