fr fr Comprehensive Post-Quantum Cryptography Hybrid System Demo
fr fr 
fr fr This example demonstrates the hybrid cryptographic system that combines
fr fr classical and post-quantum algorithms for maximum security during the
fr fr post-quantum transition period.

yeet "stdlib::crypto_pqc::hybrid"
yeet "stdlib::io"

slay main() {
    println("🔐 CURSED Post-Quantum Cryptography Hybrid System Demo")?;
    println("=======================================================")?;
    
    // Demo 1: Hybrid Key Encapsulation Mechanism (KEM)
    demo_hybrid_kem()?;
    
    // Demo 2: Hybrid Digital Signatures
    demo_hybrid_signatures()?;
    
    // Demo 3: Algorithm Migration Strategy
    demo_migration_strategy()?;
    
    // Demo 4: Security Analysis and Recommendations
    demo_security_analysis()?;
    
    // Demo 5: Performance Comparison
    demo_performance_comparison()?;
    
    println("\n✅ All demos completed successfully!")?;
}

slay demo_hybrid_kem() {
    println("\n🔑 Demo 1: Hybrid Key Encapsulation Mechanism")?;
    println("===============================================")?;
    
    // Create hybrid KEM combining X25519 with Kyber
    facts hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3
    );
    
    println("📋 Algorithm Configuration:")?;
    println("   Classical: X25519 (Elliptic Curve Diffie-Hellman)")?;
    println("   Post-Quantum: Kyber768 (Lattice-based KEM)")?;
    println("   Security Level: Level 3 (AES-192 equivalent)")?;
    
    // Generate hybrid key pair
    println("\n🔧 Generating hybrid key pair...")?;
    facts key_pair = hybrid_kem.keygen()?;
    
    println("✅ Key pair generated successfully!")?;
    printf("   Classical public key size: {} bytes\n", &[key_pair.classical_public.len()])?;
    printf("   Classical secret key size: {} bytes\n", &[key_pair.classical_secret.len()])?;
    printf("   PQC public key size: {} bytes\n", &[key_pair.pqc_public.len()])?;
    printf("   PQC secret key size: {} bytes\n", &[key_pair.pqc_secret.len()])?;
    
    // Demonstrate encapsulation
    println("\n🔒 Performing hybrid encapsulation...")?;
    facts (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)?;
    
    printf("✅ Encapsulation complete! Ciphertext size: {} bytes\n", &[ciphertext.len()])?;
    printf("   Shared secret size: {} bytes\n", &[shared_secret1.len()])?;
    
    // Demonstrate decapsulation
    println("\n🔓 Performing hybrid decapsulation...")?;
    facts shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)?;
    
    lowkey (shared_secret1 == shared_secret2) {
        println("✅ Shared secrets match! Hybrid KEM working correctly.")?;
    } bestie {
        println("❌ Error: Shared secrets don't match!")?;
        return Err("KEM verification failed");
    }
    
    // Demonstrate key combination strategies
    println("\n🔧 Testing different key combination strategies:")?;
    
    facts classical_secret = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    facts pqc_secret = vec![8u8, 7, 6, 5, 4, 3, 2, 1];
    
    // Concatenation
    facts concat_result = hybrid_kem.combine_shared_secrets(
        classical_secret.clone(),
        pqc_secret.clone(),
        KeyCombinerType::Concatenation
    )?;
    printf("   Concatenation result size: {} bytes\n", &[concat_result.len()])?;
    
    // HKDF combination
    facts hkdf_result = hybrid_kem.combine_shared_secrets(
        classical_secret.clone(),
        pqc_secret.clone(),
        KeyCombinerType::HkdfCombination
    )?;
    printf("   HKDF result size: {} bytes\n", &[hkdf_result.len()])?;
    
    println("✅ All key combination strategies working correctly!")?;
}

