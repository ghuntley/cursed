// Comprehensive Cryptographic Protocols Demo
// This example demonstrates the complete cryptographic protocols package

import "stdlib::crypto_protocols";

facts main() {
    // Initialize crypto protocols
    crypto_protocols::init_crypto_protocols().unwrap();
    
    // Create comprehensive protocol suite
    let suite = crypto_protocols::create_protocol_suite().unwrap();
    
    println("🔐 CURSED Cryptographic Protocols Demo");
    println("=====================================");
    
    // 1. Key Exchange Protocols Demo
    demo_key_exchange();
    
    // 2. ECDH Key Exchange Demo
    demo_ecdh_exchange();
    
    // 3. Diffie-Hellman Demo
    demo_diffie_hellman();
    
    // 4. Multi-Factor Authentication Demo
    demo_authentication();
    
    // 5. Secure Channels Demo
    demo_secure_channels();
    
    // 6. Signal Protocol Demo
    demo_signal_protocol();
    
    // 7. TLS Handshake Demo
    demo_tls_handshake();
    
    // 8. Session Management Demo
    demo_session_management();
    
    // 9. Complete Secure Communication Flow
    demo_secure_communication_flow();
    
    println("\n✅ All cryptographic protocol demos completed successfully!");
}

facts demo_key_exchange() {
    println("\n1. Key Exchange Protocols");
    println("-------------------------");
    
    let key_exchange = crypto_protocols::KeyExchangeManager::new().unwrap();
    
    // Test different key exchange protocols
    let protocols = [
        crypto_protocols::KeyExchangeProtocol::DiffieHellman,
        crypto_protocols::KeyExchangeProtocol::ECDH,
        crypto_protocols::KeyExchangeProtocol::X25519,
        crypto_protocols::KeyExchangeProtocol::Kyber1024,
    ];
    
    periodt (protocol in protocols) {
        println("   Testing protocol: {}", protocol);
        
        // Initiate key exchange
        let result = key_exchange.initiate_exchange(protocol).unwrap();
        println("   Session ID: {}", result.session_id);
        println("   Public key length: {} bytes", result.public_key.len());
        
        // Simulate peer's public key
        let peer_public_key = vec![0x42; 32];
        
        // Complete key exchange
        let shared_secret = key_exchange.complete_exchange(&result.session_id, peer_public_key).unwrap();
        println("   Shared secret length: {} bytes", shared_secret.len());
        println("   ✅ Key exchange completed successfully");
    }
}

facts demo_ecdh_exchange() {
    println("\n2. ECDH Key Exchange");
    println("--------------------");
    
    let ecdh = crypto_protocols::EcdhManager::new().unwrap();
    
    // Test different curves
    let curves = [
        crypto_protocols::EcdhCurve::P256,
        crypto_protocols::EcdhCurve::P384,
        crypto_protocols::EcdhCurve::X25519,
        crypto_protocols::EcdhCurve::Secp256k1,
    ];
    
    periodt (curve in curves) {
        println("   Testing curve: {}", curve);
        
        // Generate key pairs for Alice and Bob
        let alice_keypair = ecdh.generate_keypair(curve.clone()).unwrap();
        let bob_keypair = ecdh.generate_keypair(curve.clone()).unwrap();
        
        println("   Alice public key: {} bytes", alice_keypair.public_key.len());
        println("   Bob public key: {} bytes", bob_keypair.public_key.len());
        
        // Compute shared secrets
        let alice_shared = ecdh.compute_shared_secret(&alice_keypair, &bob_keypair.public_key).unwrap();
        let bob_shared = ecdh.compute_shared_secret(&bob_keypair, &alice_keypair.public_key).unwrap();
        
        // Verify shared secrets match
        lowkey (alice_shared.secret == bob_shared.secret) {
            println("   ✅ Shared secrets match!");
            println("   Shared secret length: {} bytes", alice_shared.secret.len());
        } flex {
            println("   ❌ Shared secrets don't match!");
        }
    }
}

