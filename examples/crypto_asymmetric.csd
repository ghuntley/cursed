fr fr/ fr fr CURSED Asymmetric Cryptography Examples - secure communication periodt
fr fr/ 
fr fr/ This example demonstrates how to use the CURSED asymmetric crypto library
fr fr/ for key generation, encryption, digital signatures, and certificate handling.

fr fr Import crypto modules
yeet crypto.asymmetric as asymm
yeet crypto.certificates as certs
yeet stdlib.print as print
yeet stdlib.string as str

fr fr/ slay Main function demonstrating asymmetric crypto
slay main() {
    print("🔐 CURSED Asymmetric Crypto Examples - let's get secure bestie!")
    
    // Demonstrate RSA operations
    rsa_examples()
    
    // Demonstrate ECDSA operations  
    ecdsa_examples()
    
    // Demonstrate key exchange
    key_exchange_examples()
    
    // Demonstrate modern crypto (Ed25519/X25519)
    modern_crypto_examples()
    
    // Demonstrate certificate handling
    certificate_examples()
    
    // Demonstrate secure communication
    secure_communication_example()
    
    print("✨ All crypto examples completed successfully - maximum security achieved periodt!")
}

fr fr/ slay RSA encryption and signing examples
slay rsa_examples() {
    print("\n🔑 RSA Cryptography Examples")
    print("=" * 40)
    
    // Generate RSA key pair (4096-bit for maximum security)
    facts rsa_keypair = asymm.rsa_generate_keypair(4096)
    print(f"✅ Generated RSA-4096 key pair")
    print(f"   Public key size: {rsa_keypair.public_key.key_size} bits")
    print(f"   Private key size: {rsa_keypair.private_key.key_size} bits")
    
    // Encrypt a message
    facts message = "This is a secret message that needs RSA encryption bestie!"
    facts encrypted = asymm.rsa_encrypt(rsa_keypair.public_key, message, "OAEP-SHA256")
    print(f"✅ Encrypted message length: {encrypted.length} bytes")
    
    // Decrypt the message
    facts decrypted = asymm.rsa_decrypt(rsa_keypair.private_key, encrypted, "OAEP-SHA256")
    print(f"✅ Decrypted message: {decrypted}")
    
    // Create digital signature
    facts document = "Important document that needs digital signature"
    facts signature = asymm.rsa_sign(rsa_keypair.private_key, document, "PSS")
    print(f"✅ Created RSA signature length: {signature.length} bytes")
    
    // Verify signature
    facts is_valid = asymm.rsa_verify(rsa_keypair.public_key, document, signature, "PSS")
    lowkey (is_valid) {
        print("✅ RSA signature verification: VALID")
    } highkey {
        print("❌ RSA signature verification: INVALID")
    }
}

fr fr/ slay ECDSA signing and verification examples
slay ecdsa_examples() {
    print("\n🔵 ECDSA Cryptography Examples")
    print("=" * 40)
    
    // Generate ECDSA key pairs for different curves
    facts curves = ["P-256", "P-384", "P-521"]
    
    periodt curve in curves {
        print(f"\n📋 Testing curve: {curve}")
        
        facts keypair = asymm.ecdsa_generate_keypair(curve)
        print(f"✅ Generated ECDSA-{curve} key pair")
        print(f"   Security level: {keypair.curve.security_level} bits")
        
        // Sign a message
        facts message = f"ECDSA signature test with {curve} curve"
        facts signature = asymm.ecdsa_sign(keypair.private_key, message)
        print(f"✅ Created ECDSA signature (r: {signature.r.length}, s: {signature.s.length})")
        
        // Verify signature
        facts is_valid = asymm.ecdsa_verify(keypair.public_key, message, signature)
        lowkey (is_valid) {
            print(f"✅ ECDSA-{curve} signature verification: VALID")
        } highkey {
            print(f"❌ ECDSA-{curve} signature verification: INVALID")
        }
    }
}

