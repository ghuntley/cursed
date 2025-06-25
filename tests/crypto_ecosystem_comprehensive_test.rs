/// Comprehensive test suite for the complete CURSED cryptographic ecosystem
/// Tests integration between all crypto modules and validates production readiness

use cursed::stdlib::packages::crypto_advanced::{
    memory_protection::*,
    constant_time::*,
    chacha20_poly1305::*,
    nonce_generator::*,
    authenticated_encryption::*,
    errors::*,
};

#[cfg(test)]
mod comprehensive_crypto_tests {
    use super::*;
    
    /// Test complete cryptographic workflow with memory protection
    #[test]
    fn test_complete_crypto_workflow() {
        // 1. Generate secure key using memory protection
        let key_data = vec![42u8; 32];
        let secure_key = SecureMemory::new(key_data).unwrap();
        
        // 2. Create ChaCha20-Poly1305 cipher
        let cipher = ChaCha20Poly1305::new(secure_key.as_bytes()).unwrap();
        
        // 3. Generate secure nonce
        let nonce = cipher.generate_nonce().unwrap();
        assert_eq!(nonce.len(), 12);
        
        // 4. Encrypt data with associated data
        let plaintext = b"Comprehensive crypto ecosystem test with sensitive data";
        let associated_data = b"Test metadata for AEAD verification";
        
        let encrypted = cipher.encrypt_with_aad(plaintext, associated_data).unwrap();
        
        // 5. Verify encryption results
        assert_eq!(encrypted.nonce.len(), 12);
        assert_eq!(encrypted.tag.len(), 16);
        assert_ne!(encrypted.ciphertext, plaintext);
        
        // 6. Decrypt and verify
        let decrypted = cipher.decrypt_with_aad(
            &encrypted.ciphertext,
            &encrypted.nonce,
            &encrypted.tag,
            associated_data
        ).unwrap();
        
        assert!(decrypted.verified);
        assert_eq!(decrypted.plaintext, plaintext);
        
        // 7. Test constant-time tag verification
        assert!(cipher.verify_tag(&encrypted.tag, &encrypted.tag));
        
        let mut tampered_tag = encrypted.tag.clone();
        tampered_tag[0] ^= 1;
        assert!(!cipher.verify_tag(&encrypted.tag, &tampered_tag));
    }
    
    /// Test memory protection and secure operations
    #[test]
    fn test_memory_protection_integration() {
        // Test SecureMemory with crypto operations
        let sensitive_data = b"super_secret_key_material_that_must_be_protected";
        let secure_mem = SecureMemory::new(sensitive_data.to_vec()).unwrap();
        
        // Verify memory locking works
        assert!(secure_mem.lock_memory().is_ok());
        assert_eq!(secure_mem.as_bytes(), sensitive_data);
        
        // Test ZeroOnDrop functionality
        let mut zero_drop_data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        {
            let mut zero_drop = ZeroOnDrop::new(zero_drop_data.clone());
            assert_eq!(zero_drop.as_bytes(), &zero_drop_data);
            
            // Data should be automatically zeroed when dropped
        }
        
        // Test ProtectedBytes with access limits
        let protected_key = ProtectedBytes::new(vec![0x12; 32], 3).unwrap();
        
        // Should allow limited access
        let result1 = protected_key.access(|data| data.len());
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 32);
        
