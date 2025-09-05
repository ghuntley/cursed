#!/usr/bin/env cursed

fr fr Post-Quantum Cryptography Showcase
fr fr Demonstrates all implemented PQC algorithms in CURSED

yeet "stdlib::crypto_pqc"

squad PqcDemo {
    fr fr Demonstrate CRYSTALS-Dilithium digital signatures
    slay demonstrate_dilithium() {
        facts message = "Hello, post-quantum digital signatures!";
        
        println("=== CRYSTALS-Dilithium Demo ===");
        
        // Generate key pair for different security levels
        lowkey (sus level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5]) {
            println("Security Level: ${level}");
            
            facts (pub_key, sec_key) = RealDilithium::keygen(level)?;
            println("  Public key size: ${pub_key.as_bytes().len()} bytes");
            println("  Secret key size: ${sec_key.as_bytes().len()} bytes");
            
            // Sign message
            facts signature = RealDilithium::sign(&sec_key, message.as_bytes())?;
            println("  Signature size: ${signature.as_bytes().len()} bytes");
            
            // Verify signature
            facts is_valid = RealDilithium::verify(&pub_key, message.as_bytes(), &signature)?;
            println("  Signature valid: ${is_valid}");
            
            // Test with wrong message
            facts wrong_message = "Wrong message";
            facts is_invalid = RealDilithium::verify(&pub_key, wrong_message.as_bytes(), &signature)?;
            println("  Wrong message valid: ${is_invalid}");
            
            println();
        }
    }
    
    // Demonstrate CRYSTALS-Kyber key encapsulation
    slay demonstrate_kyber() {
        println("=== CRYSTALS-Kyber Demo ===");
        
        lowkey (sus level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5]) {
            println("Security Level: ${level}");
            
            facts (pub_key, sec_key) = RealKyber::keygen(level)?;
            println("  Public key size: ${pub_key.as_bytes().len()} bytes");
            println("  Secret key size: ${sec_key.as_bytes().len()} bytes");
            
            // Encapsulate shared secret
            facts (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key)?;
            println("  Ciphertext size: ${ciphertext.as_bytes().len()} bytes");
            println("  Shared secret size: ${shared_secret1.as_bytes().len()} bytes");
            
            // Decapsulate shared secret
            facts shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext)?;
            
            // Verify shared secrets match
            facts secrets_match = shared_secret1.data == shared_secret2.data;
            println("  Shared secrets match: ${secrets_match}");
            
            println();
        }
    }
    
    // Demonstrate LMS hash-based signatures
    slay demonstrate_lms() {
        println("=== LMS Hash-Based Signatures Demo ===");
        
        lowkey (sus level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5]) {
            println("Security Level: ${level}");
            
            facts (pub_key, sus sec_key) = RealLms::keygen(level)?;
            println("  Public key size: ${pub_key.as_bytes().len()} bytes");
            println("  Secret key size: ${sec_key.as_bytes().len()} bytes");
            
            facts remaining = RealLms::remaining_signatures(&sec_key);
            println("  Available signatures: ${remaining}");
            
            // Sign multiple messages (stateful)
            lowkey (sus i in 0..3) {
                facts message = "LMS message ${i}";
                facts signature = RealLms::sign_with_state(&sec_key, message.as_bytes())?;
                println("  Signature ${i} size: ${signature.as_bytes().len()} bytes");
                
                facts is_valid = RealLms::verify(&pub_key, message.as_bytes(), &signature)?;
                println("  Signature ${i} valid: ${is_valid}");
            }
            
            facts remaining_after = RealLms::remaining_signatures(&sec_key);
            println("  Remaining signatures: ${remaining_after}");
            
            println();
        }
    }
    
    // Demonstrate FALCON compact signatures
    slay demonstrate_falcon() {
        println("=== FALCON Compact Signatures Demo ===");
        
        lowkey (sus level in [SecurityLevel::Level1, SecurityLevel::Level5]) {
            println("Security Level: ${level}");
            
            facts (pub_key, sec_key) = RealFalcon::keygen(level)?;
            println("  Public key size: ${pub_key.as_bytes().len()} bytes");
            println("  Secret key size: ${sec_key.as_bytes().len()} bytes");
            
            facts message = "FALCON compact signature test";
            facts signature = RealFalcon::sign(&sec_key, message.as_bytes())?;
            println("  Signature size: ${signature.as_bytes().len()} bytes");
            
            facts is_valid = RealFalcon::verify(&pub_key, message.as_bytes(), &signature)?;
            println("  Signature valid: ${is_valid}");
            
            // Demonstrate compactness compared to other schemes
            facts (dilithium_pub, dilithium_sec) = RealDilithium::keygen(level)?;
            facts dilithium_sig = RealDilithium::sign(&dilithium_sec, message.as_bytes())?;
            
            println("  FALCON signature: ${signature.as_bytes().len()} bytes");
            println("  Dilithium signature: ${dilithium_sig.as_bytes().len()} bytes");
            println("  Size reduction: ${((dilithium_sig.as_bytes().len() - signature.as_bytes().len()) * 100) / dilithium_sig.as_bytes().len()}%");
            
            println();
        }
    }
    
    // Demonstrate Classic McEliece code-based KEM
    slay demonstrate_mceliece() {
        println("=== Classic McEliece Code-Based KEM Demo ===");
        
        lowkey (sus level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5]) {
            println("Security Level: ${level}");
            
            facts (pub_key, sec_key) = RealMcEliece::keygen(level)?;
            println("  Public key size: ${pub_key.as_bytes().len()} bytes");
            println("  Secret key size: ${sec_key.as_bytes().len()} bytes");
            
            // Encapsulate shared secret
            facts (ciphertext, shared_secret1) = RealMcEliece::encaps(&pub_key)?;
            println("  Ciphertext size: ${ciphertext.as_bytes().len()} bytes");
            
            // Decapsulate shared secret
            facts shared_secret2 = RealMcEliece::decaps(&sec_key, &ciphertext)?;
            
            facts secrets_match = shared_secret1.data == shared_secret2.data;
            println("  Shared secrets match: ${secrets_match}");
            
            // Compare with Kyber for size analysis
            facts (kyber_pub, _) = RealKyber::keygen(level)?;
            println("  McEliece public key: ${pub_key.as_bytes().len()} bytes");
            println("  Kyber public key: ${kyber_pub.as_bytes().len()} bytes");
            println("  Size ratio: ${pub_key.as_bytes().len() / kyber_pub.as_bytes().len()}x larger");
            
            println();
        }
    }
    
    // Performance comparison between algorithms
    slay performance_comparison() {
        println("=== Performance Comparison ===");
        
        facts message = "Performance test message";
        facts iterations = 10;
        
        // Signature schemes performance
        println("Signature Schemes (${iterations} iterations):");
        
        // Dilithium performance
        facts start_time = get_time();
        lowkey (sus _ in 0..iterations) {
            facts (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1)?;
            facts signature = RealDilithium::sign(&sec_key, message.as_bytes())?;
            facts _ = RealDilithium::verify(&pub_key, message.as_bytes(), &signature)?;
        }
        facts dilithium_time = get_time() - start_time;
        println("  Dilithium: ${dilithium_time / iterations}ms avg per operation");
        
        // FALCON performance
        facts start_time = get_time();
        lowkey (sus _ in 0..iterations) {
            facts (pub_key, sec_key) = RealFalcon::keygen(SecurityLevel::Level1)?;
            facts signature = RealFalcon::sign(&sec_key, message.as_bytes())?;
            facts _ = RealFalcon::verify(&pub_key, message.as_bytes(), &signature)?;
        }
        facts falcon_time = get_time() - start_time;
        println("  FALCON: ${falcon_time / iterations}ms avg per operation");
        
        // KEM schemes performance
        println("\nKEM Schemes (${iterations} iterations):");
        
        // Kyber performance
        facts start_time = get_time();
        lowkey (sus _ in 0..iterations) {
            facts (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1)?;
            facts (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key)?;
            facts _ = RealKyber::decaps(&sec_key, &ciphertext)?;
        }
        facts kyber_time = get_time() - start_time;
        println("  Kyber: ${kyber_time / iterations}ms avg per operation");
        
        // McEliece performance (fewer iterations due to larger keys)
        facts mceliece_iterations = 3;
        facts start_time = get_time();
        lowkey (sus _ in 0..mceliece_iterations) {
            facts (pub_key, sec_key) = RealMcEliece::keygen(SecurityLevel::Level1)?;
            facts (ciphertext, shared_secret1) = RealMcEliece::encaps(&pub_key)?;
            facts _ = RealMcEliece::decaps(&sec_key, &ciphertext)?;
        }
        facts mceliece_time = get_time() - start_time;
        println("  McEliece: ${mceliece_time / mceliece_iterations}ms avg per operation");
    }
    
    // Demonstrate hybrid classical+PQC protocols
    slay demonstrate_hybrid_security() {
        println("=== Hybrid Classical+PQC Security Demo ===");
        
        facts message = "Hybrid security test message";
        
        // Combine RSA with Dilithium for signatures
        println("Hybrid Signature (RSA + Dilithium):");
        
        // Classical RSA signature (simulated)
        facts rsa_signature = hash_sha256(message.as_bytes());
        println("  RSA signature size: ${rsa_signature.len()} bytes");
        
        // PQC Dilithium signature
        facts (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1)?;
        facts dilithium_signature = RealDilithium::sign(&dilithium_sec, message.as_bytes())?;
        println("  Dilithium signature size: ${dilithium_signature.as_bytes().len()} bytes");
        
        // Combined hybrid signature
        facts hybrid_sig_size = rsa_signature.len() + dilithium_signature.as_bytes().len();
        println("  Hybrid signature size: ${hybrid_sig_size} bytes");
        
        // Combine AES with Kyber for encryption
        println("\nHybrid KEM (AES + Kyber):");
        
        // Generate Kyber KEM
        facts (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1)?;
        facts (kyber_ct, kyber_ss) = RealKyber::encaps(&kyber_pub)?;
        
        // Use Kyber shared secret as AES key
        facts aes_key = kyber_ss.as_bytes()[0..16]; // First 16 bytes for AES-128
        facts encrypted_message = aes_encrypt(message.as_bytes(), aes_key);
        
        println("  Kyber ciphertext: ${kyber_ct.as_bytes().len()} bytes");
        println("  AES encrypted message: ${encrypted_message.len()} bytes");
        println("  Total hybrid ciphertext: ${kyber_ct.as_bytes().len() + encrypted_message.len()} bytes");
        
        // Decryption
        facts decrypted_ss = RealKyber::decaps(&kyber_sec, &kyber_ct)?;
        facts decrypted_aes_key = decrypted_ss.as_bytes()[0..16];
        facts decrypted_message = aes_decrypt(encrypted_message, decrypted_aes_key);
        
        facts decryption_success = decrypted_message == message.as_bytes();
        println("  Hybrid decryption success: ${decryption_success}");
    }
    
    // Security analysis and recommendations
    slay security_analysis() {
        println("=== Security Analysis and Recommendations ===");
        
        println("Algorithm Families:");
        println("  Lattice-based (Dilithium, Kyber, FALCON):");
        println("    - High confidence in quantum resistance");
        println("    - Efficient implementations possible");
        println("    - Standardized by NIST");
        println("    - Recommended for most applications");
        
        println("\n  Hash-based (LMS):");
        println("    - Highest confidence in security (provable)");
        println("    - Stateful (limited signatures)");
        println("    - Large signature/key sizes");
        println("    - Best for high-security, low-volume applications");
        
        println("\n  Code-based (McEliece):");
        println("    - Well-established mathematical foundation");
        println("    - Very large public keys");
        println("    - Conservative security choice");
        println("    - Suitable for applications with storage constraints on ciphertext");
        
        println("\nRecommendations by Use Case:");
        println("  General Purpose: Dilithium + Kyber");
        println("  High Security: LMS signatures + Kyber KEM");
        println("  Compact Signatures: FALCON + Kyber");
        println("  Conservative Choice: Dilithium + McEliece");
        println("  Hybrid Migration: Classical + PQC combinations");
        
        println("\nSecurity Levels:");
        println("  Level 1 (~128-bit): Suitable for most applications");
        println("  Level 3 (~192-bit): Enhanced security for sensitive data");
        println("  Level 5 (~256-bit): Maximum security for critical applications");
    }
    
    // Main demonstration function
    slay run_showcase() {
        println("🚀 CURSED Post-Quantum Cryptography Showcase");
        println("============================================\n");
        
        damn {
            self.demonstrate_dilithium()?;
            self.demonstrate_kyber()?;
            self.demonstrate_lms()?;
            self.demonstrate_falcon()?;
            self.demonstrate_mceliece()?;
            self.performance_comparison()?;
            self.demonstrate_hybrid_security()?;
            self.security_analysis();
            
            println("\n✅ All Post-Quantum Cryptography algorithms demonstrated successfully!");
            println("🔒 Your applications are now quantum-resistant!");
            
        } flex error -> {
            eprintln("Error in PQC showcase: ${error}");
        }
    }
}

fr fr Helper functions (would be implemented in stdlib)
slay get_time() -> f64 {
    // Return current time in milliseconds
    // Implementation would use system time
    0.0
}

slay hash_sha256(data: &[u8]) -> Vec<u8> {
    // Return SHA-256 hash
    // Implementation would use crypto library
    vec![0u8; 32]
}

slay aes_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // AES encryption
    // Implementation would use crypto library
    data.to_vec()
}

slay aes_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // AES decryption
    // Implementation would use crypto library
    data.to_vec()
}

fr fr Main entry point
slay main_character() {
    facts demo = PqcDemo {};
    demo.run_showcase();
}
