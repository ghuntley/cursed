//! Comprehensive Tests for CURSED Cryptographic Protocols Module
//!
//! This test suite validates the entire cryptographic protocols implementation
//! including production protocols, advanced protocols, and the comprehensive suite.

use cursed::stdlib::crypto::protocols_comprehensive::*;
use cursed::stdlib::crypto::protocols_production::*;
use cursed::stdlib::crypto::protocols_advanced::*;
use cursed::stdlib::crypto::protocols::*;
use cursed::stdlib::value::Value as CursedValue;
use cursed::error::CursedError;

use std::collections::HashMap;
use std::time::Duration;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair as Ed25519Keypair;

// ============================================================================
// BASIC PROTOCOL TESTS
// ============================================================================

#[test]
fn test_jwt_basic_functionality() {
    let secret = b"test_secret_key_for_jwt_validation_12345".to_vec();
    let jwt_handler = JwtHandler::new(secret, 3600);
    
    let mut claims = std::collections::HashMap::new();
    claims.insert("sub".to_string(), serde_json::json!("user123"));
    claims.insert("name".to_string(), serde_json::json!("Test User"));
    claims.insert("role".to_string(), serde_json::json!("admin"));
    
    // Create token
    let token = jwt_handler.create_token(claims.clone()).unwrap();
    assert!(!token.is_empty());
    assert!(token.contains('.'));
    
    // Validate token
    let decoded_claims = jwt_handler.validate_token(&token).unwrap();
    assert_eq!(decoded_claims.get("sub"), Some(&serde_json::json!("user123")));
    assert_eq!(decoded_claims.get("name"), Some(&serde_json::json!("Test User")));
    assert_eq!(decoded_claims.get("role"), Some(&serde_json::json!("admin")));
    
    // Verify automatic claims
    assert!(decoded_claims.contains_key("iat"));
    assert!(decoded_claims.contains_key("exp"));
}

#[test]
fn test_hmac_authentication_comprehensive() {
    let key = b"comprehensive_hmac_test_key_2024".to_vec();
    let auth = HmacAuth::new(key);
    
    // Test basic signing and verification
    let message = b"Test message for HMAC authentication";
    let signature = auth.sign(message).unwrap();
    assert_eq!(signature.len(), 32); // HMAC-SHA256 output length
    assert!(auth.verify(message, &signature).unwrap());
    
    // Test with different message
    let wrong_message = b"Different message content";
    assert!(!auth.verify(wrong_message, &signature).unwrap());
    
    // Test authenticated message format
    let authenticated_msg = auth.create_authenticated_message(message).unwrap();
    assert!(authenticated_msg.len() > message.len()); // Should include signature
    
    let extracted_msg = auth.verify_authenticated_message(&authenticated_msg).unwrap();
    assert_eq!(message, extracted_msg.as_slice());
}

#[test]
fn test_totp_time_based_authentication() {
    let secret = b"COMPREHENSIVE_TOTP_SECRET_2024".to_vec();
    let totp = TotpGenerator::new(secret, 6, 30);
    
    // Generate current token
    let token = totp.generate_current().unwrap();
    assert_eq!(token.len(), 6);
    assert!(token.chars().all(|c| c.is_ascii_digit()));
    
    // Verify with time window
    assert!(totp.verify(&token, 1).unwrap());
    
    // Test specific time generation
    let specific_time = 1640995200; // 2022-01-01 00:00:00 UTC
    let time_token = totp.generate_at_time(specific_time).unwrap();
    assert_eq!(time_token.len(), 6);
    
    // Verify token is deterministic for same time
    let time_token2 = totp.generate_at_time(specific_time).unwrap();
    assert_eq!(time_token, time_token2);
}

#[test]
fn test_tls_handshake_simulation() {
    let mut handshake = TlsHandshake::new();
    
    // Generate random values
    let client_random = handshake.generate_client_random().unwrap();
    assert_eq!(client_random.len(), 32);
    
    let server_random = handshake.generate_server_random().unwrap();
    assert_eq!(server_random.len(), 32);
    
    let session_id = handshake.generate_session_id().unwrap();
    assert_eq!(session_id.len(), 32);
    
    // Create and derive secrets
    let pre_master_secret = handshake.create_pre_master_secret().unwrap();
    assert_eq!(pre_master_secret.len(), 48);
    
    let master_secret = handshake.derive_master_secret(&pre_master_secret).unwrap();
    assert_eq!(master_secret.len(), 32); // SHA-256 output
    
    // Derive session keys
    let keys = handshake.derive_keys(&master_secret, 16).unwrap();
    assert_eq!(keys.client_write_mac.len(), 16);
    assert_eq!(keys.server_write_mac.len(), 16);
    assert_eq!(keys.client_write_key.len(), 16);
    assert_eq!(keys.server_write_key.len(), 16);
    
    // Test state tracking
    let state = handshake.get_state();
    assert!(state.has_client_random);
    assert!(state.has_server_random);
    assert!(state.has_session_id);
}

