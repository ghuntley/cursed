/// Comprehensive test suite for the CURSED Advanced Cryptographic Hash Package
#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use cursed::stdlib::packages::crypto_hash_advanced::*;
use cursed::error::CursedError;
use std::time::Duration;

#[test]
fn test_hash_traits_registry() {
    tracing_setup::init_test_tracing();
    
    let registry = HashRegistry::new();
    
    // Test algorithm lookup
    let sha256_info = registry.get_algorithm("SHA-256").unwrap();
    assert_eq!(sha256_info.name, "SHA-256");
    assert_eq!(sha256_info.digest_size, 32);
    assert!(sha256_info.is_cryptographic);
    
    // Test algorithm categories
    let crypto_algs = registry.cryptographic_algorithms();
    assert!(crypto_algs.len() > 0);
    assert!(crypto_algs.iter().all(|alg| alg.is_cryptographic));
    
    let fast_algs = registry.fast_algorithms();
    assert!(fast_algs.len() > 0);
    assert!(fast_algs.iter().all(|alg| !alg.is_cryptographic));
}

#[test]
fn test_xxhash_functionality() {
    tracing_setup::init_test_tracing();
    
    // Test xxHash64
    let mut hasher = XxHash64::new();
    assert_eq!(hasher.algorithm(), "xxHash64");
    assert_eq!(hasher.digest_size(), 8);
    
    let result1 = hasher.hash(b"test data");
    assert_eq!(result1.len(), 8);
    
    // Test deterministic
    let result2 = hasher.hash(b"test data");
    assert_eq!(result1, result2);
    
    // Test different data
    let result3 = hasher.hash(b"different data");
    assert_ne!(result1, result3);
    
    // Test with seed
    let hasher_seeded = XxHash64::with_seed(12345);
    let result_seeded = hasher_seeded.hash(b"test data");
    assert_ne!(result1, result_seeded);
    
    // Test convenience function
    let conv_result = xxhash64(b"test", 0);
    assert_ne!(conv_result, 0);
}

#[test]
fn test_siphash_functionality() {
    tracing_setup::init_test_tracing();
    
    let key = [42u8; 16];
    let mut hasher = SipHash::new(&key);
    
    assert_eq!(hasher.algorithm(), "SipHash-2-4");
    assert_eq!(hasher.digest_size(), 8);
    
    let result = hasher.hash(b"test message");
    assert_eq!(result.len(), 8);
    
    // Test key sensitivity
    let key2 = [43u8; 16];
    let mut hasher2 = SipHash::new(&key2);
    let result2 = hasher2.hash(b"test message");
    assert_ne!(result, result2);
    
    // Test keyed hasher trait
    let mut keyed_hasher = hasher.clone();
    assert_eq!(keyed_hasher.key_length(), 16);
    keyed_hasher.set_key(&key2).unwrap();
    let result3 = keyed_hasher.hash(b"test message");
    assert_eq!(result2, result3);
    
    // Test convenience functions
    let conv_result = siphash_24(b"data", &key);
    let conv_result_13 = siphash_13(b"data", &key);
    assert_ne!(conv_result, conv_result_13);
}

#[test]
fn test_keccak_variants() {
    tracing_setup::init_test_tracing();
    
    // Test different Keccak variants
    let data = b"test data for keccak";
    
    let keccak224 = keccak224(data);
    let keccak256 = keccak256(data);
    let keccak384 = keccak384(data);
    let keccak512 = keccak512(data);
    
    assert_eq!(keccak224.len(), 28);
    assert_eq!(keccak256.len(), 32);
    assert_eq!(keccak384.len(), 48);
    assert_eq!(keccak512.len(), 64);
    
    // All should be different
    assert_ne!(keccak224, keccak256[..28]);
    assert_ne!(keccak256, keccak384[..32]);
    assert_ne!(keccak384, keccak512[..48]);
    
    // Test Ethereum compatibility
    let eth_hash = ethereum_keccak256(b"hello");
    assert_eq!(eth_hash.len(), 32);
    assert_eq!(eth_hash.to_vec(), keccak256);
    
    // Test hasher interface
    let mut hasher = KeccakHasher::keccak256();
    assert_eq!(hasher.algorithm(), "Keccak-256");
    assert_eq!(hasher.digest_size(), 32);
    
    let incremental_result = {
        hasher.update(b"test ");
        hasher.update(b"data for ");
        hasher.update(b"keccak");
        hasher.clone().finalize()
    };
    assert_eq!(incremental_result, keccak256);
}