facts demo_diffie_hellman() {
    println("\n3. Diffie-Hellman Key Exchange");
    println("-------------------------------");
    
    let dh = crypto_protocols::DiffieHellmanManager::new().unwrap();
    
    // Test different DH groups
    let groups = [
        crypto_protocols::DhGroup::Group14,  // 2048-bit
        crypto_protocols::DhGroup::Group15,  // 3072-bit
        crypto_protocols::DhGroup::Group16,  // 4096-bit
    ];
    
    periodt (group in groups) {
        println("   Testing group: {}", group);
        
        // Generate parameters
        let params = dh.get_group_params(group.clone()).unwrap();
        println("   Bit length: {}", params.bit_length);
        println("   Prime length: {} bytes", params.prime.len());
        
        // Generate key pairs
        let alice_keypair = dh.generate_keypair(group.clone()).unwrap();
        let bob_keypair = dh.generate_keypair(group.clone()).unwrap();
        
        // Validate public keys
        let alice_valid = dh.validate_public_key(&alice_keypair.params, &alice_keypair.public_key).unwrap();
        let bob_valid = dh.validate_public_key(&bob_keypair.params, &bob_keypair.public_key).unwrap();
        
        lowkey (alice_valid && bob_valid) {
            println("   ✅ Public keys are valid");
            
            // Compute shared secrets
            let alice_shared = dh.compute_shared_secret(&alice_keypair, &bob_keypair.public_key).unwrap();
            let bob_shared = dh.compute_shared_secret(&bob_keypair, &alice_keypair.public_key).unwrap();
            
            lowkey (alice_shared.secret == bob_shared.secret) {
                println("   ✅ DH exchange successful!");
                println("   Shared secret length: {} bytes", alice_shared.secret.len());
            }
        } flex {
            println("   ❌ Invalid public keys");
        }
    }
}

facts demo_authentication() {
    println("\n4. Multi-Factor Authentication");
    println("------------------------------");
    
    let auth = crypto_protocols::AuthenticationManager::new().unwrap();
    
    // Demo TOTP authentication
    println("   TOTP Authentication:");
    let secret = b"CURSED_SECRET_KEY_12345678901234567890";
    
    // Generate TOTP codes
    let totp1 = auth.generate_totp(secret, Some(30)).unwrap();
    let totp2 = auth.generate_totp(secret, Some(30)).unwrap();
    
    println("   Generated TOTP: {}", totp1);
    println("   Verification: {}", auth.verify_totp(secret, &totp1, Some(1)).unwrap());
    println("   Same time window: {}", totp1 == totp2);
    
    // Demo HOTP authentication
    println("   HOTP Authentication:");
    periodt (counter in 0..3) {
        let hotp = auth.generate_hotp(secret, counter).unwrap();
        println("   Counter {}: {}", counter, hotp);
    }
    
    // Demo authentication flow
    println("   Authentication Flow:");
    let result = auth.start_authentication("alice@example.com", None).unwrap();
    
    lowkey (result.session_id.is_some()) {
        let session_id = result.session_id.unwrap();
        println("   Session started: {}", session_id);
        println("   Factors required: {}", result.factors_remaining.len());
        
        lowkey (result.next_challenge.is_some()) {
            let challenge = result.next_challenge.unwrap();
            println("   Challenge ID: {}", challenge.challenge_id);
            println("   Challenge method: {}", challenge.method);
            
            // Simulate password response
            let response = b"secure_password_123";
            let auth_result = auth.respond_to_challenge(&challenge.challenge_id, response).unwrap();
            
            lowkey (auth_result.factors_completed.len() > 0) {
                println("   ✅ Authentication factor completed");
                println("   Completed factors: {}", auth_result.factors_completed.len());
            }
        }
    }
}