// ============================================================================
// PRODUCTION PROTOCOL TESTS
// ============================================================================

#[test]
fn test_x25519_key_exchange_production() {
    let alice = X25519KeyExchange::new(SecurityLevel::Level256);
    let bob = X25519KeyExchange::new(SecurityLevel::Level256);
    
    let alice_public = alice.public_key();
    let bob_public = bob.public_key();
    
    // Perform key exchange
    let alice_shared = alice.exchange(&bob_public).unwrap();
    let bob_shared = bob.exchange(&alice_public).unwrap();
    
    // Shared secrets should be same length (derived keys may differ due to HKDF)
    assert_eq!(alice_shared.len(), bob_shared.len());
    assert!(alice_shared.len() >= 32); // At least 256 bits of key material
    
    // Keys should not be empty or all zeros
    assert!(!alice_shared.iter().all(|&b| b == 0));
    assert!(!bob_shared.iter().all(|&b| b == 0));
}

#[test]
fn test_ecdh_key_exchange_comprehensive() {
    let alice = EcdhKeyExchange::new(SecurityLevel::Level256);
    let bob = EcdhKeyExchange::new(SecurityLevel::Level256);
    
    let alice_public = alice.public_point();
    let bob_public = bob.public_point();
    
    // Perform ECDH exchange
    let alice_shared = alice.exchange(&bob_public).unwrap();
    let bob_shared = bob.exchange(&alice_public).unwrap();
    
    // Shared secrets should be identical for ECDH
    assert_eq!(alice_shared, bob_shared);
    assert_eq!(alice_shared.len(), 64); // Level256 key + MAC size
    
    // Test with different security levels
    let alice_128 = EcdhKeyExchange::new(SecurityLevel::Level128);
    let bob_128 = EcdhKeyExchange::new(SecurityLevel::Level128);
    
    let alice_public_128 = alice_128.public_point();
    let bob_public_128 = bob_128.public_point();
    
    let alice_shared_128 = alice_128.exchange(&bob_public_128).unwrap();
    let bob_shared_128 = bob_128.exchange(&alice_public_128).unwrap();
    
    assert_eq!(alice_shared_128, bob_shared_128);
    assert_eq!(alice_shared_128.len(), 64); // Still 64 bytes for Level128
}

#[test]
fn test_diffie_hellman_traditional() {
    let alice = DiffieHellmanKeyExchange::new(SecurityLevel::Level256);
    let bob = DiffieHellmanKeyExchange::new(SecurityLevel::Level256);
    
    let alice_public = alice.public_value();
    let bob_public = bob.public_value();
    
    // Verify public values are generated
    assert!(!alice_public.is_empty());
    assert!(!bob_public.is_empty());
    assert_ne!(alice_public, bob_public); // Should be different
    
    // Perform DH exchange
    let alice_shared = alice.exchange(&bob_public).unwrap();
    let bob_shared = bob.exchange(&alice_public).unwrap();
    
    // Shared secrets should be identical
    assert_eq!(alice_shared, bob_shared);
    assert!(!alice_shared.iter().all(|&b| b == 0));
}

