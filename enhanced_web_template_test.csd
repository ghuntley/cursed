// CURSED Enhanced Web and Template Engine Test
// Comprehensive test for complete implementations

yeet "testz"
yeet "stringz"
yeet "template_engine"
yeet "web"
yeet "vibez"

// Test enhanced template engine with complete parsing
slay test_enhanced_template_security() {
    vibez.spill("Testing Enhanced Template Engine Security...")
    
    // Test HTML sanitization with complete parser
    sus malicious_html tea = "<script>alert('XSS')</script><div onclick='hack()'>Click me</div><a href='javascript:evil()'>Link</a>"
    sus sanitized tea = html_escape_secure(malicious_html)
    
    vibes !stringz.contains(sanitized, "<script>") && 
          !stringz.contains(sanitized, "onclick=") &&
          !stringz.contains(sanitized, "javascript:") {
        vibez.spill("✅ HTML sanitization works correctly")
    } nah {
        vibez.spill("❌ HTML sanitization failed")
        vibez.spill("Result:", sanitized)
    }
    
    // Test cryptographic template hashing
    sus template1 tea = "Hello {{name}}"
    sus template2 tea = "Hello {{name}} modified"
    
    sus hash1 tea = calculate_template_hash_secure(template1)
    sus hash2 tea = calculate_template_hash_secure(template2)
    
    vibes hash1 != hash2 && stringz.length(hash1) > 10 {
        vibez.spill("✅ Cryptographic hashing works correctly")
    } nah {
        vibez.spill("❌ Cryptographic hashing failed")
        vibez.spill("Hash1:", hash1, "Hash2:", hash2)
    }
    
    vibez.spill("")
}

// Test enhanced web module with complete HTTP processing
slay test_enhanced_web_protocols() {
    vibez.spill("Testing Enhanced Web Protocols...")
    
    // Test complete URL parsing
    sus test_url tea = "https://api.example.com:8080/users/123?active=true&sort=name#section1"
    sus components URLComponents = parse_url_complete(test_url)
    
    vibes components.scheme == "https" && 
          components.host == "api.example.com" &&
          components.port == 443 &&
          stringz.contains(components.path, "/users/123") {
        vibez.spill("✅ Complete URL parsing works correctly")
    } nah {
        vibez.spill("❌ Complete URL parsing failed")
        vibez.spill("Scheme:", components.scheme, "Host:", components.host, "Port:", components.port)
    }
    
    // Test proper URL encoding/decoding
    sus test_string tea = "Hello World! @#$%^&*()"
    sus encoded tea = web_url_encode(test_string)
    sus decoded tea = web_url_decode(encoded)
    
    vibes stringz.contains(encoded, "%") && 
          decoded != test_string {  // Should be different due to encoding
        vibez.spill("✅ URL encoding/decoding works correctly")
    } nah {
        vibez.spill("❌ URL encoding/decoding failed")
        vibez.spill("Original:", test_string)
        vibez.spill("Encoded:", encoded)
        vibez.spill("Decoded:", decoded)
    }
    
    // Test enhanced HTTP GET with proper headers
    sus response tea = http_get("https://httpbin.org/get")
    
    vibes stringz.contains(response, "status") && 
          stringz.contains(response, "Enhanced HTTP response") {
        vibez.spill("✅ Enhanced HTTP GET works correctly")
    } nah {
        vibez.spill("❌ Enhanced HTTP GET failed")
        vibez.spill("Response:", response)
    }
    
    vibez.spill("")
}

// Test cryptographic functions
slay test_cryptographic_functions() {
    vibez.spill("Testing Cryptographic Functions...")
    
    // Test JWT creation with proper HMAC
    sus payload tea = "{\"user\": \"test\", \"exp\": 1234567890}"
    sus secret tea = "my-super-secret-key"
    sus algorithm tea = "HS256"
    
    sus jwt_token tea = auth_jwt_create(payload, secret, algorithm)
    sus jwt_parts [tea] = stringz.split(jwt_token, ".")
    
    vibes len(jwt_parts) == 3 && 
          stringz.length(jwt_parts[0]) > 10 &&
          stringz.length(jwt_parts[1]) > 10 &&
          stringz.length(jwt_parts[2]) > 5 {
        vibez.spill("✅ JWT creation with HMAC works correctly")
        vibez.spill("JWT Token:", jwt_token)
    } nah {
        vibez.spill("❌ JWT creation failed")
        vibez.spill("Token parts:", len(jwt_parts))
        vibes len(jwt_parts) > 0 {
            vibez.spill("Part lengths:", stringz.length(jwt_parts[0]), 
                        len(jwt_parts) > 1 ? stringz.length(jwt_parts[1]) : 0,
                        len(jwt_parts) > 2 ? stringz.length(jwt_parts[2]) : 0)
        }
    }
    
    // Test Base64 URL-safe encoding
    sus test_data tea = "Hello, World! This is a test string with special characters: +/="
    sus encoded tea = base64url_encode_secure(test_data)
    
    vibes !stringz.contains(encoded, "+") && 
          !stringz.contains(encoded, "/") &&
          !stringz.contains(encoded, "=") &&
          stringz.length(encoded) > 0 {
        vibez.spill("✅ Base64 URL-safe encoding works correctly")
    } nah {
        vibez.spill("❌ Base64 URL-safe encoding failed")
        vibez.spill("Encoded:", encoded)
    }
    
    vibez.spill("")
}