fr fr/ slay Key exchange examples (ECDH)
slay key_exchange_examples() {
    print("\n🤝 Key Exchange Examples")
    print("=" * 40)
    
    print("📋 ECDH Key Exchange Simulation (Alice and Bob)")
    
    // Alice generates her key pair
    facts alice_keypair = asymm.ecdh_generate_keypair("P-256")
    print("✅ Alice generated ECDH-P256 key pair")
    
    // Bob generates his key pair
    facts bob_keypair = asymm.ecdh_generate_keypair("P-256") 
    print("✅ Bob generated ECDH-P256 key pair")
    
    // Alice computes shared secret using Bob's public key
    facts alice_shared = asymm.ecdh_exchange(alice_keypair.private_key, bob_keypair.public_key)
    print(f"✅ Alice computed shared secret: {alice_shared.length} bytes")
    
    // Bob computes shared secret using Alice's public key
    facts bob_shared = asymm.ecdh_exchange(bob_keypair.private_key, alice_keypair.public_key)
    print(f"✅ Bob computed shared secret: {bob_shared.length} bytes")
    
    // Verify shared secrets match
    lowkey (alice_shared == bob_shared) {
        print("✅ ECDH key exchange successful - shared secrets match!")
    } highkey {
        print("❌ ECDH key exchange failed - shared secrets don't match!")
    }
}

fr fr/ slay Modern cryptography examples (X25519 and Ed25519)
slay modern_crypto_examples() {
    print("\n🚀 Modern Cryptography Examples")
    print("=" * 40)
    
    // X25519 key exchange
    print("📋 X25519 Key Exchange")
    facts alice_x25519 = asymm.x25519_generate_keypair()
    facts bob_x25519 = asymm.x25519_generate_keypair()
    print("✅ Generated X25519 key pairs for Alice and Bob")
    
    facts alice_x25519_shared = asymm.x25519_exchange(alice_x25519.private_key, bob_x25519.public_key)
    facts bob_x25519_shared = asymm.x25519_exchange(bob_x25519.private_key, alice_x25519.public_key)
    
    lowkey (alice_x25519_shared == bob_x25519_shared) {
        print("✅ X25519 key exchange successful!")
        print(f"   Shared secret: {alice_x25519_shared.length} bytes")
    } highkey {
        print("❌ X25519 key exchange failed!")
    }
    
    // Ed25519 digital signatures
    print("\n📋 Ed25519 Digital Signatures")
    facts ed25519_keypair = asymm.ed25519_generate_keypair()
    print("✅ Generated Ed25519 key pair")
    
    facts ed_message = "Ed25519 signature test - modern crypto periodt!"
    facts ed_signature = asymm.ed25519_sign(ed25519_keypair.private_key, ed_message)
    print(f"✅ Created Ed25519 signature: {ed_signature.length} bytes")
    
    facts ed_valid = asymm.ed25519_verify(ed25519_keypair.public_key, ed_message, ed_signature)
    lowkey (ed_valid) {
        print("✅ Ed25519 signature verification: VALID")
    } highkey {
        print("❌ Ed25519 signature verification: INVALID")
    }
}