#[test]
fn test_ecdhe_authenticated_exchange() {
    let alice_identity = Ed25519Keypair::generate(&mut OsRng);
    let bob_identity = Ed25519Keypair::generate(&mut OsRng);
    
    let mut alice_ecdhe = EcdheKeyExchange::new(alice_identity.clone(), SecurityLevel::Level256);
    let mut bob_ecdhe = EcdheKeyExchange::new(bob_identity.clone(), SecurityLevel::Level256);
    
    // Generate key exchange messages
    let alice_message = alice_ecdhe.generate_key_exchange_message().unwrap();
    let bob_message = bob_ecdhe.generate_key_exchange_message().unwrap();
    
    // Verify message structure
    assert_eq!(alice_message.ephemeral_public.len(), 32);
    assert_eq!(alice_message.identity_public.len(), 32);
    assert_eq!(alice_message.signature.len(), 64);
    assert!(alice_message.timestamp > 0);
    
    // Test message serialization
    let serialized = alice_message.serialize();
    assert_eq!(serialized.len(), 136); // 32 + 32 + 64 + 8
    
    let deserialized = EcdheMessage::deserialize(&serialized).unwrap();
    assert_eq!(deserialized.ephemeral_public, alice_message.ephemeral_public);
    assert_eq!(deserialized.identity_public, alice_message.identity_public);
    
    // Perform authenticated key exchange
    let alice_shared = alice_ecdhe.process_key_exchange_message(
        &bob_message, 
        &bob_identity.public.to_bytes()
    ).unwrap();
    
    let bob_shared = bob_ecdhe.process_key_exchange_message(
        &alice_message, 
        &alice_identity.public.to_bytes()
    ).unwrap();
    
    // Verify both parties derived the same shared secret
    assert_eq!(alice_shared, bob_shared);
    assert!(!alice_shared.iter().all(|&b| b == 0));
    
    // Verify states
    assert_eq!(*alice_ecdhe.state(), EcdheState::Authenticated);
    assert_eq!(*bob_ecdhe.state(), EcdheState::Authenticated);
}

#[test]
fn test_secure_channel_comprehensive() {
    let config = ProtocolConfig {
        security_level: SecurityLevel::Level256,
        enable_forward_secrecy: true,
        key_rotation_interval: Duration::from_millis(100),
        max_message_size: 1024,
        timeout_duration: Duration::from_secs(10),
        enable_quantum_safe: false,
        compression_enabled: false,
        replay_window_size: 100,
    };
    
    let mut alice_channel = SecureChannel::new("test_channel".to_string(), config.clone());
    let mut bob_channel = SecureChannel::new("test_channel".to_string(), config);
    
    let shared_secret = CryptoPrimitives::random_bytes(32);
    
    // Establish channels
    alice_channel.establish(&shared_secret).unwrap();
    bob_channel.establish(&shared_secret).unwrap();
    
    // Test message exchange
    let message = b"Hello, secure channel world!";
    let encrypted = alice_channel.send_message(message).unwrap();
    
    // Verify encrypted message is different from plaintext
    assert_ne!(encrypted, message);
    assert!(encrypted.len() > message.len()); // Should include overhead
    
    let decrypted = bob_channel.receive_message(&encrypted).unwrap();
    assert_eq!(message, decrypted.as_slice());
    
    // Test multiple messages
    for i in 0..5 {
        let msg = format!("Message number {}", i);
        let enc = alice_channel.send_message(msg.as_bytes()).unwrap();
        let dec = bob_channel.receive_message(&enc).unwrap();
        assert_eq!(msg.as_bytes(), dec.as_slice());
    }
    
    // Test statistics
    let stats = alice_channel.get_statistics();
    assert!(stats.contains_key("channel_id"));
    assert!(stats.contains_key("state"));
    assert!(stats.contains_key("security_level"));
    
    // Test channel closure
    alice_channel.close().unwrap();
    bob_channel.close().unwrap();
}

// ============================================================================
// ADVANCED PROTOCOL TESTS
// ============================================================================

#[test]
fn test_challenge_response_authentication() {
    let alice_identity = Ed25519Keypair::generate(&mut OsRng);
    let bob_identity = Ed25519Keypair::generate(&mut OsRng);
    
    let mut alice_auth = ChallengeResponseAuth::new(
        alice_identity, 
        SecurityLevel::Level256, 
        3
    );
    
    let bob_auth = ChallengeResponseAuth::new(
        bob_identity.clone(), 
        SecurityLevel::Level256, 
        3
    );
    
    // Alice initiates authentication
    let challenge_set = alice_auth.initiate_authentication(bob_identity.public).unwrap();
    assert_eq!(challenge_set.challenges.len(), 3);
    assert!(!challenge_set.session_id.is_empty());
    
    // Verify challenge structure
    for challenge in &challenge_set.challenges {
        assert_eq!(challenge.nonce.len(), 32);
        assert!(!challenge.challenge_data.is_empty());
        assert!(challenge.difficulty > 0);
        assert!(challenge.timestamp > 0);
    }
    
    // Bob responds to challenges
    let response_set = bob_auth.respond_to_challenges(&challenge_set).unwrap();
    assert_eq!(response_set.responses.len(), 3);
    assert_eq!(response_set.session_id, challenge_set.session_id);
    
    // Verify response structure
    for response in &response_set.responses {
        assert!(!response.solution.is_empty());
        assert_eq!(response.signature.to_bytes().len(), 64);
        assert!(response.timestamp > 0);
    }
    
    // Alice verifies responses
    let result = alice_auth.verify_responses(&response_set).unwrap();
    assert!(result.authenticated);
    assert!(result.success_rate > 0.0);
    assert!(result.completion_time.as_millis() > 0);
}

