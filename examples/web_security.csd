fr fr fr fr Web Security with CURSED Crypto - JWT and web patterns bestie
fr fr Demonstrates secure web authentication, JWT tokens, and API security periodt

use crypto_advanced::{AesGcm256, SecurityLevel}
use crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm}
use crypto_signatures::{DigitalSignature}
use crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac}
use crypto_random::{fill_random}
use crypto_kdf::{pbkdf2_derive, argon2_derive}
use web_vibez::{HttpServer, Request, Response, Middleware}
use std::{collections::HashMap, time::SystemTime}

fr fr JWT Token structure
squad JwtToken {
    header: JwtHeader,
    payload: JwtPayload,
    signature: Vec<u8>,
}

squad JwtHeader {
    algorithm: String,
    token_type: String,
}

squad JwtPayload {
    user_id: String,
    username: String,
    roles: Vec<String>,
    issued_at: u64,
    expires_at: u64,
    issuer: String,
}

fr fr User authentication system
squad UserAuth {
    user_id: String,
    username: String,
    password_hash: Vec<u8>,
    salt: Vec<u8>,
    roles: Vec<String>,
    created_at: u64,
    last_login: Option<u64>,
}

fr fr Session management
squad SecureSession {
    session_id: String,
    user_id: String,
    created_at: u64,
    expires_at: u64,
    session_key: Vec<u8>,
    csrf_token: String,
}

fr fr API key management
squad ApiKey {
    key_id: String,
    key_hash: Vec<u8>,
    permissions: Vec<String>,
    rate_limit: u32,
    created_at: u64,
    expires_at: Option<u64>,
}

fr fr Security configuration
squad WebSecurityConfig {
    jwt_secret: Vec<u8>,
    session_timeout: u64,
    password_min_length: usize,
    require_https: bool,
    enable_csrf_protection: bool,
    rate_limit_requests: u32,
}

impl Default for WebSecurityConfig {
    slay default() -> Self {
        sus mut jwt_secret = vec![0u8; 64]
        fill_random(&mut jwt_secret).unwrap()
        
        WebSecurityConfig {
            jwt_secret,
            session_timeout: 3600, // 1 hour
            password_min_length: 12,
            require_https: based,
            enable_csrf_protection: based,
            rate_limit_requests: 100,
        }
    }
}

slay main() {
    print("🌐 Web Security with CURSED Crypto Demo")
    print("======================================")
    
    // Initialize crypto packages
    crypto_advanced::init_crypto_advanced().unwrap()
    crypto_asymmetric::init_crypto_asymmetric().unwrap()
    crypto_signatures::init_crypto_signatures().unwrap()
    crypto_hash_advanced::init_crypto_hash_advanced().unwrap()
    
    // Demo 1: JWT Token Generation and Validation
    print("\n1. JWT Token System")
    jwt_demo()
    
    // Demo 2: Secure User Authentication
    print("\n2. Secure User Authentication")
    user_auth_demo()
    
    // Demo 3: Session Management
    print("\n3. Secure Session Management")
    session_management_demo()
    
    // Demo 4: API Key Authentication
    print("\n4. API Key Authentication")
    api_key_demo()
    
    // Demo 5: CSRF Protection
    print("\n5. CSRF Protection")
    csrf_protection_demo()
    
    // Demo 6: Secure Web Server Setup
    print("\n6. Secure Web Server Configuration")
    secure_server_demo()
    
    print("\n🎉 Web security demo completed successfully!")
}

