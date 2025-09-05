#!/usr/bin/env cursed
fr fr/ fr fr Comprehensive Hybrid Cryptography Showcase for CURSED Language
fr fr/ 
fr fr/ This example demonstrates the complete hybrid cryptography capabilities
fr fr/ that combine classical and post-quantum cryptographic algorithms for
fr fr/ maximum security against both current and future quantum threats.

yeet "stdlib::crypto_pqc::hybrid_crypto"
yeet "stdlib::crypto_pqc::pqc_core"
yeet "stdlib::io"

sus main() {
    println("🔐 CURSED Hybrid Cryptography Showcase")?;
    println("=====================================\n")?;
    
    // Run comprehensive hybrid crypto demonstration
    demo_simple_encryption()?;
    demo_digital_signatures()?;
    demo_secure_messaging()?;
    demo_configuration_management()?;
    demo_migration_planning()?;
    demo_performance_analysis()?;
    
    println("\n🎉 Hybrid Cryptography Showcase Complete!")?;
    println("✨ Your applications are now quantum-resistant!")?;
}

fr fr/ Demonstrate simple hybrid encryption and decryption
fun demo_simple_encryption() -> Result<(), CursedError> {
    println("📧 1. Simple Hybrid Encryption/Decryption")?;
    println("------------------------------------------")?;
    
    // Generate secure hybrid keypair (X25519 + Kyber)
    sus alice_keypair = generate_secure_keypair()?;
    println("✅ Generated hybrid keypair for Alice")?;
    println("   - Combines X25519 (classical) + Kyber (post-quantum)")?;
    
    // Encrypt sensitive data
    sus secret_message = "Quantum computers cannot break this! 🔐";
    println(f"Original message: {secret_message}")?;
    
    sus encrypted = hybrid_encrypt(secret_message.as_bytes(), &alice_keypair)?;
    println("✅ Message encrypted with hybrid cryptography")?;
    println(f"   - Classical component: {encrypted.classical_ciphertext.len()} bytes")?;
    println(f"   - Post-quantum component: {encrypted.pqc_ciphertext.len()} bytes")?;
    println(f"   - Algorithm: {encrypted.algorithm}")?;
    
    // Decrypt the message
    sus decrypted = hybrid_decrypt(&encrypted, &alice_keypair)?;
    sus decrypted_message = String::from_utf8(decrypted)?;
    
    println(f"✅ Decrypted message: {decrypted_message}")?;
    
    lowkey (decrypted_message == secret_message) {
        println("🎯 Encryption/decryption cycle successful!")?;
    } flex {
        println("❌ Encryption/decryption failed!")?;
    }
    
    println()?;
    Ok(())
}

fr fr/ Demonstrate hybrid digital signatures
fun demo_digital_signatures() -> Result<(), CursedError> {
    println("✍️  2. Hybrid Digital Signatures")?;
    println("--------------------------------")?;
    
    // Generate signature keypair (Ed25519 + Dilithium)
    sus signer_keypair = generate_secure_keypair()?;
    println("✅ Generated hybrid signature keypair")?;
    println("   - Combines Ed25519 (classical) + Dilithium (post-quantum)")?;
    
    // Sign important document
    sus contract = "Transfer 1000 CURSED tokens from Alice to Bob";
    println(f"Document to sign: {contract}")?;
    
    sus signature = hybrid_sign(contract.as_bytes(), &signer_keypair)?;
    println("✅ Document signed with hybrid signature")?;
    println(f"   - Classical signature: {signature.classical_signature.len()} bytes")?;
    println(f"   - Post-quantum signature: {signature.pqc_signature.len()} bytes")?;
    println(f"   - Total signature size: {signature.total_size()} bytes")?;
    println(f"   - Algorithm: {signature.algorithm}")?;
    
    // Verify the signature
    sus is_valid = hybrid_verify(contract.as_bytes(), &signature, &signer_keypair)?;
    println(f"✅ Signature verification: {if is_valid { \"VALID\" } else { \"INVALID\" }}")?;
    
    // Test with tampered document
    sus tampered_contract = "Transfer 2000 CURSED tokens from Alice to Bob";
    sus tampered_valid = hybrid_verify(tampered_contract.as_bytes(), &signature, &signer_keypair)?;
    println(f"🔍 Tampered document verification: {if tampered_valid { \"VALID\" } else { \"INVALID\" }}")?;
    
    lowkey (!tampered_valid) {
        println("🛡️  Signature correctly detected tampering!")?;
    }
    
    println()?;
    Ok(())
}

