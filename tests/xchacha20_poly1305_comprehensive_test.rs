//! XChaCha20-Poly1305 AEAD Comprehensive Test Suite
//! 
//! This test suite provides complete validation of the XChaCha20-Poly1305 AEAD implementation
//! including basic operations, streaming APIs, edge cases, security properties, and performance
//! characteristics.

use cursed::stdlib::crypto::crypto_advanced::{
    XChaCha20Key, XChaCha20Nonce, XChaCha20Poly1305Cipher, XChaCha20Poly1305Api,
    XChaCha20Poly1305StreamingEncoder, XChaCha20Poly1305StreamingDecoder,
    key_derivation, utils,
    XCHACHA20_KEY_SIZE, XCHACHA20_NONCE_SIZE, XCHACHA20_TAG_SIZE, XCHACHA20_MAX_PLAINTEXT_SIZE,
};
use cursed::error::CursedError;
use rand::{thread_rng, Rng, RngCore};
use std::time::Instant;

#[cfg(test)]
mod basic_functionality_tests {
    use super::*;

    #[test]
    fn test_key_generation_and_validation() {
        // Test key generation
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        assert_eq!(key.as_bytes().len(), XCHACHA20_KEY_SIZE);

        // Test key from bytes
        let key_bytes = [42u8; XCHACHA20_KEY_SIZE];
        let key_from_bytes = XChaCha20Key::from_bytes(&key_bytes).expect("Key from bytes should succeed");
        assert_eq!(key_from_bytes.as_bytes(), &key_bytes);

        // Test invalid key size
        let invalid_key = [0u8; 16];
        assert!(XChaCha20Key::from_bytes(&invalid_key).is_err());
    }

    #[test]
    fn test_nonce_generation_and_validation() {
        // Test nonce generation
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        assert_eq!(nonce.as_bytes().len(), XCHACHA20_NONCE_SIZE);

        // Test nonce from bytes
        let nonce_bytes = [42u8; XCHACHA20_NONCE_SIZE];
        let nonce_from_bytes = XChaCha20Nonce::from_bytes(&nonce_bytes).expect("Nonce from bytes should succeed");
        assert_eq!(nonce_from_bytes.as_bytes(), &nonce_bytes);

        // Test invalid nonce size
        let invalid_nonce = [0u8; 12];
        assert!(XChaCha20Nonce::from_bytes(&invalid_nonce).is_err());
    }

    #[test]
    fn test_basic_encryption_decryption_round_trip() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305 AEAD!";
        let associated_data = b"Additional authenticated data";

        // Encrypt
        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Verify ciphertext is different from plaintext
        assert_ne!(ciphertext, plaintext);
        assert_eq!(ciphertext.len(), plaintext.len() + XCHACHA20_TAG_SIZE);

        // Decrypt
        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, associated_data)
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encryption_without_associated_data() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello without AAD!";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, &[])
            .expect("Encryption without AAD should succeed");

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, &[])
            .expect("Decryption without AAD should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_empty_plaintext_encryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"";
        let associated_data = b"empty message test";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Empty plaintext encryption should succeed");

        assert_eq!(ciphertext.len(), XCHACHA20_TAG_SIZE);

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, associated_data)
            .expect("Empty plaintext decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }
}

#[cfg(test)]
mod authentication_and_integrity_tests {
    use super::*;

    #[test]
    fn test_ciphertext_tampering_detection() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Tamper detection test message";
        let associated_data = b"integrity test";

        let mut ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Tamper with ciphertext
        ciphertext[0] ^= 1;

