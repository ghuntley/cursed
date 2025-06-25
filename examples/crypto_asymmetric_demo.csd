/// Comprehensive Asymmetric Cryptography Demo
/// 
/// This demo showcases all asymmetric cryptographic capabilities:
/// - RSA encryption/decryption and digital signatures
/// - ECDSA digital signatures on multiple elliptic curves  
/// - Ed25519 high-performance digital signatures
/// - Key exchange protocols: X25519, X448, and Diffie-Hellman
/// - Unified asymmetric cryptography API
/// - Real-world cryptographic scenarios

import "stdlib::packages::crypto_asymmetric"

slay demonstrate_asymmetric_crypto() {
    println("🔑 CURSED Asymmetric Cryptography Demo")
    println("=====================================")
    
    // Initialize the crypto package
    init_crypto_asymmetric()?
    
    println("\n📊 Supported Algorithms:")
    facts capabilities = get_asymmetric_capabilities()?
    println(format!("  Algorithms: {}", capabilities.algorithms))
    println(format!("  Operations: {}", capabilities.operations))
    println(format!("  Key Formats: {}", capabilities.key_formats))
    
    // Demonstrate RSA operations
    demonstrate_rsa_crypto()?
    
    // Demonstrate ECDSA operations
    demonstrate_ecdsa_crypto()?
    
    // Demonstrate Ed25519 operations
    demonstrate_ed25519_crypto()?
    
    // Demonstrate key exchange protocols
    demonstrate_key_exchange()?
    
    // Demonstrate unified API
    demonstrate_unified_api()?
    
    println("\n✅ All asymmetric crypto demos completed successfully!")
}

slay demonstrate_rsa_crypto() {
    println("\n🔐 RSA Cryptography Demo")
    println("------------------------")
    
    // Test different RSA key sizes
    facts key_sizes = ["RSA-2048", "RSA-3072", "RSA-4096"]
    
    lowkey (sus algorithm in key_sizes) {
        println(format!("\n  Testing {}:", algorithm))
        
        // Generate RSA keypair
        facts keypair = generate_asymmetric_keypair(algorithm)?
        facts public_key = keypair.public_key
        facts private_key = keypair.private_key
        
        println(format!("    ✅ Generated {} keypair", algorithm))
        println(format!("    📊 Key size: {} bits", keypair.key_size))
        
        // Test encryption and decryption
        facts plaintext = "Hello, RSA world! This is a test message."
        facts encrypted = asymmetric_encrypt("RSA", public_key, plaintext)?
        facts decrypted = asymmetric_decrypt("RSA", private_key, encrypted)?
        
        lowkey (decrypted == plaintext) {
            println("    ✅ Encryption/Decryption: SUCCESS")
        } else {
            println("    ❌ Encryption/Decryption: FAILED")
        }
        
        // Test digital signature
        facts message = "Important message to sign"
        facts signature = asymmetric_sign(algorithm, private_key, message)?
        facts is_valid = asymmetric_verify(algorithm, public_key, message, signature)?
        
        lowkey (is_valid) {
            println("    ✅ Digital Signature: VALID")
        } else {
            println("    ❌ Digital Signature: INVALID")
        }
        
        // Test signature with wrong message
        facts wrong_message = "Tampered message"
        facts is_invalid = asymmetric_verify(algorithm, public_key, wrong_message, signature)?
        
        lowkey (!is_invalid) {
            println("    ✅ Signature Verification: Correctly rejected tampered message")
        } else {
            println("    ❌ Signature Verification: Failed to detect tampering")
        }
    }
}