#[test]
fn test_shake_extendable_output() {
    tracing_setup::init_test_tracing();
    
    // Test SHAKE128
    let mut shake128 = Shake128::new();
    shake128.update(b"test input");
    
    let output_32 = shake128.read(32);
    assert_eq!(output_32.len(), 32);
    
    let mut shake128_2 = Shake128::new();
    shake128_2.update(b"test input");
    let output_64 = shake128_2.read(64);
    assert_eq!(output_64.len(), 64);
    
    // First 32 bytes should match
    assert_eq!(output_32, output_64[..32]);
    
    // Test SHAKE256
    let mut shake256 = Shake256::new();
    shake256.update(b"test input");
    let shake256_output = shake256.read(32);
    assert_eq!(shake256_output.len(), 32);
    assert_ne!(shake256_output, output_32); // Different algorithms
}

#[test]
fn test_collision_resistance_analysis() {
    tracing_setup::init_test_tracing();
    
    let analyzer = CollisionAnalyzer::with_limits(1000, Duration::from_millis(100));
    let hasher = XxHash64::new();
    
    let result = analyzer.analyze_collisions(hasher).unwrap();
    assert_eq!(result.algorithm, "xxHash64");
    assert!(result.total_hashes > 0);
    assert!(result.analysis_duration.as_millis() > 0);
    
    // For a good hash function with small test, shouldn't find many collisions
    assert!(result.collision_rate < 0.1);
}

#[test]
fn test_hash_validation() {
    tracing_setup::init_test_tracing();
    
    let data = b"important data to validate";
    let hasher = XxHash64::new();
    
    // Create integrity checker
    let checker = HashIntegrityChecker::new(hasher.clone(), data);
    
    // Verify correct data
    let result = checker.verify(hasher.clone(), data).unwrap();
    assert!(result.status.is_valid());
    assert_eq!(result.data_size, data.len());
    
    // Verify tampered data
    let tampered_data = b"tampered data to validate";
    let tampered_result = checker.verify(hasher, tampered_data).unwrap();
    assert!(!tampered_result.status.is_valid());
    
    // Test format validation
    assert!(validate_hash_format(&[1, 2, 3, 4], 4).is_ok());
    assert!(validate_hash_format(&[1, 2, 3], 4).is_err());
    assert!(validate_hash_format(&[0, 0, 0, 0], 4).is_err());
}

#[test]
fn test_merkle_tree() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let mut tree = MerkleTree::new(hasher);
    
    // Add leaves
    tree.add_leaf(b"leaf1");
    tree.add_leaf(b"leaf2");
    tree.add_leaf(b"leaf3");
    tree.add_leaf(b"leaf4");
    
    // Build tree
    let root = tree.build().unwrap();
    assert_eq!(root.len(), 8);
    assert_eq!(tree.leaf_count(), 4);
    
    // Generate proof
    let proof = tree.generate_proof(1).unwrap();
    assert_eq!(proof.leaf_data, b"leaf2");
    
    // Verify proof
    assert!(tree.verify_proof(&proof));
    
    // Test proof serialization
    let proof_bytes = proof.to_bytes();
    let parsed_proof = MerkleProof::from_bytes(&proof_bytes).unwrap();
    assert!(tree.verify_proof(&parsed_proof));
    
    // Update leaf
    tree.update_leaf(1, b"updated_leaf2").unwrap();
    let new_root = tree.root().unwrap();
    assert_ne!(root, new_root);
}

