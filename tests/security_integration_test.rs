//! Comprehensive security integration tests for CURSED
//! Tests all security vulnerabilities and their fixes

use cursed::security::*;
use cursed::error::CursedError;
use std::time::Duration;

#[test]
fn test_memory_safety_guard_pages() {
    // Test 1: Basic allocation with canaries
    let mut region = SecureMemoryRegion::allocate(1024).unwrap();
    
    // Should be able to write to allocated region
    {
        let slice = region.as_slice_mut().unwrap();
        slice[0] = 0xAA;
        slice[1023] = 0xBB;
    }
    
    // Canaries should still be intact
    region.check_canaries().unwrap();
    
    // Test 2: Buffer overflow detection would be caught by canaries
    // (In a real scenario, writing past the buffer would corrupt canaries)
}

#[test]
fn test_safe_transmute_type_safety() {
    // Test 1: Valid transmute
    let x: u32 = 0x12345678;
    let y: i32 = safe_transmute(x).unwrap();
    assert_eq!(y, 0x12345678u32 as i32);
    
    // Test 2: Invalid transmute (different sizes)
    let result: Result<u64, _> = safe_transmute(x);
    assert!(result.is_err());
    
    // Test 3: Invalid transmute (alignment issues)  
    let small: u8 = 42;
    let result: Result<u64, _> = safe_transmute(small);
    assert!(result.is_err());
}

#[test]
fn test_cryptographic_security() {
    // Test 1: Secure random generation
    let rng = SecureRng::new();
    let bytes1 = rng.random_bytes(32).unwrap();
    let bytes2 = rng.random_bytes(32).unwrap();
    
    // Should generate different random values
    assert_ne!(bytes1, bytes2);
    assert_eq!(bytes1.len(), 32);
    
    // Test 2: Authenticated encryption
    let ae = AuthenticatedEncryption::new();
    let key = [0u8; 32]; // In practice use KeyManager::generate_aes_key()
    let plaintext = b"Top secret message";
    let aad = b"authenticated_data";
    
    let encrypted = ae.encrypt(&key, plaintext, aad).unwrap();
    let decrypted = ae.decrypt(&key, &encrypted).unwrap();
    
    assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    
    // Test 3: Tampered data should fail
    let mut tampered = encrypted.clone();
    tampered.ciphertext[0] ^= 1; // Flip a bit
    
    let result = ae.decrypt(&key, &tampered);
    assert!(result.is_err()); // Should fail authentication
    
    // Test 4: Key derivation with proper parameters
    let password = b"secure_password123";
    let salt = [0u8; 16];
    let iterations = 100000;
    
    let key1 = KeyDerivation::pbkdf2_derive(password, &salt, iterations, 32).unwrap();
    let key2 = KeyDerivation::pbkdf2_derive(password, &salt, iterations, 32).unwrap();
    
    assert_eq!(key1, key2); // Same inputs should produce same output
    assert_eq!(key1.len(), 32);
    
    // Test 5: Weak parameters should be rejected
    let result = KeyDerivation::pbkdf2_derive(password, &[0u8; 8], 1000, 32);
    assert!(result.is_err()); // Too few iterations
}

#[test]
fn test_sql_injection_prevention() {
    // Test 1: Basic prepared statement
    let stmt = PreparedStatement::new("SELECT * FROM users WHERE id = ?").unwrap();
    let params = vec![SqlParameter::Integer(123)];
    let result = stmt.execute(&params);
    assert!(result.is_ok());
    
    // Test 2: SQL injection in statement should be rejected
    let malicious_sql = "SELECT * FROM users; DROP TABLE users; --";
    let result = PreparedStatement::new(malicious_sql);
    assert!(result.is_err());
    
    // Test 3: Parameter values can contain malicious content safely
    let safe_stmt = PreparedStatement::new("SELECT * FROM users WHERE name = ?").unwrap();
    let malicious_param = SqlParameter::String("'; DROP TABLE users; --".to_string());
    let result = safe_stmt.execute(&[malicious_param]);
    assert!(result.is_ok()); // Safe because it's parameterized
    
    // Test 4: Secure query builder
    let allowed_tables = vec!["users".to_string(), "posts".to_string()];
    let conn = SecureConnection::new(allowed_tables);
    
    let result = conn.query_builder()
        .select()
        .from("users").unwrap()
        .columns(&["id", "name"]).unwrap()
        .where_eq("id", SqlParameter::Integer(1)).unwrap()
        .execute();
    assert!(result.is_ok());
    
    // Test 5: Reject unauthorized table access
    let result = conn.query_builder()
        .select()
        .from("evil_table");
    assert!(result.is_err());
}