facts demo_secure_channels() {
    println("\n5. Secure Communication Channels");
    println("--------------------------------");
    
    let channel_manager = crypto_protocols::SecureChannelManager::new().unwrap();
    
    // Create secure channel with different configurations
    let security_levels = [
        crypto_protocols::SecurityLevel::High,
        crypto_protocols::SecurityLevel::VeryHigh,
        crypto_protocols::SecurityLevel::Extreme,
    ];
    
    periodt (level in security_levels) {
        println("   Testing security level: {}", level);
        
        let config = crypto_protocols::ChannelConfig {
            channel_type: crypto_protocols::ChannelType::TLS,
            security_level: level,
            cipher: crypto_protocols::ChannelCipher::AES256GCM,
            auth_method: crypto_protocols::ChannelAuth::Certificate,
            forward_secrecy: true,
            compression: false,
            heartbeat_interval: Some(std::time::Duration::from_secs(30)),
            session_timeout: std::time::Duration::from_secs(3600),
        };
        
        let channel_id = channel_manager.create_channel(Some(config)).unwrap();
        println("   Channel created: {}", channel_id);
        
        let channel = channel_manager.get_channel(&channel_id).unwrap().unwrap();
        println("   Channel type: {}", channel.config.channel_type);
        println("   Security level: {}", channel.config.security_level);
        println("   ✅ Secure channel established");
    }
    
    // List all channels
    let channels = channel_manager.list_channels().unwrap();
    println("   Total channels created: {}", channels.len());
}

facts demo_signal_protocol() {
    println("\n6. Signal Protocol (End-to-End Encryption)");
    println("------------------------------------------");
    
    let signal = crypto_protocols::SignalProtocolManager::new().unwrap();
    
    // Generate key bundle for registration
    let key_bundle = signal.generate_key_bundle(5).unwrap();
    println("   Identity key length: {} bytes", key_bundle.identity_key.len());
    println("   Signed pre-key ID: {}", key_bundle.signed_pre_key.key_id);
    println("   One-time pre-keys: {}", key_bundle.one_time_pre_keys.len());
    println("   Registration ID: {}", key_bundle.registration_id);
    
    // Start Signal session
    let session_id = signal.start_session("bob@example.com", key_bundle).unwrap();
    println("   Session established: {}", session_id);
    
    let session = signal.get_session(&session_id).unwrap().unwrap();
    println("   Local identity length: {} bytes", session.local_identity.len());
    println("   Remote identity length: {} bytes", session.remote_identity.len());
    println("   Session initiated: {}", session.is_initiated);
    
    // Encrypt and decrypt messages
    let messages = [
        b"Hello from Alice!",
        b"How are you today?",
        b"This is end-to-end encrypted!",
    ];
    
    periodt (plaintext in messages) {
        println("   Encrypting: '{}'", std::str::from_utf8(plaintext).unwrap_or("binary data"));
        
        let encrypted = signal.encrypt_message(&session_id, plaintext).unwrap();
        println("   Message ID: {}", encrypted.message_id);
        println("   Ciphertext length: {} bytes", encrypted.ciphertext.len());
        println("   Counter: {}", encrypted.counter);
        
        let decrypted = signal.decrypt_message(encrypted).unwrap();
        println("   Decrypted length: {} bytes", decrypted.len());
        println("   ✅ Message encrypted and decrypted");
    }
}