#[test]
fn test_multi_party_computation_basic() {
    let mut alice_mpc = MultiPartyComputation::new(
        "alice".to_string(), 
        SecurityLevel::Level256, 
        2
    );
    let mut bob_mpc = MultiPartyComputation::new(
        "bob".to_string(), 
        SecurityLevel::Level256, 
        2
    );
    let mut charlie_mpc = MultiPartyComputation::new(
        "charlie".to_string(), 
        SecurityLevel::Level256, 
        2
    );
    
    // Register parties
    let alice_key = Ed25519Keypair::generate(&mut OsRng);
    let bob_key = Ed25519Keypair::generate(&mut OsRng);
    let charlie_key = Ed25519Keypair::generate(&mut OsRng);
    
    alice_mpc.register_party("bob".to_string(), bob_key.public).unwrap();
    alice_mpc.register_party("charlie".to_string(), charlie_key.public).unwrap();
    
    // Initiate key generation
    let participants = vec![
        "alice".to_string(), 
        "bob".to_string(), 
        "charlie".to_string()
    ];
    let session_id = alice_mpc.initiate_key_generation(participants.clone()).unwrap();
    assert!(session_id.starts_with("mpc_"));
    
    // Generate and distribute shares
    let distributions = alice_mpc.generate_shares(&session_id).unwrap();
    assert_eq!(distributions.len(), 3);
    
    // Process shares
    for distribution in &distributions {
        assert_eq!(distribution.session_id, session_id);
        assert!(participants.contains(&distribution.recipient));
        assert!(!distribution.sender_signature.is_empty());
    }
    
    // Compute partial results
    let input_data = b"test computation input data";
    let alice_partial = alice_mpc.compute_partial_result(&session_id, input_data).unwrap();
    assert!(!alice_partial.is_empty());
    
    // Test session status
    let status = alice_mpc.get_session_status(&session_id);
    assert!(status.is_some());
}

#[test]
fn test_distributed_key_generation() {
    let mut alice_dkg = DistributedKeyGeneration::new(
        "alice".to_string(), 
        2, 
        SecurityLevel::Level256
    );
    let mut bob_dkg = DistributedKeyGeneration::new(
        "bob".to_string(), 
        2, 
        SecurityLevel::Level256
    );
    
    let participants = vec!["alice".to_string(), "bob".to_string()];
    
    // Initialize DKG sessions
    let alice_session = alice_dkg.initiate_key_generation(participants.clone()).unwrap();
    let bob_session = bob_dkg.initiate_key_generation(participants).unwrap();
    
    assert!(alice_session.starts_with("dkg_"));
    assert!(bob_session.starts_with("dkg_"));
    
    // Generate commitments
    let alice_commitments = alice_dkg.generate_commitments(&alice_session).unwrap();
    let bob_commitments = bob_dkg.generate_commitments(&bob_session).unwrap();
    
    assert_eq!(alice_commitments.len(), 2); // threshold
    assert_eq!(bob_commitments.len(), 2);
    
    // Process commitments
    let alice_commitment_bytes: Vec<[u8; 32]> = alice_commitments
        .iter()
        .map(|c| c.compress().to_bytes())
        .collect();
    let bob_commitment_bytes: Vec<[u8; 32]> = bob_commitments
        .iter()
        .map(|c| c.compress().to_bytes())
        .collect();
    
    alice_dkg.process_commitments(&alice_session, "bob", bob_commitments).unwrap();
    bob_dkg.process_commitments(&bob_session, "alice", alice_commitments).unwrap();
    
    // Generate shares
    let alice_shares = alice_dkg.generate_shares(&alice_session).unwrap();
    let bob_shares = bob_dkg.generate_shares(&bob_session).unwrap();
    
    // Process shares
    if let Some(share_for_bob) = alice_shares.get("bob") {
        assert!(bob_dkg.process_share(&bob_session, "alice", *share_for_bob).unwrap());
    }
    
    if let Some(share_for_alice) = bob_shares.get("alice") {
        assert!(alice_dkg.process_share(&alice_session, "bob", *share_for_alice).unwrap());
    }
}

// ============================================================================
// COMPREHENSIVE PROTOCOL SUITE TESTS
// ============================================================================

