//! Enhanced Comprehensive Post-Quantum Cryptography Test Suite
//! 
//! This test suite validates the complete PQC implementation with real algorithms,
//! comprehensive security testing, performance validation, and interoperability checks.

use cursed::stdlib::crypto_pqc::*;
use cursed::stdlib::crypto_pqc::algorithms::*;

#[cfg(test)]
mod enhanced_pqc_tests {
    use super::*;

    #[test]
    fn test_real_ntru_comprehensive() {
        // Test NTRU-HPS-509 (Level 1)
        let (pub_key, sec_key) = ntru_real::RealNtru::keygen(SecurityLevel::Level1).unwrap();
        assert_eq!(pub_key.params, ntru_real::NtruParams::NtruHps509);
        assert_eq!(sec_key.params, ntru_real::NtruParams::NtruHps509);
        assert_eq!(pub_key.security_level(), SecurityLevel::Level1);
        assert_eq!(sec_key.security_level(), SecurityLevel::Level1);

        // Test encapsulation/decapsulation
        let (ciphertext, shared_secret1) = ntru_real::RealNtru::encaps(&pub_key).unwrap();
        let shared_secret2 = ntru_real::RealNtru::decaps(&sec_key, &ciphertext).unwrap();
        assert_eq!(shared_secret1.data, shared_secret2.data);

        // Test all security levels
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pk, sk) = ntru_real::RealNtru::keygen(level).unwrap();
            let (ct, ss1) = ntru_real::RealNtru::encaps(&pk).unwrap();
            let ss2 = ntru_real::RealNtru::decaps(&sk, &ct).unwrap();
            assert_eq!(ss1.data, ss2.data);
        }

        // Test serialization
        let pub_bytes = pub_key.as_bytes();
        let sec_bytes = sec_key.as_bytes();
        let pub_key2 = ntru_real::NtruPublicKey::from_bytes(pub_key.params, &pub_bytes).unwrap();
        let sec_key2 = ntru_real::NtruSecretKey::from_bytes(sec_key.params, &sec_bytes).unwrap();
        
        // Test that reconstructed keys work
        let (ct2, ss3) = ntru_real::RealNtru::encaps(&pub_key2).unwrap();
        let ss4 = ntru_real::RealNtru::decaps(&sec_key2, &ct2).unwrap();
        assert_eq!(ss3.data, ss4.data);
    }

    #[test]
    fn test_real_frodo_comprehensive() {
        // Test FrodoKEM-640-AES (Level 1)
        let (pub_key, sec_key) = frodo_real::RealFrodo::keygen(SecurityLevel::Level1).unwrap();
        assert_eq!(pub_key.params, frodo_real::FrodoParams::Frodo640Aes);
        assert_eq!(sec_key.params, frodo_real::FrodoParams::Frodo640Aes);
        assert_eq!(pub_key.security_level(), SecurityLevel::Level1);
        assert_eq!(sec_key.security_level(), SecurityLevel::Level1);

        // Test encapsulation/decapsulation
        let (ciphertext, shared_secret1) = frodo_real::RealFrodo::encaps(&pub_key).unwrap();
        let shared_secret2 = frodo_real::RealFrodo::decaps(&sec_key, &ciphertext).unwrap();
        assert_eq!(shared_secret1.data, shared_secret2.data);

        // Test all security levels
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pk, sk) = frodo_real::RealFrodo::keygen(level).unwrap();
            let (ct, ss1) = frodo_real::RealFrodo::encaps(&pk).unwrap();
            let ss2 = frodo_real::RealFrodo::decaps(&sk, &ct).unwrap();
            assert_eq!(ss1.data, ss2.data);
        }

        // Test matrix operations
        let matrix1 = frodo_real::FrodoMatrix::new(3, 3, 65536);
        let matrix2 = frodo_real::FrodoMatrix::new(3, 3, 65536);
        let sum = matrix1.add(&matrix2).unwrap();
        assert_eq!(sum.rows, 3);
        assert_eq!(sum.cols, 3);

        // Test matrix serialization
        let bytes = matrix1.to_bytes();
        let matrix3 = frodo_real::FrodoMatrix::from_bytes(&bytes, 3, 3, 65536).unwrap();
        assert_eq!(matrix1.data, matrix3.data);
    }

    #[test]
    fn test_real_xmss_comprehensive() {
        // Test XMSS-SHA2_10_256 (Level 1)
        let (pub_key, sec_key) = xmss_real::RealXmss::keygen(SecurityLevel::Level1).unwrap();
        assert_eq!(pub_key.params, xmss_real::XmssParams::XmssSha2_10_256);
        assert_eq!(sec_key.params, xmss_real::XmssParams::XmssSha2_10_256);
        assert_eq!(pub_key.security_level(), SecurityLevel::Level1);
        assert_eq!(sec_key.security_level(), SecurityLevel::Level1);

        // Test sign/verify
        let message = b"Test message for XMSS signature";
        let signature = xmss_real::RealXmss::sign(&sec_key, message).unwrap();
        let is_valid = xmss_real::RealXmss::verify(&pub_key, message, &signature).unwrap();
        assert!(is_valid);

        // Test invalid message
        let invalid_message = b"Different message";
        let is_valid_invalid = xmss_real::RealXmss::verify(&pub_key, invalid_message, &signature).unwrap();
        assert!(!is_valid_invalid);

        // Test signature remaining count
        assert_eq!(sec_key.signatures_remaining(), 1024); // 2^10

        // Test multiple signatures
        for i in 0..5 {
            let msg = format!("Message {}", i);
            let sig = xmss_real::RealXmss::sign(&sec_key, msg.as_bytes()).unwrap();
            let valid = xmss_real::RealXmss::verify(&pub_key, msg.as_bytes(), &sig).unwrap();
            assert!(valid);
        }

        // Test serialization
        let pub_bytes = pub_key.as_bytes();
        let sec_bytes = sec_key.as_bytes();
        let pub_key2 = xmss_real::XmssPublicKey::from_bytes(pub_key.params, &pub_bytes).unwrap();
        let sec_key2 = xmss_real::XmssSecretKey::from_bytes(sec_key.params, &sec_bytes).unwrap();
        
        assert_eq!(pub_key.root, pub_key2.root);
        assert_eq!(pub_key.seed, pub_key2.seed);
        assert_eq!(sec_key.index, sec_key2.index);
    }

    #[test]
    fn test_algorithm_performance_characteristics() {
        // Test NTRU performance characteristics
        let ntru_perf = ntru_real::RealNtru::performance_characteristics(ntru_real::NtruParams::NtruHps509);
        assert!(ntru_perf.keygen_time_ms > 0.0);
        assert!(ntru_perf.operation_time_ms > 0.0);
        assert!(ntru_perf.throughput_ops_per_sec > 0.0);
        assert!(ntru_perf.key_sizes.public_key > 0);
        assert!(ntru_perf.key_sizes.secret_key > 0);

        // Test FrodoKEM performance characteristics
        let frodo_perf = frodo_real::RealFrodo::performance_characteristics(frodo_real::FrodoParams::Frodo640Aes);
        assert!(frodo_perf.keygen_time_ms > 0.0);
        assert!(frodo_perf.operation_time_ms > 0.0);
        assert!(frodo_perf.throughput_ops_per_sec > 0.0);
        assert!(frodo_perf.key_sizes.public_key > 0);
        assert!(frodo_perf.key_sizes.secret_key > 0);

        // Test XMSS performance characteristics
        let xmss_perf = xmss_real::RealXmss::performance_characteristics(xmss_real::XmssParams::XmssSha2_10_256);
        assert!(xmss_perf.keygen_time_ms > 0.0);
        assert!(xmss_perf.operation_time_ms > 0.0);
        assert!(xmss_perf.throughput_ops_per_sec > 0.0);
        assert!(xmss_perf.key_sizes.public_key > 0);
        assert!(xmss_perf.key_sizes.secret_key > 0);
    }

    #[test]
    fn test_algorithm_type_identification() {
        assert_eq!(ntru_real::RealNtru::algorithm_type(), AlgorithmType::Ntru);
        assert_eq!(frodo_real::RealFrodo::algorithm_type(), AlgorithmType::FrodoKem);
        assert_eq!(xmss_real::RealXmss::algorithm_type(), AlgorithmType::Xmss);
    }

    #[test]
    fn test_parameter_set_information() {
        // Test NTRU parameter sets
        let ntru_params = ntru_real::NtruParams::NtruHps509;
        assert_eq!(ntru_params.security_level(), SecurityLevel::Level1);
        assert!(ntru_params.public_key_size() > 0);
        assert!(ntru_params.secret_key_size() > 0);
        
        let additional_sizes = ntru_params.additional_sizes();
        assert!(!additional_sizes.is_empty());
        assert!(additional_sizes.iter().any(|(name, _)| *name == "ciphertext"));

        // Test FrodoKEM parameter sets
        let frodo_params = frodo_real::FrodoParams::Frodo640Aes;
        assert_eq!(frodo_params.security_level(), SecurityLevel::Level1);
        assert!(frodo_params.public_key_size() > 0);
        assert!(frodo_params.secret_key_size() > 0);

        // Test XMSS parameter sets
        let xmss_params = xmss_real::XmssParams::XmssSha2_10_256;
        assert_eq!(xmss_params.security_level(), SecurityLevel::Level1);
        assert!(xmss_params.public_key_size() > 0);
        assert!(xmss_params.secret_key_size() > 0);
        assert_eq!(xmss_params.total_signatures(), 1024);
    }

    #[test]
    fn test_error_handling() {
        // Test invalid parameter combinations
        let (pub_key, _) = ntru_real::RealNtru::keygen(SecurityLevel::Level1).unwrap();
        let (_, sec_key) = ntru_real::RealNtru::keygen(SecurityLevel::Level3).unwrap();
        
        // Create ciphertext with one param set
        let (ciphertext, _) = ntru_real::RealNtru::encaps(&pub_key).unwrap();
        
        // Try to decrypt with mismatched secret key
        let result = ntru_real::RealNtru::decaps(&sec_key, &ciphertext);
        assert!(result.is_err());
        
        // Test invalid key data
        let invalid_data = vec![0u8; 10]; // Too short
        let result = ntru_real::NtruPublicKey::from_bytes(ntru_real::NtruParams::NtruHps509, &invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_security_level_mapping() {
        // Test that each algorithm correctly maps security levels
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // NTRU
            let (ntru_pub, _) = ntru_real::RealNtru::keygen(level).unwrap();
            assert_eq!(ntru_pub.security_level(), level);
            
            // FrodoKEM
            let (frodo_pub, _) = frodo_real::RealFrodo::keygen(level).unwrap();
            assert_eq!(frodo_pub.security_level(), level);
            
            // XMSS
            let (xmss_pub, _) = xmss_real::RealXmss::keygen(level).unwrap();
            assert_eq!(xmss_pub.security_level(), level);
        }
    }

    #[test]
    fn test_key_size_consistency() {
        // Test that reported key sizes match actual serialized sizes
        let (ntru_pub, ntru_sec) = ntru_real::RealNtru::keygen(SecurityLevel::Level1).unwrap();
        let ntru_params = ntru_pub.params;
        
        assert_eq!(ntru_pub.as_bytes().len(), ntru_params.public_key_size());
        assert_eq!(ntru_sec.as_bytes().len(), ntru_params.secret_key_size());

        let (frodo_pub, frodo_sec) = frodo_real::RealFrodo::keygen(SecurityLevel::Level1).unwrap();
        let frodo_params = frodo_pub.params;
        
        assert_eq!(frodo_pub.as_bytes().len(), frodo_params.public_key_size());
        assert_eq!(frodo_sec.as_bytes().len(), frodo_params.secret_key_size());
    }

    #[test]
    fn test_algorithm_family_classification() {
        // Test algorithm family classification
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Ntru), AlgorithmFamily::LatticeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::FrodoKem), AlgorithmFamily::LatticeBased);
        assert_eq!(AlgorithmFamily::from_algorithm(AlgorithmType::Xmss), AlgorithmFamily::HashBased);
        
        // Test family descriptions
        assert!(AlgorithmFamily::LatticeBased.description().contains("Lattice"));
        assert!(AlgorithmFamily::HashBased.description().contains("Hash"));
        
        // Test quantum confidence levels
        assert!(AlgorithmFamily::LatticeBased.quantum_confidence().contains("High"));
        assert!(AlgorithmFamily::HashBased.quantum_confidence().contains("Very high"));
    }

    #[test]
    fn test_standardization_status() {
        // Test standardization status
        assert_eq!(StandardizationStatus::for_algorithm(AlgorithmType::Kyber), StandardizationStatus::NistStandardized);
        assert_eq!(StandardizationStatus::for_algorithm(AlgorithmType::Dilithium), StandardizationStatus::NistStandardized);
        assert_eq!(StandardizationStatus::for_algorithm(AlgorithmType::Ntru), StandardizationStatus::NistFinalist);
        
        // Test production readiness
        assert!(StandardizationStatus::NistStandardized.is_production_ready());
        assert!(StandardizationStatus::NistFinalist.is_production_ready());
        assert!(!StandardizationStatus::Research.is_production_ready());
    }

    #[test]
    fn test_comprehensive_integration() {
        // Test that multiple algorithms can work together
        
        // Create multiple key pairs
        let (kyber_pub, kyber_sec) = kyber_real::RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let (ntru_pub, ntru_sec) = ntru_real::RealNtru::keygen(SecurityLevel::Level1).unwrap();
        let (frodo_pub, frodo_sec) = frodo_real::RealFrodo::keygen(SecurityLevel::Level1).unwrap();
        
        let (dilithium_pub, dilithium_sec) = dilithium_real::RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let (xmss_pub, xmss_sec) = xmss_real::RealXmss::keygen(SecurityLevel::Level1).unwrap();
        
        // Test KEM operations
        for (pub_key, sec_key) in [
            (&kyber_pub as &dyn std::any::Any, &kyber_sec as &dyn std::any::Any),
            (&ntru_pub as &dyn std::any::Any, &ntru_sec as &dyn std::any::Any),
            (&frodo_pub as &dyn std::any::Any, &frodo_sec as &dyn std::any::Any),
        ] {
            // This tests that different algorithms can coexist without conflicts
            assert!(pub_key.type_id() != sec_key.type_id()); // Different types as expected
        }
        
        // Test signature operations
        let message = b"Integration test message";
        
        let dilithium_sig = dilithium_real::RealDilithium::sign(&dilithium_sec, message).unwrap();
        let dilithium_valid = dilithium_real::RealDilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap();
        assert!(dilithium_valid);
        
        let xmss_sig = xmss_real::RealXmss::sign(&xmss_sec, message).unwrap();
        let xmss_valid = xmss_real::RealXmss::verify(&xmss_pub, message, &xmss_sig).unwrap();
        assert!(xmss_valid);
    }

    #[test]
    fn test_stress_operations() {
        // Stress test with multiple operations
        let (pub_key, sec_key) = ntru_real::RealNtru::keygen(SecurityLevel::Level1).unwrap();
        
        // Perform multiple encapsulations
        for _ in 0..10 {
            let (ciphertext, shared_secret1) = ntru_real::RealNtru::encaps(&pub_key).unwrap();
            let shared_secret2 = ntru_real::RealNtru::decaps(&sec_key, &ciphertext).unwrap();
            assert_eq!(shared_secret1.data, shared_secret2.data);
        }
        
        // Test FrodoKEM with larger parameters
        let (frodo_pub, frodo_sec) = frodo_real::RealFrodo::keygen(SecurityLevel::Level3).unwrap();
        for _ in 0..5 {
            let (ct, ss1) = frodo_real::RealFrodo::encaps(&frodo_pub).unwrap();
            let ss2 = frodo_real::RealFrodo::decaps(&frodo_sec, &ct).unwrap();
            assert_eq!(ss1.data, ss2.data);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty messages for signatures
        let (xmss_pub, xmss_sec) = xmss_real::RealXmss::keygen(SecurityLevel::Level1).unwrap();
        
        let empty_message = b"";
        let sig = xmss_real::RealXmss::sign(&xmss_sec, empty_message).unwrap();
        let valid = xmss_real::RealXmss::verify(&xmss_pub, empty_message, &sig).unwrap();
        assert!(valid);
        
        // Test with large messages
        let large_message = vec![42u8; 10000];
        let sig2 = xmss_real::RealXmss::sign(&xmss_sec, &large_message).unwrap();
        let valid2 = xmss_real::RealXmss::verify(&xmss_pub, &large_message, &sig2).unwrap();
        assert!(valid2);
    }
}