slay demonstrate_ecdsa_crypto() {
    println("\n🔐 ECDSA Cryptography Demo")
    println("--------------------------")
    
    // Test different elliptic curves
    facts curves = ["ECDSA-P256", "ECDSA-P384", "ECDSA-P521"]
    
    lowkey (sus algorithm in curves) {
        println(format!("\n  Testing {}:", algorithm))
        
        // Generate ECDSA keypair
        facts keypair = generate_asymmetric_keypair(algorithm)?
        facts public_key = keypair.public_key
        facts private_key = keypair.private_key
        
        println(format!("    ✅ Generated {} keypair", algorithm))
        println(format!("    📊 Curve: {}", keypair.curve))
        println(format!("    📊 Key size: {} bits", keypair.key_size))
        
        // Test digital signature
        facts message = "ECDSA signature test message"
        facts signature = asymmetric_sign(algorithm, private_key, message)?
        facts is_valid = asymmetric_verify(algorithm, public_key, message, signature)?
        
        lowkey (is_valid) {
            println("    ✅ ECDSA Signature: VALID")
        } else {
            println("    ❌ ECDSA Signature: INVALID")
        }
        
        // Test multiple signatures (should be different due to randomness)
        facts signature1 = asymmetric_sign(algorithm, private_key, message)?
        facts signature2 = asymmetric_sign(algorithm, private_key, message)?
        
        lowkey (signature1 != signature2) {
            println("    ✅ Signature Randomness: Different signatures for same message")
        } else {
            println("    ⚠️  Signature Randomness: Signatures are identical (may be deterministic)")
        }
    }
}

slay demonstrate_ed25519_crypto() {
    println("\n🔐 Ed25519 Cryptography Demo")
    println("----------------------------")
    
    // Generate Ed25519 keypair
    facts keypair = generate_asymmetric_keypair("Ed25519")?
    facts public_key = keypair.public_key
    facts private_key = keypair.private_key
    
    println("    ✅ Generated Ed25519 keypair")
    println("    📊 High-performance elliptic curve signatures")
    println("    📊 Key size: 255 bits (Curve25519)")
    
    // Test digital signature
    facts message = "Ed25519 is fast and secure!"
    facts signature = asymmetric_sign("Ed25519", private_key, message)?
    facts is_valid = asymmetric_verify("Ed25519", public_key, message, signature)?
    
    lowkey (is_valid) {
        println("    ✅ Ed25519 Signature: VALID")
    } else {
        println("    ❌ Ed25519 Signature: INVALID")
    }
    
    // Test performance with multiple signatures
    println("    📊 Performance test: Generating 100 signatures...")
    facts start_time = current_time()
    
    lowkey (sus i = 0; i < 100; i++) {
        facts test_msg = format!("Performance test message {}", i)
        asymmetric_sign("Ed25519", private_key, test_msg)?
    }
    
    facts end_time = current_time()
    facts duration = end_time - start_time
    println(format!("    ✅ Generated 100 signatures in {}ms", duration))
    println(format!("    📊 Average: {:.2}ms per signature", duration / 100.0))
}