#[test]
fn test_password_hashing() {
    tracing_setup::init_test_tracing();
    
    let password = "my_secure_password_123!";
    
    // Test with different configurations
    let configs = [
        PasswordConfig::fast(),
        PasswordConfig::interactive(),
    ];
    
    for config in &configs {
        let hasher = PasswordHasher::new(config.clone());
        
        // Hash password
        let hash_result = hasher.hash_password(password).unwrap();
        assert!(!hash_result.hash.is_empty());
        assert!(!hash_result.salt.is_empty());
        assert_eq!(hash_result.algorithm, config.algorithm);
        
        // Verify correct password
        assert!(hasher.verify_password(password, &hash_result).unwrap());
        
        // Verify incorrect password
        assert!(!hasher.verify_password("wrong_password", &hash_result).unwrap());
        
        // Test PHC string format
        let phc_string = hash_result.to_string();
        assert!(phc_string.starts_with("$"));
        
        let parsed = PasswordHash::from_string(&phc_string).unwrap();
        assert_eq!(parsed.algorithm, hash_result.algorithm);
    }
}

#[test]
fn test_password_strength_analysis() {
    tracing_setup::init_test_tracing();
    
    let test_passwords = [
        ("123", PasswordStrengthLevel::VeryWeak),
        ("password", PasswordStrengthLevel::VeryWeak),
        ("Password123", PasswordStrengthLevel::Weak),
        ("MyStr0ng!P@ssw0rd", PasswordStrengthLevel::Strong),
    ];
    
    for (password, expected_min_level) in &test_passwords {
        let strength = PasswordStrengthAnalyzer::analyze(password);
        
        // Check that strength is at least the expected level
        let level_value = match strength.level {
            PasswordStrengthLevel::VeryWeak => 0,
            PasswordStrengthLevel::Weak => 1, 
            PasswordStrengthLevel::Moderate => 2,
            PasswordStrengthLevel::Strong => 3,
            PasswordStrengthLevel::VeryStrong => 4,
        };
        
        let expected_value = match expected_min_level {
            PasswordStrengthLevel::VeryWeak => 0,
            PasswordStrengthLevel::Weak => 1,
            PasswordStrengthLevel::Moderate => 2,
            PasswordStrengthLevel::Strong => 3,
            PasswordStrengthLevel::VeryStrong => 4,
        };
        
        assert!(level_value >= expected_value, 
               "Password '{}' expected at least {:?}, got {:?}", 
               password, expected_min_level, strength.level);
        
        assert!(strength.estimated_crack_time.as_nanos() > 0);
        assert!(!strength.feedback.is_empty() || strength.level == PasswordStrengthLevel::VeryStrong);
    }
}

#[test]
fn test_hmac_variants() {
    tracing_setup::init_test_tracing();
    
    let key = b"test_hmac_key";
    let data = b"data to authenticate";
    
    // Test different HMAC variants
    let sha256_mac = hmac_sha256(key, data);
    let sha512_mac = hmac_sha512(key, data);
    let blake3_mac = hmac_blake3(key, data);
    let keccak_mac = hmac_keccak256(key, data);
    
    assert_eq!(sha256_mac.len(), 32);
    assert_eq!(sha512_mac.len(), 64);
    assert_eq!(blake3_mac.len(), 32);
    assert_eq!(keccak_mac.len(), 32);
    
    // All should be different
    assert_ne!(sha256_mac, blake3_mac);
    assert_ne!(sha256_mac, keccak_mac);
    assert_ne!(blake3_mac, keccak_mac);
    
    // Test verification
    assert!(verify_hmac_sha256(key, data, &sha256_mac));
    assert!(!verify_hmac_sha256(b"wrong_key", data, &sha256_mac));
    assert!(!verify_hmac_sha256(key, b"wrong_data", &sha256_mac));
}

#[test]
fn test_performance_benchmarking() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let benchmark = HashBenchmark::with_custom_sizes(vec![64, 1024]);
    
    let metrics = benchmark.benchmark(hasher).unwrap();
    
    assert_eq!(metrics.algorithm, "xxHash64");
    assert!(metrics.throughput_bytes_per_second > 0.0);
    assert!(metrics.throughput_hashes_per_second > 0.0);
    assert!(metrics.latency_per_hash.as_nanos() > 0);
    assert!(metrics.efficiency_score >= 0.0);
    assert!(metrics.efficiency_score <= 100.0);
    
    // Test performance monitor
    let mut monitor = PerformanceMonitor::new(XxHash64::new());
    let _result = monitor.record_operation(b"test data");
    let stats = monitor.get_stats().unwrap();
    assert_eq!(stats.sample_count, 1);
}

