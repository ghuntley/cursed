// fr fr CURSED Crypto Showcase - all the security bestie
// This example demonstrates the complete crypto package suite periodt

use crypto_advanced::{AesGcm256, ChaCha20Poly1305, SecurityLevel}
use crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm}
use crypto_signatures::{DigitalSignature, SignatureVerification}
use crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac}
use crypto_random::{fill_random, CryptographicRng}
use crypto_kdf::{pbkdf2_derive, argon2_derive}
use crypto_zk::{ZkProofSystem, ZkProof}
use crypto_pqc::{assess_quantum_threat, QuantumThreatLevel}

fn main() {
    // Initialize all crypto packages bestie
    crypto_advanced::init_crypto_advanced()?
    crypto_asymmetric::init_crypto_asymmetric()?
    crypto_signatures::init_crypto_signatures()?
    crypto_hash_advanced::init_crypto_hash_advanced()?
    crypto_zk::init_crypto_zk()?
    crypto_pqc::init_crypto_pqc()?
    
    print("🔒 CURSED Crypto Showcase - Maximum Security Edition")
    print("==================================================")
    
    // 1. Symmetric Encryption Demo
    print("\n1. Symmetric Encryption with AES-GCM-256")
    symmetric_encryption_demo()
    
    // 2. Asymmetric Encryption Demo  
    print("\n2. Asymmetric Encryption with RSA & ECC")
    asymmetric_encryption_demo()
    
    // 3. Digital Signatures Demo
    print("\n3. Digital Signatures with Ed25519")
    digital_signatures_demo()
    
    // 4. Cryptographic Hashing Demo
    print("\n4. Advanced Cryptographic Hashing")
    hashing_demo()
    
    // 5. Key Derivation Demo
    print("\n5. Secure Key Derivation Functions")
    key_derivation_demo()
    
    // 6. Random Number Generation Demo
    print("\n6. Cryptographically Secure Random Numbers")
    random_generation_demo()
    
    // 7. Post-Quantum Cryptography Demo
    print("\n7. Post-Quantum Cryptography Assessment")
    post_quantum_demo()
    
    // 8. Zero-Knowledge Proofs Demo
    print("\n8. Zero-Knowledge Proofs")
    zero_knowledge_demo()
    
    print("\n🎉 Crypto showcase complete - security maximized bestie!")
}

fn symmetric_encryption_demo() {
    print("   Generating 256-bit encryption key...")
    
    sus key = make Vec<u8> with length 32
    fill_random(&mut key)?
    
    facts plaintext = "slay this secret message needs protection periodt"
    print(f"   Original message: '{plaintext}'")
    
    // AES-GCM-256 encryption
    sus aes_cipher = AesGcm256::new(&key)?
    sus encrypted = aes_cipher.encrypt(plaintext.as_bytes())?
    print(f"   AES-GCM encrypted length: {encrypted.len()} bytes")
    
    sus decrypted = aes_cipher.decrypt(&encrypted)?
    sus decrypted_text = String::from_utf8(decrypted)?
    print(f"   Decrypted message: '{decrypted_text}'")
    
    assert_eq!(plaintext, decrypted_text)
    
    // ChaCha20-Poly1305 encryption
    sus chacha_cipher = ChaCha20Poly1305::new(&key)?
    sus chacha_encrypted = chacha_cipher.encrypt(plaintext.as_bytes())?
    print(f"   ChaCha20-Poly1305 encrypted length: {chacha_encrypted.len()} bytes")
    
    sus chacha_decrypted = chacha_cipher.decrypt(&chacha_encrypted)?
    sus chacha_text = String::from_utf8(chacha_decrypted)?
    print(f"   ChaCha20 decrypted: '{chacha_text}'")
    
    assert_eq!(plaintext, chacha_text)
    print("   ✅ Symmetric encryption working perfectly!")
}

fn asymmetric_encryption_demo() {
    print("   Generating RSA-2048 key pair...")
    
    sus rsa_keypair = KeyGenerator::generate_rsa_keypair(2048)?
    print("   RSA key pair generated successfully")
    
    facts message = "asymmetric encryption test message bestie"
    print(f"   Message to encrypt: '{message}'")
    
    // RSA encryption
    sus rsa_encrypted = rsa_keypair.public_key().encrypt(message.as_bytes())?
    print(f"   RSA encrypted length: {rsa_encrypted.len()} bytes")
    
    sus rsa_decrypted = rsa_keypair.private_key().decrypt(&rsa_encrypted)?
    sus rsa_text = String::from_utf8(rsa_decrypted)?
    print(f"   RSA decrypted: '{rsa_text}'")
    
    assert_eq!(message, rsa_text)
    
    // Elliptic Curve key generation
    print("   Generating EC P-256 key pair...")
    sus ec_keypair = KeyGenerator::generate_ec_keypair("P-256")?
    print("   EC key pair generated successfully")
    
    // Key exchange demonstration
    sus ec_keypair2 = KeyGenerator::generate_ec_keypair("P-256")?
    sus shared_secret1 = ec_keypair.key_exchange(&ec_keypair2.public_key())?
    sus shared_secret2 = ec_keypair2.key_exchange(&ec_keypair.public_key())?
    
    assert_eq!(shared_secret1, shared_secret2)
    print("   ✅ Key exchange successful - shared secrets match!")
}