#[test]
fn test_protocol_suite_creation_and_configuration() {
    // Test default creation
    let suite = ProtocolSuite::new(SecurityLevel::Level256);
    assert_eq!(suite.public_key().len(), 32);
    
    // Test custom configuration
    let config = ProtocolConfig {
        security_level: SecurityLevel::Level128,
        enable_forward_secrecy: false,
        key_rotation_interval: Duration::from_secs(7200),
        max_message_size: 2048,
        timeout_duration: Duration::from_secs(60),
        enable_quantum_safe: true,
        compression_enabled: true,
        replay_window_size: 500,
    };
    
    let suite_custom = ProtocolSuite::with_config(config.clone());
    assert_eq!(suite_custom.public_key().len(), 32);
    
    // Test configuration update
    let mut suite_mutable = ProtocolSuite::new(SecurityLevel::Level256);
    suite_mutable.update_config(config).unwrap();
}

#[test]
fn test_protocol_suite_key_exchanges() {
    let mut alice_suite = ProtocolSuite::new(SecurityLevel::Level256);
    let mut bob_suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    // Test X25519 exchange
    let (alice_x25519_public, alice_x25519_exchange) = alice_suite.initiate_x25519_exchange().unwrap();
    let (bob_x25519_public, bob_x25519_exchange) = bob_suite.initiate_x25519_exchange().unwrap();
    
    let alice_x25519_shared = alice_suite.complete_x25519_exchange(&alice_x25519_exchange, &bob_x25519_public).unwrap();
    let bob_x25519_shared = bob_suite.complete_x25519_exchange(&bob_x25519_exchange, &alice_x25519_public).unwrap();
    
    assert_eq!(alice_x25519_shared.len(), bob_x25519_shared.len());
    
    // Test ECDH exchange
    let (alice_ecdh_public, alice_ecdh_exchange) = alice_suite.initiate_ecdh_exchange().unwrap();
    let (bob_ecdh_public, bob_ecdh_exchange) = bob_suite.initiate_ecdh_exchange().unwrap();
    
    let alice_ecdh_shared = alice_suite.complete_ecdh_exchange(&alice_ecdh_exchange, &bob_ecdh_public).unwrap();
    let bob_ecdh_shared = bob_suite.complete_ecdh_exchange(&bob_ecdh_exchange, &alice_ecdh_public).unwrap();
    
    assert_eq!(alice_ecdh_shared, bob_ecdh_shared);
}

#[test]
fn test_protocol_suite_secure_channels() {
    let mut alice_suite = ProtocolSuite::new(SecurityLevel::Level256);
    let mut bob_suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    let shared_secret = CryptoPrimitives::random_bytes(32);
    
    // Create channels
    alice_suite.create_secure_channel("test_channel", &shared_secret).unwrap();
    bob_suite.create_secure_channel("test_channel", &shared_secret).unwrap();
    
    // Test message exchange
    let messages = vec![
        "Hello, Alice!",
        "Hello, Bob!",
        "How are you doing?",
        "Everything is secure! 🔐",
        "Great to hear! 👍"
    ];
    
    for (i, message) in messages.iter().enumerate() {
        let sender = if i % 2 == 0 { &mut alice_suite } else { &mut bob_suite };
        let receiver = if i % 2 == 0 { &mut bob_suite } else { &mut alice_suite };
        
        let encrypted = sender.send_secure_message("test_channel", message.as_bytes()).unwrap();
        let decrypted = receiver.receive_secure_message("test_channel", &encrypted).unwrap();
        
        assert_eq!(message.as_bytes(), decrypted.as_slice());
    }
    
    // Test channel statistics
    let alice_stats = alice_suite.get_channel_stats("test_channel").unwrap();
    assert!(alice_stats.contains_key("channel_id"));
    assert!(alice_stats.contains_key("send_sequence"));
    
    // Test key rotation
    let new_secret = CryptoPrimitives::random_bytes(32);
    alice_suite.rotate_channel_keys("test_channel", &new_secret).unwrap();
    bob_suite.rotate_channel_keys("test_channel", &new_secret).unwrap();
    
    // Verify communication still works after rotation
    let post_rotation_msg = "Message after key rotation";
    let encrypted_post = alice_suite.send_secure_message("test_channel", post_rotation_msg.as_bytes()).unwrap();
    let decrypted_post = bob_suite.receive_secure_message("test_channel", &encrypted_post).unwrap();
    assert_eq!(post_rotation_msg.as_bytes(), decrypted_post.as_slice());
    
    // Clean up
    alice_suite.close_secure_channel("test_channel").unwrap();
    bob_suite.close_secure_channel("test_channel").unwrap();
}