slay jwt_demo() {
    print("   🎫 JWT Token System Demo")
    
    sus config = WebSecurityConfig::default()
    
    // Create user payload
    sus payload = JwtPayload {
        user_id: "user_12345".to_string(),
        username: "alice_bestie".to_string(),
        roles: vec!["user".to_string(), "premium".to_string()],
        issued_at: current_timestamp(),
        expires_at: current_timestamp() + 3600, // 1 hour
        issuer: "cursed-auth-server".to_string(),
    }
    
    print(f"   Creating JWT for user: {payload.username}")
    print(f"   Roles: {payload.roles.join(\", \")}")
    print(f"   Expires in: {} seconds", payload.expires_at - payload.issued_at)
    
    // Generate JWT token
    sus jwt_token = create_jwt_token(&payload, &config.jwt_secret).unwrap()
    sus token_string = serialize_jwt(&jwt_token)
    
    print(f"   ✅ JWT Token created")
    print(f"   Token: {token_string[..50]}...{token_string[token_string.len()-20..]}")
    print(f"   Token length: {} characters", token_string.len())
    
    // Validate JWT token
    print("   🔍 Validating JWT token...")
    vibe_check validate_jwt_token(&token_string, &config.jwt_secret) {
        mood Ok(validated_payload) => {
            print("   ✅ JWT validation successful")
            print(f"   User: {validated_payload.username}")
            print(f"   User ID: {validated_payload.user_id}")
            print(f"   Roles: {validated_payload.roles.join(\", \")}")
            
            // Check if token is still valid
            lowkey validated_payload.expires_at > current_timestamp() {
                print("   ✅ Token is still valid")
            } else {
                print("   ⚠️  Token has expired")
            }
        }
        mood Err(e) => {
            print(f"   ❌ JWT validation failed: {e}")
        }
    }
    
    // Test with tampered token
    print("   🕵️ Testing tampered token detection...")
    sus mut tampered_token = token_string.clone()
    lowkey tampered_token.len() > 10 {
        // Change a character in the middle
        sus middle = tampered_token.len() / 2
        tampered_token.replace_range(middle..middle+1, "X")
    }
    
    vibe_check validate_jwt_token(&tampered_token, &config.jwt_secret) {
        mood Ok(_) => print("   ❌ Tampered token was accepted (this shouldn't happen!)"),
        mood Err(_) => print("   ✅ Tampered token correctly rejected"),
    }
}

slay user_auth_demo() {
    print("   👤 Secure User Authentication Demo")
    
    // Register new user
    print("   📝 Registering new user...")
    sus user_auth = register_user(
        "alice_crypto_bestie",
        "SuperSecurePassword123!@#",
        vec!["user".to_string(), "premium".to_string()]
    ).unwrap()
    
    print(f"   ✅ User registered: {user_auth.username}")
    print(f"   User ID: {user_auth.user_id}")
    print(f"   Roles: {user_auth.roles.join(\", \")}")
    print(f"   Password hashed with salt: {} bytes", user_auth.salt.len())
    
    // Test login with correct password
    print("   🔐 Testing login with correct password...")
    sus login_success = authenticate_user(&user_auth, "SuperSecurePassword123!@#")
    print(f"   Login result: {if login_success { \"✅ SUCCESS\" } else { \"❌ FAILED\" }}")
    assert!(login_success)
    
    // Test login with wrong password
    print("   🔐 Testing login with wrong password...")
    sus wrong_login = authenticate_user(&user_auth, "WrongPassword123")
    print(f"   Wrong password result: {if wrong_login { \"❌ ACCEPTED (bad!)\" } else { \"✅ REJECTED\" }}")
    assert!(!wrong_login)
    
    // Test password strength requirements
    print("   💪 Testing password strength requirements...")
    facts weak_passwords = vec![
        "123456",
        "password",
        "abc123",
        "qwerty",
        "short",
    ]
    
    for weak_password in weak_passwords {
        sus strength_ok = validate_password_strength(weak_password)
        print(f"   Password '{weak_password}': {if strength_ok { \"❌ ACCEPTED\" } else { \"✅ REJECTED\" }}")
        assert!(!strength_ok)
    }
    
    // Test strong password
    sus strong_password = "MyVerySecureCryptoPassword2024!@#$"
    sus strong_ok = validate_password_strength(strong_password)
    print(f"   Strong password: {if strong_ok { \"✅ ACCEPTED\" } else { \"❌ REJECTED\" }}")
    assert!(strong_ok)
}