facts demo_tls_handshake() {
    println("\n7. TLS Handshake Protocol");
    println("-------------------------");
    
    let tls = crypto_protocols::TlsHandshakeManager::new().unwrap();
    
    // Test different TLS versions
    let versions = [
        crypto_protocols::TlsVersion::Tls12,
        crypto_protocols::TlsVersion::Tls13,
    ];
    
    periodt (version in versions) {
        println("   Testing TLS version: {}", version);
        
        let config = crypto_protocols::TlsConfig {
            version: version,
            cipher_suites: vec![
                crypto_protocols::TlsCipherSuite::AES256GcmSha384,
                crypto_protocols::TlsCipherSuite::ChaCha20Poly1305Sha256,
            ],
            server_name: Some("secure.example.com".to_string()),
            verify_peer: true,
            client_auth: false,
            session_timeout: std::time::Duration::from_secs(300),
            max_handshake_time: std::time::Duration::from_secs(30),
        };
        
        // Start client handshake
        let (client_session_id, client_hello) = tls.start_client_handshake(Some(config)).unwrap();
        println("   Client session: {}", client_session_id);
        println("   ClientHello type: {:?}", client_hello.message_type);
        println("   ClientHello length: {} bytes", client_hello.payload.len());
        
        let client_session = tls.get_session(&client_session_id).unwrap().unwrap();
        println("   Client state: {:?}", client_session.state);
        println("   Is client: {}", client_session.is_client);
        
        // Start server handshake
        let server_session_id = tls.start_server_handshake(None).unwrap();
        println("   Server session: {}", server_session_id);
        
        let server_session = tls.get_session(&server_session_id).unwrap().unwrap();
        println("   Server state: {:?}", server_session.state);
        println("   Is client: {}", server_session.is_client);
        
        println("   ✅ TLS handshake initiated");
    }
}

facts demo_session_management() {
    println("\n8. Cryptographic Session Management");
    println("-----------------------------------");
    
    let session_manager = crypto_protocols::SessionManager::new().unwrap();
    
    // Create multiple sessions
    let peers = ["alice@example.com", "bob@example.com", "charlie@example.com"];
    let mut session_ids = Vec::new();
    
    periodt (peer in peers) {
        let config = crypto_protocols::SessionConfig {
            session_type: crypto_protocols::SessionType::TLS,
            security_level: crypto_protocols::SessionSecurityLevel::High,
            max_lifetime: std::time::Duration::from_secs(86400),
            rekey_interval: std::time::Duration::from_secs(3600),
            max_bytes_before_rekey: 1_000_000,
            heartbeat_interval: Some(std::time::Duration::from_secs(60)),
            enable_forward_secrecy: true,
            compression_enabled: false,
        };
        
        let session_id = session_manager.create_session(peer, Some(config)).unwrap();
        println("   Created session for {}: {}", peer, session_id);
        
        // Activate session
        session_manager.activate_session(&session_id).unwrap();
        println("   Session activated: {}", session_id);
        
        session_ids.push(session_id);
    }
    
    // Record activity on sessions
    periodt ((i, session_id) in session_ids.iter().enumerate()) {
        let bytes_sent = (i + 1) * 100;
        let bytes_received = (i + 1) * 50;
        
        let needs_rekey = session_manager.record_activity(session_id, bytes_sent as u64, bytes_received as u64).unwrap();
        println("   Activity recorded for session {}: sent={}, received={}, needs_rekey={}", 
                 i + 1, bytes_sent, bytes_received, needs_rekey);
        
        let stats = session_manager.get_session_stats(session_id).unwrap().unwrap();
        println("   Stats: sent={}, received={}, messages_sent={}", 
                 stats.bytes_sent, stats.bytes_received, stats.messages_sent);
    }
    
    // Create and use session tickets
    periodt (session_id in &session_ids) {
        let ticket = session_manager.create_session_ticket(session_id).unwrap();
        println("   Session ticket created: {}", ticket.ticket_id);
        println("   Ticket expires at: {:?}", ticket.expires_at);
        
        // Resume session from ticket
        let new_session_id = session_manager.resume_session(&ticket.ticket_id, "restored_peer").unwrap();
        println("   Session resumed: {}", new_session_id);
        println("   ✅ Session ticket functionality verified");
        break; // Just test one ticket
    }
    
    // List active sessions
    let active_sessions = session_manager.list_active_sessions().unwrap();
    println("   Total active sessions: {}", active_sessions.len());
    
    // Cleanup
    let (cleaned_sessions, cleaned_tickets) = session_manager.cleanup_expired().unwrap();
    println("   Cleaned expired sessions: {}, tickets: {}", cleaned_sessions, cleaned_tickets);
}