fn digital_signatures_demo() {
    print("   Generating Ed25519 signing key pair...")
    
    sus keypair = KeyGenerator::generate_ed25519_keypair()?
    print("   Ed25519 key pair generated")
    
    facts document = "important document that needs digital signature periodt"
    print(f"   Document to sign: '{document}'")
    
    // Create digital signature
    sus signature = keypair.sign(document.as_bytes())?
    print(f"   Digital signature created (length: {signature.len()} bytes)")
    
    // Verify signature
    sus is_valid = keypair.verify(document.as_bytes(), &signature)?
    print(f"   Signature verification: {is_valid}")
    assert!(is_valid)
    
    // Test with tampered document
    facts tampered_doc = "tampered document that should fail verification"
    sus tampered_valid = keypair.verify(tampered_doc.as_bytes(), &signature)?
    print(f"   Tampered document verification: {tampered_valid}")
    assert!(!tampered_valid)
    
    print("   ✅ Digital signatures working correctly!")
}

fn hashing_demo() {
    facts test_data = "data to hash with various algorithms bestie"
    print(f"   Data to hash: '{test_data}'")
    
    // SHA-256
    sus sha256_hash = hash_with_algorithm(test_data.as_bytes(), AdvancedHashAlgorithm::Sha256)?
    print(f"   SHA-256: {hex_encode(&sha256_hash)}")
    
    // SHA-3-256
    sus sha3_hash = hash_with_algorithm(test_data.as_bytes(), AdvancedHashAlgorithm::Sha3_256)?
    print(f"   SHA-3-256: {hex_encode(&sha3_hash)}")
    
    // BLAKE3
    sus blake3_hash = hash_with_algorithm(test_data.as_bytes(), AdvancedHashAlgorithm::Blake3)?
    print(f"   BLAKE3: {hex_encode(&blake3_hash)}")
    
    // HMAC demonstration
    facts hmac_key = "secret_hmac_key_bestie"
    sus hmac = compute_hmac(test_data.as_bytes(), hmac_key.as_bytes(), AdvancedHashAlgorithm::Sha256)?
    print(f"   HMAC-SHA256: {hex_encode(&hmac)}")
    
    print("   ✅ All hash algorithms working perfectly!")
}

fn key_derivation_demo() {
    facts password = "user_password_bestie"
    facts salt = "random_salt_12345"
    facts key_length = 32
    
    print(f"   Deriving keys from password: '{password}'")
    print(f"   Using salt: '{salt}'")
    
    // PBKDF2 key derivation
    sus pbkdf2_key = pbkdf2_derive(password.as_bytes(), salt.as_bytes(), 100000, key_length)?
    print(f"   PBKDF2 (100k iter): {hex_encode(&pbkdf2_key)}")
    
    // Argon2 key derivation
    sus argon2_key = argon2_derive(password.as_bytes(), salt.as_bytes(), key_length)?
    print(f"   Argon2: {hex_encode(&argon2_key)}")
    
    // scrypt key derivation
    sus scrypt_key = scrypt_derive(password.as_bytes(), salt.as_bytes(), key_length)?
    print(f"   scrypt: {hex_encode(&scrypt_key)}")
    
    // Verify different algorithms produce different keys
    assert_ne!(pbkdf2_key, argon2_key)
    assert_ne!(pbkdf2_key, scrypt_key)
    assert_ne!(argon2_key, scrypt_key)
    
    print("   ✅ Key derivation functions working correctly!")
}

fn random_generation_demo() {
    print("   Generating cryptographically secure random data...")
    
    // Generate various sizes of random data
    sus random_16 = make Vec<u8> with length 16
    fill_random(&mut random_16)?
    print(f"   Random 16 bytes: {hex_encode(&random_16)}")
    
    sus random_32 = make Vec<u8> with length 32
    fill_random(&mut random_32)?
    print(f"   Random 32 bytes: {hex_encode(&random_32)}")
    
    // Generate multiple samples to verify uniqueness
    sus sample1 = make Vec<u8> with length 16
    sus sample2 = make Vec<u8> with length 16
    sus sample3 = make Vec<u8> with length 16
    
    fill_random(&mut sample1)?
    fill_random(&mut sample2)?
    fill_random(&mut sample3)?
    
    assert_ne!(sample1, sample2)
    assert_ne!(sample2, sample3)
    assert_ne!(sample1, sample3)
    
    print("   ✅ Random number generation working perfectly!")
}

fn post_quantum_demo() {
    print("   Assessing quantum threat level...")
    
    sus threat_level = assess_quantum_threat()?
    print(f"   Current quantum threat: {threat_level}")
    
    vibe_check threat_level {
        mood QuantumThreatLevel::None => {
            print("   Classical cryptography is still secure")
        }
        mood QuantumThreatLevel::Emerging => {
            print("   Start planning post-quantum migration")
        }
        mood QuantumThreatLevel::Significant => {
            print("   Post-quantum crypto should be prioritized")
        }
        mood QuantumThreatLevel::Catastrophic => {
            print("   Immediate post-quantum crypto required!")
        }
        basic => {
            print("   Unknown threat level")
        }
    }
    
    print("   ✅ Post-quantum assessment complete!")
}

fn zero_knowledge_demo() {
    print("   Demonstrating zero-knowledge proofs...")
    
    // Create a simple ZK proof system
    sus zk_system = ZkProofSystem::new("simple_range_proof")?
    
    // Secret value (age = 25)
    facts secret_age = 25
    
    // Generate proof that age is >= 18 without revealing actual age
    sus proof = zk_system.generate_proof(secret_age, "age_over_18")?
    print("   Generated ZK proof that age >= 18 (without revealing age)")
    
    // Verify the proof
    sus is_valid = zk_system.verify_proof(&proof, "age_over_18")?
    print(f"   ZK proof verification: {is_valid}")
    assert!(is_valid)
    
    print("   ✅ Zero-knowledge proofs working!")
}

// Helper function to encode bytes as hex
fn hex_encode(data: &[u8]) -> String {
    sus result = String::new()
    for byte in data {
        result.push_str(&format!("{:02x}", byte))
    }
    result
}