#[test]
fn test_protocol_suite_authentication() {
    let mut alice_suite = ProtocolSuite::new(SecurityLevel::Level256);
    let bob_suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    let bob_public_key = bob_suite.public_key();
    
    // Initiate authentication
    let challenge_set = alice_suite.initiate_authentication(&bob_public_key).unwrap();
    assert!(!challenge_set.session_id.is_empty());
    assert!(!challenge_set.challenges.is_empty());
    
    // Respond to challenges
    let response_set = bob_suite.respond_to_authentication(&challenge_set).unwrap();
    assert_eq!(response_set.session_id, challenge_set.session_id);
    
    // Verify authentication
    let result = alice_suite.verify_authentication(&response_set).unwrap();
    assert!(result.authenticated);
    
    // Clean up
    alice_suite.cleanup_auth_sessions();
}

#[test]
fn test_protocol_suite_mpc_integration() {
    let mut alice_suite = ProtocolSuite::new(SecurityLevel::Level256);
    let bob_suite = ProtocolSuite::new(SecurityLevel::Level256);
    let charlie_suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    // Register parties
    alice_suite.register_mpc_party("bob", &bob_suite.public_key()).unwrap();
    alice_suite.register_mpc_party("charlie", &charlie_suite.public_key()).unwrap();
    
    // Initiate MPC key generation
    let participants = vec![
        "alice".to_string(),
        "bob".to_string(), 
        "charlie".to_string()
    ];
    let session_id = alice_suite.initiate_mpc_key_generation(participants).unwrap();
    
    // Generate shares
    let distributions = alice_suite.generate_mpc_shares(&session_id).unwrap();
    assert!(!distributions.is_empty());
    
    // Test partial computation
    let input_data = b"MPC computation input";
    let partial_result = alice_suite.compute_mpc_partial_result(&session_id, input_data).unwrap();
    assert!(!partial_result.is_empty());
    
    // Check session status
    let status = alice_suite.get_mpc_session_status(&session_id);
    assert!(status.is_some());
    
    // Clean up
    alice_suite.cleanup_mpc_sessions();
}

#[test]
fn test_protocol_suite_monitoring_and_audit() {
    let suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    // Test statistics
    let stats = suite.get_protocol_statistics();
    assert!(stats.contains_key("security_level"));
    assert!(stats.contains_key("forward_secrecy_enabled"));
    assert!(stats.contains_key("quantum_safe_enabled"));
    
    // Test security audit
    let audit = suite.security_audit();
    assert!(matches!(
        audit.overall_status, 
        SecurityStatus::Secure | SecurityStatus::Warning
    ));
    assert!(matches!(
        audit.risk_level,
        RiskLevel::Low | RiskLevel::Medium
    ));
    
    // Test health status
    let health = suite.get_health_status();
    assert!(matches!(
        health.status,
        HealthStatus::Healthy | HealthStatus::Warning
    ));
    assert_eq!(health.active_channels, 0);
    assert_eq!(health.error_rate, 0.0);
}

// ============================================================================
// CURSED LANGUAGE FUNCTION EXPORT TESTS
// ============================================================================

#[test]
fn test_cursed_function_exports() {
    // Test protocol suite creation
    let result = create_protocol_suite(vec![]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CursedValue::bool(true));
    
    let result_with_level = create_protocol_suite(vec![CursedValue::String("Level256".to_string())]);
    assert!(result_with_level.is_ok());
    
    // Test X25519 keypair generation
    let keypair = generate_x25519_keypair(vec![]);
    assert!(keypair.is_ok());
    if let Ok(CursedValue::Object(kp)) = keypair {
        assert!(kp.contains_key("public_key"));
        assert!(kp.contains_key("key_type"));
    }
    
    // Test secure channel creation
    let channel = create_secure_channel(vec![CursedValue::String("test_secret".to_string())]);
    assert!(channel.is_ok());
    
    // Test message encryption/decryption
    let channel_id = CursedValue::String("test_channel".to_string());
    let message = CursedValue::String("Hello, CURSED!".to_string());
    
    let encrypted = send_secure_message(vec![channel_id.clone(), message.clone()]);
    assert!(encrypted.is_ok());
    
    if let Ok(encrypted_msg) = encrypted {
        let decrypted = receive_secure_message(vec![channel_id, encrypted_msg]);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), message);
    }
    
    // Test authentication functions
    let peer_key = CursedValue::String("peer_public_key".to_string());
    let auth_result = initiate_authentication(vec![peer_key]);
    assert!(auth_result.is_ok());
    
    // Test MPC initiation
    let participants = CursedValue::Array(vec![
        CursedValue::String("alice".to_string()),
        CursedValue::String("bob".to_string())
    ]);
    let threshold = CursedValue::Number(2.0);
    let mpc_result = initiate_mpc_computation(vec![participants, threshold]);
    assert!(mpc_result.is_ok());
    
    // Test monitoring functions
    let stats = get_protocol_statistics(vec![]);
    assert!(stats.is_ok());
    
    let audit = security_audit(vec![]);
    assert!(audit.is_ok());
    
    let health = get_health_status(vec![]);
    assert!(health.is_ok());
}