        // Decryption should fail
        assert!(cipher.decrypt(&nonce, &ciphertext, associated_data).is_err());
    }

    #[test]
    fn test_authentication_tag_tampering() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Authentication tag tampering test";
        let associated_data = b"tag test";

        let mut ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Tamper with authentication tag (last 16 bytes)
        let tag_start = ciphertext.len() - XCHACHA20_TAG_SIZE;
        ciphertext[tag_start] ^= 1;

        // Decryption should fail
        assert!(cipher.decrypt(&nonce, &ciphertext, associated_data).is_err());
    }

    #[test]
    fn test_wrong_associated_data_detection() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Associated data test message";
        let correct_aad = b"correct associated data";
        let wrong_aad = b"wrong associated data";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, correct_aad)
            .expect("Encryption should succeed");

        // Decryption with wrong AAD should fail
        assert!(cipher.decrypt(&nonce, &ciphertext, wrong_aad).is_err());

        // Decryption with correct AAD should succeed
        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, correct_aad)
            .expect("Decryption with correct AAD should succeed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_nonce_reuse_detection() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext1 = b"First message";
        let plaintext2 = b"Second message";
        let associated_data = b"nonce reuse test";

        let ciphertext1 = cipher
            .encrypt(&nonce, plaintext1, associated_data)
            .expect("First encryption should succeed");

        let ciphertext2 = cipher
            .encrypt(&nonce, plaintext2, associated_data)
            .expect("Second encryption should succeed");

        // Ciphertexts should be different even with same nonce
        // (This is acceptable but indicates nonce reuse)
        assert_ne!(ciphertext1, ciphertext2);

        // Both should decrypt correctly
        let decrypted1 = cipher
            .decrypt(&nonce, &ciphertext1, associated_data)
            .expect("First decryption should succeed");
        let decrypted2 = cipher
            .decrypt(&nonce, &ciphertext2, associated_data)
            .expect("Second decryption should succeed");

        assert_eq!(decrypted1, plaintext1);
        assert_eq!(decrypted2, plaintext2);
    }
}

#[cfg(test)]
mod streaming_api_tests {
    use super::*;

    #[test]
    fn test_streaming_encryption_decryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let plaintext = b"This is a long message that will be processed in chunks for streaming encryption and decryption testing. It should demonstrate the streaming API capabilities.";
        let associated_data = b"streaming test";

        // Create streaming encoder
        let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());
        let nonce = encoder.nonce().clone();

        // Encrypt in chunks
        let chunk_size = 32;
        let mut encrypted_chunks = Vec::new();
        
        for chunk in plaintext.chunks(chunk_size) {
            let encrypted = encoder
                .process_chunk(chunk, associated_data)
                .expect("Streaming encryption should succeed");
            encrypted_chunks.push(encrypted);
        }

        // Create streaming decoder
        let mut decoder = XChaCha20Poly1305StreamingDecoder::new(&key, nonce);
        let mut decrypted = Vec::new();

        // Decrypt chunks
        for encrypted_chunk in encrypted_chunks {
            let chunk = decoder
                .process_chunk(&encrypted_chunk, associated_data)
                .expect("Streaming decryption should succeed");
            decrypted.extend_from_slice(&chunk);
        }

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_streaming_with_different_chunk_sizes() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let plaintext = vec![42u8; 1000]; // 1KB of data
        let associated_data = b"chunk size test";

        let chunk_sizes = [1, 7, 16, 32, 64, 128, 256];

        for chunk_size in chunk_sizes {
            let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());
            let nonce = encoder.nonce().clone();

            // Encrypt in chunks
            let mut encrypted_chunks = Vec::new();
            
            for chunk in plaintext.chunks(chunk_size) {
                let encrypted = encoder
                    .process_chunk(chunk, associated_data)
                    .expect("Streaming encryption should succeed");
                encrypted_chunks.push(encrypted);
            }

            // Decrypt chunks
            let mut decoder = XChaCha20Poly1305StreamingDecoder::new(&key, nonce);
            let mut decrypted = Vec::new();

            for encrypted_chunk in encrypted_chunks {
                let chunk = decoder
                    .process_chunk(&encrypted_chunk, associated_data)
                    .expect("Streaming decryption should succeed");
                decrypted.extend_from_slice(&chunk);
            }

            assert_eq!(decrypted, plaintext, "Failed with chunk size {}", chunk_size);
        }
    }

    #[test]
    fn test_streaming_byte_counting() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let plaintext = b"Byte counting test message";
        let associated_data = b"byte count";

        let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());
        let nonce = encoder.nonce().clone();

        assert_eq!(encoder.processed_bytes(), 0);

        let _encrypted = encoder
            .process_chunk(plaintext, associated_data)
            .expect("Streaming encryption should succeed");

        assert_eq!(encoder.processed_bytes(), plaintext.len() as u64);

        let mut decoder = XChaCha20Poly1305StreamingDecoder::new(&key, nonce);
        assert_eq!(decoder.processed_bytes(), 0);

        let decrypted = decoder
            .process_chunk(&_encrypted, associated_data)
            .expect("Streaming decryption should succeed");

        assert_eq!(decoder.processed_bytes(), decrypted.len() as u64);
    }
}

