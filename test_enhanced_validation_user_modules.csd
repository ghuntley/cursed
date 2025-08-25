// Comprehensive Test Suite for Enhanced Validation and User Management Modules

yeet "validation/mod_enhanced"
yeet "user_check/mod_enhanced"
yeet "vibez"
yeet "timez"

// Test Results Tracking
be_like TestResult squad {
    test_name tea
    passed lit
    error_message tea
    execution_time drip
    severity tea
}

sus test_results []TestResult = []
sus total_tests drip = 0
sus passed_tests drip = 0

slay add_test_result(name tea, passed lit, error tea, exec_time drip, severity tea) {
    test_results = append(test_results, TestResult{
        test_name: name,
        passed: passed,
        error_message: error,
        execution_time: exec_time,
        severity: severity,
    })
    
    total_tests += 1
    ready passed {
        passed_tests += 1
    }
}

slay run_test(name tea, test_func slay() yikes<tea>) {
    vibez.spill("Running test:", name)
    sus start_time drip = timez.now_microseconds()
    
    sus test_error tea = test_func() fam {
        when err -> err
    }
    
    sus end_time drip = timez.now_microseconds()
    sus execution_time drip = end_time - start_time
    
    ready test_error == "" {
        vibez.spill("✓ PASSED:", name, "(" + tea(execution_time) + "μs)")
        add_test_result(name, based, "", execution_time, "info")
    } else {
        vibez.spill("✗ FAILED:", name, "-", test_error)
        add_test_result(name, cap, test_error, execution_time, "error")
    }
}

// ENHANCED VALIDATION MODULE TESTS

slay test_unicode_string_processing() yikes<tea> {
    // Test Unicode string creation and manipulation
    sus unicode_text tea = "Hello 世界! 🌍 Тест"
    sus u_str validation.UnicodeString = validation.create_unicode_string(unicode_text)
    
    ready u_str.byte_length != len(unicode_text) {
        yikes "Unicode byte length mismatch"
    }
    
    ready u_str.rune_length <= 0 {
        yikes "Unicode rune length should be positive"
    }
    
    // Test safe character access
    sus first_rune rune = validation.safe_char_at(unicode_text, 0) fam {
        when err -> yikes "Safe char access failed: " + err
    }
    
    ready first_rune != 'H' {
        yikes "First rune should be 'H'"
    }
    
    // Test safe substring
    sus substr tea = validation.safe_substring(unicode_text, 0, 5) fam {
        when err -> yikes "Safe substring failed: " + err
    }
    
    ready substr != "Hello" {
        yikes "Substring mismatch: expected 'Hello', got '" + substr + "'"
    }
    
    damn ""
}

slay test_input_sanitization() yikes<tea> {
    // Test malicious input sanitization
    sus malicious_input tea = "test\x00\x01\x02<script>alert('xss')</script>"
    sus sanitized tea = validation.sanitize_input(malicious_input, 100)
    
    // Should remove null bytes and control characters
    ready stringz.contains(sanitized, "\x00") {
        yikes "Null bytes not removed from sanitized input"
    }
    
    // Test length limiting
    sus long_input tea = stringz.repeat("a", 200)
    sus limited tea = validation.sanitize_input(long_input, 50)
    
    ready len(limited) > 50 {
        yikes "Input length not properly limited"
    }
    
    damn ""
}

slay test_enhanced_email_validation() yikes<tea> {
    // Test comprehensive email validation
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        session_id: "test_session",
        rate_limit_key: "email_test",
    }
    
    // Valid email test
    sus valid_email tea = "user@example.com"
    sus valid_result validation.ValidationResult = validation.validate_email_comprehensive(valid_email, &context)
    
    ready !valid_result.is_valid {
        sus errors tea = ""
        bestie (error := range valid_result.errors) {
            errors += error + "; "
        }
        yikes "Valid email failed validation: " + errors
    }
    
    // Invalid email tests
    sus invalid_emails []tea = [
        "",                          // Empty
        "invalid",                   // No @
        "user@",                     // No domain
        "@domain.com",               // No local part
        "user@domain",               // No TLD
        "user..double@domain.com",   // Consecutive dots
        "user@domain..com",          // Consecutive dots in domain
    ]
    
    bestie (invalid_email := range invalid_emails) {
        sus invalid_result validation.ValidationResult = validation.validate_email_comprehensive(invalid_email, &context)
        ready invalid_result.is_valid {
            yikes "Invalid email passed validation: " + invalid_email
        }
    }
    
    // Security test - dangerous characters
    sus dangerous_email tea = "user<script>@domain.com"
    sus dangerous_result validation.ValidationResult = validation.validate_email_comprehensive(dangerous_email, &context)
    
    ready dangerous_result.severity != "high" {
        yikes "Dangerous email should have high severity"
    }
    
    damn ""
}

