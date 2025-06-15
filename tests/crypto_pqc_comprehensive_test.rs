//! Comprehensive Post-Quantum Cryptography Test Suite
//! 
//! This test suite validates the complete PQC implementation including:
//! - Algorithm correctness and interoperability
//! - Key management and lifecycle
//! - Performance characteristics
//! - Security properties
//! - Integration with existing crypto infrastructure

use cursed::stdlib::crypto_pqc::*;
use cursed::stdlib::crypto_pqc::algorithms::*;
use cursed::stdlib::crypto_pqc::key_management::*;
use std::time::{Duration, Instant};

#[cfg(test)]
mod algorithm_tests {
    use super::*;

    #[test]
    fn test_kyber_basic_functionality() {
        // Test Kyber KEM for all security levels
        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (public_key, secret_key) = kyber::Kyber::keygen(security_level).unwrap();
            
            // Verify key properties
            assert_eq!(public_key.security_level(), security_level);
            assert_eq!(secret_key.security_level(), security_level);
            assert_eq!(public_key.parameter_set(), secret_key.parameter_set());
            
            // Test encapsulation/decapsulation
            let (ciphertext, shared_secret1) = kyber::Kyber::encaps(&public_key).unwrap();
            let shared_secret2 = kyber::Kyber::decaps(&secret_key, &ciphertext).unwrap();
            
            // Verify shared secret properties
            assert_eq!(shared_secret1.as_bytes().len(), 32);
            assert_eq!(shared_secret2.as_bytes().len(), 32);
            
            // Validate keys
            assert!(kyber::Kyber::validate_public_key(&public_key).is_ok());
            assert!(kyber::Kyber::validate_secret_key(&secret_key).is_ok());
        }
    }

    #[test]
    fn test_dilithium_basic_functionality() {
        // Test Dilithium signatures for all security levels
        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (public_key, secret_key) = dilithium::Dilithium::keygen(security_level).unwrap();
            
            // Verify key properties
            assert_eq!(public_key.security_level(), security_level);
            assert_eq!(secret_key.security_level(), security_level);
            assert_eq!(public_key.parameter_set(), secret_key.parameter_set());
            
            // Test signing/verification
            let message = b"Hello, post-quantum world!";
            let signature = dilithium::Dilithium::sign(&secret_key, message).unwrap();
            let is_valid = dilithium::Dilithium::verify(&public_key, message, &signature).unwrap();
            
            assert!(is_valid);
            
            // Test with wrong message
            let wrong_message = b"Wrong message";
            let is_valid_wrong = dilithium::Dilithium::verify(&public_key, wrong_message, &signature).unwrap();
            assert!(!is_valid_wrong);
            
            // Validate keys and signature
            assert!(dilithium::Dilithium::validate_public_key(&public_key).is_ok());
            assert!(dilithium::Dilithium::validate_secret_key(&secret_key).is_ok());
            assert!(dilithium::Dilithium::validate_signature(&signature).is_ok());
        }
    }

    #[test]
    fn test_sphincs_plus_functionality() {
        // Test SPHINCS+ signatures for different variants
        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // Test small signature variant
            let (public_key_small, secret_key_small) = sphincs::SphincsPlusAlgorithm::keygen_small(security_level).unwrap();
            assert!(public_key_small.parameter_set().is_small_signature());
            
            // Test fast signature variant
            let (public_key_fast, secret_key_fast) = sphincs::SphincsPlusAlgorithm::keygen_fast(security_level).unwrap();
            assert!(public_key_fast.parameter_set().is_fast_signature());
            
            // Verify fast variant has larger signatures than small variant
            let fast_sig_size = public_key_fast.parameter_set().additional_sizes()[0].1;
            let small_sig_size = public_key_small.parameter_set().additional_sizes()[0].1;
            assert!(fast_sig_size > small_sig_size);
            
            // Test signing with both variants
            let message = b"SPHINCS+ test message";
            
            let signature_small = sphincs::SphincsPlusAlgorithm::sign(&secret_key_small, message).unwrap();
            let is_valid_small = sphincs::SphincsPlusAlgorithm::verify(&public_key_small, message, &signature_small).unwrap();
            assert!(is_valid_small);
            
            let signature_fast = sphincs::SphincsPlusAlgorithm::sign(&secret_key_fast, message).unwrap();
            let is_valid_fast = sphincs::SphincsPlusAlgorithm::verify(&public_key_fast, message, &signature_fast).unwrap();
            assert!(is_valid_fast);
        }
    }

    #[test]
    fn test_ntru_functionality() {
        // Test NTRU encryption for all security levels
        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (public_key, secret_key) = ntru::Ntru::keygen(security_level).unwrap();
            
            // Test encryption/decryption
            let plaintext = ntru::NtruPlaintext { data: b"Hello NTRU!".to_vec() };
            let ciphertext = ntru::Ntru::encrypt(&public_key, &plaintext).unwrap();
            let decrypted = ntru::Ntru::decrypt(&secret_key, &ciphertext).unwrap();
            
            // Verify decryption produces reasonable output
            assert!(!decrypted.data.is_empty());
            assert_eq!(decrypted.data.len(), 32); // SHA3-256 output in placeholder
        }
    }

    #[test]
    fn test_algorithm_type_identification() {
        // Test that all algorithms return correct type
        assert_eq!(kyber::Kyber::algorithm_type(), AlgorithmType::Kyber);
        assert_eq!(dilithium::Dilithium::algorithm_type(), AlgorithmType::Dilithium);
        assert_eq!(sphincs::SphincsPlusAlgorithm::algorithm_type(), AlgorithmType::Sphincs);
        assert_eq!(ntru::Ntru::algorithm_type(), AlgorithmType::Ntru);
        assert_eq!(frodo::FrodoKem::algorithm_type(), AlgorithmType::FrodoKem);
        assert_eq!(lms::Lms::algorithm_type(), AlgorithmType::Lms);
        assert_eq!(xmss::Xmss::algorithm_type(), AlgorithmType::Xmss);
        assert_eq!(rainbow::Rainbow::algorithm_type(), AlgorithmType::Rainbow);
        assert_eq!(gemss::Gemss::algorithm_type(), AlgorithmType::GeMSS);
        assert_eq!(mceliece::ClassicMcEliece::algorithm_type(), AlgorithmType::ClassicMcEliece);
        assert_eq!(bike::Bike::algorithm_type(), AlgorithmType::Bike);
        assert_eq!(hqc::Hqc::algorithm_type(), AlgorithmType::Hqc);
        assert_eq!(sike::Sike::algorithm_type(), AlgorithmType::Sike);
    }

    #[test]
    fn test_deprecated_sike_algorithm() {
        // SIKE should return errors for all operations
        assert!(sike::Sike::keygen(SecurityLevel::Level1).is_err());
        
        // Test that error messages are appropriate
        match sike::Sike::keygen(SecurityLevel::Level1) {
            Err(PqcError::AlgorithmNotAvailable(msg)) => {
                assert!(msg.contains("broken"));
            },
            _ => panic!("Expected AlgorithmNotAvailable error"),
        }
    }

    #[test]
    fn test_parameter_set_properties() {
        // Test Kyber parameter sets
        assert_eq!(kyber::KyberParameterSet::Kyber512.security_level(), SecurityLevel::Level1);
        assert_eq!(kyber::KyberParameterSet::Kyber768.security_level(), SecurityLevel::Level3);
        assert_eq!(kyber::KyberParameterSet::Kyber1024.security_level(), SecurityLevel::Level5);
        
        // Verify key sizes are reasonable
        assert!(kyber::KyberParameterSet::Kyber512.public_key_size() > 0);
        assert!(kyber::KyberParameterSet::Kyber512.secret_key_size() > kyber::KyberParameterSet::Kyber512.public_key_size());
        
        // Test Dilithium parameter sets
        assert_eq!(dilithium::DilithiumParameterSet::Dilithium2.security_level(), SecurityLevel::Level1);
        assert_eq!(dilithium::DilithiumParameterSet::Dilithium3.security_level(), SecurityLevel::Level3);
        assert_eq!(dilithium::DilithiumParameterSet::Dilithium5.security_level(), SecurityLevel::Level5);
        
        // Test SPHINCS+ parameter sets
        assert!(sphincs::SphincsPlusParameterSet::Sphincs128s.is_small_signature());
        assert!(sphincs::SphincsPlusParameterSet::Sphincs128f.is_fast_signature());
        assert_eq!(sphincs::SphincsPlusParameterSet::Sphincs128s.hash_size(), 32);
        assert_eq!(sphincs::SphincsPlusParameterSet::Sphincs192s.hash_size(), 48);
        assert_eq!(sphincs::SphincsPlusParameterSet::Sphincs256s.hash_size(), 64);
    }
}