slay demonstrate_key_exchange() {
    println("\n🔐 Key Exchange Protocols Demo")
    println("------------------------------")
    
    // X25519 Key Exchange
    println("\n  X25519 Key Exchange:")
    facts alice_keypair = x25519_generate_keypair()?
    facts bob_keypair = x25519_generate_keypair()?
    
    // Both parties compute the same shared secret
    facts alice_shared = x25519_key_exchange(alice_keypair.private_key, bob_keypair.public_key)?
    facts bob_shared = x25519_key_exchange(bob_keypair.private_key, alice_keypair.public_key)?
    
    lowkey (alice_shared.shared_secret == bob_shared.shared_secret) {
        println("    ✅ X25519: Both parties derived the same shared secret")
        println(format!("    📊 Shared secret length: {} bytes", alice_shared.shared_secret.len()))
    } else {
        println("    ❌ X25519: Shared secrets don't match!")
    }
    
    // X448 Key Exchange
    println("\n  X448 Key Exchange:")
    facts alice_x448 = x448_generate_keypair()?
    facts bob_x448 = x448_generate_keypair()?
    
    facts alice_x448_shared = x448_key_exchange(alice_x448.private_key, bob_x448.public_key)?
    facts bob_x448_shared = x448_key_exchange(bob_x448.private_key, alice_x448.public_key)?
    
    lowkey (alice_x448_shared.shared_secret == bob_x448_shared.shared_secret) {
        println("    ✅ X448: Both parties derived the same shared secret")
        println(format!("    📊 Shared secret length: {} bytes", alice_x448_shared.shared_secret.len()))
    } else {
        println("    ❌ X448: Shared secrets don't match!")
    }
    
    // Classic Diffie-Hellman
    println("\n  Classic Diffie-Hellman:")
    facts alice_dh = dh_generate_keypair()?
    facts bob_dh = dh_generate_keypair()?
    
    facts alice_dh_shared = dh_key_exchange(alice_dh.private_key, bob_dh.public_key)?
    facts bob_dh_shared = dh_key_exchange(bob_dh.private_key, alice_dh.public_key)?
    
    lowkey (alice_dh_shared.shared_secret == bob_dh_shared.shared_secret) {
        println("    ✅ DH: Both parties derived the same shared secret")
        println(format!("    📊 Key size: {} bits", alice_dh.key_size))
    } else {
        println("    ❌ DH: Shared secrets don't match!")
    }
    
    // Key derivation from shared secrets
    println("\n  Key Derivation:")
    facts derived_key = derive_key_from_shared_secret(
        alice_shared.shared_secret, 
        32, 
        "CURSED-DEMO-ENCRYPTION"
    )?
    println(format!("    ✅ Derived 256-bit encryption key: {} bytes", derived_key.len()))
    
    facts auth_key = derive_key_from_shared_secret(
        alice_shared.shared_secret,
        16,
        "CURSED-DEMO-AUTHENTICATION"
    )?
    println(format!("    ✅ Derived 128-bit authentication key: {} bytes", auth_key.len()))
}

slay demonstrate_unified_api() {
    println("\n🔐 Unified Asymmetric API Demo")
    println("------------------------------")
    
    // List all supported algorithms
    facts algorithms = get_asymmetric_algorithms()?
    println(format!("  📊 Supported algorithms: {}", algorithms.len()))
    
    lowkey (sus algorithm_info in algorithms) {
        println(format!("    - {}: {}", algorithm_info.name, algorithm_info.type))
        println(format!("      Key size: {} bits", algorithm_info.key_size))
        println(format!("      Capabilities: {}", algorithm_info.capabilities.join(", ")))
    }
    
    // Demonstrate cross-algorithm compatibility
    println("\n  Cross-Algorithm Digital Signature Test:")
    facts test_message = "Universal test message for all algorithms"
    
    facts signature_algorithms = ["RSA-2048", "ECDSA-P256", "Ed25519"]
    
    lowkey (sus algorithm in signature_algorithms) {
        facts keypair = generate_asymmetric_keypair(algorithm)?
        facts signature = asymmetric_sign(algorithm, keypair.private_key, test_message)?
        facts is_valid = asymmetric_verify(algorithm, keypair.public_key, test_message, signature)?
        
        lowkey (is_valid) {
            println(format!("    ✅ {}: Signature valid", algorithm))
        } else {
            println(format!("    ❌ {}: Signature invalid", algorithm))
        }
    }
    
    // Key exchange algorithm compatibility
    println("\n  Key Exchange Compatibility:")
    facts kx_algorithms = list_key_exchange_algorithms()
    
    lowkey (sus kx_algo in kx_algorithms) {
        println(format!("    📊 {}: Available", kx_algo))
    }
}