slay test_password_security_validation() yikes<tea> {
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        rate_limit_key: "password_test",
    }
    
    // Strong password test
    sus strong_password tea = "MyStr0ng!P@ssw0rd2024"
    sus strong_result validation.ValidationResult = validation.validate_password_security(strong_password, &context)
    
    ready !strong_result.is_valid {
        sus errors tea = ""
        bestie (error := range strong_result.errors) {
            errors += error + "; "
        }
        yikes "Strong password failed validation: " + errors
    }
    
    // Weak password tests
    sus weak_passwords []tea = [
        "password",           // Common password
        "12345",             // Too short, only digits
        "abcdefgh",          // No uppercase, digits, special chars
        "PASSWORD",          // No lowercase, digits, special chars
        "Password1",         // No special characters
    ]
    
    bestie (weak_password := range weak_passwords) {
        sus weak_result validation.ValidationResult = validation.validate_password_security(weak_password, &context)
        ready weak_result.is_valid && weak_result.severity != "high" && weak_result.severity != "critical" {
            yikes "Weak password should not pass validation: " + weak_password
        }
    }
    
    // Test entropy calculation
    sus low_entropy_password tea = "aaaaaaaaaaaaa"
    sus entropy_result validation.ValidationResult = validation.validate_password_security(low_entropy_password, &context)
    
    ready entropy_result.is_valid {
        yikes "Low entropy password should fail validation"
    }
    
    damn ""
}

slay test_url_comprehensive_validation() yikes<tea> {
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        rate_limit_key: "url_test",
    }
    
    // Valid URL tests
    sus valid_urls []tea = [
        "https://www.example.com",
        "http://subdomain.example.com:8080/path?query=value",
        "ftp://files.example.com/file.txt",
        "https://unicode-ドメイン.com",
    ]
    
    bestie (valid_url := range valid_urls) {
        sus result validation.ValidationResult = validation.validate_url_comprehensive(valid_url, &context)
        ready !result.is_valid {
            sus errors tea = ""
            bestie (error := range result.errors) {
                errors += error + "; "
            }
            yikes "Valid URL failed validation: " + valid_url + " - " + errors
        }
    }
    
    // Security threat URLs
    sus malicious_urls []tea = [
        "http://example.com/../../../etc/passwd",     // Path traversal
        "javascript:alert('xss')",                    // JavaScript protocol
        "data:text/html,<script>alert('xss')</script>", // Data protocol
        "https://example.com/%2e%2e%2f%2e%2e%2f",    // Encoded path traversal
    ]
    
    bestie (malicious_url := range malicious_urls) {
        sus result validation.ValidationResult = validation.validate_url_comprehensive(malicious_url, &context)
        ready result.severity != "critical" {
            yikes "Malicious URL should have critical severity: " + malicious_url
        }
    }
    
    damn ""
}

slay test_ip_address_comprehensive_validation() yikes<tea> {
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        rate_limit_key: "ip_test",
    }
    
    // Valid IPv4 addresses
    sus valid_ipv4 []tea = [
        "192.168.1.1",
        "10.0.0.1", 
        "127.0.0.1",
        "255.255.255.255",
        "0.0.0.0",
    ]
    
    bestie (ip := range valid_ipv4) {
        sus result validation.ValidationResult = validation.validate_ip_comprehensive(ip, &context)
        ready !result.is_valid {
            yikes "Valid IPv4 failed validation: " + ip
        }
        
        ready result.metadata["ip_version"] != "4" {
            yikes "IPv4 detection failed for: " + ip
        }
    }
    
    // Valid IPv6 addresses
    sus valid_ipv6 []tea = [
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "2001:db8:85a3::8a2e:370:7334",
        "::1",
        "2001:db8::1",
    ]
    
    bestie (ip := range valid_ipv6) {
        sus result validation.ValidationResult = validation.validate_ip_comprehensive(ip, &context)
        ready !result.is_valid {
            yikes "Valid IPv6 failed validation: " + ip
        }
        
        ready result.metadata["ip_version"] != "6" {
            yikes "IPv6 detection failed for: " + ip
        }
    }
    
    // Invalid IP addresses
    sus invalid_ips []tea = [
        "256.256.256.256",   // IPv4 octets too large
        "192.168.1",         // IPv4 too few octets
        "192.168.1.1.1",     // IPv4 too many octets
        ":::",               // IPv6 too many colons
        "gggg::1",           // IPv6 invalid hex
    ]
    
    bestie (ip := range invalid_ips) {
        sus result validation.ValidationResult = validation.validate_ip_comprehensive(ip, &context)
        ready result.is_valid {
            yikes "Invalid IP passed validation: " + ip
        }
    }
    
    damn ""
}