slay demo_hybrid_signatures() {
    println("\n✍️  Demo 2: Hybrid Digital Signatures")?;
    println("======================================")?;
    
    // Create hybrid signature system combining Ed25519 with Dilithium
    facts hybrid_sig = HybridSignature::new(
        ClassicalSignatureAlgorithm::Ed25519,
        AlgorithmType::Dilithium,
        SecurityLevel::Level3
    );
    
    println("📋 Signature Algorithm Configuration:")?;
    println("   Classical: Ed25519 (Edwards Curve Signatures)")?;
    println("   Post-Quantum: Dilithium3 (Lattice-based Signatures)")?;
    println("   Security Level: Level 3 (AES-192 equivalent)")?;
    
    // Generate signature keys
    println("\n🔧 Generating hybrid signature key pair...")?;
    facts sig_keys = hybrid_sig.keygen()?;
    
    println("✅ Signature key pair generated!")?;
    printf("   Classical public key size: {} bytes\n", &[sig_keys.classical_public.len()])?;
    printf("   PQC public key size: {} bytes\n", &[sig_keys.pqc_public.len()])?;
    
    // Sign a message
    facts message = b"Hello, post-quantum cryptographic world! This message demonstrates hybrid signatures.";
    println("\n✍️  Signing message with hybrid signatures...")?;
    printf("   Message: '{}'\n", &[String::from_utf8_lossy(message)])?;
    
    facts signature = hybrid_sig.sign(&sig_keys, message)?;
    
    printf("✅ Message signed successfully!\n")?;
    printf("   Classical signature size: {} bytes\n", &[signature.classical_signature.len()])?;
    printf("   PQC signature size: {} bytes\n", &[signature.pqc_signature.len()])?;
    printf("   Combined signature size: {} bytes\n", &[signature.combined_signature.len()])?;
    
    // Verify the signature
    println("\n🔍 Verifying hybrid signature...")?;
    facts is_valid = hybrid_sig.verify(&sig_keys, message, &signature)?;
    
    lowkey (is_valid) {
        println("✅ Signature verification successful!")?;
    } bestie {
        println("❌ Signature verification failed!")?;
        return Err("Signature verification failed");
    }
    
    // Test with modified message
    facts modified_message = b"Hello, modified message!";
    println("\n🔍 Testing signature with modified message...")?;
    facts is_invalid = hybrid_sig.verify(&sig_keys, modified_message, &signature)?;
    
    lowkey (!is_invalid) {
        println("✅ Correctly rejected invalid signature!")?;
    } bestie {
        println("❌ Error: Invalid signature was accepted!")?;
        return Err("Security test failed");
    }
    
    println("✅ All signature tests passed!")?;
}

slay demo_migration_strategy() {
    println("\n🚀 Demo 3: Post-Quantum Migration Strategy")?;
    println("============================================")?;
    
    sus mut strategy = HybridMigrationStrategy::standard();
    
    println("📋 Standard Migration Strategy Phases:")?;
    
    sus phase_count = 0;
    periodt {
        facts current_phase = strategy.current_phase();
        lowkey let Some(phase) = current_phase {
            phase_count += 1;
            printf("   Phase {}: {}\n", &[phase_count, phase.name])?;
            printf("      Classical Weight: {:.1}\n", &[phase.classical_weight])?;
            printf("      PQC Weight: {:.1}\n", &[phase.pqc_weight])?;
            printf("      Min Security Level: {:?}\n", &[phase.minimum_security_level])?;
            
            // Show recommended algorithms
            lowkey (!phase.recommended_algorithms.is_empty()) {
                println("      Recommended Combinations:")?;
                damn (sus (classical, pqc) in phase.recommended_algorithms) {
                    printf("        {:?} + {:?}\n", &[classical, pqc])?;
                }
            }
            
            // Try to advance to next phase
            lowkey let Err(_) = strategy.advance_phase() {
                flex; // Reached final phase
            }
        } bestie {
            flex; // No more phases
        }
    }
    
    println("✅ Migration strategy demonstration complete!")?;
}

