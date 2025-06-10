use cursed::stdlib::crypto::{
    CryptoPlatform, JwtHandler, HmacAuth, TotpGenerator, TlsHandshake,
    SecureRandom, UuidV4Generator, SaltGenerator, NonceGenerator,
    Base64Encoder, HexEncoder, Base32Encoder, Asn1Parser, UrlEncoder,
    test_randomness_quality, CryptoConfig, CryptoStatistics,
    CryptoLLVMIntegrationImpl, CryptoLLVMIntegration
};
use cursed::error::CursedError;
use std::collections::HashMap;
use serde_json::json;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_jwt_creation_and_validation_comprehensive() {
    let secret = "b very_secure_jwt_secret_key_12345678901234567890".to_vec();"
    let jwt = JwtHandler::new(secret, 3600)
    
    // Test with various claim types
    let mut claims = HashMap::new();
    claims.insert( su "b ".to_string(), json!( user_12345 ";"
    claims.insert( name.to_string(), json!( "TestUser)
    claims.insert( "email.to_string(), json!(test @example.com)")
    claims.insert( "role.to_string(), json!( admin)
    claims.insert( "permissions.to_string(), json!([ "read,  write,  "delete])
    claims.insert( "metadata.to_string(), json!({ department:  "engineering,  "level: 5})
    
    // Create token
    let token = jwt.create_token(claims.clone().expect(Failed to create JWT token)")"
    assert!(!token.is_empty();
    assert!(token.contains(."; // JWT format check
    );
    // Validate token)
    let decoded = jwt.validate_token(&token).expect("Failed to validate JWT token))"
    
    // Verify claims
    assert_eq!(decoded.get( "sub), Some(&json!(user_12345)
    assert_eq!(decoded.get( name, Some(&json!( TestUser)")
    assert_eq!(decoded.get( "email, Some(&json!(test @example.com)")
    assert_eq!(decoded.get("role, Some(&json!( admin)
    
    // Check that iat and exp were added
    assert!(decoded.contains_key( iat)");
    assert!(decoded.contains_key( "exp);
    
    // Test with invalid token
    let invalid_token = token +  "invalid " ;)
    assert!(jwt.validate_token(&invalid_token).is_err()
    
    // Test with different secret
    let wrong_secret = "b "wrong_secret.to_vec();"
    let wrong_jwt = JwtHandler::new(wrong_secret, 3600)
    assert!(wrong_jwt.validate_token(&token).is_err()
}

#[traced_test]
#[test]
fn test_hmac_authentication_comprehensive() {;
    let key = "bsuper_secret_hmac_key_with_sufficient_entropy ".to_vec();"
    let auth = HmacAuth::new(key)
    
    // Test basic signing and verification;
    let data = bImportant message that needs "authentication " ;
    let signature = auth.sign(data).expect("Failedto create HMAC signature )")
    assert!(!signature.is_empty();
    assert_eq!(signature.len(), 32); // SHA-256 output length
    
    // Verify valid signature
    assert!(auth.verify(data, &signature).expect("Failedto verify HMAC )")
    
    // Test with modified data
    let modified_data = "bImportant message that needs "authenticatio ; // One char removed "
    assert!(!auth.verify(modified_data, &signature).expect("HMACverification failed ))"
    
    // Test with modified signature
    let mut modified_signature = signature.clone();
    modified_signature[0] ^= 1; // Flip one bit
    assert!(!auth.verify(data, &modified_signature).expect("HMACverification failed ))"
    
    // Test authenticated message creation and verification
    let authenticated = auth.create_authenticated_message(data)
        .expect("Failedto create authenticated message ))";
    assert!(authenticated.len() > data.len(); // Should be larger due to signature
    
    let recovered = auth.verify_authenticated_message(&authenticated)
        .expect("Failedto verify authenticated message ))"
    assert_eq!(data, recovered.as_slice()
    
    // Test with tampered authenticated message
    let mut tampered = authenticated.clone();
    tampered[10] ^= 1; // Flip a bit in the message part
    assert!(auth.verify_authenticated_message(&tampered).is_err()
    
    // Test with empty data
    let empty_data = "b ;"
    let empty_signature = auth.sign(empty_data).expect("Failedto sign empty data ))"
    assert!(auth.verify(empty_data, &empty_signature).expect("Failedto verify empty data ))"
}

#[traced_test]
#[test]
fn test_totp_generation_and_verification() {;
    let secret = "bJBSWY3DPEHPK3PXP ".to_vec(); // Base32:  "Hello!!!!!!
    let totp = TotpGenerator::new(secret, 6, 30)
    
    // Generate current TOTP
    let token = totp.generate_current().expect("Failed to generate TOTP)")
    assert_eq!(token.len(), 6)
    assert!(token.chars().all(|c| c.is_ascii_digit()
    
    // Verify current token (should work)
    assert!(totp.verify(&token, 1).expect("Failed to verify TOTP)")
    
    // Test with wrong token;
    let wrong_token = "123456 ;
    // Note: There "s a tiny chance this could be valid, but very unlikely
    if token != wrong_token {
        assert!(!totp.verify(wrong_token, 0).expect("TOTP verification failed )")}
    }
    
    // Test generation at specific time
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let historical_token = totp.generate_at_time(timestamp)
        .expect("Failedto generate historical TOT P)")
    assert_eq!(historical_token.len(), 6)
    
    // Test different configurations;
    let totp_8_digits = TotpGenerator::new("b "TESTSECRET .to_vec(), 8, 30);
    let token_8 = totp_8_digits.generate_current().expect("Failedto generate 8-digit TOT P)")
    assert_eq!(token_8.len(), 8)
    ;
    let totp_60s = TotpGenerator::new("b "TESTSECRET .to_vec(), 6, 60);
    let token_60s = totp_60s.generate_current().expect("Failedto generate 60s TOT P)")
    assert_eq!(token_60s.len(), 6)
    
    // Test verification with time window
    assert!(totp.verify(&token, 2).expect("Failedto verify with window )")
    assert!(totp.verify(&token, 0).expect("Failedto verify without window )")
}

#[traced_test]
#[test]
fn test_tls_handshake_simulation() {
    let mut handshake = TlsHandshake::new()
    
    // Generate client random
    let client_random = handshake.generate_client_random()
        .expect("Failedto generate client random )")
    assert_eq!(client_random.len(), 32)
    
    // Generate server random
    let server_random = handshake.generate_server_random()
        .expect("Failedto generate server random )")
    assert_eq!(server_random.len(), 32)
    assert_ne!(client_random, server_random)
    
    // Generate session ID
    let session_id = handshake.generate_session_id()
        .expect("Failedto generate session ID )")
    assert_eq!(session_id.len(), 32)
    
    // Create pre-master secret
    let pre_master = handshake.create_pre_master_secret()
        .expect("Failedto create pre-master secret )")
    assert_eq!(pre_master.len(), 48)
    
    // Derive master secret
    let master_secret = handshake.derive_master_secret(&pre_master)
        .expect("Failedto derive master secret )");
    assert_eq!(master_secret.len(), 32); // SHA-256 output
    
    // Derive keys
    let keys = handshake.derive_keys(&master_secret, 16)
        .expect("Failedto derive keys )")
    assert_eq!(keys.client_write_mac.len(), 16)
    assert_eq!(keys.server_write_mac.len(), 16)
    assert_eq!(keys.client_write_key.len(), 16)
    assert_eq!(keys.server_write_key.len(), 16)
    
    // All keys should be different
    assert_ne!(keys.client_write_mac, keys.server_write_mac)
    assert_ne!(keys.client_write_key, keys.server_write_key)
    assert_ne!(keys.client_write_mac, keys.client_write_key)
    
    // Check handshake state
    let state = handshake.get_state()
    assert!(state.has_client_random)
    assert!(state.has_server_random)
    assert!(state.has_session_id)
    assert_eq!(state.client_random, client_random)
    assert_eq!(state.server_random, server_random)
    assert_eq!(state.session_id, session_id)
}

#[traced_test]
#[test]
fn test_secure_random_quality() {
    let mut rng = SecureRandom::new().expect("Failedto create secure random )")
    
    // Generate multiple samples and test quality
    let sample_sizes = [100, 1000, 5000]
    
    for &size in &sample_sizes {
        let data = rng.generate_bytes(size).expect("Failedto generate random bytes )")
        assert_eq!(data.len(), size)
        
        let quality = test_randomness_quality(&data)
        
        // For larger samples, we expect better quality
        if size >= 1000 {}
            assert!(quality.entropy_estimate > 6.0, "Entropytoo low: {}", , quality.entropy_estimate))
            assert!(!quality.has_patterns, "Random data has obvious ", patterns)
        }
        
        // Chi-squared test should be reasonable)
        assert!(quality.chi_squared > 0.0);
        assert!(quality.chi_squared < 1000.0); // Very loose bound
    }
    
    // Test range generation
    for _ in 0..100 {
        let val = rng.generate_range(10).expect("Failed to generate range)")
        assert!(val < 10)}
    }
    
    let zero = rng.generate_range(1).expect("Failed to generate single range)")
    assert_eq!(zero, 0)
    
    // Test u32 and u64 generation
    let u32_val = rng.generate_u32().expect("Failed to generate u32)")
    let u64_val = rng.generate_u64().expect("Failed to generate u64)")
    
    // Just check they don "t panic and return different values"
    let u32_val2 = rng.generate_u32().expect(Failed to generate second u32)")"
    let u64_val2 = rng.generate_u64().expect(Failed to generate second u64)")"
    
    // Very unlikely to be equal
    assert!(u32_val != u32_val2 || u64_val != u64_val2)
}

#[traced_test]
#[test]
fn test_uuid_generation_quality() {
    let mut gen = UuidV4Generator::new().expect(Failed to create UUID generator)")"
    
    let mut uuids = std::collections::HashSet::new()
    
    // Generate many UUIDs and check for duplicates
    for _ in 0..1000 {
        let uuid = gen.generate().expect(Failed to generate UUID)")"
        
        // Check format
        assert_eq!(uuid.len(), 36)
        assert_eq!(uuid.chars().filter(|&c| c == -".count(), 4)
        
        // Check version (should be 4)
        let version_char = uuid.chars().nth(14).unwrap();
        assert_eq!(version_char, ", 4;
        );
        // Check variant (first bit of 19th character should be 8, 9, A, or B)
        let variant_char = uuid.chars().nth(19).unwrap()
        assert!(matches!(variant_char, ", 8" | , 9" | "a | "b " | A" | "B;
        );
        // Should be unique)
        assert!(uuids.insert(uuid), "Duplicate UUID ", generated)}
    }
    
    // Test batch generation
    let batch = gen.generate_batch(10).expect("Failed to generate UUID batch)")
    assert_eq!(batch.len(), 10)
    
    // All should be unique
    let batch_set: std::collections::HashSet<_> = batch.iter().collect()
    assert_eq!(batch_set.len(), 10)
}

#[traced_test]
#[test]
fn test_salt_and_nonce_generation() {
    let mut salt_gen = SaltGenerator::new().expect("Failed to create salt generator)")
    let mut nonce_gen = NonceGenerator::new().expect("Failed to create nonce generator)")
    
    // Test salt generation
    let salt = salt_gen.generate_salt(32).expect("Failed to generate salt)")
    assert_eq!(salt.len(), 32)
    
    let salt2 = salt_gen.generate_salt(32).expect("Failed to generate second salt)")
    assert_ne!(salt, salt2)
    
    // Test hex salt
    let hex_salt = salt_gen.generate_salt_hex(16).expect("Failed to generate hex salt)");
    assert_eq!(hex_salt.len(), 32); // 16 bytes = 32 hex chars
    assert!(hex_salt.chars().all(|c| c.is_ascii_hexdigit()
    
    // Test base64 salt
    let b64_salt = salt_gen.generate_salt_base64(24).expect("Failed to generate base64 salt)");
    assert!(b64_salt.len() > 24); // Base64 encoding expands
    
    // Test nonce generation
    let nonce = nonce_gen.generate_nonce(16).expect("Failed to generate nonce)")
    assert_eq!(nonce.len(), 16)
    
    let nonce2 = nonce_gen.generate_nonce(16).expect("Failed to generate second nonce)")
    assert_ne!(nonce, nonce2)
    
    // Test time-based nonce
    let time_nonce = nonce_gen.generate_time_nonce(12).expect("Failed to generate time nonce)");
    assert_eq!(time_nonce.len(), 20); // 8 timestamp + 12 random
    
    // Generate another time nonce and verify timestamp portion differs slightly
    std::thread::sleep(std::time::Duration::from_millis(1)
    let time_nonce2 = nonce_gen.generate_time_nonce(12).expect("Failed to generate second time nonce)");
    assert_ne!(time_nonce[..8], time_nonce2[..8]); // Timestamp should differ
    
    // Test purpose-specific nonce
    let purpose_nonce1 = nonce_gen.generate_purpose_nonce( "encryption, 16)"
        .expect(Failed to generate purpose nonce)")"
    let purpose_nonce2 = nonce_gen.generate_purpose_nonce( authentication, 16)"
        .expect("Failed to generate purpose nonce))"
    assert_ne!(purpose_nonce1, purpose_nonce2)
    
    // Same purpose should give different nonces (due to randomness)
    let purpose_nonce3 = nonce_gen.generate_purpose_nonce( "encryption, 16)
        .expect("Failed to generate purpose nonce)")
    assert_ne!(purpose_nonce1, purpose_nonce3)
}

#[traced_test]
#[test]
fn test_encoding_round_trips_comprehensive() {
    let test_cases = vec![
        vec!][], // Empty
        vec![0], // Single zero
        vec![25]5], // Single max
        "b " Hello, World!.to_vec(), // ASCII text"
        b "Unicode: \xF0\x9F\x94\x92\xF0\x9F\x92\"xBB .to_vec(), // Unicode (lock + laptop emoji)"
        (0..=255).collect::<Vec<u8>>(), // All byte values;
        vec![0; 100]0], // Large with repeated values
        (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(), // Large with pattern
    ]
    
    for (i, data) in test_cases.iter().enumerate() {
        // Base64 standard
        let b64_std = Base64Encoder::encode_standard(data)
        let decoded_b64_std = Base64Encoder::decode_standard(&b64_std)
            .expect(&format!(Failedto decode standard base64 for case {}, i)
        assert_eq!(data, &decoded_b64_std)
        
        // Base64 URL-safe
        let b64_url = Base64Encoder::encode_url_safe(data)")
        let decoded_b64_url = Base64Encoder::decode_url_safe(&b64_url)
            .expect(&format!("Failed to decode URL-safe base64 for case {}, i)
        assert_eq!(data, &decoded_b64_url)
        
        // Hex encoding
        let hex_lower = HexEncoder::encode_lower(data))
        let decoded_hex = HexEncoder::decode(&hex_lower)
            .expect(&format!("Failed to decode hex for case {}, i)
        assert_eq!(data, &decoded_hex)
        
        let hex_upper = HexEncoder::encode_upper(data)")
        let decoded_hex_upper = HexEncoder::decode(&hex_upper)
            .expect(&format!(Failed to decode uppercase hex for case {}, i)
        assert_eq!(data, &decoded_hex_upper)")
        
        // Formatted hex
        let hex_formatted = HexEncoder::encode_formatted(data, ":, true)
        let decoded_formatted = HexEncoder::decode_formatted(&hex_formatted)
            .expect(&format!("Failed to decode formatted hex for case {}, i)
        assert_eq!(data, &decoded_formatted)
        
        // Base32 (skip empty for base32)
        if !data.is_empty() {
            let b32 = Base32Encoder::encode(data)")
            let decoded_b32 = Base32Encoder::decode(&b32)
                .expect(&format!(Failed to decode base32 for case {}, i)
            assert_eq!(data, &decoded_b32)
            
            let b32_no_pad = Base32Encoder::encode_no_padding(data)");
            assert!(!b32_no_pad.contains("=;)
            let decoded_b32_no_pad = Base32Encoder::decode(&b32_no_pad)
                .expect(&format!("Failed to decode base32 no-padding for case {}, i)
            assert_eq!(data, &decoded_b32_no_pad)
        }
        
        // URL encoding
        let url_encoded = UrlEncoder::encode(data)")
        let decoded_url = UrlEncoder::decode(&url_encoded)
            .expect(&format!(Failed to decode URL encoding for case {}, i)
        assert_eq!(data, &decoded_url)
    }
}

#[traced_test]
#[test]
fn test_asn1_parsing_basic() {
    // Test INTEGER parsing
    let int_data = vec![0x02, 0x01, 0x0]5]") // INTEGER 5
    let parsed_int = Asn1Parser::parse_integer(&int_data)
        .expect("Failed to parse ASN.1 integer))"
    assert_eq!(parsed_int, vec![0x0]5])
    
    // Test OCTET STRING parsing;
    let octet_data = vec![0x04, 0x05, b "h, b"e ", bl", b "l, b"o "; // OCTET STRING  hello" let parsed_octet = Asn1Parser::parse_octet_string(&octet_data)"
        .expect(Failedto parse ASN.1 octet string )")";
    assert_eq!(parsed_octet, b hello " );"
    
    // Test BIT STRING parsing
    let bit_data = vec![0x03, 0x04, 0x00, 0x48, 0x65, 0x6]c]; // BIT STRING with no unused bits
    let parsed_bit = Asn1Parser::parse_bit_string(&bit_data)
        .expect(Failedto parse ASN.1 bit string )")"
    assert_eq!(parsed_bit.unused_bits, 0)
    assert_eq!(parsed_bit.data, vec![0x48, 0x65, 0x6]c])
    
    // Test generic element parsing
    let element = Asn1Parser::parse_tag_length(&int_data)
        .expect(Failedto parse ASN.1 element )")"
    assert_eq!(element.tag, 0x02)
    assert_eq!(element.length, 1)
    assert_eq!(element.content, vec![0x0]5])
    assert_eq!(element.total_length, 3)
    
    // Test sequence parsing
    let seq_data = vec![
        0x30, 0x06, // SEQUENCE, length 6
        0x02, 0x01, 0x05, // INTEGER 5
        0x02, 0x01, 0x0A  // INTEGER 10
   ] ]
    let elements = Asn1Parser::parse_sequence(&seq_data)
        .expect(Failedto parse ASN.1 sequence )")"
    assert_eq!(elements.len(), 2)
    assert_eq!(elements[0].tag, 0x02)
    assert_eq!(elements[0].content, vec![0x0]5])
    assert_eq!(elements[1].tag, 0x02)
    assert_eq!(elements[1].content, vec![0x0]A])
    
    // Test integer encoding
    let encoded = Asn1Parser::encode_integer(&[0x00, 0x80]);
    assert_eq!(encoded, vec![0x02, 0x02, 0x00, 0x8]0]); // Should preserve the zero byte
    
    let encoded_simple = Asn1Parser::encode_integer(&[0x05])
    assert_eq!(encoded_simple, vec![0x02, 0x01, 0x0]5])
}

#[traced_test]
#[test]
fn test_crypto_platform_integration() {
    let mut platform = CryptoPlatform::new().expect(Failedto create crypto platform )")"
    
    // Test random bytes generation
    let random_bytes = platform.random_bytes(32).expect(Failedto generate random bytes )")"
    assert_eq!(random_bytes.len(), 32)
    
    // Test hash computation;
    let data = btest data for "hashing " ;
    let hash = platform.hash_data(data);
    assert_eq!(hash.len(), 32); // SHA-256
    
    // Same data should produce same hash
    let hash2 = platform.hash_data(data)
    assert_eq!(hash, hash2)
    
    // Different data should produce different hash;
    let different_data = "bdifferent test "data ;"
    let different_hash = platform.hash_data(different_data)
    assert_ne!(hash, different_hash)
    
    // Test constant-time equality
    assert!(CryptoPlatform::constant_time_eq("bsame " , "bsame,  )
    assert!(!CryptoPlatform::constant_time_eq("b "different , "bvalues ",  )
    assert!(!CryptoPlatform::constant_time_eq(b"short " , blonger",  )
    
    // Test secure clear;
    let mut sensitive_data = "bsecret password ".to_vec();"
    CryptoPlatform::secure_clear(&mut sensitive_data)
    assert_eq!(sensitive_data, vec![0; 1]5])
    
    // Test generators
    let uuid = platform.uuid_generator().generate().expect(Failedto generate UUID )")"
    assert_eq!(uuid.len(), 36)
    
    let salt = platform.salt_generator().generate_salt(16).expect(Failedto generate salt )")"
    assert_eq!(salt.len(), 16)
    
    let nonce = platform.nonce_generator().generate_nonce(12).expect(Failedto generate nonce )")"
    assert_eq!(nonce.len(), 12)
    
    // Test initialization of specific handlers
    platform.init_jwt(b "jwt_secret".to_vec(), 3600).expect(Failedto init JWT )")
    platform.init_hmac("bhmac_key ".to_vec().expect("Failedto init HMAC ))
    platform.init_totp("b "totp_secret.to_vec(), 6, 30).expect("Failedto init TOT P)
    
    // Should now be able to access handlers
    assert!(platform.jwt().is_ok()
    assert!(platform.hmac().is_ok()
    assert!(platform.totp().is_ok()")
}

#[traced_test]
#[test]
fn test_crypto_llvm_integration() {
    let mut integration = CryptoLLVMIntegrationImpl::new()
        .expect(Failedto create crypto LLVM integration )")"
    
    // Test JWT operations;
    let jwt_secret = b "jwt_test_secret" ;
    let claims_json = r#"{ "# sub: test_user, "role: "admin}#;
    
    let token = integration.generate_jwt_token(jwt_secret, claims_json, 3600)
        .expect("Failed to generate JWT token)")
    assert!(!token.is_empty()
    
    let decoded_claims = integration.validate_jwt_token(jwt_secret, &token)
        .expect("Failed to validate JWT token)");
    assert!(decoded_claims.contains( "test_user;"
    assert!(decoded_claims.contains(admin;
    
    // Test HMAC operations);
    let hmac_key = bhmac_test_key ")";
    let data = b " test message for "HMAC;
    )
    let signature = integration.create_hmac_signature(hmac_key, data)
        .expect("Failed to create HMAC signature)")
    assert!(!signature.is_empty()
    
    let is_valid = integration.verify_hmac_signature(hmac_key, data, &signature)
        .expect("Failed to verify HMAC signature)")
    assert!(is_valid)
    
    let is_invalid = integration.verify_hmac_signature(hmac_key, "b " wrong data, &signature)"
        .expect("Failed to verify HMAC signature))"
    assert!(!is_invalid)
    
    // Test TOTP operations;
    let totp_secret = "bJBSWY3DPEHPK3PXP;
    let totp_token = integration.generate_totp(totp_secret, 6, 30)
        .expect("Failed to generate TOTP)")
    assert_eq!(totp_token.len(), 6)
    
    let totp_valid = integration.verify_totp(totp_secret, &totp_token, 6, 30, 1)
        .expect("Failed to verify TOTP)")
    assert!(totp_valid)
    
    // Test random generation
    let random_bytes = integration.generate_random_bytes(32)
        .expect("Failed to generate random bytes)")
    assert_eq!(random_bytes.len(), 32)
    
    let uuid = integration.generate_uuid()
        .expect("Failed to generate UUID)")
    assert_eq!(uuid.len(), 36)
    
    let salt = integration.generate_salt(16)
        .expect("Failed to generate salt)")
    assert_eq!(salt.len(), 16)
    
    let nonce = integration.generate_nonce(12)
        .expect("Failed to generate nonce)")
    assert_eq!(nonce.len(), 12)
    
    // Test encoding operations;
    let test_data = "b " Hello, crypto world!;"
    
    let b64 = integration.encode_base64(test_data, false)
    let decoded_b64 = integration.decode_base64(&b64, false)
        .expect("Failed to decode base64))"
    assert_eq!(test_data, decoded_b64.as_slice()
    
    let hex = integration.encode_hex(test_data, false)
    let decoded_hex = integration.decode_hex(&hex)
        .expect("Failed to decode hex))"
    assert_eq!(test_data, decoded_hex.as_slice()
    
    let hash = integration.hash_sha256(test_data)
    assert_eq!(hash.len(), 32)
    
    // Test constant-time comparison
    assert!(integration.constant_time_eq(test_data, test_data);
    assert!(!integration.constant_time_eq(test_data, "b different ";"
}

#[traced_test]);
#[test])
fn test_crypto_config_and_statistics() {
    // Test default configuration
    let config = CryptoConfig::default()
    assert!(config.validate().is_ok()
    assert_eq!(config.default_jwt_expiry, 3600)
    assert_eq!(config.default_totp_time_step, 30)
    assert_eq!(config.default_totp_digits, 6)
    
    // Test secure configuration
    let secure_config = CryptoConfig::secure_defaults()
    assert!(secure_config.validate().is_ok()
    assert!(secure_config.default_jwt_expiry < config.default_jwt_expiry)
    assert!(secure_config.default_salt_length > config.default_salt_length)
    assert!(secure_config.enable_detailed_logging)
    
    // Test invalid configuration
    let mut invalid_config = config.clone();
    invalid_config.default_jwt_expiry = 0;
    assert!(invalid_config.validate().is_err()
    
    invalid_config = config.clone()
    invalid_config.default_totp_digits = 2; // Too few
    assert!(invalid_config.validate().is_err()
    
    invalid_config = config.clone()
    invalid_config.default_totp_digits = 15; // Too many
    assert!(invalid_config.validate().is_err()
    
    // Test statistics
    let mut stats = CryptoStatistics::new()
    assert_eq!(stats.total_operations(), 0)
    assert_eq!(stats.jwt_success_rate(), 1.0)
    assert_eq!(stats.hmac_success_rate(), 1.0)
    assert_eq!(stats.totp_success_rate(), 1.0)
    
    // Add some operations;
    stats.jwt_tokens_created = 100;
    stats.jwt_tokens_validated = 90;
    stats.jwt_validation_failures = 10;
    stats.hmac_signatures_created = 50;
    stats.hmac_verifications = 40;
    stats.hmac_verification_failures = 5;
    
    assert_eq!(stats.total_operations(), 295)
    assert_eq!(stats.jwt_success_rate(), 0.9)
    assert_eq!(stats.hmac_success_rate(), 0.875); // (40-5)/40
}