slay test_validation_rate_limiting() yikes<tea> {
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        rate_limit_key: "rate_limit_test",
    }
    
    // Test that rate limiting works after many requests
    bestie (i := 0; i < 50; i += 1) {
        sus result validation.ValidationResult = validation.validate_email_comprehensive("test@example.com", &context)
        // Should work for first requests, then start rate limiting
        ready i < 40 && !result.is_valid && len(result.errors) > 0 {
            ready stringz.contains(result.errors[0], "Rate limit") {
                yikes "Rate limiting triggered too early at iteration: " + tea(i)
            }
        }
    }
    
    damn ""
}

// ENHANCED USER MANAGEMENT MODULE TESTS

slay test_secure_uid_gid_generation() yikes<tea> {
    // Test UID generation
    sus uid1 tea = user_check.generateSecureUID() fam {
        when err -> yikes "UID generation failed: " + err
    }
    
    sus uid2 tea = user_check.generateSecureUID() fam {
        when err -> yikes "UID generation failed: " + err
    }
    
    ready uid1 == uid2 {
        yikes "Generated UIDs should be unique"
    }
    
    // Verify UID is in valid range
    sus uid1_num drip = stringz.to_int(uid1) fam {
        when _ -> yikes "Generated UID is not numeric"
    }
    
    ready uid1_num < 1000 || uid1_num > 65535 {
        yikes "Generated UID out of valid range: " + uid1
    }
    
    // Test GID generation
    sus gid1 tea = user_check.generateSecureGID() fam {
        when err -> yikes "GID generation failed: " + err
    }
    
    sus gid1_num drip = stringz.to_int(gid1) fam {
        when _ -> yikes "Generated GID is not numeric"
    }
    
    ready gid1_num < 1000 || gid1_num > 65535 {
        yikes "Generated GID out of valid range: " + gid1
    }
    
    damn ""
}

slay test_password_hashing_algorithms() yikes<tea> {
    sus test_password tea = "TestP@ssw0rd123!"
    
    // Test Argon2 hashing
    sus argon2_hash tea = user_check.hashPasswordSecure(test_password, "argon2id") fam {
        when err -> yikes "Argon2 hashing failed: " + err
    }
    
    ready !stringz.starts_with(argon2_hash, "$argon2id$") {
        yikes "Argon2 hash format incorrect"
    }
    
    // Test bcrypt hashing
    sus bcrypt_hash tea = user_check.hashPasswordSecure(test_password, "bcrypt") fam {
        when err -> yikes "Bcrypt hashing failed: " + err
    }
    
    ready !stringz.starts_with(bcrypt_hash, "$2") {
        yikes "Bcrypt hash format incorrect"
    }
    
    // Test PBKDF2 hashing
    sus pbkdf2_hash tea = user_check.hashPasswordSecure(test_password, "pbkdf2_sha512") fam {
        when err -> yikes "PBKDF2 hashing failed: " + err
    }
    
    ready !stringz.starts_with(pbkdf2_hash, "$pbkdf2-sha512$") {
        yikes "PBKDF2 hash format incorrect"
    }
    
    // Verify hashes are different
    ready argon2_hash == bcrypt_hash || bcrypt_hash == pbkdf2_hash {
        yikes "Different hashing algorithms should produce different hashes"
    }
    
    damn ""
}