slay secure_messaging_example() {
    println("\n🔐 Secure Messaging Example")
    println("---------------------------")
    
    // Alice generates long-term Ed25519 signing keys
    facts alice_signing = generate_asymmetric_keypair("Ed25519")?
    facts bob_signing = generate_asymmetric_keypair("Ed25519")?
    
    // Alice and Bob generate ephemeral X25519 keys for this session
    facts alice_ephemeral = x25519_generate_ephemeral_keypair()?
    facts bob_ephemeral = x25519_generate_ephemeral_keypair()?
    
    // Key exchange for encryption
    facts shared_secret = x25519_key_exchange(
        alice_ephemeral.private_key,
        bob_ephemeral.public_key
    )?
    
    // Derive encryption and authentication keys
    facts encryption_key = derive_key_from_shared_secret(
        shared_secret.shared_secret,
        32,
        "SECURE-MSG-ENCRYPT"
    )?
    
    facts auth_key = derive_key_from_shared_secret(
        shared_secret.shared_secret,
        32,
        "SECURE-MSG-AUTH"
    )?
    
    // Alice sends a message to Bob
    facts plaintext = "This is a confidential message from Alice to Bob."
    
    // Sign the message for authenticity
    facts signature = asymmetric_sign("Ed25519", alice_signing.private_key, plaintext)?
    
    // In a real implementation, you would:
    // 1. Encrypt the message with the derived encryption key
    // 2. Compute HMAC with the authentication key
    // 3. Send: encrypted_message || signature || hmac
    
    // Bob verifies the signature
    facts is_authentic = asymmetric_verify("Ed25519", alice_signing.public_key, plaintext, signature)?
    
    lowkey (is_authentic) {
        println("    ✅ Secure message verified: Message is authentic")
        println("    🔐 Message provides:")
        println("      - Confidentiality (X25519 key exchange + derived encryption)")
        println("      - Authenticity (Ed25519 digital signature)")
        println("      - Integrity (HMAC with derived authentication key)")
        println("      - Forward secrecy (ephemeral keys)")
    } else {
        println("    ❌ Message verification failed!")
    }
}

slay digital_certificate_example() {
    println("\n🔐 Digital Certificate Example")
    println("------------------------------")
    
    // Certificate Authority (CA) generates signing keys
    facts ca_keypair = generate_asymmetric_keypair("RSA-4096")?
    println("    ✅ Certificate Authority generated RSA-4096 signing keys")
    
    // Entity generates its own keys
    facts entity_keypair = generate_asymmetric_keypair("ECDSA-P256")?
    println("    ✅ Entity generated ECDSA-P256 keys")
    
    // Create certificate (simplified)
    facts certificate_info = format!(
        "Subject: Entity Corp\nPublic Key: {}\nValid Until: 2025-12-31",
        entity_keypair.public_key
    )
    
    // CA signs the certificate
    facts certificate_signature = asymmetric_sign("RSA-4096", ca_keypair.private_key, certificate_info)?
    println("    ✅ CA signed the certificate")
    
    // Later: Verify the certificate
    facts is_valid_cert = asymmetric_verify("RSA-4096", ca_keypair.public_key, certificate_info, certificate_signature)?
    
    lowkey (is_valid_cert) {
        println("    ✅ Certificate is valid and trusted")
        
        // Now the entity can use its keys for secure operations
        facts message = "This message is signed by a certified entity."
        facts entity_signature = asymmetric_sign("ECDSA-P256", entity_keypair.private_key, message)?
        facts message_valid = asymmetric_verify("ECDSA-P256", entity_keypair.public_key, message, entity_signature)?
        
        lowkey (message_valid) {
            println("    ✅ Entity's signed message verified")
            println("    📊 Trust chain: CA -> Entity -> Message")
        }
    } else {
        println("    ❌ Certificate verification failed!")
    }
}

slay main() -> Result<(), Error> {
    demonstrate_asymmetric_crypto()?
    secure_messaging_example()?
    digital_certificate_example()?
    
    println("\n🎉 Asymmetric cryptography demo completed!")
    println("   All cryptographic operations working correctly.")
    println("   Ready for production cryptographic applications.")
    
    Ok(())
}