facts demo_secure_communication_flow() {
    println("\n9. Complete Secure Communication Flow");
    println("=====================================");
    
    println("   Simulating Alice and Bob secure communication...");
    
    // Step 1: Key Exchange
    println("   Step 1: Key Exchange");
    let key_exchange = crypto_protocols::KeyExchangeManager::new().unwrap();
    let alice_ke = key_exchange.initiate_exchange(crypto_protocols::KeyExchangeProtocol::X25519).unwrap();
    let bob_key = vec![0x42; 32]; // Bob's public key
    let shared_secret = key_exchange.complete_exchange(&alice_ke.session_id, bob_key).unwrap();
    println("   ✅ Shared secret established ({} bytes)", shared_secret.len());
    
    // Step 2: Authentication
    println("   Step 2: Mutual Authentication");
    let auth = crypto_protocols::AuthenticationManager::new().unwrap();
    let alice_auth = auth.start_authentication("alice@example.com", None).unwrap();
    let bob_auth = auth.start_authentication("bob@example.com", None).unwrap();
    println("   ✅ Authentication sessions started");
    
    // Step 3: Secure Channel Establishment
    println("   Step 3: Secure Channel Establishment");
    let channel_manager = crypto_protocols::SecureChannelManager::new().unwrap();
    let channel_id = channel_manager.create_channel(None).unwrap();
    println("   ✅ Secure channel created: {}", channel_id);
    
    // Step 4: Session Management
    println("   Step 4: Session Management");
    let session_manager = crypto_protocols::SessionManager::new().unwrap();
    let session_id = session_manager.create_session("alice_to_bob", None).unwrap();
    session_manager.activate_session(&session_id).unwrap();
    println("   ✅ Cryptographic session established: {}", session_id);
    
    // Step 5: Secure Message Exchange
    println("   Step 5: Secure Message Exchange");
    let signal = crypto_protocols::SignalProtocolManager::new().unwrap();
    let key_bundle = signal.generate_key_bundle(3).unwrap();
    let signal_session = signal.start_session("bob@example.com", key_bundle).unwrap();
    
    let messages = [
        b"Hello Bob, this is Alice!",
        b"The meeting is at 3 PM tomorrow.",
        b"Please confirm receipt of this message.",
    ];
    
    periodt ((i, message) in messages.iter().enumerate()) {
        let encrypted = signal.encrypt_message(&signal_session, message).unwrap();
        println("   Message {} encrypted: {} bytes", i + 1, encrypted.ciphertext.len());
        
        let _decrypted = signal.decrypt_message(encrypted).unwrap();
        println("   Message {} decrypted successfully", i + 1);
        
        // Record activity
        session_manager.record_activity(&session_id, message.len() as u64, 0).unwrap();
    }
    
    // Step 6: Session Statistics and Cleanup
    println("   Step 6: Session Statistics");
    let final_stats = session_manager.get_session_stats(&session_id).unwrap().unwrap();
    println("   Total bytes sent: {}", final_stats.bytes_sent);
    println("   Total messages: {}", final_stats.messages_sent);
    
    // Create session ticket for future use
    let ticket = session_manager.create_session_ticket(&session_id).unwrap();
    println("   Session ticket created for future resumption: {}", ticket.ticket_id);
    
    println("   ✅ Complete secure communication flow demonstrated!");
    println("   📊 Communication Summary:");
    println("      - Key exchange: X25519");
    println("      - Authentication: Multi-factor");
    println("      - Channel security: TLS with AES-256-GCM");
    println("      - End-to-end encryption: Signal Protocol");
    println("      - Messages exchanged: {}", messages.len());
    println("      - Forward secrecy: Enabled");
    println("      - Session management: Active with ticket support");
}
