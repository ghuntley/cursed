/// fr fr Comprehensive demonstration of asymmetric cryptography operations
/// 
/// This example showcases all the asymmetric crypto capabilities including:
/// - RSA encryption/decryption and digital signatures
/// - ECDSA signatures with multiple curves
/// - Ed25519 high-performance signatures  
/// - X25519 elliptic curve Diffie-Hellman key exchange
/// - Key serialization and management

import "stdlib::packages::crypto_asymmetric";

/// slay Main demo function
func main() {
    periodt {
        demo_rsa_operations();
        demo_ecdsa_operations(); 
        demo_ed25519_operations();
        demo_x25519_key_exchange();
        demo_unified_key_generation();
        demo_security_comparison();
    } bestie error {
        println("Demo failed: " + error.message);
    }
}

/// slay Demonstrate RSA encryption, decryption, and signatures
func demo_rsa_operations() {
    println("\n🔐 RSA Cryptography Demo");
    println("========================");
    
    // Generate RSA-2048 key pair
    sus rsa_keys = rsa_generate_keypair(2048);
    println("Generated RSA-2048 key pair");
    println("Algorithm: " + rsa_keys.algorithm);
    println("Key size: " + rsa_keys.key_size + " bits");
    
    // Encryption and decryption demo
    sus message = "Hello, RSA encryption with OAEP padding!";
    println("\nOriginal message: " + message);
    
    sus encrypted = rsa_encrypt(rsa_keys.public_key_pem, message, "OAEP-SHA256");
    println("Encrypted (base64): " + encrypted);
    
    sus decrypted = rsa_decrypt(rsa_keys.private_key_pem, encrypted, "OAEP-SHA256");
    println("Decrypted: " + decrypted);
    
    // Digital signature demo
    sus signature_message = "Document to be signed with RSA-PSS";
    println("\nSigning message: " + signature_message);
    
    sus signature = rsa_sign(rsa_keys.private_key_pem, signature_message, "PSS-SHA256");
    println("Signature (base64): " + signature);
    
    sus verified = rsa_verify(rsa_keys.public_key_pem, signature_message, signature, "PSS-SHA256");
    println("Signature verified: " + verified);
    
    // Test with tampered message
    sus tampered_message = "Tampered document";
    sus tampered_verified = rsa_verify(rsa_keys.public_key_pem, tampered_message, signature, "PSS-SHA256");
    println("Tampered message verified: " + tampered_verified);
}

/// slay Demonstrate ECDSA signatures with multiple curves
func demo_ecdsa_operations() {
    println("\n📊 ECDSA Signatures Demo");
    println("=========================");
    
    sus curves = ["P-256", "P-384", "P-521"];
    sus message = "ECDSA signature test message";
    
    for curve in curves {
        println("\n--- " + curve + " Curve ---");
        
        // Generate key pair for curve
        sus ecc_keys = ecc_generate_keypair(curve);
        println("Generated " + curve + " key pair");
        println("Algorithm: " + ecc_keys.algorithm);
        println("Curve: " + ecc_keys.curve);
        println("Key size: " + ecc_keys.key_size + " bits");
        
        // Sign message
        sus signature = ecdsa_sign(ecc_keys.private_key_pem, message, curve, "SHA-256");
        println("Signature (base64): " + signature);
        
        // Verify signature
        sus verified = ecdsa_verify(ecc_keys.public_key_pem, message, signature, curve, "SHA-256");
        println("Signature verified: " + verified);
        
        // Display public key coordinates
        println("Public X: " + ecc_keys.public_x);
        println("Public Y: " + ecc_keys.public_y);
    }
}

/// slay Demonstrate Ed25519 signatures
func demo_ed25519_operations() {
    println("\n✍️  Ed25519 Signatures Demo");
    println("============================");
    
    // Generate Ed25519 key pair
    sus ed_keys = ed25519_generate_keypair();
    println("Generated Ed25519 key pair");
    println("Algorithm: " + ed_keys.algorithm);
    println("Curve: " + ed_keys.curve);
    println("Key size: " + ed_keys.key_size + " bits (security level)");
    println("Public key (hex): " + ed_keys.public_key_hex);
    
    // Sign message
    sus message = "Ed25519 high-performance signature test";
    println("\nSigning message: " + message);
    
    sus signature = ed25519_sign(ed_keys.private_key_pem, message);
    println("Signature (base64): " + signature);
    
    // Verify with PEM key
    sus verified_pem = ed25519_verify(ed_keys.public_key_pem, message, signature);
    println("PEM verification: " + verified_pem);
    
    // Verify with raw key
    sus verified_raw = ed25519_verify_raw(ed_keys.public_key_hex, message, signature);
    println("Raw verification: " + verified_raw);
    
    // Deterministic key generation demo
    println("\n--- Deterministic Key Generation ---");
    sus seed = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    sus det_keys1 = ed25519_generate_keypair_from_seed(seed);
    sus det_keys2 = ed25519_generate_keypair_from_seed(seed);
    
    println("Deterministic key 1: " + det_keys1.public_key_hex);
    println("Deterministic key 2: " + det_keys2.public_key_hex);
    println("Keys match: " + (det_keys1.public_key_hex == det_keys2.public_key_hex));
    
    // Public key derivation demo
    sus private_key_hex = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
    sus derived_public = ed25519_derive_public_key(private_key_hex);
    println("Derived public key: " + derived_public);
}