fr fr/ slay Certificate handling examples
slay certificate_examples() {
    print("\n📜 Certificate Handling Examples")
    print("=" * 40)
    
    // Example X.509 certificate in PEM format
    facts cert_pem = """-----BEGIN CERTIFICATE-----
MIICijCCAXICCQCKuukrJSISpjANBgkqhkiG9w0BAQsFADA0MQswCQYDVQQGEwJV
UzETMBEGA1UECAwKQ2FsaWZvcm5pYTEQMA4GA1UEBwwHU3RhbmZvcmQwHhcNMjMw
MTAxMDAwMDAwWhcNMjQwMTAxMDAwMDAwWjA0MQswCQYDVQQGEwJVUzETMBEGA1UE
CAwKQ2FsaWZvcm5pYTEQMA4GA1UEBwwHU3RhbmZvcmQwXDANBgkqhkiG9w0BAQEF
AANLADBIAkEAyZ5BaFhZsOAoY8gzH9i8K2vKJBjOVoE5W9w+WOQjJKR8I3VZqE5U
8k6rGzYvZ5ZsKZyXSzCwGqOKjOpH9fZHZQIDAQABMA0GCSqGSIb3DQEBCwUAA0EA
K9G6Yc5U3D8+xH5DwZ4fKjX7vN5sGz9k7Wx2ZJzF8jR5qE3vZxD2QqK5BzXsGnO
-----END CERTIFICATE-----"""
    
    // Parse certificate
    facts cert = certs.parse_certificate_pem(cert_pem)
    print("✅ Parsed X.509 certificate from PEM")
    print(f"   Subject: {cert.subject}")
    print(f"   Issuer: {cert.issuer}")
    print(f"   Serial Number: {cert.serial_number}")
    print(f"   Version: {cert.version}")
    
    // Validate certificate
    facts validation_result = certs.validate_certificate(cert, "example.com")
    lowkey (validation_result.valid) {
        print("✅ Certificate validation: VALID")
    } highkey {
        print(f"❌ Certificate validation: INVALID - {validation_result.error}")
    }
    
    // Get certificate fingerprint
    facts fingerprint = certs.get_certificate_fingerprint(cert)
    print(f"✅ Certificate SHA-256 fingerprint: {fingerprint}")
    
    // Convert between PEM and DER formats
    facts der_data = certs.pem_to_der(cert_pem)
    print(f"✅ Converted PEM to DER: {der_data.length} bytes")
    
    facts converted_pem = certs.der_to_pem(der_data)
    print("✅ Converted DER back to PEM format")
    
    // Example CSR parsing
    facts csr_pem = """-----BEGIN CERTIFICATE REQUEST-----
MIICVjCCAT4CAQAwEzERMA8GA1UEAwwIZXhhbXBsZS5jb20wggEiMA0GCSqGSIb3
DQEBAQUAA4IBDwAwggEKAoIBAQDJnkFoWFmw4ChjyDMf2LwqasokkM5WgTlb3D5Y
5CMkpHwjdVmoTlTyTqsbNi9nlmwpnJdLMLAao4qM6kf19kdlAgMBAAGgADANBgkq
hkiG9w0BAQsFAAOCAQEAyGfLOZzP0DdGY6zMQd5v2sR3d5FGjVGdYoGjBSs9LoNG
o3FdMbqvB5E7RKZzuU8bN5R9LD5MkHy7UyOPxJ3G8vNx5Q5s8YaVw3rZfWnKJ7Qx
-----END CERTIFICATE REQUEST-----"""
    
    facts csr = certs.parse_csr_pem(csr_pem)
    print("✅ Parsed Certificate Signing Request (CSR)")
    print(f"   Subject: {csr.subject}")
    print(f"   Algorithm: {csr.algorithm}")
}