        let result2 = protected_key.access(|data| data[0]);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 0x12);
        
        let result3 = protected_key.access(|data| data.len());
        assert!(result3.is_ok());
        
        // Fourth access should fail
        let result4 = protected_key.access(|data| data.len());
        assert!(result4.is_err());
    }
    
    /// Test constant-time operations for security
    #[test]
    fn test_constant_time_security() {
        let secret_data = b"secret_authentication_token";
        let user_input = b"secret_authentication_token";
        let wrong_input = b"wrong_authentication_token!";
        
        // Test constant-time comparison
        assert!(constant_time_compare(secret_data, user_input));
        assert!(!constant_time_compare(secret_data, wrong_input));
        
        // Test constant-time selection
        let option_a = b"option_a";
        let option_b = b"option_b";
        
        let selected_true = constant_time_select(true, option_a, option_b).unwrap();
        let selected_false = constant_time_select(false, option_a, option_b).unwrap();
        
        assert_eq!(selected_true, option_a);
        assert_eq!(selected_false, option_b);
        
        // Test constant-time XOR
        let data1 = b"hello";
        let data2 = b"world";
        let mut xor_result = vec![0u8; 5];
        
        constant_time_xor(data1, data2, &mut xor_result).unwrap();
        
        // XOR should be reversible
        let mut original = vec![0u8; 5];
        constant_time_xor(&xor_result, data2, &mut original).unwrap();
        assert_eq!(original, data1);
        
        // Test secure operations utility
        let ops = ConstantTimeOps::new();
        assert!(ops.compare(b"test", b"test"));
        assert!(!ops.compare(b"test", b"fail"));
        
        assert_eq!(ops.select_u32(true, 100, 200), 100);
        assert_eq!(ops.select_u32(false, 100, 200), 200);
    }
    
    /// Test nonce generation and uniqueness guarantees
    #[test]
    fn test_nonce_generation_comprehensive() {
        let generator = NonceGenerator::new().unwrap();
        
        // Test basic nonce generation
        let nonce1 = generator.generate_nonce(12).unwrap();
        let nonce2 = generator.generate_nonce(12).unwrap();
        
        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
        assert!(nonce1.verify_uniqueness(&nonce2));
        
        // Test timestamped nonces
        let timestamped_nonce = generator.generate_timestamped_nonce(16).unwrap();
        assert_eq!(timestamped_nonce.len(), 16);
        assert!(timestamped_nonce.is_fresh(10));
        
        // Test counter-based nonces
        let base = vec![0x42; 12];
        let counter_nonce1 = generator.generate_counter_nonce(&base, 1).unwrap();
        let counter_nonce2 = generator.generate_counter_nonce(&base, 2).unwrap();
        
        assert_ne!(counter_nonce1.as_bytes(), counter_nonce2.as_bytes());
        assert_eq!(counter_nonce1.counter(), 1);
        assert_eq!(counter_nonce2.counter(), 2);
        
        // Test nonce collision detection
        let nonces = vec![nonce1.clone(), nonce2.clone(), timestamped_nonce.clone()];
        assert!(!NonceUtils::check_collision(&nonces));
        
        // Test nonce validation for algorithms
        assert!(NonceUtils::validate_nonce_size("ChaCha20-Poly1305", 12).is_ok());
        assert!(NonceUtils::validate_nonce_size("ChaCha20-Poly1305", 8).is_err());
        
        // Test random mode nonces
        let random_mode = NonceRandomMode::new(NonceEntropySource::ChaCha20Rng);
        let random_nonce1 = random_mode.generate_nonce(12).unwrap();
        let random_nonce2 = random_mode.generate_nonce(12).unwrap();
        
        assert_eq!(random_nonce1.len(), 12);
        assert_eq!(random_nonce2.len(), 12);
        assert_ne!(random_nonce1.as_bytes(), random_nonce2.as_bytes());
    }
    
    /// Test AEAD integration and factory patterns
    #[test]
    fn test_aead_integration() {
        // Test AEAD cipher creation through factory
        let key = vec![42u8; 32];
        let cipher = AeadCipherFactory::create_cipher("ChaCha20-Poly1305", &key).unwrap();
        
        assert_eq!(cipher.algorithm_name(), "ChaCha20-Poly1305");
        assert_eq!(cipher.key_size(), 32);
        assert_eq!(cipher.nonce_size(), 12);
        assert_eq!(cipher.tag_size(), 16);
        
        // Test encryption with associated data
        let plaintext = b"AEAD test message with authentication";
        let associated_data = b"public metadata for verification";
        
        let result = cipher.encrypt_with_auth(plaintext, associated_data).unwrap();
        
        // Verify result structure
        assert_eq!(result.algorithm, "ChaCha20-Poly1305");
        assert_eq!(result.nonce.len(), 12);
        assert_eq!(result.tag.len(), 16);
        assert_eq!(result.tag.algorithm(), "ChaCha20-Poly1305");
        
        // Test decryption and verification
        let decrypted = cipher.decrypt_with_auth(
            &result.ciphertext,
            &result.nonce,
            result.tag.as_bytes(),
            associated_data
        ).unwrap();
        
        assert_eq!(decrypted, plaintext);
        
        // Test tag verification methods
        let same_tag = AuthenticationTag::new(result.tag.to_vec(), "ChaCha20-Poly1305".to_string());
        assert!(result.tag.verify(&same_tag));
        assert!(result.tag.verify_bytes(result.tag.as_bytes()));
        
        // Test batch operations
        let messages = vec![
            (b"message1".as_ref(), b"aad1".as_ref()),
            (b"message2".as_ref(), b"aad2".as_ref()),
            (b"message3".as_ref(), b"aad3".as_ref()),
        ];
        
        let encrypted_batch = AeadUtils::encrypt_batch(cipher.as_ref(), &messages).unwrap();
        assert_eq!(encrypted_batch.len(), 3);
        
        // Verify each encrypted message has unique nonce
        for i in 0..encrypted_batch.len() {
            for j in (i + 1)..encrypted_batch.len() {
                assert_ne!(encrypted_batch[i].nonce, encrypted_batch[j].nonce);
            }
        }
        
        // Test batch decryption
        let decrypt_messages = vec![
            (encrypted_batch[0].ciphertext.clone(), encrypted_batch[0].nonce.clone(), 
             encrypted_batch[0].tag.to_vec(), b"aad1".to_vec()),
            (encrypted_batch[1].ciphertext.clone(), encrypted_batch[1].nonce.clone(), 
             encrypted_batch[1].tag.to_vec(), b"aad2".to_vec()),
            (encrypted_batch[2].ciphertext.clone(), encrypted_batch[2].nonce.clone(), 
             encrypted_batch[2].tag.to_vec(), b"aad3".to_vec()),
        ];
        
        let decrypted_batch = AeadUtils::decrypt_batch(cipher.as_ref(), &decrypt_messages).unwrap();
        assert_eq!(decrypted_batch.len(), 3);
        assert_eq!(decrypted_batch[0], b"message1");
        assert_eq!(decrypted_batch[1], b"message2");
        assert_eq!(decrypted_batch[2], b"message3");
    }
    
    /// Test password-based key derivation integration
    #[test]
    fn test_password_based_crypto() {
        let password = b"strong_user_password_with_entropy";
        let salt = b"unique_salt_for_key_derivation";
        let iterations = 100_000;
        
        // Test ChaCha20-Poly1305 with password-derived key
        let cipher = ChaCha20Poly1305::from_password(password, salt, iterations).unwrap();
        assert!(cipher.is_key_valid());
        
        // Test encryption with derived key
        let plaintext = b"Password-protected message content";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        
        // Create second cipher with same password - should work
        let cipher2 = ChaCha20Poly1305::from_password(password, salt, iterations).unwrap();
        let decrypted = cipher2.decrypt(&encrypted.ciphertext, &encrypted.nonce, &encrypted.tag).unwrap();
        
        assert_eq!(decrypted.plaintext, plaintext);
        assert!(decrypted.verified);
        
        // Test with wrong password - should fail
        let wrong_password = b"wrong_password_should_fail";
        let cipher3 = ChaCha20Poly1305::from_password(wrong_password, salt, iterations).unwrap();
        let wrong_decrypt = cipher3.decrypt(&encrypted.ciphertext, &encrypted.nonce, &encrypted.tag);
        assert!(wrong_decrypt.is_err());
    }
    
    /// Test serialization and data handling
    #[test]
    fn test_crypto_serialization() {
        let key = vec![42u8; 32];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Test data for serialization with crypto";
        let associated_data = b"serialization metadata";
        
        // Test encrypt and serialize
        let serialized = cipher.encrypt_and_serialize(plaintext, associated_data).unwrap();
        assert!(!serialized.is_empty());
        
        // Test decrypt and deserialize
        let deserialized = cipher.decrypt_and_deserialize(&serialized, associated_data).unwrap();
        assert_eq!(deserialized, plaintext);
        
        // Test with wrong associated data should fail
        let wrong_aad = b"wrong_metadata";
        let wrong_deserialize = cipher.decrypt_and_deserialize(&serialized, wrong_aad);
        assert!(wrong_deserialize.is_err());
    }
    
    /// Test error handling across the crypto ecosystem
    #[test]
    fn test_crypto_error_handling() {
        // Test invalid key sizes
        let short_key = vec![42u8; 16];
        let chacha_result = ChaCha20Poly1305::new(&short_key);
        assert!(chacha_result.is_err());
        
        let aead_result = AeadCipher::new("ChaCha20-Poly1305".to_string(), short_key);
        assert!(aead_result.is_err());
        
        // Test invalid nonce sizes
        let generator = NonceGenerator::new().unwrap();
        assert!(generator.generate_nonce(0).is_err());
        assert!(generator.generate_nonce(2000).is_err());
        assert!(generator.generate_timestamped_nonce(8).is_err());
        
        // Test unsupported algorithms
        let key = vec![42u8; 32];
        let unsupported_result = AeadCipherFactory::create_cipher("UnsupportedAlgorithm", &key);
        assert!(unsupported_result.is_err());
        
        // Test memory protection errors
        let empty_data = vec![];
        let secure_mem_result = SecureMemory::new(empty_data);
        assert!(secure_mem_result.is_err());
        
        // Test constant-time operation errors
        let mismatched_lengths = constant_time_select(true, b"short", b"longer_data");
        assert!(mismatched_lengths.is_err());
        
        // Test protected bytes access limit
        let protected = ProtectedBytes::new(vec![1, 2, 3], 1).unwrap();
        assert!(protected.access(|_| ()).is_ok()); // First access OK
        assert!(protected.access(|_| ()).is_err()); // Second access should fail
    }
    
    /// Test performance and limits
    #[test]
    fn test_crypto_performance_limits() {
        let key = vec![42u8; 32];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        // Test with various data sizes
        let small_data = b"small";
        let medium_data = vec![42u8; 1024]; // 1KB
        let large_data = vec![42u8; 65536]; // 64KB
        
        for data in [small_data.as_ref(), &medium_data, &large_data] {
            let encrypted = cipher.encrypt(data).unwrap();
            let decrypted = cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, &encrypted.tag).unwrap();
            assert_eq!(decrypted.plaintext, data);
        }
        
        // Test nonce uniqueness with many generations
        let generator = NonceGenerator::new().unwrap();
        let mut nonces = Vec::new();
        
        for _ in 0..1000 {
            let nonce = generator.generate_nonce(12).unwrap();
            nonces.push(nonce);
        }
        
        // Verify no collisions in 1000 nonces
        assert!(!NonceUtils::check_collision(&nonces));
        
        // Test memory protection with different sizes
        for size in [16, 32, 64, 128, 256, 512, 1024] {
            let data = vec![0xAA; size];
            let secure_mem = SecureMemory::new(data.clone()).unwrap();
            assert_eq!(secure_mem.as_bytes(), &data);
        }
    }
    
    /// Test crypto ecosystem integration with realistic scenarios
    #[test]
    fn test_realistic_crypto_scenarios() {
        // Scenario 1: Secure file encryption
        let file_content = b"Important document content that needs protection";
        let file_metadata = b"document.txt|application/text|2024-01-01";
        
        let key = ChaCha20Poly1305Util::generate_key().unwrap();
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let encrypted_file = cipher.encrypt_and_serialize(file_content, file_metadata).unwrap();
        let decrypted_file = cipher.decrypt_and_deserialize(&encrypted_file, file_metadata).unwrap();
        assert_eq!(decrypted_file, file_content);
        
        // Scenario 2: Secure messaging
        let messages = vec![
            (b"Hello, this is a secure message".as_ref(), b"chat:user1:user2".as_ref()),
            (b"Reply: Message received and verified".as_ref(), b"chat:user2:user1".as_ref()),
            (b"Final: Conversation complete".as_ref(), b"chat:user1:user2".as_ref()),
        ];
        
        let aead_cipher = AeadCipherFactory::create_cipher("ChaCha20-Poly1305", &key).unwrap();
        let encrypted_messages = AeadUtils::encrypt_batch(aead_cipher.as_ref(), &messages).unwrap();
        
        // Verify all messages encrypted successfully
        assert_eq!(encrypted_messages.len(), 3);
        for encrypted in &encrypted_messages {
            assert!(!encrypted.ciphertext.is_empty());
            assert_eq!(encrypted.nonce.len(), 12);
            assert_eq!(encrypted.tag.len(), 16);
        }
        
        // Scenario 3: Key rotation
        let old_key = vec![0x11; 32];
        let new_key = vec![0x22; 32];
        
        let old_cipher = ChaCha20Poly1305::new(&old_key).unwrap();
        let new_cipher = ChaCha20Poly1305::new(&new_key).unwrap();
        
        let data = b"Data encrypted with old key";
        let old_encrypted = old_cipher.encrypt(data).unwrap();
        
        // Decrypt with old key
        let decrypted = old_cipher.decrypt(&old_encrypted.ciphertext, &old_encrypted.nonce, &old_encrypted.tag).unwrap();
        assert_eq!(decrypted.plaintext, data);
        
        // Re-encrypt with new key
        let new_encrypted = new_cipher.encrypt(&decrypted.plaintext).unwrap();
        let final_decrypted = new_cipher.decrypt(&new_encrypted.ciphertext, &new_encrypted.nonce, &new_encrypted.tag).unwrap();
        assert_eq!(final_decrypted.plaintext, data);
    }
}