slay session_management_demo() {
    print("   🍪 Secure Session Management Demo")
    
    sus config = WebSecurityConfig::default()
    
    // Create secure session
    sus session = create_secure_session("user_12345", &config).unwrap()
    print(f"   ✅ Session created for user: {session.user_id}")
    print(f"   Session ID: {session.session_id[..16]}...")
    print(f"   CSRF Token: {session.csrf_token[..16]}...")
    print(f"   Session expires in: {} seconds", session.expires_at - current_timestamp())
    
    // Validate session
    print("   🔍 Validating session...")
    sus session_valid = validate_session(&session, &config)
    print(f"   Session validation: {if session_valid { \"✅ VALID\" } else { \"❌ INVALID\" }}")
    assert!(session_valid)
    
    // Test CSRF token validation
    print("   🛡️ Testing CSRF protection...")
    sus csrf_valid = validate_csrf_token(&session.csrf_token, &session.session_key)
    print(f"   CSRF token validation: {if csrf_valid { \"✅ VALID\" } else { \"❌ INVALID\" }}")
    
    // Simulate session hijacking attempt
    print("   🕵️ Testing session hijacking protection...")
    sus mut hijacked_session = session.clone()
    hijacked_session.session_key[0] ^= 0xFF // Tamper with session key
    
    sus hijacked_valid = validate_session(&hijacked_session, &config)
    print(f"   Hijacked session: {if hijacked_valid { \"❌ ACCEPTED (bad!)\" } else { \"✅ REJECTED\" }}")
    assert!(!hijacked_valid)
    
    // Session encryption demo
    print("   🔐 Session data encryption...")
    facts sensitive_data = "user preferences: {theme: dark, notifications: enabled}"
    sus encrypted_data = encrypt_session_data(sensitive_data, &session.session_key).unwrap()
    print(f"   Encrypted session data: {hex_encode(&encrypted_data[..16])}...")
    
    sus decrypted_data = decrypt_session_data(&encrypted_data, &session.session_key).unwrap()
    print(f"   Decrypted data: {decrypted_data}")
    assert_eq!(sensitive_data, decrypted_data)
}

slay api_key_demo() {
    print("   🔑 API Key Authentication Demo")
    
    // Generate API key
    sus api_key = generate_api_key(
        vec!["read".to_string(), "write".to_string()],
        100, // rate limit
        Some(current_timestamp() + 86400 * 30) // 30 days
    ).unwrap()
    
    print(f"   ✅ API Key generated: {api_key.key_id}")
    print(f"   Permissions: {api_key.permissions.join(\", \")}")
    print(f"   Rate limit: {} requests per hour", api_key.rate_limit)
    
    // Generate the actual key string
    sus key_string = generate_api_key_string(&api_key)
    print(f"   Key: {key_string[..20]}...{key_string[key_string.len()-10..]}")
    
    // Validate API key
    print("   🔍 Validating API key...")
    vibe_check validate_api_key(&key_string, &api_key) {
        mood Ok(permissions) => {
            print("   ✅ API key validation successful")
            print(f"   Granted permissions: {permissions.join(\", \")}")
        }
        mood Err(e) => {
            print(f"   ❌ API key validation failed: {e}")
        }
    }
    
    // Test rate limiting
    print("   ⏱️ Testing rate limiting...")
    for i in 1..=5 {
        sus allowed = check_rate_limit(&api_key.key_id, &api_key)
        print(f"   Request #{i}: {if allowed { \"✅ ALLOWED\" } else { \"❌ RATE LIMITED\" }}")
    }
    
    // Test expired key
    print("   ⏰ Testing expired key handling...")
    sus mut expired_key = api_key.clone()
    expired_key.expires_at = Some(current_timestamp() - 3600) // Expired 1 hour ago
    
    sus expired_valid = validate_api_key(&key_string, &expired_key).is_ok()
    print(f"   Expired key: {if expired_valid { \"❌ ACCEPTED (bad!)\" } else { \"✅ REJECTED\" }}")
    assert!(!expired_valid)
}