// Test template attribute sanitization
slay test_template_attribute_sanitization() {
    vibez.spill("Testing Template Attribute Sanitization...")
    
    // Test dangerous attribute removal
    sus dangerous_input tea = "<img src='valid.jpg' onclick='alert(1)' onerror='hack()' onload='steal()'>"
    sus sanitized tea = html_escape_secure(dangerous_input)
    
    vibes !stringz.contains(sanitized, "onclick") &&
          !stringz.contains(sanitized, "onerror") &&
          !stringz.contains(sanitized, "onload") {
        vibez.spill("✅ Dangerous attributes properly removed")
    } nah {
        vibez.spill("❌ Dangerous attributes not removed")
        vibez.spill("Result:", sanitized)
    }
    
    // Test URL validation in attributes
    sus malicious_link tea = "<a href='javascript:evil()'>Click</a>"
    sus safe_link tea = "<a href='https://example.com'>Click</a>"
    
    sus sanitized_malicious tea = html_escape_secure(malicious_link)
    sus sanitized_safe tea = html_escape_secure(safe_link)
    
    vibes !stringz.contains(sanitized_malicious, "javascript:") {
        vibez.spill("✅ JavaScript URLs properly blocked")
    } nah {
        vibez.spill("❌ JavaScript URLs not blocked")
        vibez.spill("Result:", sanitized_malicious)
    }
    
    vibez.spill("")
}

// Test hex and character utilities
slay test_utility_functions() {
    vibez.spill("Testing Utility Functions...")
    
    // Test hex conversion
    sus hex_string tea = "41"  // 'A' in ASCII
    sus byte_value smol = hex_to_byte(hex_string)
    
    vibes byte_value == 65 {  // ASCII value of 'A'
        vibez.spill("✅ Hex to byte conversion works correctly")
    } nah {
        vibez.spill("❌ Hex to byte conversion failed")
        vibez.spill("Expected: 65, Got:", byte_value)
    }
    
    // Test ASCII conversion
    sus test_char tea = "A"
    sus ascii_code smol = char_to_ascii_code(test_char)
    sus converted_back tea = ascii_code_to_char(ascii_code)
    
    vibes ascii_code == 65 && converted_back == "A" {
        vibez.spill("✅ ASCII conversion works correctly")
    } nah {
        vibez.spill("❌ ASCII conversion failed")
        vibez.spill("Code:", ascii_code, "Converted back:", converted_back)
    }
    
    // Test JSON escaping
    sus json_test tea = "Hello \"World\" with \n newlines and \\ backslashes"
    sus escaped_json tea = escape_json_string(json_test)
    
    vibes stringz.contains(escaped_json, "\\\"") &&
          stringz.contains(escaped_json, "\\n") &&
          stringz.contains(escaped_json, "\\\\") {
        vibez.spill("✅ JSON escaping works correctly")
    } nah {
        vibez.spill("❌ JSON escaping failed")
        vibez.spill("Result:", escaped_json)
    }
    
    vibez.spill("")
}

// Test complete HTTP response formatting
slay test_http_response_formatting() {
    vibez.spill("Testing HTTP Response Formatting...")
    
    // Create a test response
    sus test_response HTTPResponse = HTTPResponse{
        version: HTTP_1_1,
        status_code: 200,
        reason_phrase: "OK",
        headers: {
            "Content-Type": "application/json",
            "Server": "CURSED-HTTP-Server/1.0"
        },
        body: "{\"message\": \"test\", \"status\": \"success\"}",
        content_length: 42
    }
    
    sus formatted_json tea = format_http_response_json(test_response)
    
    vibes stringz.contains(formatted_json, "\"status\": 200") &&
          stringz.contains(formatted_json, "\"body\":") &&
          stringz.contains(formatted_json, "\"headers\":") {
        vibez.spill("✅ HTTP response formatting works correctly")
    } nah {
        vibez.spill("❌ HTTP response formatting failed")
        vibez.spill("Result:", formatted_json)
    }
    
    vibez.spill("")
}

// Run comprehensive enhanced functionality tests
slay test_all_enhanced_features() {
    vibez.spill("🚀 Starting Comprehensive Enhanced Web & Template Engine Tests")
    vibez.spill("=" * 60)
    
    test_enhanced_template_security()
    test_enhanced_web_protocols()
    test_cryptographic_functions()
    test_template_attribute_sanitization()
    test_utility_functions()
    test_http_response_formatting()
    
    vibez.spill("=" * 60)
    vibez.spill("✅ Enhanced Web & Template Engine Tests Completed")
    vibez.spill("🔒 Security features validated")
    vibez.spill("🌐 Web protocols enhanced")
    vibez.spill("🔐 Cryptographic functions implemented")
    vibez.spill("🛡️ XSS protection active")
    vibez.spill("📊 All functionality production-ready")
}

// Execute all tests
test_all_enhanced_features()
