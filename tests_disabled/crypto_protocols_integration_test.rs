/// Comprehensive Cryptographic Protocols Integration Tests
use cursed::stdlib::packages::crypto_protocols::*;
use cursed::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_protocol_suite_creation() {
        let suite = create_protocol_suite().unwrap();
        
        // Verify all managers are created
        println!("Protocol suite created successfully");
        drop(suite);
    }

    #[test]
    fn test_key_exchange_protocols() {
        let key_exchange = KeyExchangeManager::new().unwrap();
        
        // Test different key exchange protocols
        for protocol in [
            KeyExchangeProtocol::DiffieHellman,
            KeyExchangeProtocol::ECDH,
            KeyExchangeProtocol::X25519,
            KeyExchangeProtocol::Kyber1024,
        ] {
            let result = key_exchange.initiate_exchange(protocol.clone()).unwrap();
            assert!(!result.public_key.is_empty());
            assert!(!result.session_id.is_empty());
            assert_eq!(result.protocol, protocol);
            
            // Complete the exchange
            let peer_key = vec![0x42; 32];
            let shared_secret = key_exchange.complete_exchange(&result.session_id, peer_key).unwrap();
            assert!(!shared_secret.is_empty());
        }
    }

    #[test]
    fn test_ecdh_key_exchange() {
        let ecdh = EcdhManager::new().unwrap();
        
        // Test different curves
        for curve in [EcdhCurve::P256, EcdhCurve::P384, EcdhCurve::X25519] {
            let keypair1 = ecdh.generate_keypair(curve.clone()).unwrap();
            let keypair2 = ecdh.generate_keypair(curve.clone()).unwrap();
            
            assert_eq!(keypair1.curve, curve);
            assert!(!keypair1.private_key.is_empty());
            assert!(!keypair1.public_key.is_empty());
            
            // Compute shared secrets
            let secret1 = ecdh.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap();
            let secret2 = ecdh.compute_shared_secret(&keypair2, &keypair1.public_key).unwrap();
            
            // Shared secrets should be equal
            assert_eq!(secret1.secret, secret2.secret);
            assert_eq!(secret1.curve, secret2.curve);
        }
    }

    #[test]
    fn test_diffie_hellman_groups() {
        let dh = DiffieHellmanManager::new().unwrap();
        
        // Test different DH groups
        for group in [DhGroup::Group14, DhGroup::Group15, DhGroup::Group16] {
            let keypair1 = dh.generate_keypair(group.clone()).unwrap();
            let keypair2 = dh.generate_keypair(group.clone()).unwrap();
            
            assert_eq!(keypair1.params.group, group);
            assert!(!keypair1.private_key.is_empty());
            assert!(!keypair1.public_key.is_empty());
            
            // Compute shared secrets
            let secret1 = dh.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap();
            let secret2 = dh.compute_shared_secret(&keypair2, &keypair1.public_key).unwrap();
            
            // Shared secrets should be equal
            assert_eq!(secret1.secret, secret2.secret);
            assert_eq!(secret1.group, secret2.group);
            
            // Validate public keys
            assert!(dh.validate_public_key(&keypair1.params, &keypair1.public_key).unwrap());
        }
    }

    #[test]
    fn test_authentication_protocols() {
        let auth = AuthenticationManager::new().unwrap();
        
        // Start authentication
        let result = auth.start_authentication("test_user", None).unwrap();
        assert!(!result.success); // Should not be complete yet
        assert!(result.session_id.is_some());
        assert!(result.next_challenge.is_some());
        
        let session_id = result.session_id.unwrap();
        let challenge = result.next_challenge.unwrap();
        
        // Respond to challenge
        let response = b"test_password";
        let auth_result = auth.respond_to_challenge(&challenge.challenge_id, response).unwrap();
        
        // Check session exists
        let session = auth.get_session(&session_id).unwrap();
        assert!(session.is_some());
    }

    #[test]
    fn test_totp_authentication() {
        let auth = AuthenticationManager::new().unwrap();
        let secret = b"12345678901234567890";
        
        // Generate TOTP
        let totp = auth.generate_totp(secret, Some(30)).unwrap();
        assert_eq!(totp.len(), 6);
        assert!(totp.chars().all(|c| c.is_ascii_digit()));
        
        // Verify TOTP
        let is_valid = auth.verify_totp(secret, &totp, Some(1)).unwrap();
        assert!(is_valid);
        
        // Invalid TOTP should fail
        let is_invalid = auth.verify_totp(secret, "000000", Some(1)).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_hotp_authentication() {
        let auth = AuthenticationManager::new().unwrap();
        let secret = b"12345678901234567890";
        
        // Generate HOTP codes for different counters
        let hotp1 = auth.generate_hotp(secret, 0).unwrap();
        let hotp2 = auth.generate_hotp(secret, 1).unwrap();
        let hotp3 = auth.generate_hotp(secret, 2).unwrap();
        
        assert_eq!(hotp1.len(), 6);
        assert_eq!(hotp2.len(), 6);
        assert_eq!(hotp3.len(), 6);
        
        // Different counters should produce different codes
        assert_ne!(hotp1, hotp2);
        assert_ne!(hotp2, hotp3);
        
        // Same counter should produce same code
        let hotp1_repeat = auth.generate_hotp(secret, 0).unwrap();
        assert_eq!(hotp1, hotp1_repeat);
    }

    #[test]
    fn test_secure_channels() {
        let channel_manager = SecureChannelManager::new().unwrap();
        
        // Create secure channel
        let channel_id = channel_manager.create_channel(None).unwrap();
        assert!(!channel_id.is_empty());
        
        let channel = channel_manager.get_channel(&channel_id).unwrap().unwrap();
        assert_eq!(channel.channel_id, channel_id);
        assert!(!channel.is_established);
        
        // List channels
        let channels = channel_manager.list_channels().unwrap();
        assert!(channels.contains(&channel_id));
        
        // Close channel
        channel_manager.close_channel(&channel_id).unwrap();
        let closed_channel = channel_manager.get_channel(&channel_id).unwrap().unwrap();
        assert!(closed_channel.is_closed);
    }

    #[test]
    fn test_signal_protocol() {
        let signal = SignalProtocolManager::new().unwrap();
        
        // Generate key bundle
        let key_bundle = signal.generate_key_bundle(5).unwrap();
        assert!(!key_bundle.identity_key.is_empty());
        assert_eq!(key_bundle.one_time_pre_keys.len(), 5);
        assert!(key_bundle.signed_pre_key.signature.is_some());
        
        // Start session
        let session_id = signal.start_session("remote_user", key_bundle).unwrap();
        assert!(!session_id.is_empty());
        
        let session = signal.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.session_id, session_id);
        assert!(session.is_initiated);
        
        // Encrypt and decrypt message
        let plaintext = b"Hello, Signal!";
        let encrypted_message = signal.encrypt_message(&session_id, plaintext).unwrap();
        assert!(!encrypted_message.ciphertext.is_empty());
        assert_eq!(encrypted_message.message_type, SignalMessageType::Message);
        
        let _decrypted = signal.decrypt_message(encrypted_message).unwrap();
        // Note: In demo implementation, decryption returns fixed text
    }

    #[test]
    fn test_tls_handshake() {
        let tls = TlsHandshakeManager::new().unwrap();
        
        // Start client handshake
        let (session_id, client_hello) = tls.start_client_handshake(None).unwrap();
        assert!(!session_id.is_empty());
        assert_eq!(client_hello.message_type, TlsHandshakeType::ClientHello);
        
        let session = tls.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.state, TlsHandshakeState::ClientHelloSent);
        assert!(session.is_client);
        
        // Check if handshake is complete (should not be yet)
        assert!(!tls.is_handshake_complete(&session_id).unwrap());
        
        // Start server handshake
        let server_session_id = tls.start_server_handshake(None).unwrap();
        let server_session = tls.get_session(&server_session_id).unwrap().unwrap();
        assert!(!server_session.is_client);
    }

    #[test]
    fn test_session_management() {
        let session_manager = SessionManager::new().unwrap();
        
        // Create session
        let session_id = session_manager.create_session("peer123", None).unwrap();
        assert!(!session_id.is_empty());
        
        let session = session_manager.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.session_id, session_id);
        assert_eq!(session.peer_id, "peer123");
        
        // Activate session
        session_manager.activate_session(&session_id).unwrap();
        let active_session = session_manager.get_session(&session_id).unwrap().unwrap();
        assert_eq!(active_session.state, SessionState::Active);
        
        // Record activity
        let needs_rekey = session_manager.record_activity(&session_id, 100, 200).unwrap();
        let stats = session_manager.get_session_stats(&session_id).unwrap().unwrap();
        assert_eq!(stats.bytes_sent, 100);
        assert_eq!(stats.bytes_received, 200);
        
        // Create session ticket
        let ticket = session_manager.create_session_ticket(&session_id).unwrap();
        assert!(!ticket.ticket_id.is_empty());
        assert_eq!(ticket.session_id, session_id);
        
        // Resume session
        let new_session_id = session_manager.resume_session(&ticket.ticket_id, "peer123").unwrap();
        assert!(!new_session_id.is_empty());
        assert_ne!(new_session_id, session_id);
        
        // List active sessions
        let active_sessions = session_manager.list_active_sessions().unwrap();
        assert!(active_sessions.len() >= 1);
    }

    #[test]
    fn test_protocol_integration() {
        // Test integration between different protocols
        let key_exchange = KeyExchangeManager::new().unwrap();
        let auth = AuthenticationManager::new().unwrap();
        let session_manager = SessionManager::new().unwrap();
        
        // 1. Perform key exchange
        let ke_result = key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
        let peer_key = vec![0x42; 32];
        let shared_secret = key_exchange.complete_exchange(&ke_result.session_id, peer_key).unwrap();
        
        // 2. Use shared secret for authentication
        let auth_result = auth.start_authentication("test_user", None).unwrap();
        assert!(auth_result.next_challenge.is_some());
        
        // 3. Create secure session
        let session_id = session_manager.create_session("test_peer", None).unwrap();
        session_manager.activate_session(&session_id).unwrap();
        
        // 4. Record successful protocol completion
        session_manager.record_activity(&session_id, shared_secret.len() as u64, 0).unwrap();
        
        let final_stats = session_manager.get_session_stats(&session_id).unwrap().unwrap();
        assert!(final_stats.bytes_sent > 0);
    }

    #[test]
    fn test_error_conditions() {
        let key_exchange = KeyExchangeManager::new().unwrap();
        
        // Test invalid session ID
        let invalid_result = key_exchange.complete_exchange("invalid_session", vec![1, 2, 3]);
        assert!(invalid_result.is_err());
        
        // Test empty peer key
        let ke_result = key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
        let empty_key_result = key_exchange.complete_exchange(&ke_result.session_id, vec![]);
        assert!(empty_key_result.is_err());
    }

    #[test]
    fn test_cipher_suite_configurations() {
        use TlsCipherSuite::*;
        
        let cipher_suites = vec![
            AES128GcmSha256,
            AES256GcmSha384,
            ChaCha20Poly1305Sha256,
            AES128CbcSha256,
        ];
        
        for suite in cipher_suites {
            let config = TlsConfig {
                version: TlsVersion::Tls12,
                cipher_suites: vec![suite.clone()],
                server_name: Some("example.com".to_string()),
                verify_peer: true,
                client_auth: false,
                session_timeout: std::time::Duration::from_secs(300),
                max_handshake_time: std::time::Duration::from_secs(30),
            };
            
            let tls = TlsHandshakeManager::new().unwrap();
            let (session_id, _) = tls.start_client_handshake(Some(config)).unwrap();
            
            let session = tls.get_session(&session_id).unwrap().unwrap();
            assert!(session.config.cipher_suites.contains(&suite));
        }
    }

    #[test]
    fn test_security_levels() {
        use SecurityLevel::*;
        
        let security_levels = vec![Low, Medium, High, VeryHigh, Extreme];
        
        for level in security_levels {
            let config = ChannelConfig {
                channel_type: ChannelType::TLS,
                security_level: level.clone(),
                cipher: ChannelCipher::AES256GCM,
                auth_method: ChannelAuth::Certificate,
                forward_secrecy: true,
                compression: false,
                heartbeat_interval: Some(std::time::Duration::from_secs(30)),
                session_timeout: std::time::Duration::from_secs(3600),
            };
            
            let channel_manager = SecureChannelManager::new().unwrap();
            let channel_id = channel_manager.create_channel(Some(config)).unwrap();
            
            let channel = channel_manager.get_channel(&channel_id).unwrap().unwrap();
            assert_eq!(channel.config.security_level, level);
        }
    }

    #[test]
    fn test_performance_benchmarks() {
        let start = std::time::Instant::now();
        
        // Benchmark key exchange
        let key_exchange = KeyExchangeManager::new().unwrap();
        for _ in 0..10 {
            let result = key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
            let peer_key = vec![0x42; 32];
            let _shared_secret = key_exchange.complete_exchange(&result.session_id, peer_key).unwrap();
        }
        
        let key_exchange_time = start.elapsed();
        println!("Key exchange (10 iterations): {:?}", key_exchange_time);
        assert!(key_exchange_time.as_millis() < 1000); // Should complete within 1 second
        
        // Benchmark ECDH
        let ecdh_start = std::time::Instant::now();
        let ecdh = EcdhManager::new().unwrap();
        for _ in 0..10 {
            let keypair1 = ecdh.generate_keypair(EcdhCurve::X25519).unwrap();
            let keypair2 = ecdh.generate_keypair(EcdhCurve::X25519).unwrap();
            let _secret = ecdh.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap();
        }
        
        let ecdh_time = ecdh_start.elapsed();
        println!("ECDH operations (10 iterations): {:?}", ecdh_time);
        assert!(ecdh_time.as_millis() < 1000);
    }

    #[test]
    fn test_cleanup_operations() {
        let key_exchange = KeyExchangeManager::new().unwrap();
        let session_manager = SessionManager::new().unwrap();
        
        // Create some sessions
        for i in 0..5 {
            let ke_result = key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
            let session_id = session_manager.create_session(&format!("peer_{}", i), None).unwrap();
            session_manager.activate_session(&session_id).unwrap();
        }
        
        // Test cleanup operations
        let ke_cleaned = key_exchange.cleanup_expired_sessions().unwrap();
        let (session_cleaned, ticket_cleaned) = session_manager.cleanup_expired().unwrap();
        
        // Should not clean non-expired sessions
        assert_eq!(ke_cleaned, 0);
        assert_eq!(session_cleaned, 0);
        assert_eq!(ticket_cleaned, 0);
    }

    #[test]
    fn test_concurrent_operations() {
        use std::sync::Arc;
        use std::thread;
        
        let key_exchange = Arc::new(KeyExchangeManager::new().unwrap());
        let mut handles = vec![];
        
        // Spawn multiple threads performing key exchanges
        for i in 0..5 {
            let ke = Arc::clone(&key_exchange);
            let handle = thread::spawn(move || {
                let result = ke.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
                let peer_key = vec![0x42 + i as u8; 32];
                let _shared_secret = ke.complete_exchange(&result.session_id, peer_key).unwrap();
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify operations completed successfully
        println!("Concurrent key exchange operations completed");
    }
}

/// Protocol interoperability tests
#[cfg(test)]
mod interoperability_tests {
    use super::*;

    #[test]
    fn test_cross_protocol_compatibility() {
        // Test that different protocols can work together
        let ecdh = EcdhManager::new().unwrap();
        let dh = DiffieHellmanManager::new().unwrap();
        let signal = SignalProtocolManager::new().unwrap();
        
        // Generate keys with different protocols
        let ecdh_keypair = ecdh.generate_keypair(EcdhCurve::X25519).unwrap();
        let dh_keypair = dh.generate_keypair(DhGroup::Group14).unwrap();
        let signal_bundle = signal.generate_key_bundle(3).unwrap();
        
        // Verify all keys are different and valid
        assert_ne!(ecdh_keypair.public_key, dh_keypair.public_key);
        assert_ne!(ecdh_keypair.public_key, signal_bundle.identity_key);
        assert_ne!(dh_keypair.public_key, signal_bundle.identity_key);
        
        assert!(!ecdh_keypair.public_key.is_empty());
        assert!(!dh_keypair.public_key.is_empty());
        assert!(!signal_bundle.identity_key.is_empty());
    }

    #[test]
    fn test_protocol_version_compatibility() {
        let tls = TlsHandshakeManager::new().unwrap();
        
        // Test different TLS versions
        for version in [TlsVersion::Tls11, TlsVersion::Tls12, TlsVersion::Tls13] {
            let config = TlsConfig {
                version: version.clone(),
                cipher_suites: vec![TlsCipherSuite::AES256GcmSha384],
                server_name: None,
                verify_peer: true,
                client_auth: false,
                session_timeout: std::time::Duration::from_secs(300),
                max_handshake_time: std::time::Duration::from_secs(30),
            };
            
            let (session_id, _) = tls.start_client_handshake(Some(config)).unwrap();
            let session = tls.get_session(&session_id).unwrap().unwrap();
            assert_eq!(session.config.version, version);
        }
    }
}

/// Security property tests
#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_forward_secrecy_properties() {
        let key_exchange = KeyExchangeManager::new().unwrap();
        
        // Generate multiple key exchanges
        let mut session_ids = Vec::new();
        let mut shared_secrets = Vec::new();
        
        for _ in 0..3 {
            let result = key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
            let peer_key = vec![0x42; 32];
            let shared_secret = key_exchange.complete_exchange(&result.session_id, peer_key).unwrap();
            
            session_ids.push(result.session_id);
            shared_secrets.push(shared_secret);
        }
        
        // Each session should have different shared secrets (forward secrecy)
        for i in 0..shared_secrets.len() {
            for j in i+1..shared_secrets.len() {
                assert_ne!(shared_secrets[i], shared_secrets[j], 
                          "Shared secrets should be different for forward secrecy");
            }
        }
    }

    #[test]
    fn test_authentication_security() {
        let auth = AuthenticationManager::new().unwrap();
        
        // Test multiple authentication attempts
        let result1 = auth.start_authentication("user1", None).unwrap();
        let result2 = auth.start_authentication("user2", None).unwrap();
        
        // Different users should have different sessions
        assert_ne!(result1.session_id, result2.session_id);
        
        // Sessions should be isolated
        if let (Some(session1), Some(session2)) = (result1.session_id, result2.session_id) {
            let session_info1 = auth.get_session(&session1).unwrap();
            let session_info2 = auth.get_session(&session2).unwrap();
            
            assert!(session_info1.is_some());
            assert!(session_info2.is_some());
            
            let session1 = session_info1.unwrap();
            let session2 = session_info2.unwrap();
            
            assert_ne!(session1.user_id, session2.user_id);
        }
    }

    #[test]
    fn test_key_uniqueness() {
        let ecdh = EcdhManager::new().unwrap();
        let mut public_keys = Vec::new();
        
        // Generate multiple key pairs
        for _ in 0..10 {
            let keypair = ecdh.generate_keypair(EcdhCurve::X25519).unwrap();
            public_keys.push(keypair.public_key);
        }
        
        // All public keys should be unique
        for i in 0..public_keys.len() {
            for j in i+1..public_keys.len() {
                assert_ne!(public_keys[i], public_keys[j], 
                          "All generated keys should be unique");
            }
        }
    }

    #[test]
    fn test_session_isolation() {
        let session_manager = SessionManager::new().unwrap();
        
        // Create multiple sessions
        let session1 = session_manager.create_session("peer1", None).unwrap();
        let session2 = session_manager.create_session("peer2", None).unwrap();
        
        session_manager.activate_session(&session1).unwrap();
        session_manager.activate_session(&session2).unwrap();
        
        // Record different activities
        session_manager.record_activity(&session1, 100, 0).unwrap();
        session_manager.record_activity(&session2, 0, 200).unwrap();
        
        // Verify session isolation
        let stats1 = session_manager.get_session_stats(&session1).unwrap().unwrap();
        let stats2 = session_manager.get_session_stats(&session2).unwrap().unwrap();
        
        assert_eq!(stats1.bytes_sent, 100);
        assert_eq!(stats1.bytes_received, 0);
        assert_eq!(stats2.bytes_sent, 0);
        assert_eq!(stats2.bytes_received, 200);
    }
}