slay test_password_verification() yikes<tea> {
    sus test_password tea = "SecureP@ssw0rd2024!"
    
    // Test Argon2 verification
    sus argon2_hash tea = user_check.hashPasswordArgon2(test_password) fam {
        when err -> yikes "Argon2 hashing failed: " + err
    }
    
    sus argon2_valid lit = user_check.verifyArgon2Password(test_password, argon2_hash) fam {
        when err -> yikes "Argon2 verification failed: " + err
    }
    
    ready !argon2_valid {
        yikes "Argon2 password verification should succeed"
    }
    
    // Test with wrong password
    sus wrong_password tea = "WrongPassword123!"
    sus argon2_invalid lit = user_check.verifyArgon2Password(wrong_password, argon2_hash) fam {
        when err -> cap // Expected to fail
    }
    
    ready argon2_invalid {
        yikes "Argon2 password verification should fail for wrong password"
    }
    
    damn ""
}

slay test_username_validation() yikes<tea> {
    // Valid usernames
    sus valid_usernames []tea = [
        "user123",
        "test_user",
        "user-name",
        "user.name",
        "_underscore",
    ]
    
    bestie (username := range valid_usernames) {
        ready !user_check.isValidUsernameSecure(username) {
            yikes "Valid username rejected: " + username
        }
    }
    
    // Invalid usernames
    sus invalid_usernames []tea = [
        "",                    // Empty
        "123user",             // Starts with digit
        "-user",               // Starts with hyphen
        ".user",               // Starts with dot
        "user@domain",         // Contains @
        "user space",          // Contains space
        "root",                // Reserved username
        "admin",               // Reserved username
        "a",                   // Too short context
        stringz.repeat("a", 33), // Too long
    ]
    
    bestie (username := range invalid_usernames) {
        ready user_check.isValidUsernameSecure(username) {
            yikes "Invalid username accepted: " + username
        }
    }
    
    damn ""
}

slay test_authentication_security_features() yikes<tea> {
    // Test authentication with client info
    sus client_info map[tea]tea = map[tea]tea{
        "source_ip": "192.168.1.100",
        "user_agent": "TestUserAgent/1.0",
        "session_id": "test_session_123",
    }
    
    // This will likely fail since we don't have actual system users set up for testing
    // But we can test the validation and error handling
    sus auth_result user_check.AuthResult = user_check.authenticateUserSecure("nonexistent_user", "password", client_info) fam {
        when result -> result // Expected to fail
    }
    
    ready auth_result.success {
        yikes "Authentication should fail for nonexistent user"
    }
    
    ready auth_result.errorMessage != "USER_NOT_FOUND: nonexistent_user" {
        yikes "Expected USER_NOT_FOUND error, got: " + auth_result.errorMessage
    }
    
    damn ""
}

slay test_session_token_generation() yikes<tea> {
    // Test secure session token generation
    sus token1 tea = user_check.generateSecureSessionToken() fam {
        when err -> yikes "Session token generation failed: " + err
    }
    
    sus token2 tea = user_check.generateSecureSessionToken() fam {
        when err -> yikes "Session token generation failed: " + err
    }
    
    ready token1 == token2 {
        yikes "Session tokens should be unique"
    }
    
    ready len(token1) != 64 {
        yikes "Session token should be 64 characters (32 bytes hex): " + tea(len(token1))
    }
    
    // Verify token is valid hex
    sus hex_chars tea = "0123456789abcdef"
    bestie (i := 0; i < len(token1); i += 1) {
        sus char tea = stringz.to_lower(tea(token1[i]))
        ready !stringz.contains(hex_chars, char) {
            yikes "Session token contains non-hex character: " + char
        }
    }
    
    damn ""
}

slay test_constant_time_comparison() yikes<tea> {
    // Test constant-time string comparison
    sus string1 tea = "same_string"
    sus string2 tea = "same_string"
    sus string3 tea = "different_string"
    
    ready !user_check.constantTimeStringCompare(string1, string2) {
        yikes "Identical strings should compare equal"
    }
    
    ready user_check.constantTimeStringCompare(string1, string3) {
        yikes "Different strings should compare unequal"
    }
    
    ready user_check.constantTimeStringCompare(string1, "") {
        yikes "Different length strings should compare unequal"
    }
    
    damn ""
}