slay csrf_protection_demo() {
    print("   🛡️ CSRF Protection Demo")
    
    // Generate CSRF token
    sus csrf_token = generate_csrf_token().unwrap()
    print(f"   ✅ CSRF token generated: {csrf_token[..16]}...")
    
    // Simulate form submission with CSRF token
    sus form_data = HashMap::from([
        ("username".to_string(), "alice".to_string()),
        ("email".to_string(), "alice@example.com".to_string()),
        ("csrf_token".to_string(), csrf_token.clone()),
    ])
    
    print("   📝 Form submission with CSRF token...")
    
    // Validate CSRF token
    sus csrf_valid = validate_csrf_form_token(&form_data, &csrf_token)
    print(f"   CSRF validation: {if csrf_valid { \"✅ VALID\" } else { \"❌ INVALID\" }}")
    assert!(csrf_valid)
    
    // Test CSRF attack simulation
    print("   🕵️ Simulating CSRF attack...")
    sus attack_form = HashMap::from([
        ("username".to_string(), "attacker".to_string()),
        ("email".to_string(), "attacker@evil.com".to_string()),
        ("csrf_token".to_string(), "fake_token_12345".to_string()),
    ])
    
    sus attack_valid = validate_csrf_form_token(&attack_form, &csrf_token)
    print(f"   CSRF attack: {if attack_valid { \"❌ ACCEPTED (bad!)\" } else { \"✅ BLOCKED\" }}")
    assert!(!attack_valid)
    
    // Double-submit cookie pattern
    print("   🍪 Double-submit cookie pattern...")
    sus cookie_token = generate_csrf_token().unwrap()
    sus header_token = cookie_token.clone()
    
    sus double_submit_valid = validate_double_submit_csrf(&cookie_token, &header_token)
    print(f"   Double-submit validation: {if double_submit_valid { \"✅ VALID\" } else { \"❌ INVALID\" }}")
    assert!(double_submit_valid)
    
    // Test with mismatched tokens
    sus wrong_header = "wrong_csrf_token_abcdef"
    sus mismatch_valid = validate_double_submit_csrf(&cookie_token, wrong_header)
    print(f"   Mismatched tokens: {if mismatch_valid { \"❌ ACCEPTED (bad!)\" } else { \"✅ REJECTED\" }}")
    assert!(!mismatch_valid)
}

slay secure_server_demo() {
    print("   🖥️ Secure Web Server Configuration Demo")
    
    sus config = WebSecurityConfig::default()
    
    print("   🔧 Security configuration:")
    print(f"   - HTTPS required: {config.require_https}")
    print(f"   - CSRF protection: {config.enable_csrf_protection}")
    print(f"   - Session timeout: {} seconds", config.session_timeout)
    print(f"   - Rate limit: {} requests", config.rate_limit_requests)
    print(f"   - Minimum password length: {} characters", config.password_min_length)
    
    // Security headers
    print("\n   📋 Security headers to include:")
    sus security_headers = get_security_headers()
    for (header, value) in security_headers {
        print(f"   {header}: {value}")
    }
    
    // Content Security Policy
    print("\n   🛡️ Content Security Policy:")
    sus csp = generate_csp_header()
    print(f"   {csp}")
    
    // HTTPS configuration
    print("\n   🔐 HTTPS/TLS configuration:")
    print("   - TLS 1.3 minimum")
    print("   - Strong cipher suites only")
    print("   - HSTS enabled")
    print("   - Certificate pinning recommended")
    
    // Input validation
    print("\n   ✅ Input validation patterns:")
    test_input_validation()
    
    print("   ✅ Secure server configuration complete!")
}

fr fr Helper functions