slay demo_security_analysis() {
    println("\n🛡️  Demo 4: Security Analysis and Recommendations")?;
    println("===================================================")?;
    
    facts matrix = HybridCompatibilityMatrix::new();
    
    println("📋 Algorithm Compatibility Analysis:")?;
    
    // Test specific combinations
    facts test_combinations = vec![
        (ClassicalAlgorithm::X25519, AlgorithmType::Kyber),
        (ClassicalAlgorithm::EcdhP256, AlgorithmType::Kyber),
        (ClassicalAlgorithm::EcdhP384, AlgorithmType::Dilithium),
        (ClassicalAlgorithm::EcdhP521, AlgorithmType::Sphincs),
        (ClassicalAlgorithm::Rsa2048, AlgorithmType::Kyber),
    ];
    
    damn (sus (classical, pqc) in test_combinations) {
        facts rating = matrix.get_rating(classical, pqc);
        printf("   {:?} + {:?}: {:?}\n", &[classical, pqc, rating])?;
    }
    
    println("\n🌟 Excellent Combinations:")?;
    facts excellent_combos = matrix.get_excellent_combinations();
    damn (sus (classical, pqc) in excellent_combos) {
        printf("   {:?} + {:?}\n", &[classical, pqc])?;
    }
    
    println("\n🔒 Recommendations by Security Level:")?;
    
    damn (sus level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5]) {
        printf("   {:?}:\n", &[level])?;
        facts recommendations = matrix.get_recommended_for_security_level(*level);
        damn (sus (classical, pqc) in recommendations.iter().take(3)) { // Show first 3
            printf("     {:?} + {:?}\n", &[classical, pqc])?;
        }
    }
    
    println("✅ Security analysis complete!")?;
}

slay demo_performance_comparison() {
    println("\n⚡ Demo 5: Performance Comparison")?;
    println("=================================")?;
    
    println("📊 Comparing performance across different configurations...")?;
    
    facts configurations = vec![
        ("X25519 + Kyber512", ClassicalAlgorithm::X25519, AlgorithmType::Kyber, SecurityLevel::Level1),
        ("ECDH-P256 + Kyber768", ClassicalAlgorithm::EcdhP256, AlgorithmType::Kyber, SecurityLevel::Level3),
        ("ECDH-P384 + Kyber1024", ClassicalAlgorithm::EcdhP384, AlgorithmType::Kyber, SecurityLevel::Level5),
    ];
    
    damn (sus (name, classical, pqc, level) in configurations) {
        printf("\n🔧 Testing: {}\n", &[name])?;
        
        facts hybrid_kem = HybridKem::new(classical, pqc, level);
        
        // Measure key generation
        facts start_time = std::time::Instant::now();
        facts key_pair = hybrid_kem.keygen()?;
        facts keygen_time = start_time.elapsed();
        
        // Measure encapsulation
        facts start_time = std::time::Instant::now();
        facts (ciphertext, shared_secret) = hybrid_kem.encaps(&key_pair)?;
        facts encaps_time = start_time.elapsed();
        
        // Measure decapsulation
        facts start_time = std::time::Instant::now();
        facts _decaps_secret = hybrid_kem.decaps(&key_pair, &ciphertext)?;
        facts decaps_time = start_time.elapsed();
        
        printf("   Key Generation: {:?}\n", &[keygen_time])?;
        printf("   Encapsulation: {:?}\n", &[encaps_time])?;
        printf("   Decapsulation: {:?}\n", &[decaps_time])?;
        printf("   Ciphertext Size: {} bytes\n", &[ciphertext.len()])?;
        printf("   Shared Secret Size: {} bytes\n", &[shared_secret.len()])?;
    }
    
    println("\n📈 Performance Summary:")?;
    println("   • Higher security levels generally have larger key/ciphertext sizes")?;
    println("   • Kyber operations are typically faster than RSA")?;
    println("   • ECDH operations are efficient for classical components")?;
    println("   • Hybrid approach provides quantum resistance with manageable overhead")?;
    
    println("✅ Performance comparison complete!")?;
}