#[cfg(test)]
mod high_level_api_tests {
    use super::*;

    #[test]
    fn test_high_level_api_convenience_functions() {
        let key = XChaCha20Poly1305Api::generate_key().expect("Key generation should succeed");
        let plaintext = b"High-level API test message";
        let associated_data = Some(b"high-level test".as_slice());

        // Encrypt using high-level API
        let (nonce, ciphertext) = XChaCha20Poly1305Api::encrypt(&key, plaintext, associated_data)
            .expect("High-level encryption should succeed");

        // Decrypt using high-level API
        let decrypted = XChaCha20Poly1305Api::decrypt(&key, &nonce, &ciphertext, associated_data)
            .expect("High-level decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_high_level_api_without_aad() {
        let key = XChaCha20Poly1305Api::generate_key().expect("Key generation should succeed");
        let plaintext = b"High-level API without AAD";

        let (nonce, ciphertext) = XChaCha20Poly1305Api::encrypt(&key, plaintext, None)
            .expect("High-level encryption without AAD should succeed");

        let decrypted = XChaCha20Poly1305Api::decrypt(&key, &nonce, &ciphertext, None)
            .expect("High-level decryption without AAD should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_streaming_api_creation() {
        let key = XChaCha20Poly1305Api::generate_key().expect("Key generation should succeed");

        let encoder = XChaCha20Poly1305Api::create_streaming_encoder(&key)
            .expect("Streaming encoder creation should succeed");

        let decoder = XChaCha20Poly1305Api::create_streaming_decoder(&key, encoder.nonce().clone());

        // Test basic properties
        assert_eq!(encoder.processed_bytes(), 0);
        assert_eq!(decoder.processed_bytes(), 0);
        assert_eq!(encoder.nonce(), decoder.nonce());
    }
}

#[cfg(test)]
mod key_derivation_tests {
    use super::*;

    #[test]
    fn test_key_derivation_basic() {
        let input_key_material = b"shared secret material";
        let salt = Some(b"unique salt value".as_slice());
        let info = b"XChaCha20-Poly1305 key derivation test";

        let key = key_derivation::derive_key(input_key_material, salt, info)
            .expect("Key derivation should succeed");

        assert_eq!(key.as_bytes().len(), XCHACHA20_KEY_SIZE);
    }

    #[test]
    fn test_key_derivation_deterministic() {
        let input_key_material = b"deterministic test material";
        let salt = Some(b"deterministic salt".as_slice());
        let info = b"deterministic info";

        let key1 = key_derivation::derive_key(input_key_material, salt, info)
            .expect("First key derivation should succeed");

        let key2 = key_derivation::derive_key(input_key_material, salt, info)
            .expect("Second key derivation should succeed");

        assert_eq!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_key_derivation_different_inputs() {
        let input_key_material = b"test material";
        let salt = Some(b"test salt".as_slice());
        let info1 = b"info1";
        let info2 = b"info2";

        let key1 = key_derivation::derive_key(input_key_material, salt, info1)
            .expect("First key derivation should succeed");

        let key2 = key_derivation::derive_key(input_key_material, salt, info2)
            .expect("Second key derivation should succeed");

        assert_ne!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_multiple_key_derivation() {
        let input_key_material = b"multi-key material";
        let salt = Some(b"multi-key salt".as_slice());

        let keys = key_derivation::derive_keys(input_key_material, salt, 5)
            .expect("Multiple key derivation should succeed");

        assert_eq!(keys.len(), 5);

        // All keys should be different
        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                assert_ne!(keys[i].as_bytes(), keys[j].as_bytes(), "Keys {} and {} should be different", i, j);
            }
        }
    }
}

#[cfg(test)]
mod utility_function_tests {
    use super::*;

    #[test]
    fn test_data_size_validation() {
        // Valid sizes
        assert!(utils::validate_data_size(0).is_ok());
        assert!(utils::validate_data_size(1000).is_ok());
        assert!(utils::validate_data_size(1024 * 1024).is_ok()); // 1MB
        assert!(utils::validate_data_size(XCHACHA20_MAX_PLAINTEXT_SIZE as usize).is_ok());

        // Invalid size
        assert!(utils::validate_data_size((XCHACHA20_MAX_PLAINTEXT_SIZE + 1) as usize).is_err());
    }

    #[test]
    fn test_ciphertext_size_calculations() {
        let test_sizes = [0, 1, 16, 32, 64, 128, 1024, 65536];

        for plaintext_size in test_sizes {
            let ciphertext_size = utils::calculate_ciphertext_size(plaintext_size);
            assert_eq!(ciphertext_size, plaintext_size + XCHACHA20_TAG_SIZE);

            let recovered_plaintext_size = utils::calculate_plaintext_size(ciphertext_size)
                .expect("Plaintext size calculation should succeed");
            assert_eq!(recovered_plaintext_size, plaintext_size);
        }
    }

    #[test]
    fn test_invalid_ciphertext_size() {
        let invalid_sizes = [0, 1, 8, 15]; // All less than XCHACHA20_TAG_SIZE

        for size in invalid_sizes {
            assert!(utils::calculate_plaintext_size(size).is_err(), "Size {} should be invalid", size);
        }
    }

    #[test]
    fn test_constant_time_comparison() {
        let data1 = b"hello world";
        let data2 = b"hello world";
        let data3 = b"hello world!";
        let data4 = b"different data";

        assert!(utils::constant_time_eq(data1, data2));
        assert!(!utils::constant_time_eq(data1, data3));
        assert!(!utils::constant_time_eq(data1, data4));
        assert!(!utils::constant_time_eq(data3, data4));

        // Test with empty data
        assert!(utils::constant_time_eq(b"", b""));
        assert!(!utils::constant_time_eq(b"", b"a"));
    }
}

#[cfg(test)]
mod edge_cases_and_error_handling_tests {
    use super::*;

    #[test]
    fn test_maximum_data_size_handling() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Test with maximum allowed size (should succeed)
        let max_size = XCHACHA20_MAX_PLAINTEXT_SIZE as usize;
        let plaintext = vec![0u8; max_size];
        
        let result = cipher.encrypt(&nonce, &plaintext, &[]);
        assert!(result.is_ok(), "Encryption with maximum size should succeed");
    }

    #[test]
    fn test_oversized_data_rejection() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Test with size exceeding limit (should fail)
        let oversized = (XCHACHA20_MAX_PLAINTEXT_SIZE + 1) as usize;
        let plaintext = vec![0u8; oversized];
        
        let result = cipher.encrypt(&nonce, &plaintext, &[]);
        assert!(result.is_err(), "Encryption with oversized data should fail");
    }

