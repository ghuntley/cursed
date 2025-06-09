/// fr fr Comprehensive tests for CURSED crypto hash functions - security testing periodt
/// 
/// Tests all hash algorithms with known test vectors, performance benchmarks,
/// and edge cases to ensure solid cryptographic implementation.

use cursed::stdlib::crypto::{
    HashFunction, Sha256, Sha512, Md5, HashUtils, HashAlgorithm, HashResult
};
use std::time::Instant;

#[path = "common.rs"]
mod common;

/// fr fr Test SHA-256 with official NIST test vectors
#[test]
fn test_sha256_nist_vectors() {
    common::tracing::setup();
    
    // Test vector 1: Empty string
    let hash = Sha256::hash(b"");
    let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 2: "abc"
    let hash = Sha256::hash(b"abc");
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 3: "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
    let hash = Sha256::hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
    let expected = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 4: One million 'a's
    let million_as = "a".repeat(1_000_000);
    let hash = Sha256::hash(million_as.as_bytes());
    let expected = "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0";
    assert_eq!(HashUtils::to_hex(&hash), expected);
}

/// bestie Test SHA-512 with official NIST test vectors
#[test]
fn test_sha512_nist_vectors() {
    common::tracing::setup();
    
    // Test vector 1: Empty string
    let hash = Sha512::hash(b"");
    let expected = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 2: "abc"
    let hash = Sha512::hash(b"abc");
    let expected = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 3: "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
    let hash = Sha512::hash(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu");
    let expected = "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909";
    assert_eq!(HashUtils::to_hex(&hash), expected);
}

/// vibes Test MD5 with official RFC test vectors
#[test] 
fn test_md5_rfc_vectors() {
    common::tracing::setup();
    
    // Test vector 1: Empty string
    let hash = Md5::hash(b"");
    let expected = "d41d8cd98f00b204e9800998ecf8427e";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 2: "a"
    let hash = Md5::hash(b"a");
    let expected = "0cc175b9c0f1b6a831c399e269772661";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 3: "abc"
    let hash = Md5::hash(b"abc");
    let expected = "900150983cd24fb0d6963f7d28e17f72";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 4: "message digest"
    let hash = Md5::hash(b"message digest");
    let expected = "f96b697d7cb7938d525a2f31aaf161d0";
    assert_eq!(HashUtils::to_hex(&hash), expected);
    
    // Test vector 5: "abcdefghijklmnopqrstuvwxyz"
    let hash = Md5::hash(b"abcdefghijklmnopqrstuvwxyz");
    let expected = "c3fcd3d76192e4007dfb496cca67e13b";
    assert_eq!(HashUtils::to_hex(&hash), expected);
}

/// yolo Test incremental hashing (update method)
#[test]
fn test_incremental_hashing() {
    common::tracing::setup();
    
    // SHA-256 incremental vs one-shot
    let mut hasher = Sha256::new();
    hasher.update(b"Hello");
    hasher.update(b" ");
    hasher.update(b"World");
    let incremental_hash = hasher.finalize();
    
    let oneshot_hash = Sha256::hash(b"Hello World");
    
    assert_eq!(incremental_hash, oneshot_hash);
    
    // SHA-512 incremental vs one-shot
    let mut hasher = Sha512::new();
    hasher.update(b"CURSED");
    hasher.update(b" is ");
    hasher.update(b"periodt");
    let incremental_hash = hasher.finalize();
    
    let oneshot_hash = Sha512::hash(b"CURSED is periodt");
    
    assert_eq!(incremental_hash, oneshot_hash);
}

/// periodt Test edge cases and malformed inputs
#[test]
fn test_edge_cases() {
    common::tracing::setup();
    
    // Empty input
    let sha256_empty = Sha256::hash(&[]);
    let sha512_empty = Sha512::hash(&[]);
    let md5_empty = Md5::hash(&[]);
    
    assert_eq!(sha256_empty.len(), 32);
    assert_eq!(sha512_empty.len(), 64);
    assert_eq!(md5_empty.len(), 16);
    
    // Single byte inputs
    for byte in 0u8..=255u8 {
        let input = [byte];
        let _sha256 = Sha256::hash(&input);
        let _sha512 = Sha512::hash(&input);
        let _md5 = Md5::hash(&input);
        // Just ensure no panics
    }
    
    // Large inputs (boundary testing)
    let large_input = vec![0x42u8; 65536]; // 64KB
    let _sha256_large = Sha256::hash(&large_input);
    let _sha512_large = Sha512::hash(&large_input);
    let _md5_large = Md5::hash(&large_input);
    
    // Random-looking data
    let random_data = (0..1000).map(|i| (i * 37 + 17) as u8).collect::<Vec<_>>();
    let _sha256_random = Sha256::hash(&random_data);
    let _sha512_random = Sha512::hash(&random_data);
    let _md5_random = Md5::hash(&random_data);
}

/// bestie Performance benchmarks for hash functions
#[test]
fn test_hash_performance() {
    common::tracing::setup();
    
    let test_sizes = vec![1_000, 10_000, 100_000, 1_000_000]; // 1KB to 1MB
    
    for size in test_sizes {
        let data = vec![0x5A; size];
        
        // SHA-256 benchmark
        let start = Instant::now();
        let _hash = Sha256::hash(&data);
        let sha256_duration = start.elapsed();
        
        // SHA-512 benchmark
        let start = Instant::now();
        let _hash = Sha512::hash(&data);
        let sha512_duration = start.elapsed();
        
        // MD5 benchmark
        let start = Instant::now();
        let _hash = Md5::hash(&data);
        let md5_duration = start.elapsed();
        
        println!("Size: {} bytes", size);
        println!("  SHA-256: {:?} ({:.2} MB/s)", 
            sha256_duration, 
            size as f64 / sha256_duration.as_secs_f64() / 1_000_000.0
        );
        println!("  SHA-512: {:?} ({:.2} MB/s)", 
            sha512_duration,
            size as f64 / sha512_duration.as_secs_f64() / 1_000_000.0
        );
        println!("  MD5: {:?} ({:.2} MB/s)", 
            md5_duration,
            size as f64 / md5_duration.as_secs_f64() / 1_000_000.0
        );
        
        // Performance assertions (should complete reasonably quickly)
        assert!(sha256_duration.as_secs() < 5, "SHA-256 too slow for {} bytes", size);
        assert!(sha512_duration.as_secs() < 5, "SHA-512 too slow for {} bytes", size);
        assert!(md5_duration.as_secs() < 5, "MD5 too slow for {} bytes", size);
    }
}

/// lowkey Test constant time comparison function
#[test]
fn test_constant_time_compare() {
    common::tracing::setup();
    
    // Same values
    assert!(HashUtils::constant_time_compare(b"hello", b"hello"));
    assert!(HashUtils::constant_time_compare(&[1, 2, 3, 4], &[1, 2, 3, 4]));
    assert!(HashUtils::constant_time_compare(&[], &[]));
    
    // Different values
    assert!(!HashUtils::constant_time_compare(b"hello", b"world"));
    assert!(!HashUtils::constant_time_compare(&[1, 2, 3, 4], &[1, 2, 3, 5]));
    assert!(!HashUtils::constant_time_compare(&[1, 2, 3], &[1, 2, 3, 4]));
    
    // Different lengths
    assert!(!HashUtils::constant_time_compare(b"short", b"longer"));
    assert!(!HashUtils::constant_time_compare(&[1], &[1, 2]));
    
    // Test with hash outputs
    let hash1 = Sha256::hash(b"test1");
    let hash2 = Sha256::hash(b"test1");
    let hash3 = Sha256::hash(b"test2");
    
    assert!(HashUtils::constant_time_compare(&hash1, &hash2));
    assert!(!HashUtils::constant_time_compare(&hash1, &hash3));
}

/// highkey Test utility functions
#[test]
fn test_hash_utilities() {
    common::tracing::setup();
    
    // Test hex conversion
    assert_eq!(HashUtils::to_hex(&[]), "");
    assert_eq!(HashUtils::to_hex(&[0x00]), "00");
    assert_eq!(HashUtils::to_hex(&[0xFF]), "ff");
    assert_eq!(HashUtils::to_hex(&[0x12, 0x34, 0xAB, 0xCD]), "1234abcd");
    
    // Test string hashing utilities
    let sha256_hex = HashUtils::sha256_string("test");
    let sha512_hex = HashUtils::sha512_string("test");
    let md5_hex = HashUtils::md5_string("test");
    
    assert_eq!(sha256_hex.len(), 64); // 32 bytes * 2 hex chars
    assert_eq!(sha512_hex.len(), 128); // 64 bytes * 2 hex chars  
    assert_eq!(md5_hex.len(), 32); // 16 bytes * 2 hex chars
    
    // Verify they're hex strings
    assert!(sha256_hex.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(sha512_hex.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(md5_hex.chars().all(|c| c.is_ascii_hexdigit()));
}

/// facts Test algorithm metadata
#[test]
fn test_hash_algorithm_metadata() {
    common::tracing::setup();
    
    // Test algorithm enum properties
    assert_eq!(HashAlgorithm::Sha256.name(), "SHA-256");
    assert_eq!(HashAlgorithm::Sha512.name(), "SHA-512");
    assert_eq!(HashAlgorithm::Md5.name(), "MD5");
    
    assert_eq!(HashAlgorithm::Sha256.output_size(), 32);
    assert_eq!(HashAlgorithm::Sha512.output_size(), 64);
    assert_eq!(HashAlgorithm::Md5.output_size(), 16);
    
    assert!(HashAlgorithm::Sha256.is_secure());
    assert!(HashAlgorithm::Sha512.is_secure());
    assert!(!HashAlgorithm::Md5.is_secure());
    
    // Test hash function trait methods
    let sha256 = Sha256::new();
    assert_eq!(sha256.algorithm_name(), "SHA-256");
    assert_eq!(sha256.output_size(), 32);
    
    let sha512 = Sha512::new();
    assert_eq!(sha512.algorithm_name(), "SHA-512");
    assert_eq!(sha512.output_size(), 64);
    
    let md5 = Md5::new();
    assert_eq!(md5.algorithm_name(), "MD5");
    assert_eq!(md5.output_size(), 16);
}

/// sus Test HashResult wrapper
#[test]
fn test_hash_result() {
    common::tracing::setup();
    
    let sha256_hash = Sha256::hash(b"test");
    let result = HashResult::new(HashAlgorithm::Sha256, sha256_hash.to_vec());
    
    assert_eq!(result.algorithm, HashAlgorithm::Sha256);
    assert_eq!(result.len(), 32);
    assert_eq!(result.to_hex().len(), 64);
    
    let display_str = format!("{}", result);
    assert!(display_str.contains("SHA-256"));
    assert!(display_str.contains(&result.to_hex()));
}

/// flex Test cross-validation with known implementations
#[test]
fn test_cross_validation() {
    common::tracing::setup();
    
    // These test vectors are from different authoritative sources
    // to ensure our implementation matches standard behavior
    
    let test_cases = vec![
        ("", "sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        ("The quick brown fox jumps over the lazy dog", "sha256", "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
        ("", "sha512", "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
        ("The quick brown fox jumps over the lazy dog", "sha512", "07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6"),
        ("", "md5", "d41d8cd98f00b204e9800998ecf8427e"),
        ("The quick brown fox jumps over the lazy dog", "md5", "9e107d9d372bb6826bd81d3542a419d6"),
    ];
    
    for (input, algorithm, expected) in test_cases {
        let result = match algorithm {
            "sha256" => HashUtils::to_hex(&Sha256::hash(input.as_bytes())),
            "sha512" => HashUtils::to_hex(&Sha512::hash(input.as_bytes())),
            "md5" => HashUtils::to_hex(&Md5::hash(input.as_bytes())),
            _ => panic!("Unknown algorithm: {}", algorithm),
        };
        
        assert_eq!(result, expected, 
            "Mismatch for {} with input '{}'", algorithm, input);
    }
}

/// bestie Test large file simulation
#[test]
fn test_large_file_hashing() {
    common::tracing::setup();
    
    // Simulate hashing a large file in chunks
    let chunk_size = 8192;
    let total_size = 1_000_000;
    let data_pattern = (0..256).cycle().take(chunk_size).collect::<Vec<u8>>();
    
    // Hash incrementally
    let mut sha256_hasher = Sha256::new();
    let mut sha512_hasher = Sha512::new();
    let mut md5_hasher = Md5::new();
    
    let mut bytes_processed = 0;
    while bytes_processed < total_size {
        let remaining = total_size - bytes_processed;
        let current_chunk_size = chunk_size.min(remaining);
        let chunk = &data_pattern[..current_chunk_size];
        
        sha256_hasher.update(chunk);
        sha512_hasher.update(chunk);
        md5_hasher.update(chunk);
        
        bytes_processed += current_chunk_size;
    }
    
    let sha256_incremental = sha256_hasher.finalize();
    let sha512_incremental = sha512_hasher.finalize();
    let md5_incremental = md5_hasher.finalize();
    
    // Compare with one-shot hashing of the same data
    let full_data = (0..256).cycle().take(total_size).collect::<Vec<u8>>();
    let sha256_oneshot = Sha256::hash(&full_data);
    let sha512_oneshot = Sha512::hash(&full_data);
    let md5_oneshot = Md5::hash(&full_data);
    
    assert_eq!(sha256_incremental, sha256_oneshot);
    assert_eq!(sha512_incremental, sha512_oneshot);
    assert_eq!(md5_incremental, md5_oneshot);
}

/// periodt Test memory safety and bounds checking
#[test]
fn test_memory_safety() {
    common::tracing::setup();
    
    // Test with various input patterns that might expose memory issues
    let test_patterns = vec![
        vec![0x00; 1000],
        vec![0xFF; 1000], 
        (0..1000).map(|i| i as u8).collect(),
        (0..1000).map(|i| (i * 7 + 13) as u8).collect(),
    ];
    
    for pattern in test_patterns {
        // Test all hash functions don't crash or have memory issues
        let _sha256 = Sha256::hash(&pattern);
        let _sha512 = Sha512::hash(&pattern);
        let _md5 = Md5::hash(&pattern);
        
        // Test incremental hashing doesn't have issues
        let mut sha256_hasher = Sha256::new();
        for chunk in pattern.chunks(37) { // Odd chunk size to test boundaries
            sha256_hasher.update(chunk);
        }
        let _result = sha256_hasher.finalize();
    }
}