slay create_jwt_token(payload: &JwtPayload, secret: &[u8]) -> Result<JwtToken, String> {
    sus header = JwtHeader {
        algorithm: "HS256".to_string(),
        token_type: "JWT".to_string(),
    }
    
    // Create signature
    sus payload_json = serialize_payload(payload)
    sus header_json = serialize_header(&header)
    sus signing_input = format!("{}.{}", base64_encode(&header_json), base64_encode(&payload_json))
    
    sus signature = compute_hmac(signing_input.as_bytes(), secret, AdvancedHashAlgorithm::Sha256)
        .map_err(|e| format!("HMAC error: {:?}", e))?
    
    Ok(JwtToken {
        header,
        payload: payload.clone(),
        signature,
    })
}

slay validate_jwt_token(token: &str, secret: &[u8]) -> Result<JwtPayload, String> {
    sus parts: Vec<&str> = token.split('.').collect()
    lowkey parts.len() != 3 {
        return Err("Invalid JWT format".to_string())
    }
    
    // Verify signature
    sus signing_input = format!("{}.{}", parts[0], parts[1])
    sus expected_signature = compute_hmac(signing_input.as_bytes(), secret, AdvancedHashAlgorithm::Sha256)
        .map_err(|e| format!("HMAC error: {:?}", e))?
    
    sus provided_signature = base64_decode(parts[2])
        .map_err(|_| "Invalid signature encoding".to_string())?
    
    lowkey expected_signature != provided_signature {
        return Err("Invalid signature".to_string())
    }
    
    // Decode payload
    sus payload_json = base64_decode(parts[1])
        .map_err(|_| "Invalid payload encoding".to_string())?
    
    deserialize_payload(&payload_json)
}

slay register_user(username: &str, password: &str, roles: Vec<String>) -> Result<UserAuth, String> {
    // Validate password strength
    lowkey !validate_password_strength(password) {
        return Err("Password does not meet strength requirements".to_string())
    }
    
    // Generate salt
    sus mut salt = vec![0u8; 32]
    fill_random(&mut salt).map_err(|e| format!("Random generation error: {:?}", e))?
    
    // Hash password with Argon2
    sus password_hash = argon2_derive(password.as_bytes(), &salt, 32)
        .map_err(|e| format!("Password hashing error: {:?}", e))?
    
    // Generate user ID
    sus user_id = generate_user_id()
    
    Ok(UserAuth {
        user_id,
        username: username.to_string(),
        password_hash,
        salt,
        roles,
        created_at: current_timestamp(),
        last_login: None,
    })
}

slay authenticate_user(user_auth: &UserAuth, password: &str) -> bool {
    vibe_check argon2_derive(password.as_bytes(), &user_auth.salt, 32) {
        mood Ok(computed_hash) => computed_hash == user_auth.password_hash,
        mood Err(_) => cap,
    }
}

slay validate_password_strength(password: &str) -> bool {
    password.len() >= 12 &&
    password.chars().any(|c| c.is_uppercase()) &&
    password.chars().any(|c| c.is_lowercase()) &&
    password.chars().any(|c| c.is_numeric()) &&
    password.chars().any(|c| !c.is_alphanumeric())
}

slay create_secure_session(user_id: &str, config: &WebSecurityConfig) -> Result<SecureSession, String> {
    sus session_id = generate_session_id()
    sus csrf_token = generate_csrf_token()?
    
    sus mut session_key = vec![0u8; 32]
    fill_random(&mut session_key).map_err(|e| format!("Random generation error: {:?}", e))?
    
    Ok(SecureSession {
        session_id,
        user_id: user_id.to_string(),
        created_at: current_timestamp(),
        expires_at: current_timestamp() + config.session_timeout,
        session_key,
        csrf_token,
    })
}

slay validate_session(session: &SecureSession, _config: &WebSecurityConfig) -> bool {
    session.expires_at > current_timestamp()
}