/// slay Demonstrate X25519 key exchange
func demo_x25519_key_exchange() {
    println("\n🤝 X25519 Key Exchange Demo");
    println("============================");
    
    // Generate key pairs for Alice and Bob
    sus alice_keys = x25519_generate_keypair();
    sus bob_keys = x25519_generate_keypair();
    
    println("Alice's key pair:");
    println("  Algorithm: " + alice_keys.algorithm);
    println("  Public key: " + alice_keys.public_key_hex);
    
    println("\nBob's key pair:");
    println("  Algorithm: " + bob_keys.algorithm);
    println("  Public key: " + bob_keys.public_key_hex);
    
    // Perform key exchange from Alice's side
    sus alice_private = "a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0"; // Mock
    sus bob_public = bob_keys.public_key_hex;
    
    sus shared_secret_alice = x25519_key_exchange(alice_private, bob_public);
    println("\nAlice's shared secret:");
    println("  Hex: " + shared_secret_alice.shared_secret_hex);
    println("  Base64: " + shared_secret_alice.shared_secret_base64);
    println("  Length: " + shared_secret_alice.shared_secret_length + " bytes");
    
    // Demonstrate ephemeral key generation
    println("\n--- Ephemeral Key Generation ---");
    sus ephemeral = x25519_generate_ephemeral_keypair();
    println("Ephemeral key generated:");
    println("  Type: " + ephemeral.key_type);
    println("  Perfect forward secrecy: " + ephemeral.perfect_forward_secrecy);
    println("  Public key: " + ephemeral.public_key_hex);
    
    // Public key derivation and validation
    sus private_hex = "b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0";
    sus derived_public = x25519_derive_public_key(private_hex);
    println("\nDerived public key: " + derived_public);
    
    sus is_valid = x25519_validate_public_key(derived_public);
    println("Key validation: " + is_valid);
}

/// slay Demonstrate unified key generation across all algorithms
func demo_unified_key_generation() {
    println("\n🎯 Unified Key Generation Demo");
    println("===============================");
    
    // List all supported algorithms
    sus algorithms = list_asymmetric_algorithms();
    println("Supported algorithms:");
    
    for algo in algorithms {
        println("  " + algo.name + ":");
        println("    Key size: " + algo.key_size_bits + " bits");
        println("    Encryption: " + algo.supports_encryption);
        println("    Signing: " + algo.supports_signing);
        println("    Key exchange: " + algo.supports_key_exchange);
        println("");
    }
    
    // Generate keys for different algorithms
    sus test_algorithms = ["RSA-2048", "ECDSA-P256", "Ed25519", "X25519"];
    
    for algo_name in test_algorithms {
        println("--- " + algo_name + " ---");
        sus keys = generate_asymmetric_keypair(algo_name);
        println("Algorithm: " + keys.algorithm);
        println("Key size: " + keys.key_size_bits + " bits");
        println("Generated: " + keys.generation_time);
        
        lowkey (keys.has_private_key) {
            println("✅ Private key generated successfully");
        }
    }
}

/// slay Security and performance comparison
func demo_security_comparison() {
    println("\n🛡️  Security & Performance Comparison");
    println("======================================");
    
    println("Algorithm Security Levels:");
    println("RSA-2048    ≈ 112-bit security (legacy compatibility)");
    println("RSA-3072    ≈ 128-bit security (current standard)"); 
    println("RSA-4096    ≈ 150-bit security (high security)");
    println("ECDSA-P256  ≈ 128-bit security (efficient)");
    println("ECDSA-P384  ≈ 192-bit security (very high)");
    println("ECDSA-P521  ≈ 256-bit security (maximum)");
    println("Ed25519     ≈ 128-bit security (fastest signatures)");
    println("X25519      ≈ 128-bit security (fastest key exchange)");
    
    println("\nUse Case Recommendations:");
    println("🔐 General encryption: RSA-3072 with OAEP-SHA256");
    println("✍️  Digital signatures: Ed25519 (speed) or ECDSA-P256 (compatibility)");
    println("🤝 Key exchange: X25519 with ephemeral keys");
    println("🏛️  Long-term security: RSA-4096 or ECDSA-P521");
    println("⚡ Performance critical: Ed25519 signatures, X25519 key exchange");
    println("🌐 Web/TLS compatibility: ECDSA-P256, RSA-2048 (legacy)");
    
    println("\nPadding Scheme Recommendations:");
    println("RSA Encryption: OAEP-SHA256 (secure), avoid PKCS#1 v1.5");
    println("RSA Signatures: PSS-SHA256 (secure), PKCS#1 v1.5 for compatibility");
    println("ECDSA Hashing: SHA-256 minimum, SHA-384/SHA-512 for higher security");
    
    println("\nKey Management Best Practices:");
    println("• Store private keys encrypted at rest");
    println("• Use hardware security modules (HSMs) when possible");
    println("• Implement key rotation policies");
    println("• Use ephemeral keys for forward secrecy");
    println("• Validate all public keys before use");
    println("• Use cryptographically secure random number generation");
}