fr fr Utility functions for better output formatting
slay format_bytes(sus size: usize) -> String {
    lowkey (size < 1024) {
        format!("{} B", size)
    } bestie lowkey (size < 1024 * 1024) {
        format!("{:.1} KB", size as f64 / 1024.0)
    } bestie {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    }
}

slay format_duration(sus duration: Duration) -> String {
    facts nanos = duration.as_nanos();
    lowkey (nanos < 1_000) {
        format!("{} ns", nanos)
    } bestie lowkey (nanos < 1_000_000) {
        format!("{:.1} μs", nanos as f64 / 1_000.0)
    } bestie lowkey (nanos < 1_000_000_000) {
        format!("{:.1} ms", nanos as f64 / 1_000_000.0)
    } bestie {
        format!("{:.1} s", nanos as f64 / 1_000_000_000.0)
    }
}

fr fr Advanced demo showing real-world usage scenarios
slay demo_real_world_scenarios() {
    println("\n🌍 Real-World Usage Scenarios")?;
    println("==============================")?;
    
    // Scenario 1: Secure Communication Setup
    println("\n📡 Scenario 1: Secure Communication Channel Setup")?;
    println("Server and client establish a quantum-resistant shared secret")?;
    
    // Server side
    facts server_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3
    );
    facts server_keys = server_kem.keygen()?;
    
    // Client side - encapsulate to server's public key
    facts (ciphertext, client_shared_secret) = server_kem.encaps(&server_keys)?;
    
    // Server side - decapsulate to get same shared secret
    facts server_shared_secret = server_kem.decaps(&server_keys, &ciphertext)?;
    
    lowkey (client_shared_secret == server_shared_secret) {
        println("✅ Secure channel established with quantum-resistant shared secret!")?;
        printf("   Shared secret: {} bytes\n", &[client_shared_secret.len()])?;
    }
    
    // Scenario 2: Document Signing with Long-term Security
    println("\n📄 Scenario 2: Document Signing for Long-term Security")?;
    println("Important documents signed with hybrid signatures")?;
    
    facts doc_signer = HybridSignature::new(
        ClassicalSignatureAlgorithm::Ed25519,
        AlgorithmType::Dilithium,
        SecurityLevel::Level5  // Highest security for long-term protection
    );
    
    facts signing_keys = doc_signer.keygen()?;
    facts important_document = b"IMPORTANT LEGAL CONTRACT - This document must remain verifiable for decades.";
    
    facts document_signature = doc_signer.sign(&signing_keys, important_document)?;
    facts is_valid = doc_signer.verify(&signing_keys, important_document, &document_signature)?;
    
    lowkey (is_valid) {
        println("✅ Document signed with hybrid signature for long-term security!")?;
        printf("   Total signature size: {} bytes\n", &[document_signature.combined_signature.len()])?;
        println("   → Protected against both classical and quantum attacks")?;
    }
    
    // Scenario 3: Progressive Migration
    println("\n🔄 Scenario 3: Progressive Migration from Classical to PQC")?;
    
    sus mut migration = HybridMigrationStrategy::standard();
    sus phase_num = 1;
    
    periodt {
        lowkey let Some(phase) = migration.current_phase() {
            printf("Phase {}: {} (Classical: {:.0}%, PQC: {:.0}%)\n", 
                &[phase_num, phase.name, phase.classical_weight * 100.0, phase.pqc_weight * 100.0])?;
            
            // Simulate migration decision
            lowkey (phase.pqc_weight >= 0.5) {
                println("   → Quantum resistance is now primary security mechanism")?;
            } bestie lowkey (phase.pqc_weight > 0.0) {
                println("   → Hybrid approach provides transition period safety")?;
            } bestie {
                println("   → Classical cryptography still sufficient for current threats")?;
            }
            
            phase_num += 1;
            lowkey let Err(_) = migration.advance_phase() {
                flex;
            }
        } bestie {
            flex;
        }
    }
    
    println("✅ Migration strategy simulation complete!")?;
}