slay test_ip_security_analysis() yikes<tea> {
    // Test private IP detection
    sus private_ip tea = "192.168.1.1"
    ready !user_check.is_private_ip(private_ip) {
        yikes "192.168.1.1 should be detected as private IP"
    }
    
    sus public_ip tea = "8.8.8.8"
    ready user_check.is_private_ip(public_ip) {
        yikes "8.8.8.8 should not be detected as private IP"
    }
    
    // Test loopback detection
    sus loopback_ip tea = "127.0.0.1"
    ready !user_check.is_loopback_ip(loopback_ip) {
        yikes "127.0.0.1 should be detected as loopback"
    }
    
    // Test reserved IP detection
    sus reserved_ip tea = "224.0.0.1"
    ready !user_check.is_reserved_ip(reserved_ip) {
        yikes "224.0.0.1 should be detected as reserved (multicast)"
    }
    
    damn ""
}

slay test_validation_suite_execution() yikes<tea> {
    sus context validation.ValidationContext = validation.ValidationContext{
        source_ip: "127.0.0.1",
        rate_limit_key: "suite_test",
    }
    
    // Create multiple validation results
    sus validations []validation.ValidationResult = []
    
    // Add some passing validations
    validations = append(validations, validation.validate_email_comprehensive("valid@example.com", &context))
    validations = append(validations, validation.validate_ip_comprehensive("192.168.1.1", &context))
    
    // Add some failing validations  
    validations = append(validations, validation.validate_email_comprehensive("invalid_email", &context))
    validations = append(validations, validation.validate_password_security("weak", &context))
    
    // Execute validation suite
    sus suite_result validation.ValidationResult = validation.execute_validation_suite(validations, &context)
    
    ready suite_result.is_valid {
        yikes "Validation suite should fail when some validations fail"
    }
    
    ready len(suite_result.errors) == 0 {
        yikes "Validation suite should aggregate errors from failed validations"
    }
    
    // Check metadata
    ready suite_result.metadata["total_validations"] == "" {
        yikes "Validation suite should include total validation count in metadata"
    }
    
    damn ""
}

// MAIN TEST EXECUTION

slay main() {
    vibez.spill("=== CURSED Enhanced Validation & User Management Test Suite ===")
    vibez.spill("Starting comprehensive security module tests...")
    vibez.spill("")
    
    // Enhanced Validation Module Tests
    vibez.spill("--- Enhanced Validation Module Tests ---")
    run_test("Unicode String Processing", test_unicode_string_processing)
    run_test("Input Sanitization", test_input_sanitization)  
    run_test("Enhanced Email Validation", test_enhanced_email_validation)
    run_test("Password Security Validation", test_password_security_validation)
    run_test("URL Comprehensive Validation", test_url_comprehensive_validation)
    run_test("IP Address Comprehensive Validation", test_ip_address_comprehensive_validation)
    run_test("Validation Rate Limiting", test_validation_rate_limiting)
    run_test("Validation Suite Execution", test_validation_suite_execution)
    
    vibez.spill("")
    
    // Enhanced User Management Module Tests
    vibez.spill("--- Enhanced User Management Module Tests ---")
    run_test("Secure UID/GID Generation", test_secure_uid_gid_generation)
    run_test("Password Hashing Algorithms", test_password_hashing_algorithms)
    run_test("Password Verification", test_password_verification)
    run_test("Username Validation", test_username_validation)
    run_test("Authentication Security Features", test_authentication_security_features)
    run_test("Session Token Generation", test_session_token_generation)
    run_test("Constant Time Comparison", test_constant_time_comparison)
    run_test("IP Security Analysis", test_ip_security_analysis)
    
    vibez.spill("")
    vibez.spill("=== Test Results Summary ===")
    vibez.spill("Total tests:", total_tests)
    vibez.spill("Passed:", passed_tests)
    vibez.spill("Failed:", total_tests - passed_tests)
    
    ready passed_tests == total_tests {
        vibez.spill("🎉 ALL TESTS PASSED! Enhanced security modules are working correctly.")
    } else {
        vibez.spill("⚠️ Some tests failed. Review the results above.")
        
        vibez.spill("\nFailed Tests:")
        bestie (result := range test_results) {
            ready !result.passed {
                vibez.spill("- " + result.test_name + ": " + result.error_message)
            }
        }
    }
    
    // Performance summary
    sus total_execution_time drip = 0
    bestie (result := range test_results) {
        total_execution_time += result.execution_time
    }
    
    vibez.spill("\nPerformance Summary:")
    vibez.spill("Total execution time:", total_execution_time, "microseconds")
    vibez.spill("Average per test:", total_execution_time / total_tests, "microseconds")
    
    vibez.spill("\n=== Enhanced Security Module Testing Complete ===")
}