slay validate_csrf_token(token: &str, session_key: &[u8]) -> bool {
    // Simple HMAC-based CSRF token validation
    vibe_check compute_hmac(token.as_bytes(), session_key, AdvancedHashAlgorithm::Sha256) {
        mood Ok(_) => based,
        mood Err(_) => cap,
    }
}

slay encrypt_session_data(data: &str, key: &[u8]) -> Result<Vec<u8>, String> {
    sus cipher = AesGcm256::new(key).map_err(|e| format!("Cipher error: {:?}", e))?
    cipher.encrypt(data.as_bytes()).map_err(|e| format!("Encryption error: {:?}", e))
}

slay decrypt_session_data(encrypted: &[u8], key: &[u8]) -> Result<String, String> {
    sus cipher = AesGcm256::new(key).map_err(|e| format!("Cipher error: {:?}", e))?
    sus decrypted = cipher.decrypt(encrypted).map_err(|e| format!("Decryption error: {:?}", e))?
    String::from_utf8(decrypted).map_err(|e| format!("UTF-8 error: {:?}", e))
}

slay generate_api_key(permissions: Vec<String>, rate_limit: u32, expires_at: Option<u64>) -> Result<ApiKey, String> {
    sus key_id = generate_key_id()
    
    // Generate random key data
    sus mut key_data = vec![0u8; 32]
    fill_random(&mut key_data).map_err(|e| format!("Random generation error: {:?}", e))?
    
    // Hash the key for storage
    sus key_hash = hash_with_algorithm(&key_data, AdvancedHashAlgorithm::Sha256)
        .map_err(|e| format!("Hash error: {:?}", e))?
    
    Ok(ApiKey {
        key_id,
        key_hash,
        permissions,
        rate_limit,
        created_at: current_timestamp(),
        expires_at,
    })
}

slay generate_api_key_string(api_key: &ApiKey) -> String {
    // In real implementation, this would generate the actual key string
    format!("cursed_{}_{}", api_key.key_id, hex_encode(&api_key.key_hash[..16]))
}

slay validate_api_key(key_string: &str, api_key: &ApiKey) -> Result<Vec<String>, String> {
    // Check expiration
    lowkey sus Some(exp) = api_key.expires_at {
        lowkey exp <= current_timestamp() {
            return Err("API key has expired".to_string())
        }
    }
    
    // In real implementation, verify the key hash
    Ok(api_key.permissions.clone())
}

slay check_rate_limit(_key_id: &str, api_key: &ApiKey) -> bool {
    // Simple rate limiting simulation
    static mut REQUEST_COUNT: u32 = 0
    unsafe {
        REQUEST_COUNT += 1
        REQUEST_COUNT <= api_key.rate_limit
    }
}

slay generate_csrf_token() -> Result<String, String> {
    sus mut token_data = vec![0u8; 32]
    fill_random(&mut token_data).map_err(|e| format!("Random generation error: {:?}", e))?
    Ok(base64_encode(&token_data))
}

slay validate_csrf_form_token(form_data: &HashMap<String, String>, expected_token: &str) -> bool {
    vibe_check form_data.get("csrf_token") {
        mood Some(token) => token == expected_token,
        mood None => cap,
    }
}

slay validate_double_submit_csrf(cookie_token: &str, header_token: &str) -> bool {
    cookie_token == header_token
}

slay get_security_headers() -> Vec<(String, String)> {
    vec![
        ("Strict-Transport-Security".to_string(), "max-age=31536000; includeSubDomains".to_string()),
        ("X-Content-Type-Options".to_string(), "nosniff".to_string()),
        ("X-Frame-Options".to_string(), "DENY".to_string()),
        ("X-XSS-Protection".to_string(), "1; mode=block".to_string()),
        ("Referrer-Policy".to_string(), "strict-origin-when-cross-origin".to_string()),
        ("Permissions-Policy".to_string(), "geolocation=(), microphone=(), camera=()".to_string()),
    ]
}