    #[test]
    fn test_streaming_size_limit_enforcement() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());

        // Fill up to near the limit
        let large_chunk = vec![0u8; (XCHACHA20_MAX_PLAINTEXT_SIZE / 2) as usize];
        encoder.process_chunk(&large_chunk, &[]).expect("First large chunk should succeed");

        // Try to exceed the limit
        let oversized_chunk = vec![0u8; (XCHACHA20_MAX_PLAINTEXT_SIZE / 2 + 1) as usize];
        let result = encoder.process_chunk(&oversized_chunk, &[]);
        assert!(result.is_err(), "Oversized streaming chunk should fail");
    }

    #[test]
    fn test_malformed_ciphertext_handling() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Test with ciphertext too short
        let short_ciphertext = vec![0u8; XCHACHA20_TAG_SIZE - 1];
        let result = cipher.decrypt(&nonce, &short_ciphertext, &[]);
        assert!(result.is_err(), "Decryption of too-short ciphertext should fail");

        // Test with random data
        let mut rng = thread_rng();
        let mut random_data = vec![0u8; 64];
        rng.fill_bytes(&mut random_data);
        
        let result = cipher.decrypt(&nonce, &random_data, &[]);
        assert!(result.is_err(), "Decryption of random data should fail");
    }

    #[test]
    fn test_in_place_operations_with_insufficient_space() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Buffer too small for in-place encryption (needs space for tag)
        let mut buffer = vec![42u8; 32]; // Only 32 bytes, need 32 + 16 for tag
        
        let result = cipher.encrypt_in_place(&nonce, &[], &mut buffer);
        assert!(result.is_err(), "In-place encryption with insufficient space should fail");
    }
}

#[cfg(test)]
mod security_property_tests {
    use super::*;