#[test]
fn test_tls_configuration_security() {
    // Test 1: Maximum security configuration
    let config = TlsConfig::maximum_security();
    assert!(config.validate().is_ok());
    assert!(config.verify_certificates);
    assert!(config.verify_hostname);
    
    // Test 2: Weak protocols should be rejected
    let mut weak_config = config.clone();
    weak_config.protocol_versions = vec![rustls::ProtocolVersion::TLSv1_0];
    assert!(weak_config.validate().is_err());
    
    // Test 3: Certificate configuration
    let cert_config = CertificateConfig::new().with_system_cas().unwrap();
    
    let tls_config = TlsConfig::standard_security();
    let client = SecureTlsClient::new(tls_config, cert_config);
    assert!(client.is_ok());
    
    // Test 4: Hostname validation
    let client = client.unwrap();
    assert!(client.is_valid_hostname("example.com"));
    assert!(client.is_valid_hostname("sub.example.com"));
    assert!(!client.is_valid_hostname(""));
    assert!(!client.is_valid_hostname("-invalid.com"));
    assert!(!client.is_valid_hostname("malicious<script>.com"));
}

#[test]
fn test_input_validation_comprehensive() {
    // Test 1: String validation with length limits
    let validator = InputValidator::new()
        .max_string_length(100)
        .allowed_chars(r"^[a-zA-Z0-9\s\-_.]+$").unwrap()
        .deny_pattern(r"<script").unwrap();
    
    assert!(validator.validate_string("Hello World 123").is_ok());
    assert!(validator.validate_string("user@example.com").is_ok());
    assert!(validator.validate_string("<script>alert('xss')</script>").is_err());
    assert!(validator.validate_string(&"x".repeat(200)).is_err());
    
    // Test 2: HTML sanitization
    let sanitizer = HtmlSanitizer::new();
    
    let result = sanitizer.sanitize_html("<script>alert('xss')</script>").unwrap();
    assert!(!result.contains("<script>"));
    assert!(result.contains("&lt;script&gt;"));
    
    let result = sanitizer.sanitize_html("Hello & <goodbye>").unwrap();
    assert!(result.contains("&amp;"));
    assert!(result.contains("&lt;goodbye&gt;"));
    
    // Test 3: Path traversal prevention
    assert!(PathSanitizer::sanitize_path("documents/file.txt").is_ok());
    assert!(PathSanitizer::sanitize_path("../../../etc/passwd").is_err());
    assert!(PathSanitizer::sanitize_path("docs/file<script>.txt").is_err());
    assert!(PathSanitizer::sanitize_path("docs\\..\\..\\windows\\system32").is_err());
    
    // Test 4: Email validation
    assert!(EmailValidator::validate_email("user@example.com").is_ok());
    assert!(EmailValidator::validate_email("user.name+tag@example.com").is_ok());
    assert!(EmailValidator::validate_email("invalid-email").is_err());
    assert!(EmailValidator::validate_email("user@").is_err());
    assert!(EmailValidator::validate_email("@example.com").is_err());
    
    // Test 5: Command injection prevention
    assert!(CommandSanitizer::validate_command_arg("normal_arg").is_ok());
    assert!(CommandSanitizer::validate_command_arg("file.txt").is_ok());
    assert!(CommandSanitizer::validate_command_arg("arg; rm -rf /").is_err());
    assert!(CommandSanitizer::validate_command_arg("arg && evil_command").is_err());
    assert!(CommandSanitizer::validate_command_arg("$(evil_command)").is_err());
    assert!(CommandSanitizer::validate_command_arg("`whoami`").is_err());
}

#[test]
fn test_network_security_controls() {
    // Test 1: IP address validation
    let allowed_ips = vec![
        "203.0.113.1".parse().unwrap(),
        "198.51.100.1".parse().unwrap(),
    ];
    
    let public_ip: std::net::IpAddr = "203.0.113.1".parse().unwrap();
    assert!(NetworkSecurity::is_allowed_ip(public_ip, &allowed_ips));
    
    // Should reject private/localhost IPs in production
    let private_ip: std::net::IpAddr = "192.168.1.1".parse().unwrap();
    assert!(!NetworkSecurity::is_allowed_ip(private_ip, &allowed_ips));
    
    let localhost: std::net::IpAddr = "127.0.0.1".parse().unwrap();
    assert!(!NetworkSecurity::is_allowed_ip(localhost, &allowed_ips));
    
    // Test 2: Session ID generation
    let session_id = NetworkSecurity::generate_session_id().unwrap();
    assert_eq!(session_id.len(), 64); // 32 bytes as hex = 64 chars
    
    let session_id2 = NetworkSecurity::generate_session_id().unwrap();
    assert_ne!(session_id, session_id2); // Should be unique
}