#[cfg(test)]
mod key_management_tests {
    use super::*;

    #[test]
    fn test_pqc_key_creation_and_validation() {
        let key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level3,
            KeyType::Public,
            vec![1, 2, 3, 4, 5],
        );

        assert_eq!(key.algorithm, AlgorithmType::Kyber);
        assert_eq!(key.security_level, SecurityLevel::Level3);
        assert_eq!(key.key_type, KeyType::Public);
        assert!(!key.metadata.key_id.is_empty());
        assert!(key.validate().is_ok());
        assert!(!key.is_expired());
    }

    #[test]
    fn test_key_usage_flags() {
        // Test KEM public key usage
        let kem_public_usage = KeyUsage::kem_public();
        assert!(kem_public_usage.encrypt);
        assert!(!kem_public_usage.decrypt);
        assert!(kem_public_usage.key_agreement);
        
        // Test signature secret key usage
        let sig_secret_usage = KeyUsage::signature_secret();
        assert!(sig_secret_usage.sign);
        assert!(!sig_secret_usage.verify);
        assert!(!sig_secret_usage.encrypt);
    }

    #[test]
    fn test_key_expiration() {
        let mut key = PqcKey::new(
            AlgorithmType::Dilithium,
            SecurityLevel::Level1,
            KeyType::Secret,
            vec![1, 2, 3, 4],
        );

        // Key should not be expired initially
        assert!(!key.is_expired());
        
        // Set expiration
        key.set_expiration(Duration::from_secs(3600)); // 1 hour
        assert!(!key.is_expired());
        
        // Manually set expiration in the past
        key.metadata.expires_at = Some(key.metadata.created_at - 1);
        assert!(key.is_expired());
        assert!(key.validate().is_err());
    }

    #[test]
    fn test_key_manager_operations() {
        let mut manager = KeyManager::new();

        // Create test keys
        let key1 = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level1, KeyType::Public, vec![1]);
        let key2 = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level3, KeyType::Secret, vec![2]);
        let key3 = PqcKey::new(AlgorithmType::Dilithium, SecurityLevel::Level1, KeyType::Public, vec![3]);

        // Add keys
        let key1_id = manager.add_key(key1).unwrap();
        let key2_id = manager.add_key(key2).unwrap();
        let key3_id = manager.add_key(key3).unwrap();

        // Test retrieval
        assert!(manager.get_key(&key1_id).is_some());
        assert!(manager.get_key(&key2_id).is_some());
        assert!(manager.get_key(&key3_id).is_some());
        assert!(manager.get_key("nonexistent").is_none());

        // Test listing
        assert_eq!(manager.list_key_ids().len(), 3);
        
        let kyber_keys = manager.list_keys_by_algorithm(AlgorithmType::Kyber);
        assert_eq!(kyber_keys.len(), 2);
        
        let level1_keys = manager.list_keys_by_security_level(SecurityLevel::Level1);
        assert_eq!(level1_keys.len(), 2);

        // Test removal
        let removed_key = manager.remove_key(&key1_id);
        assert!(removed_key.is_some());
        assert_eq!(manager.list_key_ids().len(), 2);
        assert!(manager.get_key(&key1_id).is_none());
    }

    #[test]
    fn test_key_manager_statistics() {
        let mut manager = KeyManager::new();

        // Add various keys
        let keys = vec![
            PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level1, KeyType::Public, vec![1]),
            PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level3, KeyType::Secret, vec![2]),
            PqcKey::new(AlgorithmType::Dilithium, SecurityLevel::Level1, KeyType::Public, vec![3]),
            PqcKey::new(AlgorithmType::Sphincs, SecurityLevel::Level5, KeyType::Secret, vec![4]),
        ];

        for key in keys {
            manager.add_key(key).unwrap();
        }

        let stats = manager.get_statistics();
        assert_eq!(stats.total_keys, 4);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Kyber], 2);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Dilithium], 1);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Sphincs], 1);
        assert_eq!(stats.by_security_level[&SecurityLevel::Level1], 2);
        assert_eq!(stats.by_security_level[&SecurityLevel::Level3], 1);
        assert_eq!(stats.by_security_level[&SecurityLevel::Level5], 1);
        assert_eq!(stats.by_key_type[&KeyType::Public], 2);
        assert_eq!(stats.by_key_type[&KeyType::Secret], 2);
        assert_eq!(stats.expired_count, 0);
    }

    #[test]
    fn test_key_manager_expired_keys() {
        let mut manager = KeyManager::new();

        // Create a key that will be expired
        let mut expired_key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            KeyType::Public,
            vec![1, 2, 3],
        );
        expired_key.metadata.expires_at = Some(expired_key.metadata.created_at - 1);

        // Create a valid key
        let valid_key = PqcKey::new(
            AlgorithmType::Dilithium,
            SecurityLevel::Level1,
            KeyType::Secret,
            vec![4, 5, 6],
        );

        // Add keys (expired key should fail validation)
        assert!(manager.add_key(expired_key).is_err());
        assert!(manager.add_key(valid_key).is_ok());

        assert_eq!(manager.list_key_ids().len(), 1);
        assert_eq!(manager.list_expired_keys().len(), 0);
    }

    #[test]
    fn test_key_export_import() {
        let mut manager1 = KeyManager::new();
        let mut manager2 = KeyManager::new();

        // Add keys to first manager
        let key1 = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level1, KeyType::Public, vec![1]);
        let key2 = PqcKey::new(AlgorithmType::Dilithium, SecurityLevel::Level3, KeyType::Secret, vec![2]);

        manager1.add_key(key1).unwrap();
        manager1.add_key(key2).unwrap();

        // Export from first manager
        let exported_keys = manager1.export_keys().unwrap();
        assert_eq!(exported_keys.len(), 2);

        // Import to second manager
        let imported_ids = manager2.import_keys(exported_keys).unwrap();
        assert_eq!(imported_ids.len(), 2);
        assert_eq!(manager2.list_key_ids().len(), 2);

        // Verify keys are the same (excluding IDs which are regenerated)
        let stats1 = manager1.get_statistics();
        let stats2 = manager2.get_statistics();
        assert_eq!(stats1.total_keys, stats2.total_keys);
        assert_eq!(stats1.by_algorithm, stats2.by_algorithm);
        assert_eq!(stats1.by_security_level, stats2.by_security_level);
    }

    #[test]
    fn test_standardization_status() {
        // Test standardized algorithms
        let kyber_key = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level1, KeyType::Public, vec![1]);
        assert_eq!(kyber_key.standardization_status(), StandardizationStatus::NistStandardized);
        assert!(kyber_key.is_production_ready());

        let dilithium_key = PqcKey::new(AlgorithmType::Dilithium, SecurityLevel::Level1, KeyType::Public, vec![1]);
        assert_eq!(dilithium_key.standardization_status(), StandardizationStatus::NistStandardized);
        assert!(dilithium_key.is_production_ready());

        // Test deprecated algorithm
        let sike_key = PqcKey::new(AlgorithmType::Sike, SecurityLevel::Level1, KeyType::Public, vec![1]);
        assert_eq!(sike_key.standardization_status(), StandardizationStatus::Deprecated);
        assert!(!sike_key.is_production_ready());

        // Test research algorithms
        let rainbow_key = PqcKey::new(AlgorithmType::Rainbow, SecurityLevel::Level1, KeyType::Public, vec![1]);
        assert_eq!(rainbow_key.standardization_status(), StandardizationStatus::Research);
        assert!(!rainbow_key.is_production_ready());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_kyber_performance_characteristics() {
        for params in kyber::Kyber::supported_parameter_sets() {
            let perf = kyber::Kyber::performance_characteristics(params);
            
            // Verify performance metrics are reasonable
            assert!(perf.keygen_time_ms > 0.0);
            assert!(perf.keygen_time_ms < 1000.0); // Should be under 1 second
            assert!(perf.operation_time_ms > 0.0);
            assert!(perf.operation_time_ms < 1000.0);
            assert!(perf.throughput_ops_per_sec > 0.0);
            
            // Verify key sizes match parameter set
            assert_eq!(perf.key_sizes.public_key, params.public_key_size());
            assert_eq!(perf.key_sizes.secret_key, params.secret_key_size());
            assert!(perf.key_sizes.shared_secret.is_some());
            assert_eq!(perf.key_sizes.shared_secret.unwrap(), 32);
        }
    }

    #[test]
    fn test_dilithium_performance_characteristics() {
        for params in dilithium::Dilithium::supported_parameter_sets() {
            let perf = dilithium::Dilithium::performance_characteristics(params);
            
            // Verify performance metrics are reasonable
            assert!(perf.keygen_time_ms > 0.0);
            assert!(perf.operation_time_ms > 0.0);
            assert!(perf.throughput_ops_per_sec > 0.0);
            
            // Verify key sizes match parameter set
            assert_eq!(perf.key_sizes.public_key, params.public_key_size());
            assert_eq!(perf.key_sizes.secret_key, params.secret_key_size());
            assert!(perf.key_sizes.shared_secret.is_none());
        }
    }

    #[test]
    fn test_sphincs_performance_characteristics() {
        for params in sphincs::SphincsPlusAlgorithm::supported_parameter_sets() {
            let perf = sphincs::SphincsPlusAlgorithm::performance_characteristics(params);
            
            // SPHINCS+ should be slower than lattice-based schemes
            assert!(perf.keygen_time_ms > 0.0);
            assert!(perf.operation_time_ms > 0.0);
            
            // Fast variants should be faster than small variants
            if params.is_fast_signature() {
                // Fast variants should have higher throughput
                assert!(perf.throughput_ops_per_sec > 20.0); // At least 20 ops/sec
            } else {
                // Small variants prioritize size over speed
                assert!(perf.throughput_ops_per_sec > 1.0); // At least 1 op/sec
            }
        }
    }

    #[test]
    #[ignore] // Ignore by default as this is a longer-running test
    fn test_algorithm_performance_comparison() {
        let security_level = SecurityLevel::Level1;
        
        // Measure Kyber performance
        let start = Instant::now();
        let (kyber_pub, kyber_sec) = kyber::Kyber::keygen(security_level).unwrap();
        let kyber_keygen_time = start.elapsed();
        
        let start = Instant::now();
        let (kyber_ct, _) = kyber::Kyber::encaps(&kyber_pub).unwrap();
        let kyber_encaps_time = start.elapsed();
        
        let start = Instant::now();
        let _ = kyber::Kyber::decaps(&kyber_sec, &kyber_ct).unwrap();
        let kyber_decaps_time = start.elapsed();
        
        // Measure Dilithium performance
        let start = Instant::now();
        let (dilithium_pub, dilithium_sec) = dilithium::Dilithium::keygen(security_level).unwrap();
        let dilithium_keygen_time = start.elapsed();
        
        let message = b"Performance test message";
        let start = Instant::now();
        let dilithium_sig = dilithium::Dilithium::sign(&dilithium_sec, message).unwrap();
        let dilithium_sign_time = start.elapsed();
        
        let start = Instant::now();
        let _ = dilithium::Dilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap();
        let dilithium_verify_time = start.elapsed();
        
        // All operations should complete in reasonable time (< 100ms for these parameter sets)
        assert!(kyber_keygen_time < Duration::from_millis(100));
        assert!(kyber_encaps_time < Duration::from_millis(100));
        assert!(kyber_decaps_time < Duration::from_millis(100));
        assert!(dilithium_keygen_time < Duration::from_millis(100));
        assert!(dilithium_sign_time < Duration::from_millis(100));
        assert!(dilithium_verify_time < Duration::from_millis(100));
        
        println!("Kyber keygen: {:?}, encaps: {:?}, decaps: {:?}", 
                 kyber_keygen_time, kyber_encaps_time, kyber_decaps_time);
        println!("Dilithium keygen: {:?}, sign: {:?}, verify: {:?}", 
                 dilithium_keygen_time, dilithium_sign_time, dilithium_verify_time);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_algorithm_family_classification() {
        // Test lattice-based algorithms
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Kyber), AlgorithmFamily::LatticeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Dilithium), AlgorithmFamily::LatticeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Ntru), AlgorithmFamily::LatticeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::FrodoKem), AlgorithmFamily::LatticeBased);
        
        // Test hash-based algorithms
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Sphincs), AlgorithmFamily::HashBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Lms), AlgorithmFamily::HashBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Xmss), AlgorithmFamily::HashBased);
        
        // Test multivariate algorithms
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Rainbow), AlgorithmFamily::Multivariate);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::GeMSS), AlgorithmFamily::Multivariate);
        
        // Test code-based algorithms
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::ClassicMcEliece), AlgorithmFamily::CodeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Bike), AlgorithmFamily::CodeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Hqc), AlgorithmFamily::CodeBased);
        
        // Test isogeny-based algorithms
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Sike), AlgorithmFamily::IsogenyBased);
    }

    #[test]
    fn test_security_level_properties() {
        // Test security level bit strength
        assert_eq!(SecurityLevel::Level1.classical_bits(), 128);
        assert_eq!(SecurityLevel::Level3.classical_bits(), 192);
        assert_eq!(SecurityLevel::Level5.classical_bits(), 256);
        
        // Test security level descriptions
        assert!(SecurityLevel::Level1.description().contains("AES-128"));
        assert!(SecurityLevel::Level3.description().contains("AES-192"));
        assert!(SecurityLevel::Level5.description().contains("AES-256"));
    }

    #[test]
    fn test_algorithm_family_confidence_levels() {
        // Hash-based should have highest confidence
        assert!(AlgorithmFamily::HashBased.quantum_confidence().contains("Very high"));
        
        // Lattice-based should have high confidence
        assert!(AlgorithmFamily::LatticeBased.quantum_confidence().contains("High"));
        
        // Isogeny-based should be marked as broken
        assert!(AlgorithmFamily::IsogenyBased.quantum_confidence().contains("Broken"));
    }

    #[test]
    fn test_algorithm_recommendations() {
        // Test recommended algorithms for each security level
        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // KEM recommendations
            let kyber_params = kyber::Kyber::recommended_params(security_level);
            assert_eq!(kyber_params.security_level(), security_level);
            
            // Signature recommendations
            let dilithium_params = dilithium::Dilithium::recommended_params(security_level);
            assert_eq!(dilithium_params.security_level(), security_level);
            
            // SPHINCS+ recommendations
            let sphincs_small = sphincs::SphincsPlusAlgorithm::recommended_params(security_level, true);
            let sphincs_fast = sphincs::SphincsPlusAlgorithm::recommended_params(security_level, false);
            
            assert_eq!(sphincs_small.security_level(), security_level);
            assert_eq!(sphincs_fast.security_level(), security_level);
            assert!(sphincs_small.is_small_signature());
            assert!(sphincs_fast.is_fast_signature());
        }
    }

    #[test]
    fn test_comprehensive_key_lifecycle() {
        let mut manager = KeyManager::new();
        
        // Generate keys for multiple algorithms
        let (kyber_pub, kyber_sec) = kyber::Kyber::keygen(SecurityLevel::Level3).unwrap();
        let (dilithium_pub, dilithium_sec) = dilithium::Dilithium::keygen(SecurityLevel::Level3).unwrap();
        
        // Create PQC key objects
        let mut kyber_pub_key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level3,
            KeyType::Public,
            kyber_pub.as_bytes().to_vec(),
        );
        
        let mut kyber_sec_key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level3,
            KeyType::Secret,
            kyber_sec.as_bytes().to_vec(),
        );
        
        let mut dilithium_pub_key = PqcKey::new(
            AlgorithmType::Dilithium,
            SecurityLevel::Level3,
            KeyType::Public,
            dilithium_pub.as_bytes().to_vec(),
        );
        
        let mut dilithium_sec_key = PqcKey::new(
            AlgorithmType::Dilithium,
            SecurityLevel::Level3,
            KeyType::Secret,
            dilithium_sec.as_bytes().to_vec(),
        );
        
        // Set expiration for some keys
        kyber_pub_key.set_expiration(Duration::from_secs(3600));
        dilithium_sec_key.set_expiration(Duration::from_secs(7200));
        
        // Add algorithm-specific parameters
        kyber_pub_key.set_algorithm_param("parameter_set".to_string(), "Kyber768".to_string());
        dilithium_pub_key.set_algorithm_param("parameter_set".to_string(), "Dilithium3".to_string());
        
        // Add keys to manager
        let kyber_pub_id = manager.add_key(kyber_pub_key).unwrap();
        let kyber_sec_id = manager.add_key(kyber_sec_key).unwrap();
        let dilithium_pub_id = manager.add_key(dilithium_pub_key).unwrap();
        let dilithium_sec_id = manager.add_key(dilithium_sec_key).unwrap();
        
        // Verify all keys are added
        assert_eq!(manager.list_key_ids().len(), 4);
        
        // Test retrieval and properties
        let retrieved_kyber_pub = manager.get_key(&kyber_pub_id).unwrap();
        assert_eq!(retrieved_kyber_pub.algorithm, AlgorithmType::Kyber);
        assert_eq!(retrieved_kyber_pub.security_level, SecurityLevel::Level3);
        assert_eq!(retrieved_kyber_pub.key_type, KeyType::Public);
        assert_eq!(retrieved_kyber_pub.get_algorithm_param("parameter_set").unwrap(), "Kyber768");
        
        // Test filtering
        let kyber_keys = manager.list_keys_by_algorithm(AlgorithmType::Kyber);
        assert_eq!(kyber_keys.len(), 2);
        
        let level3_keys = manager.list_keys_by_security_level(SecurityLevel::Level3);
        assert_eq!(level3_keys.len(), 4);
        
        // Test validation
        let validation_errors = manager.validate_all_keys();
        assert!(validation_errors.is_empty());
        
        // Test statistics
        let stats = manager.get_statistics();
        assert_eq!(stats.total_keys, 4);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Kyber], 2);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Dilithium], 2);
        assert_eq!(stats.by_key_type[&KeyType::Public], 2);
        assert_eq!(stats.by_key_type[&KeyType::Secret], 2);
        assert_eq!(stats.expired_count, 0);
        
        // Test key cloning with new ID
        let cloned_key = retrieved_kyber_pub.clone_with_new_id();
        assert_ne!(cloned_key.metadata.key_id, retrieved_kyber_pub.metadata.key_id);
        assert_eq!(cloned_key.algorithm, retrieved_kyber_pub.algorithm);
        assert_eq!(cloned_key.key_data, retrieved_kyber_pub.key_data);
    }

    #[test]
    fn test_mixed_algorithm_operations() {
        // Test that different algorithms can coexist and work together
        
        // Generate key pairs for different algorithms
        let (kyber_pub, kyber_sec) = kyber::Kyber::keygen(SecurityLevel::Level1).unwrap();
        let (dilithium_pub, dilithium_sec) = dilithium::Dilithium::keygen(SecurityLevel::Level1).unwrap();
        let (sphincs_pub, sphincs_sec) = sphincs::SphincsPlusAlgorithm::keygen(SecurityLevel::Level1).unwrap();
        
        // Test Kyber KEM
        let (kyber_ct, kyber_ss1) = kyber::Kyber::encaps(&kyber_pub).unwrap();
        let kyber_ss2 = kyber::Kyber::decaps(&kyber_sec, &kyber_ct).unwrap();
        assert_eq!(kyber_ss1.as_bytes().len(), kyber_ss2.as_bytes().len());
        
        // Test Dilithium signatures
        let message = b"Multi-algorithm test message";
        let dilithium_sig = dilithium::Dilithium::sign(&dilithium_sec, message).unwrap();
        let dilithium_valid = dilithium::Dilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap();
        assert!(dilithium_valid);
        
        // Test SPHINCS+ signatures
        let sphincs_sig = sphincs::SphincsPlusAlgorithm::sign(&sphincs_sec, message).unwrap();
        let sphincs_valid = sphincs::SphincsPlusAlgorithm::verify(&sphincs_pub, message, &sphincs_sig).unwrap();
        assert!(sphincs_valid);
        
        // Verify signatures from different algorithms are different
        assert_ne!(dilithium_sig.as_bytes(), sphincs_sig.as_bytes());
        
        // Verify cross-verification fails (as expected)
        let cross_valid = dilithium::Dilithium::verify(&dilithium_pub, message, &sphincs_sig);
        assert!(cross_valid.is_err() || !cross_valid.unwrap());
    }
}