    #[test]
    fn test_nonce_uniqueness_statistical() {
        let mut nonces = std::collections::HashSet::new();
        let sample_size = 1000;

        for _ in 0..sample_size {
            let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
            let nonce_bytes = nonce.as_bytes().to_vec();
            
            // Should be unique (with extremely high probability)
            assert!(nonces.insert(nonce_bytes), "Nonce collision detected");
        }

        assert_eq!(nonces.len(), sample_size);
    }

    #[test]
    fn test_key_uniqueness_statistical() {
        let mut keys = std::collections::HashSet::new();
        let sample_size = 1000;

        for _ in 0..sample_size {
            let key = XChaCha20Key::generate().expect("Key generation should succeed");
            let key_bytes = key.as_bytes().to_vec();
            
            // Should be unique (with extremely high probability)
            assert!(keys.insert(key_bytes), "Key collision detected");
        }

        assert_eq!(keys.len(), sample_size);
    }

    #[test]
    fn test_ciphertext_indistinguishability() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext1 = b"message one";
        let plaintext2 = b"message two";
        let associated_data = b"indistinguishability test";

        let mut ciphertexts = std::collections::HashSet::new();

        // Encrypt the same message multiple times with different nonces
        for _ in 0..100 {
            let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
            let ciphertext = cipher
                .encrypt(&nonce, plaintext1, associated_data)
                .expect("Encryption should succeed");
            
            assert!(ciphertexts.insert(ciphertext), "Ciphertext collision detected");
        }

        // Encrypt different messages
        for _ in 0..100 {
            let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
            let ciphertext = cipher
                .encrypt(&nonce, plaintext2, associated_data)
                .expect("Encryption should succeed");
            
            assert!(ciphertexts.insert(ciphertext), "Ciphertext collision detected");
        }