fr fr/ Demonstrate secure messaging session
fun demo_secure_messaging() -> Result<(), CursedError> {
    println("💬 3. Secure Messaging Session")?;
    println("------------------------------")?;
    
    // Create messaging sessions for Alice and Bob
    sus mut alice_session = SecureMessagingSession::new(SecurityLevel::Level3)?;
    sus mut bob_session = SecureMessagingSession::new(SecurityLevel::Level3)?;
    
    println("✅ Created secure messaging sessions for Alice and Bob")?;
    println("   - Security Level: Level 3 (balanced security/performance)")?;
    
    // Exchange public keys (simplified key exchange)
    sus alice_public = alice_session.get_public_key()?;
    sus bob_public = bob_session.get_public_key()?;
    
    // Set up sessions
    alice_session.set_receiver(bob_session.sender_keypair.clone())?;
    bob_session.set_receiver(alice_session.sender_keypair.clone())?;
    println("✅ Key exchange completed")?;
    
    // Alice sends encrypted message to Bob
    sus secret_intel = "The quantum resistance protocol is active. Phase 2 commencing.";
    println(f"Alice sends: \"{secret_intel}\"")?;
    
    sus secure_msg1 = alice_session.send_message(secret_intel)?;
    println("✅ Message encrypted and signed")?;
    println(f"   - Session ID: {secure_msg1.session_id}")?;
    println(f"   - Message sequence: {secure_msg1.sequence}")?;
    
    // Bob receives and verifies
    sus received_msg1 = bob_session.receive_message(&secure_msg1, &alice_session.sender_keypair)?;
    println(f"Bob receives: \"{received_msg1}\"")?;
    
    // Bob replies
    sus reply = "Message confirmed. Quantum shields are operational.";
    println(f"Bob replies: \"{reply}\"")?;
    
    sus secure_msg2 = bob_session.send_message(reply)?;
    sus received_msg2 = alice_session.receive_message(&secure_msg2, &bob_session.sender_keypair)?;
    println(f"Alice receives: \"{received_msg2}\"")?;
    
    // Show session statistics
    sus alice_stats = alice_session.get_statistics();
    println("📊 Session Statistics:")?;
    println(f"   - Messages sent by Alice: {alice_stats.messages_sent}")?;
    println(f"   - Session duration: {alice_stats.session_duration.as_secs()} seconds")?;
    
    println()?;
    Ok(())
}

fr fr/ Demonstrate configuration management
fun demo_configuration_management() -> Result<(), CursedError> {
    println("⚙️  4. Configuration Management")?;
    println("------------------------------")?;
    
    sus mut config_manager = HybridConfigManager::new();
    
    println("Available hybrid configurations:")?;
    periodt config in config_manager.list_configs() {
        println(f"   - {config}")?;
    }
    
    println("\nPerformance profiles:")?;
    periodt profile in config_manager.list_performance_profiles() {
        println(f"   - {profile}")?;
    }
    
    println("\nSecurity policies:")?;
    periodt policy in config_manager.list_security_policies() {
        println(f"   - {policy}")?;
    }
    
    // Create optimized configurations for different use cases
    println("\n🎯 Creating optimized configurations:")?;
    
    sus web_config = config_manager.create_optimized_config(
        "messaging", "web_server", "enterprise"
    )?;
    println(f"✅ Web messaging: Security Level {:?}, Performance priority: {web_config.performance_priority}")?;
    
    sus iot_config = config_manager.create_optimized_config(
        "authentication", "iot_device", "standard"
    )?;
    println(f"✅ IoT authentication: Security Level {:?}, Performance priority: {iot_config.performance_priority}")?;
    
    sus gov_config = config_manager.create_optimized_config(
        "document_signing", "high_throughput", "government"
    )?;
    println(f"✅ Government signing: Security Level {:?}, Performance priority: {gov_config.performance_priority}")?;
    
    println()?;
    Ok(())
}