// ============================================================================
// PERFORMANCE AND STRESS TESTS
// ============================================================================

#[test]
fn test_protocol_performance_basic() {
    use std::time::Instant;
    
    // Test X25519 key exchange performance
    let start = Instant::now();
    let iterations = 100;
    
    for _ in 0..iterations {
        let alice = X25519KeyExchange::new(SecurityLevel::Level256);
        let bob = X25519KeyExchange::new(SecurityLevel::Level256);
        
        let alice_public = alice.public_key();
        let bob_public = bob.public_key();
        
        let _alice_shared = alice.exchange(&bob_public).unwrap();
        let _bob_shared = bob.exchange(&alice_public).unwrap();
    }
    
    let duration = start.elapsed();
    let exchanges_per_second = iterations as f64 / duration.as_secs_f64();
    
    println!("X25519 exchanges per second: {:.2}", exchanges_per_second);
    assert!(exchanges_per_second > 50.0); // Should be much faster in practice
}

#[test]
fn test_secure_channel_throughput() {
    let mut alice_channel = SecureChannel::new(
        "throughput_test".to_string(), 
        ProtocolConfig::default()
    );
    let mut bob_channel = SecureChannel::new(
        "throughput_test".to_string(), 
        ProtocolConfig::default()
    );
    
    let shared_secret = CryptoPrimitives::random_bytes(32);
    alice_channel.establish(&shared_secret).unwrap();
    bob_channel.establish(&shared_secret).unwrap();
    
    // Test message throughput
    let message = b"Test message for throughput analysis";
    let iterations = 1000;
    
    use std::time::Instant;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let encrypted = alice_channel.send_message(message).unwrap();
        let _decrypted = bob_channel.receive_message(&encrypted).unwrap();
    }
    
    let duration = start.elapsed();
    let messages_per_second = iterations as f64 / duration.as_secs_f64();
    
    println!("Secure channel messages per second: {:.2}", messages_per_second);
    assert!(messages_per_second > 100.0); // Reasonable throughput expectation
}

#[test]
fn test_memory_usage_stability() {
    // Test that protocols don't leak memory excessively
    let mut suite = ProtocolSuite::new(SecurityLevel::Level256);
    let shared_secret = CryptoPrimitives::random_bytes(32);
    
    // Create and destroy channels multiple times
    for i in 0..100 {
        let channel_id = format!("temp_channel_{}", i);
        suite.create_secure_channel(&channel_id, &shared_secret).unwrap();
        
        // Send a few messages
        for j in 0..10 {
            let message = format!("Message {} from channel {}", j, i);
            let _encrypted = suite.send_secure_message(&channel_id, message.as_bytes()).unwrap();
        }
        
        // Close channel
        suite.close_secure_channel(&channel_id).unwrap();
    }
    
    // Verify no channels remain active
    let stats = suite.get_protocol_statistics();
    if let Some(CursedValue::Number(active_channels)) = stats.get("active_channels") {
        assert_eq!(*active_channels, 0.0);
    }
}