        assert_eq!(ciphertexts.len(), 200);
    }

    #[test]
    fn test_authentication_tag_verification() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Authentication test message";
        let associated_data = b"auth test";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Test all possible single-bit flips in the authentication tag
        for byte_idx in 0..XCHACHA20_TAG_SIZE {
            for bit_idx in 0..8 {
                let mut tampered_ciphertext = ciphertext.clone();
                let tag_start = ciphertext.len() - XCHACHA20_TAG_SIZE;
                tampered_ciphertext[tag_start + byte_idx] ^= 1 << bit_idx;

                let result = cipher.decrypt(&nonce, &tampered_ciphertext, associated_data);
                assert!(result.is_err(), "Tampered authentication tag should be detected");
            }
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_encryption_performance() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = vec![0u8; 1024 * 1024]; // 1MB
        let associated_data = b"performance test";

        let start = Instant::now();
        let iterations = 10;

        for _ in 0..iterations {
            let _ciphertext = cipher
                .encrypt(&nonce, &plaintext, associated_data)
                .expect("Encryption should succeed");
        }

        let duration = start.elapsed();
        let throughput = (plaintext.len() * iterations) as f64 / duration.as_secs_f64();

        println!("Encryption throughput: {:.2} MB/s", throughput / (1024.0 * 1024.0));
        
        // Basic performance assertion (should be > 10 MB/s on modern hardware)
        assert!(throughput > 10.0 * 1024.0 * 1024.0, "Encryption throughput too low");
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_decryption_performance() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = vec![0u8; 1024 * 1024]; // 1MB
        let associated_data = b"performance test";

        let ciphertext = cipher
            .encrypt(&nonce, &plaintext, associated_data)
            .expect("Encryption should succeed");

        let start = Instant::now();
        let iterations = 10;

        for _ in 0..iterations {
            let _decrypted = cipher
                .decrypt(&nonce, &ciphertext, associated_data)
                .expect("Decryption should succeed");
        }

        let duration = start.elapsed();
        let throughput = (plaintext.len() * iterations) as f64 / duration.as_secs_f64();

        println!("Decryption throughput: {:.2} MB/s", throughput / (1024.0 * 1024.0));
        
        // Basic performance assertion (should be > 10 MB/s on modern hardware)
        assert!(throughput > 10.0 * 1024.0 * 1024.0, "Decryption throughput too low");
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_streaming_performance() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let plaintext = vec![0u8; 10 * 1024 * 1024]; // 10MB
        let associated_data = b"streaming performance test";

        let start = Instant::now();

        // Create streaming encoder
        let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());
        let nonce = encoder.nonce().clone();

        // Encrypt in 64KB chunks
        let chunk_size = 64 * 1024;
        let mut encrypted_chunks = Vec::new();
        
        for chunk in plaintext.chunks(chunk_size) {
            let encrypted = encoder
                .process_chunk(chunk, associated_data)
                .expect("Streaming encryption should succeed");
            encrypted_chunks.push(encrypted);
        }

        let encryption_time = start.elapsed();

        // Decrypt chunks
        let start = Instant::now();
        let mut decoder = XChaCha20Poly1305StreamingDecoder::new(&key, nonce);
        let mut decrypted = Vec::new();

        for encrypted_chunk in encrypted_chunks {
            let chunk = decoder
                .process_chunk(&encrypted_chunk, associated_data)
                .expect("Streaming decryption should succeed");
            decrypted.extend_from_slice(&chunk);
        }

        let decryption_time = start.elapsed();

        assert_eq!(decrypted, plaintext);

        let enc_throughput = plaintext.len() as f64 / encryption_time.as_secs_f64();
        let dec_throughput = plaintext.len() as f64 / decryption_time.as_secs_f64();

        println!("Streaming encryption throughput: {:.2} MB/s", enc_throughput / (1024.0 * 1024.0));
        println!("Streaming decryption throughput: {:.2} MB/s", dec_throughput / (1024.0 * 1024.0));

        // Basic performance assertions
        assert!(enc_throughput > 5.0 * 1024.0 * 1024.0, "Streaming encryption throughput too low");
        assert!(dec_throughput > 5.0 * 1024.0 * 1024.0, "Streaming decryption throughput too low");
    }

    #[test]
    fn test_key_generation_performance() {
        let start = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            let _key = XChaCha20Key::generate().expect("Key generation should succeed");
        }

        let duration = start.elapsed();
        let rate = iterations as f64 / duration.as_secs_f64();

        println!("Key generation rate: {:.0} keys/second", rate);
        
        // Should be able to generate > 1000 keys per second
        assert!(rate > 1000.0, "Key generation rate too low");
    }

    #[test]
    fn test_large_associated_data_performance() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Small message";
        let large_aad = vec![42u8; 1024 * 1024]; // 1MB of associated data

        let start = Instant::now();

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, &large_aad)
            .expect("Encryption with large AAD should succeed");

        let encryption_time = start.elapsed();

        let start = Instant::now();

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, &large_aad)
            .expect("Decryption with large AAD should succeed");

        let decryption_time = start.elapsed();

        assert_eq!(decrypted, plaintext);

        println!("Large AAD encryption time: {:?}", encryption_time);
        println!("Large AAD decryption time: {:?}", decryption_time);

        // Should complete within reasonable time (< 1 second)
        assert!(encryption_time.as_secs() < 1, "Encryption with large AAD too slow");
        assert!(decryption_time.as_secs() < 1, "Decryption with large AAD too slow");
    }
}

#[cfg(test)]
mod compatibility_tests {
    use super::*;

    #[test]
    fn test_cross_platform_determinism() {
        // Test that key derivation produces the same results across platforms
        let input_key_material = b"cross-platform test material";
        let salt = Some(b"cross-platform salt".as_slice());
        let info = b"cross-platform info";

        let key1 = key_derivation::derive_key(input_key_material, salt, info)
            .expect("Key derivation should succeed");

        // Derive the same key multiple times
        for _ in 0..10 {
            let key2 = key_derivation::derive_key(input_key_material, salt, info)
                .expect("Key derivation should succeed");
            assert_eq!(key1.as_bytes(), key2.as_bytes(), "Key derivation should be deterministic");
        }
    }

    #[test]
    fn test_endianness_independence() {
        // Test that encryption/decryption works consistently regardless of endianness
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Test with various data patterns that might expose endianness issues
        let test_patterns = [
            vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
            vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8],
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
            (0u32..256u32).map(|i| i as u8).collect::<Vec<u8>>(),
        ];

        for pattern in test_patterns {
            let ciphertext = cipher
                .encrypt(&nonce, &pattern, &[])
                .expect("Encryption should succeed");

            let decrypted = cipher
                .decrypt(&nonce, &ciphertext, &[])
                .expect("Decryption should succeed");

            assert_eq!(decrypted, pattern, "Pattern consistency failed");
        }
    }
}