fr fr/ slay Secure communication example
slay secure_communication_example() {
    print("\n💬 Secure Communication Example")
    print("=" * 40)
    
    print("📋 Simulating secure communication between Alice and Bob")
    
    // 1. Key Exchange Phase
    print("\n🔄 Phase 1: Key Exchange")
    facts alice_x25519 = asymm.x25519_generate_keypair()
    facts bob_x25519 = asymm.x25519_generate_keypair()
    
    // Exchange public keys and compute shared secret
    facts shared_secret = asymm.x25519_exchange(alice_x25519.private_key, bob_x25519.public_key)
    print(f"✅ Established shared secret: {shared_secret.length} bytes")
    
    // 2. Digital Identity Phase (Ed25519 signing keys)
    print("\n🆔 Phase 2: Digital Identity Establishment")
    facts alice_signing = asymm.ed25519_generate_keypair()
    facts bob_signing = asymm.ed25519_generate_keypair()
    print("✅ Generated signing key pairs for authentication")
    
    // 3. Secure Message Exchange
    print("\n📩 Phase 3: Secure Message Exchange")
    
    // Alice sends a signed message to Bob
    facts alice_message = "Hello Bob! This is Alice sending a secure message periodt!"
    facts alice_signature = asymm.ed25519_sign(alice_signing.private_key, alice_message)
    
    print(f"📤 Alice sent: '{alice_message}'")
    print(f"   Signature length: {alice_signature.length} bytes")
    
    // Bob verifies Alice's message
    facts alice_verified = asymm.ed25519_verify(alice_signing.public_key, alice_message, alice_signature)
    lowkey (alice_verified) {
        print("✅ Bob verified Alice's message authenticity")
    } highkey {
        print("❌ Bob failed to verify Alice's message")
    }
    
    // Bob responds with a signed message
    facts bob_message = "Hi Alice! Message received and verified - secure comms working bestie!"
    facts bob_signature = asymm.ed25519_sign(bob_signing.private_key, bob_message)
    
    print(f"📤 Bob replied: '{bob_message}'")
    print(f"   Signature length: {bob_signature.length} bytes")
    
    // Alice verifies Bob's response
    facts bob_verified = asymm.ed25519_verify(bob_signing.public_key, bob_message, bob_signature)
    lowkey (bob_verified) {
        print("✅ Alice verified Bob's response authenticity")
    } highkey {
        print("❌ Alice failed to verify Bob's response")
    }
    
    // 4. Hybrid Encryption Example (RSA + AES simulation)
    print("\n🔐 Phase 4: Hybrid Encryption Simulation")
    
    // Generate RSA key pair for Alice
    facts alice_rsa = asymm.rsa_generate_keypair(2048)
    
    // Simulate AES key (would use symmetric crypto in practice)
    facts aes_key = "simulated_aes_key_32_bytes_long!!"
    
    // Encrypt AES key with RSA
    facts encrypted_aes_key = asymm.rsa_encrypt(alice_rsa.public_key, aes_key, "OAEP-SHA256")
    print(f"✅ Encrypted AES key with RSA: {encrypted_aes_key.length} bytes")
    
    // Decrypt AES key
    facts decrypted_aes_key = asymm.rsa_decrypt(alice_rsa.private_key, encrypted_aes_key, "OAEP-SHA256")
    print(f"✅ Decrypted AES key: {decrypted_aes_key.length} bytes")
    
    lowkey (aes_key == decrypted_aes_key) {
        print("✅ Hybrid encryption simulation successful!")
    } highkey {
        print("❌ Hybrid encryption simulation failed!")
    }
}

fr fr/ slay Certificate Authority simulation
slay certificate_authority_example() {
    print("\n🏛️ Certificate Authority Simulation")
    print("=" * 40)
    
    // Generate CA key pair
    facts ca_keypair = asymm.rsa_generate_keypair(4096)
    print("✅ Generated Certificate Authority RSA-4096 key pair")
    
    // Generate end-entity key pair
    facts entity_keypair = asymm.rsa_generate_keypair(2048)
    print("✅ Generated end-entity RSA-2048 key pair")
    
    // Create certificate signing request
    facts csr_data = squad {
        subject: "CN=example.com,O=Example Corp,C=US",
        public_key: entity_keypair.public_key,
        extensions: ["digitalSignature", "keyEncipherment"]
    }
    
    print("✅ Created certificate signing request")
    print(f"   Subject: {csr_data.subject}")
    
    // CA signs the certificate (simulation)
    facts certificate_data = f"Certificate for {csr_data.subject}"
    facts ca_signature = asymm.rsa_sign(ca_keypair.private_key, certificate_data, "PSS")
    
    print("✅ CA signed the certificate")
    print(f"   Signature length: {ca_signature.length} bytes")
    
    // Verify certificate signature
    facts cert_valid = asymm.rsa_verify(ca_keypair.public_key, certificate_data, ca_signature, "PSS")
    lowkey (cert_valid) {
        print("✅ Certificate signature verification: VALID")
    } highkey {
        print("❌ Certificate signature verification: INVALID")
    }
}