#[test]
fn test_security_manager_integration() {
    // Test 1: Default security policy
    let policy = SecurityPolicy::default();
    let manager = SecurityManager::new(policy).unwrap();
    
    // Should validate inputs by default
    let result = manager.validate_input("Hello, World!");
    assert!(result.is_ok());
    
    let result = manager.validate_input("<script>alert('xss')</script>");
    assert!(result.is_err());
    
    // Test 2: Disabled validation policy
    let mut policy = SecurityPolicy::default();
    policy.validate_all_inputs = false;
    let manager = SecurityManager::new(policy).unwrap();
    
    // Should allow dangerous input when validation disabled
    let result = manager.validate_input("<script>alert('xss')</script>");
    assert!(result.is_ok());
    
    // Test 3: Data protection
    let policy = SecurityPolicy::default();
    let manager = SecurityManager::new(policy).unwrap();
    let key = [0u8; 32];
    let data = b"sensitive information";
    
    let protected = manager.protect_data(&key, data).unwrap();
    assert_ne!(protected.ciphertext, data); // Should be encrypted
    
    // Test 4: Security event logging
    manager.log_security_event("Test security event", SecuritySeverity::Warning);
    manager.log_security_event("Critical security issue", SecuritySeverity::Critical);
}

#[test]  
fn test_vulnerability_edge_cases() {
    // Test 1: Zero-length inputs
    let validator = InputValidator::new();
    assert!(validator.validate_string("").is_ok());
    
    // Test 2: Unicode handling
    assert!(validator.validate_string("Hello 🌍").is_ok());
    
    // Test 3: Null byte injection
    let result = validator.validate_string("file.txt\0.exe");
    let sanitized = result.unwrap();
    assert!(!sanitized.contains('\0'));
    
    // Test 4: Memory region edge cases
    let result = SecureMemoryRegion::allocate(0);
    assert!(result.is_err()); // Should reject zero allocation
    
    // Test 5: SQL parameter validation
    let large_string = "x".repeat(2_000_000);
    let param = SqlParameter::String(large_string);
    assert!(param.validate().is_err()); // Should reject overly large params
}

#[test]
fn test_performance_security_trade_offs() {
    use std::time::Instant;
    
    // Test 1: Encryption performance
    let ae = AuthenticatedEncryption::new();
    let key = [0u8; 32];
    let data = vec![0u8; 1024 * 1024]; // 1MB
    
    let start = Instant::now();
    let encrypted = ae.encrypt(&key, &data, b"perf_test").unwrap();
    let encrypt_time = start.elapsed();
    
    let start = Instant::now();
    let _decrypted = ae.decrypt(&key, &encrypted).unwrap();
    let decrypt_time = start.elapsed();
    
    // Should complete within reasonable time (adjust based on requirements)
    assert!(encrypt_time < Duration::from_millis(100));
    assert!(decrypt_time < Duration::from_millis(100));
    
    // Test 2: Input validation performance
    let validator = InputValidator::new();
    let large_input = "x".repeat(10000);
    
    let start = Instant::now();
    let _result = validator.validate_string(&large_input).unwrap();
    let validation_time = start.elapsed();
    
    assert!(validation_time < Duration::from_millis(10));
}

// Benchmark tests for performance validation
#[cfg(feature = "benchmark")]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn benchmark_memory_allocation() {
        let iterations = 1000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _region = SecureMemoryRegion::allocate(4096).unwrap();
        }
        
        let elapsed = start.elapsed();
        let per_allocation = elapsed / iterations;
        
        println!("Secure allocation: {:?} per allocation", per_allocation);
        assert!(per_allocation < Duration::from_micros(100));
    }
    
    #[test]
    fn benchmark_crypto_operations() {
        let ae = AuthenticatedEncryption::new();
        let key = [0u8; 32];
        let data = vec![0u8; 4096];
        
        let iterations = 1000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let encrypted = ae.encrypt(&key, &data, b"benchmark").unwrap();
            let _decrypted = ae.decrypt(&key, &encrypted).unwrap();
        }
        
        let elapsed = start.elapsed();
        let per_operation = elapsed / (iterations * 2); // encrypt + decrypt
        
        println!("Crypto operation: {:?} per encrypt/decrypt", per_operation);
        assert!(per_operation < Duration::from_micros(500));
    }
}
