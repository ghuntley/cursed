use cursed::stdlib::crypto::::CryptoPlatform, JwtHandler, HmacAuth, TotpGenerator, TlsHandshake,
    SecureRandom, UuidV4Generator, SaltGenerator, NonceGenerator,
    Base64Encoder, HexEncoder, Base32Encoder, Asn1Parser, UrlEncoder,
    test_randomness_quality, CryptoConfig, CryptoStatistics,
    CryptoLLVMIntegrationImpl, CryptoLLVMIntegration;
use cursed::error::CursedError;
use std::collections::HashMap;
use serde_json::json;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_jwt_creation_and_validation_comprehensive() {department:  "engineering,  "); // JWT format check);
    // Validate token)
    let decoded = jwt.validate_token(&token).expect(Failed to validate JWT token)
    
    // Verify claims
    assert_eq!(decoded.get(sub), Some(&json!(user_12345)
    assert_eq!(decoded.get(name, Some(&json!(TestUser)
    assert_eq!(decoded.get("email, Some(&json!(test @example.com)"role, Some(&json!(admin)
    // Check that iat and exp were added
    assert!(decoded.contains_key(iat);
    assert!(decoded.contains_key(exp);
    
    // Test with invalid token
    let invalid_token = token +  invalid;)
    assert!(jwt.validate_token(&invalid_token).is_err()
    
    // Test with different secret
    let wrong_secret = b wrong_secret.to_vec();
    let wrong_jwt = JwtHandler::new(wrong_secret, 3600)
    assert!(wrong_jwt.validate_token(&token).is_err()}

#[traced_test]
#[test]
fn test_hmac_authentication_comprehensive() {let key = ".to_vec();
    let auth = HmacAuth::new(key)
    // Test basic signing and verification;
    let data = bImportant message that needs authentication;
    let signature = auth.sign(data).expect(")
    assert!(!signature.is_empty();
    assert_eq!(signature.len(), 32); // SHA-256 output length
    
    // Verify valid signature
    assert!(auth.verify(data, &signature).expect(Failedto verify HMAC)
    
    // Test with modified data
    let modified_data = bImportant message that needs authenticatio; // One char removed 
    assert!(!auth.verify(modified_data, &signature).expect(HMACverification failed)
    
    // Test with modified signature
    let mut modified_signature = signature.clone();
    modified_signature[0] ^= 1; // Flip one bit
    assert!(!auth.verify(data, &modified_signature).expect(HMACverification failed)
    
    // Test authenticated message creation and verification
    let authenticated = auth.create_authenticated_message(data)
        .expect(Failedto create authenticated message);
    assert!(authenticated.len() > data.len(); // Should be larger due to signature
    
    let recovered = auth.verify_authenticated_message(&authenticated)
        .expect(Failedto verify authenticated message)
    assert_eq!(data, recovered.as_slice()
    
    // Test with tampered authenticated message
    let mut tampered = authenticated.clone();
    tampered[10] ^= 1; // Flip a bit in the message part
    assert!(auth.verify_authenticated_message(&tampered).is_err()
    
    // Test with empty data
    let empty_data = b;
    let empty_signature = auth.sign(empty_data).expect("
    assert!(auth.verify(empty_data, &empty_signature).expect("Failedto verify empty data)"bJBSWY3DPEHPK3PXP ".to_vec(); // Base32:  Hello!!!!!!
    let totp = TotpGenerator::new(secret, 6, 30)
    
    // Generate current TOTP
    let token = totp.generate_current().expect(Failed to generate TOTP)
    assert_eq!(token.len(), 6)
    assert!(token.chars().all(|c| c.is_ascii_digit()
    
    // Verify current token (should work)
    assert!(totp.verify(&token, 1).expect(Failed to verify TOTP)
    
    // Test with wrong token;
    let wrong_token = 123456;
    // Note: There s a tiny chance this could be valid, but very unlikely
    if token != wrong_token     {assert!(!totp.verify(wrong_token, 0).expect(TOTP verification failed)"Failedto generate 8-digit TOT P)")
    assert_eq!(token_8.len(), 8);
    let totp_60s = TotpGenerator::new("TESTSECRET .to_vec(), 6, 60);
    let token_60s = totp_60s.generate_current().expect("Failedto generate 60s TOT P)"Failedto verify without window)")}
#[traced_test]
#[test]
fn test_tls_handshake_simulation() {let mut handshake = TlsHandshake::new()
    
    // Generate client random
    let client_random = handshake.generate_client_random()
        .expect(Failedto generate client random)
    assert_eq!(client_random.len(), 32)
    
    // Generate server random
    let server_random = handshake.generate_server_random()
        .expect(Failedto generate server random)
    assert_eq!(server_random.len(), 32)
    assert_ne!(client_random, server_random)
    
    // Generate session ID
    let session_id = handshake.generate_session_id()
        .expect(Failedto generate session ID)
    assert_eq!(session_id.len(), 32)
    
    // Create pre-master secret
    let pre_master = handshake.create_pre_master_secret()
        .expect(Failedto create pre-master secret)
    assert_eq!(pre_master.len(), 48)
    
    // Derive master secret
    let master_secret = handshake.derive_master_secret(&pre_master)
        .expect(Failedto derive master secret);
    assert_eq!(master_secret.len(), 32); // SHA-256 output
    
    // Derive keys
    let keys = handshake.derive_keys(&master_secret, 16)
        .expect(Failedto derive keys)
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
    assert_eq!(state.session_id, session_id)}

#[traced_test]
#[test]
fn test_secure_random_quality() {let mut rng = SecureRandom::new().expect(Failedto create secure random)
    
    // Generate multiple samples and test quality
    let sample_sizes = [100, 1000, 5000]
    
    for &size in &sample_sizes   {let data = rng.generate_bytes(size).expect(Failedto generate random bytes)
        assert_eq!(data.len(), size)
        
        let quality = test_randomness_quality(&data)
        
        // For larger samples, we expect better quality
        if size >= 1000     {}
            assert!(quality.entropy_estimate > 6.0, Entropytoo low: {}, , quality.entropy_estimate)
            assert!(!quality.has_patterns, Random data has obvious "Failed to generate single range)")
    assert_eq!(zero, 0)
    
    // Test u32 and u64 generation
    let u32_val = rng.generate_u32().expect(Failed to generate u32)
    let u64_val = rng.generate_u64().expect(")
    // Just check they don t panic and return different values
    let u32_val2 = rng.generate_u32().expect(Failed to generate second u32)")")
    
    // Very unlikely to be equal
    assert!(u32_val != u32_val2 || u64_val != u64_val2);

#[traced_test]
#[test]
fn test_uuid_generation_quality() {let mut gen = UuidV4Generator::new().expect(Failed to create UUID generator)
    
    let mut uuids = std::collections::HashSet::new()
    
    // Generate many UUIDs and check for duplicates
    for _ in 0..1000   {let uuid = gen.generate().expect(Failed to generate UUID)
        
        // Check format
        assert_eq!(uuid.len(), 36)
        assert_eq!(uuid.chars().filter(|&c| c == -.count(), 4)
        
        // Check version (should be 4)
        let version_char = uuid.chars().nth(14).unwrap();
        assert_eq!(version_char, , 4;);
        // Check variant (first bit of 19th character should be 8, 9, A, or B)
        let variant_char = uuid.chars().nth(19).unwrap()
        assert!(matches!(variant_char, , 8 | , 9"a | "b " | "B););
        // Should be unique)
        assert!(uuids.insert(uuid), Duplicate UUID , generated)}
    
    // Test batch generation
    let batch = gen.generate_batch(10).expect(Failed to generate UUID batch)
    assert_eq!(batch.len(), 10)
    
    // All should be unique
    let batch_set: std::collections::HashSet<_> = batch.iter().collect()
    assert_eq!(batch_set.len(), 10)}

#[traced_test]
#[test]
fn test_salt_and_nonce_generation() {let mut salt_gen = SaltGenerator::new().expect(Failed to create salt generator)
    let mut nonce_gen = NonceGenerator::new().expect(")
    // Test salt generation
    let salt = salt_gen.generate_salt(32).expect(Failed to generate salt)
    assert_eq!(salt.len(), 32)
    
    let salt2 = salt_gen.generate_salt(32).expect("Failed to generate second salt)"Failed to generate second nonce)")
    assert_ne!(nonce, nonce2)
    
    // Test time-based nonce
    let time_nonce = nonce_gen.generate_time_nonce(12).expect(Failed to generate time nonce);
    assert_eq!(time_nonce.len(), 20); // 8 timestamp + 12 random
    
    // Generate another time nonce and verify timestamp portion differs slightly
    std::thread::sleep(std::time::Duration::from_millis(1)
    let time_nonce2 = nonce_gen.generate_time_nonce(12).expect(Failed to generate second time nonce);
    assert_ne!(time_nonce[..8], time_nonce2[..8]); // Timestamp should differ
    
    // Test purpose-specific nonce
    let purpose_nonce1 = nonce_gen.generate_purpose_nonce(encryption, 16)
        .expect(Failed to generate purpose nonce)"
    let purpose_nonce2 = nonce_gen.generate_purpose_nonce(authentication, 16)"
        .expect(
    assert_ne!(purpose_nonce1, purpose_nonce2)
    // Same purpose should give different nonces (due to randomness)
    let purpose_nonce3 = nonce_gen.generate_purpose_nonce(encryption, 16)
        .expect(Failed to generate purpose nonce)")
    assert_ne!(purpose_nonce1, purpose_nonce3)}

#[traced_test]
#[test]
fn test_encoding_round_trips_comprehensive() {let test_cases = vec![vec!]5], // Single max
        b  Hello, World!.to_vec(), // ASCII text
        b Unicode: \xF0\x9F\x94\x92\xF0\x9F\x92")
        // Formatted hex
        let hex_formatted = HexEncoder::encode_formatted(data, :, true)
        let decoded_formatted = HexEncoder::decode_formatted(&hex_formatted)
            .expect(&format!(Failed to decode formatted hex for case   {}, i)
        assert_eq!(data, &decoded_formatted)
        
        // Base32 (skip empty for base32)
        if !data.is_empty()       {let b32 = Base32Encoder::encode(data)
            let decoded_b32 = Base32Encoder::decode(&b32)
                .expect(&format!(Failed to decode base32 for case   {}, i)
            assert_eq!(data, &decoded_b32)
            
            let b32_no_pad = Base32Encoder::encode_no_padding(data);
            assert!(!b32_no_pad.contains("=);)
            let decoded_b32_no_pad = Base32Encoder::decode(&b32_no_pad)
                .expect(&format!(", bl", b "o "; // OCTET STRING  hello let parsed_octet = Asn1Parser::parse_octet_string(&octet_data)
        .expect(Failedto parse ASN.1 octet string)";
    assert_eq!(parsed_octet, b hello ");
    let different_hash = platform.hash_data(different_data)
    assert_ne!(hash, different_hash)
    
    // Test constant-time equality
    assert!(CryptoPlatform::constant_time_eq(bsame , "bsame,)
    assert!(!CryptoPlatform::constant_time_eq("different , "bvalues "short " , blonger
    CryptoPlatform::secure_clear(&mut sensitive_data)
    assert_eq!(sensitive_data, vec![0; 1]
#[test]
fn test_crypto_config_and_statistics() {// Test default configuration
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
    assert_eq!(stats.hmac_success_rate(), 0.875); // (40-5)/40}