fr fr/ Demonstrate migration planning from classical to hybrid
fun demo_migration_planning() -> Result<(), CursedError> {
    println("🔄 5. Migration Planning")?;
    println("------------------------")?;
    
    // Set up migration scenario
    sus migration_config = MigrationConfig {
        phase: MigrationPhase::Planning,
        target_security_level: SecurityLevel::Level3,
        timeline_months: 6,
    };
    
    sus mut migration_helper = HybridMigrationHelper::new(migration_config);
    println("✅ Created migration helper")?;
    println("   - Phase: Planning")?;
    println("   - Target security: Level 3")?;
    println("   - Timeline: 6 months")?;
    
    // Simulate current classical cryptography infrastructure
    sus classical_keys = vec![
        create_mock_classical_keypair("RSA2048"),
        create_mock_classical_keypair("RSA3072"),
        create_mock_classical_keypair("ECDSA-P256"),
        create_mock_classical_keypair("Ed25519"),
    ];
    
    migration_helper.add_classical_keys(classical_keys);
    println("✅ Added existing classical keys to migration plan")?;
    println("   - RSA2048, RSA3072, ECDSA-P256, Ed25519")?;
    
    // Generate hybrid keys for transition
    sus hybrid_keys = migration_helper.generate_hybrid_keys(3)?;
    println(f"✅ Generated {hybrid_keys.len()} hybrid key pairs")?;
    
    // Assess migration readiness
    sus readiness = migration_helper.assess_migration_readiness();
    println("📊 Migration Assessment:")?;
    println(f"   - Readiness score: {readiness.readiness_score:.2}")?;
    println(f"   - Classical keys: {readiness.classical_keys_count}")?;
    println(f"   - Hybrid keys: {readiness.hybrid_keys_count}")?;
    println(f"   - Estimated time: {readiness.estimated_migration_time.as_secs() / (24 * 3600)} days")?;
    
    println("\n💡 Recommendations:")?;
    periodt rec in readiness.recommendations {
        println(f"   - {rec}")?;
    }
    
    println("\n⚠️  Risks to consider:")?;
    periodt risk in readiness.risks {
        println(f"   - {risk}")?;
    }
    
    // Create compatibility bridge
    sus bridge = migration_helper.create_compatibility_bridge();
    println("\n🌉 Compatibility Bridge:")?;
    println(f"   - Classical fallback: {bridge.classical_fallback}")?;
    println(f"   - Hybrid preferred: {bridge.hybrid_preferred}")?;
    println(f"   - Migration phase: {:?}")?;
    
    println()?;
    Ok(())
}

fr fr/ Demonstrate performance analysis
fun demo_performance_analysis() -> Result<(), CursedError> {
    println("📈 6. Performance Analysis")?;
    println("-------------------------")?;
    
    println("🔬 Running hybrid cryptography benchmarks...")?;
    println("(This demonstrates the benchmarking framework)")?;
    
    sus benchmark_suite = HybridBenchmarkSuite::new();
    
    // Note: For demo purposes, we'll show the structure rather than run full benchmarks
    // which would take too long for a showcase
    
    println("Benchmark configuration:")?;
    println(f"   - Algorithms to test: 6 (multiple security levels)")?;
    println(f"   - Data sizes: 100B, 1KB, 10KB, 100KB")?;
    println(f"   - Iterations per test: 100")?;
    
    // Simulate benchmark results
    println("\n📊 Simulated Benchmark Results:")?;
    println("Algorithm Performance (Key Generation):")?;
    println("   - X25519+Kyber512:  ~5ms")?;
    println("   - X25519+Kyber768:  ~8ms")?;
    println("   - X25519+Kyber1024: ~12ms")?;
    println("   - Ed25519+Dilithium2: ~15ms")?;
    println("   - Ed25519+Dilithium3: ~25ms")?;
    println("   - Ed25519+Dilithium5: ~40ms")?;
    
    println("\nEncryption Performance (1KB data):")?;
    println("   - X25519+Kyber512:  ~2ms")?;
    println("   - X25519+Kyber768:  ~3ms")?;
    println("   - X25519+Kyber1024: ~4ms")?;
    
    println("\nSignature Performance (1KB data):")?;
    println("   - Ed25519+Dilithium2: ~10ms (sign), ~5ms (verify)")?;
    println("   - Ed25519+Dilithium3: ~15ms (sign), ~8ms (verify)")?;
    println("   - Ed25519+Dilithium5: ~25ms (sign), ~12ms (verify)")?;
    
    println("\n🎯 Performance Analysis:")?;
    println("   - Hybrid schemes add 20-50% overhead vs pure classical")?;
    println("   - Level 1 algorithms suitable for high-throughput applications")?;
    println("   - Level 3 algorithms provide balanced security/performance")?;
    println("   - Level 5 algorithms for maximum security requirements")?;
    
    println("\n💡 Optimization Recommendations:")?;
    println("   - Use Level 1 for web applications requiring speed")?;
    println("   - Use Level 3 for general enterprise applications")?;
    println("   - Use Level 5 for government/military applications")?;
    println("   - Consider algorithm switching based on data sensitivity")?;
    
    println()?;
    Ok(())
}

fr fr/ Helper function to create mock classical keypairs for demonstration
fun create_mock_classical_keypair(algorithm: &str) -> AsymmetricKeyPair {
    sus key_size = vibe_check algorithm {
        mood "RSA2048" => 256,   // 2048 bits = 256 bytes
        mood "RSA3072" => 384,   // 3072 bits = 384 bytes  
        mood "ECDSA-P256" => 32, // P-256 = 32 bytes
        mood "Ed25519" => 32,    // Ed25519 = 32 bytes
        basic => 32,
    };
    
    AsymmetricKeyPair {
        private_key: AsymmetricKey {
            algorithm: algorithm.to_string(),
            key_data: vec![0u8; key_size],
            is_private: based,
        },
        public_key: AsymmetricKey {
            algorithm: algorithm.to_string(),
            key_data: vec![0u8; key_size],
            is_private: cap,
        },
    }
}