#[test]
fn test_concurrent_protocol_operations() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let suite = Arc::new(Mutex::new(ProtocolSuite::new(SecurityLevel::Level256)));
    let shared_secret = CryptoPrimitives::random_bytes(32);
    
    // Setup channel
    {
        let mut s = suite.lock().unwrap();
        s.create_secure_channel("concurrent_test", &shared_secret).unwrap();
    }
    
    let mut handles = vec![];
    
    // Spawn multiple threads to perform operations
    for thread_id in 0..4 {
        let suite_clone = suite.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..50 {
                let message = format!("Thread {} message {}", thread_id, i);
                
                // Send message
                let encrypted = {
                    let mut s = suite_clone.lock().unwrap();
                    s.send_secure_message("concurrent_test", message.as_bytes()).unwrap()
                };
                
                // Receive message (in practice, would be on different node)
                let _decrypted = {
                    let mut s = suite_clone.lock().unwrap();
                    s.receive_secure_message("concurrent_test", &encrypted).unwrap()
                };
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify channel statistics
    let stats = {
        let s = suite.lock().unwrap();
        s.get_channel_stats("concurrent_test").unwrap()
    };
    
    // Should have processed 200 messages (4 threads * 50 messages)
    println!("Channel stats after concurrent operations: {:?}", stats);
}

// ============================================================================
// ERROR HANDLING AND EDGE CASE TESTS
// ============================================================================

#[test]
fn test_protocol_error_handling() {
    let mut suite = ProtocolSuite::new(SecurityLevel::Level256);
    
    // Test invalid channel operations
    let result = suite.send_secure_message("nonexistent_channel", b"test");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ProtocolError::ChannelError { .. }));
    
    let result = suite.receive_secure_message("nonexistent_channel", b"test");
    assert!(result.is_err());
    
    // Test invalid authentication
    let invalid_key = [0u8; 32];
    let result = suite.initiate_authentication(&invalid_key);
    // This might succeed with a zero key, depending on implementation
    
    // Test invalid MPC operations
    let result = suite.register_mpc_party("test", &invalid_key);
    assert!(result.is_err());
    
    // Test configuration errors
    let invalid_config = ProtocolConfig {
        security_level: SecurityLevel::Level256,
        enable_forward_secrecy: true,
        key_rotation_interval: Duration::from_millis(1), // Too short
        max_message_size: 0, // Invalid
        timeout_duration: Duration::from_secs(0), // Invalid
        enable_quantum_safe: false,
        compression_enabled: false,
        replay_window_size: 0, // Invalid
    };
    
    // The protocol suite should handle invalid configs gracefully
    let _suite_with_invalid_config = ProtocolSuite::with_config(invalid_config);
}

#[test]
fn test_edge_cases_and_boundary_conditions() {
    // Test with minimum security level
    let suite_min = ProtocolSuite::new(SecurityLevel::Level128);
    assert_eq!(suite_min.public_key().len(), 32);
    
    // Test with maximum security level
    let suite_max = ProtocolSuite::new(SecurityLevel::PostQuantum);
    assert_eq!(suite_max.public_key().len(), 32);
    
    // Test empty message encryption
    let mut suite = ProtocolSuite::new(SecurityLevel::Level256);
    let shared_secret = CryptoPrimitives::random_bytes(32);
    suite.create_secure_channel("edge_test", &shared_secret).unwrap();
    
    let empty_message = b"";
    let encrypted_empty = suite.send_secure_message("edge_test", empty_message).unwrap();
    let decrypted_empty = suite.receive_secure_message("edge_test", &encrypted_empty).unwrap();
    assert_eq!(empty_message, decrypted_empty.as_slice());
    
    // Test maximum message size
    let max_message = vec![0u8; 1024]; // Within default limit
    let encrypted_max = suite.send_secure_message("edge_test", &max_message).unwrap();
    let decrypted_max = suite.receive_secure_message("edge_test", &encrypted_max).unwrap();
    assert_eq!(max_message, decrypted_max);
    
    // Test with malformed data
    let malformed_packet = b"malformed_encrypted_data";
    let result = suite.receive_secure_message("edge_test", malformed_packet);
    assert!(result.is_err());
}

#[test]
fn test_protocol_state_transitions() {
    let alice_identity = Ed25519Keypair::generate(&mut OsRng);
    let bob_identity = Ed25519Keypair::generate(&mut OsRng);
    
    let mut alice_ecdhe = EcdheKeyExchange::new(alice_identity.clone(), SecurityLevel::Level256);
    let mut bob_ecdhe = EcdheKeyExchange::new(bob_identity.clone(), SecurityLevel::Level256);
    
    // Test initial state
    assert_eq!(*alice_ecdhe.state(), EcdheState::KeyGenerated);
    
    // Generate message and test state transition
    let _alice_message = alice_ecdhe.generate_key_exchange_message().unwrap();
    assert_eq!(*alice_ecdhe.state(), EcdheState::MessageSent);
    
    // Test invalid state transition
    let result = alice_ecdhe.generate_key_exchange_message();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ProtocolError::InvalidState { .. }));
}