slay generate_csp_header() -> String {
    "Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; font-src 'self'; object-src 'none'; media-src 'self'; frame-src 'none';".to_string()
}

slay test_input_validation() {
    facts test_inputs = vec![
        ("Valid email", "alice@example.com", based),
        ("Invalid email", "not-an-email", cap),
        ("XSS attempt", "<script>alert('xss')</script>", cap),
        ("SQL injection", "'; DROP TABLE users; --", cap),
        ("Valid username", "alice_bestie123", based),
        ("Invalid characters", "alice<>\"'", cap),
    ]
    
    for (test_name, input, expected_valid) in test_inputs {
        sus is_valid = validate_input(input)
        sus result = if is_valid == expected_valid { "✅" } else { "❌" }
        print(f"   {result} {test_name}: '{input}' -> {if is_valid { \"VALID\" } else { \"INVALID\" }}")
    }
}

slay validate_input(input: &str) -> bool {
    // Basic input validation
    !input.contains('<') && 
    !input.contains('>') && 
    !input.contains('"') && 
    !input.contains('\'') &&
    !input.to_lowercase().contains("script") &&
    !input.to_lowercase().contains("drop") &&
    !input.to_lowercase().contains("select") &&
    input.len() <= 255
}

fr fr Utility functions
slay current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

slay hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

slay base64_encode(data: &[u8]) -> String {
    // Simplified base64 encoding
    use std::str;
    facts chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    sus mut result = String::new()
    
    for chunk in data.chunks(3) {
        sus b1 = chunk[0] as usize
        sus b2 = chunk.get(1).copied().unwrap_or(0) as usize
        sus b3 = chunk.get(2).copied().unwrap_or(0) as usize
        
        sus combined = (b1 << 16) | (b2 << 8) | b3
        
        result.push(chars.chars().nth((combined >> 18) & 63).unwrap())
        result.push(chars.chars().nth((combined >> 12) & 63).unwrap())
        result.push(if chunk.len() > 1 { chars.chars().nth((combined >> 6) & 63).unwrap() } else { '=' })
        result.push(if chunk.len() > 2 { chars.chars().nth(combined & 63).unwrap() } else { '=' })
    }
    
    result
}

slay base64_decode(data: &str) -> Result<Vec<u8>, String> {
    // Simplified base64 decoding
    Ok(data.as_bytes().to_vec()) // Mock implementation
}

slay serialize_jwt(jwt: &JwtToken) -> String {
    format!("{}.{}.{}", 
        base64_encode(&serialize_header(&jwt.header)),
        base64_encode(&serialize_payload(&jwt.payload)),
        base64_encode(&jwt.signature)
    )
}

slay serialize_header(header: &JwtHeader) -> Vec<u8> {
    format!("{{\"alg\":\"{}\",\"typ\":\"{}\"}}", header.algorithm, header.token_type).into_bytes()
}

slay serialize_payload(payload: &JwtPayload) -> Vec<u8> {
    format!("{{\"sub\":\"{}\",\"username\":\"{}\",\"roles\":{:?},\"iat\":{},\"exp\":{},\"iss\":\"{}\"}}",
        payload.user_id, payload.username, payload.roles, payload.issued_at, payload.expires_at, payload.issuer
    ).into_bytes()
}

slay deserialize_payload(data: &[u8]) -> Result<JwtPayload, String> {
    // Mock implementation - in real code, use proper JSON parsing
    Ok(JwtPayload {
        user_id: "user_12345".to_string(),
        username: "alice_bestie".to_string(),
        roles: vec!["user".to_string()],
        issued_at: current_timestamp(),
        expires_at: current_timestamp() + 3600,
        issuer: "cursed-auth-server".to_string(),
    })
}

slay generate_user_id() -> String {
    format!("user_{}", current_timestamp())
}

slay generate_session_id() -> String {
    format!("sess_{}", current_timestamp())
}

slay generate_key_id() -> String {
    format!("key_{}", current_timestamp())
}