#[test]
fn test_comprehensive_collision_analysis() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let report = comprehensive_collision_test(hasher).unwrap();
    
    assert_eq!(report.algorithm, "xxHash64");
    assert_eq!(report.digest_size, 8);
    assert!(!report.recommendations.is_empty());
    
    // Should complete basic analysis
    assert!(report.basic_analysis.total_hashes > 0);
    assert!(report.birthday_analysis.attempts_made > 0);
    assert!(!report.pattern_analysis.pattern_results.is_empty());
}

#[test]
fn test_quick_hash_convenience() {
    tracing_setup::init_test_tracing();
    
    let data = b"test data for quick hash";
    
    let algorithms = ["blake3", "sha3-256", "keccak256", "xxhash64", "siphash"];
    
    for algorithm in &algorithms {
        let result = quick_hash(algorithm, data).unwrap();
        assert!(!result.is_empty());
        
        // Should be deterministic
        let result2 = quick_hash(algorithm, data).unwrap();
        assert_eq!(result, result2);
    }
    
    // Test invalid algorithm
    assert!(quick_hash("invalid_algorithm", data).is_err());
}

#[test]
fn test_hash_password_convenience() {
    tracing_setup::init_test_tracing();
    
    let password = "test_password_123";
    
    // Test convenience functions
    let hash_result = hash_password(password).unwrap();
    assert!(!hash_result.hash.is_empty());
    assert!(!hash_result.salt.is_empty());
    
    let is_valid = verify_password(password, &hash_result).unwrap();
    assert!(is_valid);
    
    let is_invalid = verify_password("wrong_password", &hash_result).unwrap();
    assert!(!is_invalid);
}

#[test]
fn test_streaming_merkle_tree() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let mut streaming_tree = StreamingMerkleTree::new(hasher);
    
    // Add data items
    streaming_tree.add_data(b"item1").unwrap();
    streaming_tree.add_data(b"item2").unwrap();
    streaming_tree.add_data(b"item3").unwrap();
    streaming_tree.add_data(b"item4").unwrap();
    
    // Finalize and get root
    let root = streaming_tree.finalize().unwrap();
    assert_eq!(root.len(), 8);
}

#[test]
fn test_binary_hash_tree() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let mut tree = BinaryHashTree::new(hasher, 2); // 4 leaves max
    
    // Set leaves
    tree.set_leaf(0, b"leaf0").unwrap();
    tree.set_leaf(1, b"leaf1").unwrap();
    tree.set_leaf(2, b"leaf2").unwrap();
    tree.set_leaf(3, b"leaf3").unwrap();
    
    let root = tree.root().unwrap();
    assert_eq!(root.len(), 8);
    
    // Test bounds checking
    assert!(tree.set_leaf(4, b"overflow").is_err());
}

#[test]
fn test_advanced_mac_algorithms() {
    tracing_setup::init_test_tracing();
    
    let key = b"test_key_for_mac";
    let data = b"data to authenticate";
    
    // Test CMAC
    let cmac = CmacEngine::new(key);
    let cmac_result = cmac.compute(data);
    assert_eq!(cmac_result.len(), 16);
    assert!(cmac.verify(data, &cmac_result));
    
    // Test GMAC
    let auth_key = b"auth_key_for_gmac";
    let aad = b"additional authenticated data";
    let gmac = GmacEngine::new(key, auth_key);
    let gmac_result = gmac.compute(data, aad);
    assert_eq!(gmac_result.len(), 16);
    assert!(gmac.verify(data, aad, &gmac_result));
    
    // Test PMAC
    let pmac = PmacEngine::new(key);
    let pmac_result = pmac.compute(data);
    assert_eq!(pmac_result.len(), 16);
    assert!(pmac.verify(data, &pmac_result));
}