fr fr/ slay Performance benchmarks
slay performance_benchmarks() {
    print("\n⚡ Performance Benchmarks")
    print("=" * 40)
    
    // Benchmark key generation
    print("📊 Key Generation Benchmarks:")
    
    facts algorithms = [
        ("RSA-2048", () => asymm.rsa_generate_keypair(2048)),
        ("RSA-4096", () => asymm.rsa_generate_keypair(4096)), 
        ("ECDSA-P256", () => asymm.ecdsa_generate_keypair("P-256")),
        ("ECDSA-P384", () => asymm.ecdsa_generate_keypair("P-384")),
        ("X25519", () => asymm.x25519_generate_keypair()),
        ("Ed25519", () => asymm.ed25519_generate_keypair()),
    ]
    
    periodt (name, generator) in algorithms {
        facts start_time = time.now()
        facts keypair = generator()
        facts end_time = time.now()
        facts duration = end_time - start_time
        
        print(f"   {name}: {duration}ms")
    }
    
    // Benchmark signature operations
    print("\n📊 Signature Benchmarks:")
    
    facts test_message = "Performance test message for signature benchmarks"
    facts ecdsa_keypair = asymm.ecdsa_generate_keypair("P-256")
    facts ed25519_keypair = asymm.ed25519_generate_keypair()
    
    // ECDSA signing benchmark
    facts ecdsa_start = time.now()
    periodt i in 0..10 {
        facts sig = asymm.ecdsa_sign(ecdsa_keypair.private_key, test_message)
    }
    facts ecdsa_time = (time.now() - ecdsa_start) / 10
    print(f"   ECDSA-P256 signing (avg): {ecdsa_time}ms")
    
    // Ed25519 signing benchmark
    facts ed25519_start = time.now()
    periodt i in 0..10 {
        facts sig = asymm.ed25519_sign(ed25519_keypair.private_key, test_message)
    }
    facts ed25519_time = (time.now() - ed25519_start) / 10
    print(f"   Ed25519 signing (avg): {ed25519_time}ms")
}

fr fr/ slay Error handling examples
slay error_handling_examples() {
    print("\n🚨 Error Handling Examples")
    print("=" * 40)
    
    // Test invalid key sizes
    bestie {
        facts invalid_rsa = asymm.rsa_generate_keypair(1024) // Too small
        print("❌ Should not reach here - invalid key size accepted")
    } flex error {
        print(f"✅ Correctly caught invalid RSA key size error: {error}")
    }
    
    // Test invalid curve
    bestie {
        facts invalid_ecdsa = asymm.ecdsa_generate_keypair("INVALID-CURVE")
        print("❌ Should not reach here - invalid curve accepted")
    } flex error {
        print(f"✅ Correctly caught invalid curve error: {error}")
    }
    
    // Test signature verification with wrong key
    facts alice_keypair = asymm.ed25519_generate_keypair()
    facts bob_keypair = asymm.ed25519_generate_keypair()
    
    facts message = "Test message"
    facts alice_signature = asymm.ed25519_sign(alice_keypair.private_key, message)
    
    // Try to verify Alice's signature with Bob's public key
    facts wrong_verification = asymm.ed25519_verify(bob_keypair.public_key, message, alice_signature)
    lowkey (!wrong_verification) {
        print("✅ Correctly rejected signature verification with wrong public key")
    } highkey {
        print("❌ Incorrectly accepted signature with wrong public key")
    }
}

fr fr Run the main function when the script is executed
main()