/// Performance benchmark tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_encryption_performance() {
        let key = vec![42u8; 32];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        let data = vec![0xAA; 1024 * 1024]; // 1MB of data
        
        let start = Instant::now();
        let encrypted = cipher.encrypt(&data).unwrap();
        let encrypt_duration = start.elapsed();
        
        let start = Instant::now();
        let decrypted = cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, &encrypted.tag).unwrap();
        let decrypt_duration = start.elapsed();
        
        // Performance assertions (should complete within reasonable time)
        assert!(encrypt_duration.as_millis() < 1000); // Less than 1 second for 1MB
        assert!(decrypt_duration.as_millis() < 1000);
        assert_eq!(decrypted.plaintext, data);
        
        println!("Encryption of 1MB took: {:?}", encrypt_duration);
        println!("Decryption of 1MB took: {:?}", decrypt_duration);
    }
    
    #[test]
    fn test_nonce_generation_performance() {
        let generator = NonceGenerator::new().unwrap();
        
        let start = Instant::now();
        for _ in 0..10_000 {
            let _nonce = generator.generate_nonce(12).unwrap();
        }
        let duration = start.elapsed();
        
        // Should generate 10,000 nonces quickly
        assert!(duration.as_millis() < 1000);
        println!("Generated 10,000 nonces in: {:?}", duration);
    }
    
    #[test]
    fn test_memory_protection_performance() {
        let data = vec![0xBB; 4096]; // 4KB
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _secure_mem = SecureMemory::new(data.clone()).unwrap();
        }
        let duration = start.elapsed();
        
        // Should create 1000 secure memory instances quickly
        assert!(duration.as_millis() < 1000);
        println!("Created 1000 SecureMemory instances in: {:?}", duration);
    }
}