#[test]
fn test_hash_chain_validation() {
    tracing_setup::init_test_tracing();
    
    let hasher = XxHash64::new();
    let mut validator = HashChainValidator::new(hasher.clone());
    
    // Create a simple hash chain
    let block1 = HashChainBlock {
        id: "block1".to_string(),
        data: b"genesis block".to_vec(),
        hash: hasher.clone().hash(b"genesis block"),
    };
    
    let mut combined = block1.hash.clone();
    combined.extend_from_slice(b"second block");
    let block2 = HashChainBlock {
        id: "block2".to_string(),
        data: b"second block".to_vec(),
        hash: hasher.clone().hash(&combined),
    };
    
    let chain = vec![block1, block2];
    let result = validator.validate_chain(&chain).unwrap();
    
    assert_eq!(result.total_blocks, 2);
    assert!(result.overall_status.is_valid());
    assert_eq!(result.invalid_blocks.len(), 0);
}

#[test]
fn test_hex_conversion_utilities() {
    tracing_setup::init_test_tracing();
    
    let test_data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    
    // Test hash to hex
    let hex_string = hash_to_hex(&test_data);
    assert_eq!(hex_string, "0123456789abcdef");
    
    // Test hex to hash
    let parsed_data = hex_to_hash(&hex_string).unwrap();
    assert_eq!(parsed_data, test_data);
    
    // Test invalid hex
    assert!(hex_to_hash("invalid_hex").is_err());
    assert!(hex_to_hash("123").is_err()); // Odd length
}

#[test]
fn test_package_initialization() {
    tracing_setup::init_test_tracing();
    
    // Test package initialization
    assert!(init_crypto_hash_advanced().is_ok());
    
    // Test supported algorithms
    let algorithms = get_supported_algorithms();
    assert!(!algorithms.is_empty());
    
    let crypto_count = algorithms.iter()
        .filter(|alg| alg.is_cryptographic)
        .count();
    assert!(crypto_count > 0);
    
    let fast_count = algorithms.iter()
        .filter(|alg| !alg.is_cryptographic)
        .count();
    assert!(fast_count > 0);
}

#[test]
fn test_large_data_performance() {
    tracing_setup::init_test_tracing();
    
    // Test with larger data to ensure scalability
    let large_data = vec![42u8; 100_000]; // 100KB
    
    let algorithms = [
        ("xxhash64", XxHash64::new()),
    ];
    
    for (name, mut hasher) in algorithms {
        let start = std::time::Instant::now();
        let result = hasher.hash(&large_data);
        let duration = start.elapsed();
        
        assert!(!result.is_empty());
        assert!(duration.as_millis() < 1000, 
               "{} took too long: {:?}", name, duration);
        
        println!("{} processed 100KB in {:?}", name, duration);
    }
}

#[test]
fn test_security_levels() {
    tracing_setup::init_test_tracing();
    
    // Test security level ordering
    assert!(SecurityLevel::None < SecurityLevel::Weak);
    assert!(SecurityLevel::Weak < SecurityLevel::Strong);
    assert!(SecurityLevel::Strong < SecurityLevel::VeryStrong);
    assert!(SecurityLevel::VeryStrong < SecurityLevel::QuantumResistant);
    
    // Test descriptions
    assert!(!SecurityLevel::Strong.to_string().is_empty());
    assert!(SecurityLevel::Strong.bits() >= 128);
}

#[test]
fn test_constant_time_operations() {
    tracing_setup::init_test_tracing();
    
    let data1 = b"hello world";
    let data2 = b"hello world";
    let data3 = b"hello worle"; // Different by one character
    
    assert!(constant_time_eq(data1, data2));
    assert!(!constant_time_eq(data1, data3));
    assert!(!constant_time_eq(data1, b"short"));
    
    // Test secure zero
    let mut sensitive_data = vec![1, 2, 3, 4, 5];
    secure_zero(&mut sensitive_data);
    assert_eq!(sensitive_data, vec![0, 0, 0, 0, 0]);
}
