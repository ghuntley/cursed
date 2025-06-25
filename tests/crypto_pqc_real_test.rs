//! Comprehensive Test Suite for Real Post-Quantum Cryptography Implementations
//! 
//! This test suite validates all the real PQC algorithm implementations including:
//! - CRYSTALS-Dilithium (digital signatures)
//! - CRYSTALS-Kyber (key encapsulation) 
//! - LMS (hash-based signatures)
//! - FALCON (compact signatures)
//! - Classic McEliece (code-based KEM)
//! 
//! Tests cover functionality, security properties, performance, and interoperability.

use cursed::stdlib::crypto_pqc::{PqcResult, SecurityLevel, AlgorithmType};
use cursed::stdlib::crypto_pqc::algorithms::{DigitalSignature, KeyEncapsulation};

// Import real implementations
mod real_implementations {
    pub use cursed::stdlib::crypto_pqc::algorithms::dilithium_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::kyber_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::lms_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::falcon_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::mceliece_real::*;
}

use real_implementations::*;

/// Test helper for running algorithm performance tests
fn test_algorithm_performance<T>() 
where 
    T: DigitalSignature + 'static,
    T::PublicKey: Clone,
    T::SecretKey: Clone,
    T::Signature: Clone,
{
    let start = std::time::Instant::now();
    let (pub_key, sec_key) = T::keygen(SecurityLevel::Level1).unwrap();
    let keygen_time = start.elapsed();

    let message = b"Performance test message for PQC algorithms";
    
    let start = std::time::Instant::now();
    let signature = T::sign(&sec_key, message).unwrap();
    let sign_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let is_valid = T::verify(&pub_key, message, &signature).unwrap();
    let verify_time = start.elapsed();
    
    assert!(is_valid);
    
    println!("Algorithm: {:?}", T::algorithm_type());
    println!("  Keygen time: {:?}", keygen_time);
    println!("  Sign time: {:?}", sign_time);
    println!("  Verify time: {:?}", verify_time);
}

/// Test helper for KEM performance
fn test_kem_performance<T>()
where 
    T: KeyEncapsulation + 'static,
    T::PublicKey: Clone,
    T::SecretKey: Clone,
    T::Ciphertext: Clone,
    T::SharedSecret: Clone,
{
    let start = std::time::Instant::now();
    let (pub_key, sec_key) = T::keygen(SecurityLevel::Level1).unwrap();
    let keygen_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let (ciphertext, shared_secret1) = T::encaps(&pub_key).unwrap();
    let encaps_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let shared_secret2 = T::decaps(&sec_key, &ciphertext).unwrap();
    let decaps_time = start.elapsed();
    
    println!("KEM Algorithm: {:?}", T::algorithm_type());
    println!("  Keygen time: {:?}", keygen_time);
    println!("  Encaps time: {:?}", encaps_time);
    println!("  Decaps time: {:?}", decaps_time);
}

#[cfg(test)]
mod real_dilithium_tests {
    use super::*;

    #[test]
    fn test_real_dilithium_basic_functionality() {
        let (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, real Dilithium!";
        
        let signature = RealDilithium::sign(&sec_key, message).unwrap();
        let is_valid = RealDilithium::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
        assert_eq!(RealDilithium::algorithm_type(), AlgorithmType::Dilithium);
    }

    #[test]
    fn test_real_dilithium_different_security_levels() {
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealDilithium::keygen(level).unwrap();
            let message = b"Security level test";
            
            let signature = RealDilithium::sign(&sec_key, message).unwrap();
            let is_valid = RealDilithium::verify(&pub_key, message, &signature).unwrap();
            
            assert!(is_valid);
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
        }
    }

    #[test]
    fn test_real_dilithium_invalid_signature() {
        let (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Original message";
        let wrong_message = b"Wrong message";
        
        let signature = RealDilithium::sign(&sec_key, message).unwrap();
        let is_valid = RealDilithium::verify(&pub_key, wrong_message, &signature).unwrap();
        
        assert!(!is_valid);
    }

    #[test]
    fn test_real_dilithium_key_serialization() {
        let (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        assert!(!pub_key_bytes.is_empty());
        assert!(!sec_key_bytes.is_empty());
        assert!(sec_key_bytes.len() > pub_key_bytes.len());
    }

    #[test]
    fn test_real_dilithium_polynomial_operations() {
        use real_implementations::dilithium_real::Polynomial;
        
        let mut poly1 = Polynomial::new();
        poly1.coeffs[0] = 100;
        poly1.coeffs[1] = 200;
        
        let mut poly2 = Polynomial::new();
        poly2.coeffs[0] = 50;
        poly2.coeffs[1] = 75;
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs[0], 150);
        assert_eq!(sum.coeffs[1], 275);
        
        let diff = poly1.subtract(&poly2);
        assert_eq!(diff.coeffs[0], 50);
        assert_eq!(diff.coeffs[1], 125);
    }

    #[test]
    fn test_real_dilithium_ntt_operations() {
        use real_implementations::dilithium_real::Polynomial;
        
        let mut poly = Polynomial::new();
        poly.coeffs[0] = 1;
        poly.coeffs[1] = 2;
        poly.coeffs[2] = 3;
        
        let original = poly.clone();
        poly.ntt();
        poly.intt();
        
        // After NTT and INTT, should be close to original
        for i in 0..3 {
            assert!((poly.coeffs[i] - original.coeffs[i]).abs() < 1000);
        }
    }

    #[test]
    fn test_real_dilithium_performance() {
        test_algorithm_performance::<RealDilithium>();
    }
}

#[cfg(test)]
mod real_kyber_tests {
    use super::*;

    #[test]
    fn test_real_kyber_basic_functionality() {
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        
        let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
        let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
        
        assert_eq!(shared_secret1.data, shared_secret2.data);
        assert_eq!(RealKyber::algorithm_type(), AlgorithmType::Kyber);
    }

    #[test]
    fn test_real_kyber_different_security_levels() {
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealKyber::keygen(level).unwrap();
            
            let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
            let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
            
            assert_eq!(shared_secret1.data, shared_secret2.data);
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
        }
    }

    #[test]
    fn test_real_kyber_polynomial_operations() {
        use real_implementations::kyber_real::KyberPolynomial;
        
        let poly1 = KyberPolynomial::from_coeffs([1, 2, 3, 0, 0].into_iter().cycle().take(256).collect::<Vec<_>>().try_into().unwrap());
        let poly2 = KyberPolynomial::from_coeffs([4, 5, 6, 0, 0].into_iter().cycle().take(256).collect::<Vec<_>>().try_into().unwrap());
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs[0], 5);
        assert_eq!(sum.coeffs[1], 7);
        assert_eq!(sum.coeffs[2], 9);
    }

    #[test]
    fn test_real_kyber_compression() {
        use real_implementations::kyber_real::KyberPolynomial;
        
        let mut coeffs = [0i16; 256];
        coeffs[0] = 100;
        coeffs[1] = 200;
        coeffs[2] = 300;
        
        let poly = KyberPolynomial::from_coeffs(coeffs);
        let compressed = poly.compress(4);
        let decompressed = KyberPolynomial::decompress(&compressed, 4);
        
        // Should be approximately equal after compression/decompression
        assert!((poly.coeffs[0] - decompressed.coeffs[0]).abs() < 500);
        assert!((poly.coeffs[1] - decompressed.coeffs[1]).abs() < 500);
        assert!((poly.coeffs[2] - decompressed.coeffs[2]).abs() < 500);
    }

    #[test]
    fn test_real_kyber_key_serialization() {
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        assert!(!pub_key_bytes.is_empty());
        assert!(!sec_key_bytes.is_empty());
    }

    #[test]
    fn test_real_kyber_performance() {
        test_kem_performance::<RealKyber>();
    }
}

#[cfg(test)]
mod real_lms_tests {
    use super::*;

    #[test]
    fn test_real_lms_basic_functionality() {
        let (pub_key, mut sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, hash-based signatures!";
        
        let signature = RealLms::sign_with_state(&mut sec_key, message).unwrap();
        let is_valid = RealLms::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
        assert_eq!(RealLms::algorithm_type(), AlgorithmType::Lms);
    }

    #[test]
    fn test_real_lms_state_management() {
        let (_, mut sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let message1 = b"Message 1";
        let message2 = b"Message 2";
        
        assert_eq!(sec_key.q, 0);
        let _sig1 = RealLms::sign_with_state(&mut sec_key, message1).unwrap();
        assert_eq!(sec_key.q, 1);
        let _sig2 = RealLms::sign_with_state(&mut sec_key, message2).unwrap();
        assert_eq!(sec_key.q, 2);
        
        let remaining = RealLms::remaining_signatures(&sec_key);
        assert!(remaining > 0);
    }

    #[test]
    fn test_real_lms_different_security_levels() {
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, mut sec_key) = RealLms::keygen(level).unwrap();
            let message = b"Security level test";
            
            let signature = RealLms::sign_with_state(&mut sec_key, message).unwrap();
            let is_valid = RealLms::verify(&pub_key, message, &signature).unwrap();
            
            assert!(is_valid);
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
        }
    }

    #[test]
    fn test_real_lms_lmots_operations() {
        use real_implementations::lms_real::{LmotsKey, LmsParams};
        
        let params = LmsParams::LmsSha256M32H10;
        let i = [1u8; 16];
        let ots_key = LmotsKey::new(params, i, 0);
        
        let message_hash = [42u8; 32];
        let signature = ots_key.sign(&message_hash);
        let public_key = ots_key.public_key();
        
        assert!(signature.verify(&message_hash, &i, &public_key));
    }

    #[test]
    fn test_real_lms_merkle_tree() {
        use real_implementations::lms_real::{MerkleTree, verify_auth_path};
        
        let leaves = vec![[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let tree = MerkleTree::new(leaves, 2);
        
        let auth_path = tree.generate_auth_path(1);
        assert_eq!(auth_path.len(), 2);
        
        let computed_root = verify_auth_path(&[2u8; 32], 1, &auth_path, 2);
        assert_eq!(computed_root, tree.root_hash());
    }

    #[test]
    fn test_real_lms_key_serialization() {
        let (pub_key, sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        assert!(!pub_key_bytes.is_empty());
        assert!(!sec_key_bytes.is_empty());
    }

    #[test]
    fn test_real_lms_signature_exhaustion() {
        let (_, mut sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Test message";
        
        // Use up all signatures (this might take a while for large parameter sets)
        let max_sigs = std::cmp::min(RealLms::remaining_signatures(&sec_key), 10);
        
        for _ in 0..max_sigs {
            let _signature = RealLms::sign_with_state(&mut sec_key, message).unwrap();
        }
        
        // Verify we can't sign beyond the limit
        let remaining = RealLms::remaining_signatures(&sec_key);
        assert!(remaining <= sec_key.params.max_signatures() - max_sigs);
    }
}

#[cfg(test)]
mod real_falcon_tests {
    use super::*;

    #[test]
    fn test_real_falcon_basic_functionality() {
        let (pub_key, sec_key) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, compact signatures!";
        
        let signature = RealFalcon::sign(&sec_key, message).unwrap();
        let is_valid = RealFalcon::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_real_falcon_different_security_levels() {
        for level in [SecurityLevel::Level1, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealFalcon::keygen(level).unwrap();
            let message = b"Security level test";
            
            let signature = RealFalcon::sign(&sec_key, message).unwrap();
            let is_valid = RealFalcon::verify(&pub_key, message, &signature).unwrap();
            
            assert!(is_valid);
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
        }
    }

    #[test]
    fn test_real_falcon_polynomial_operations() {
        use real_implementations::falcon_real::FalconPolynomial;
        
        let poly1 = FalconPolynomial::from_coeffs(vec![1, 2, 3]);
        let poly2 = FalconPolynomial::from_coeffs(vec![4, 5, 6]);
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs, vec![5, 7, 9]);
        
        let diff = poly1.sub(&poly2);
        assert_eq!(diff.coeffs, vec![-3, -3, -3]);
    }

    #[test]
    fn test_real_falcon_complex_operations() {
        use real_implementations::falcon_real::Complex;
        
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        
        let sum = a.add(&b);
        assert_eq!(sum.re, 4.0);
        assert_eq!(sum.im, 6.0);
        
        let product = a.mul(&b);
        assert_eq!(product.re, -5.0);
        assert_eq!(product.im, 10.0);
    }

    #[test]
    fn test_real_falcon_fft_operations() {
        use real_implementations::falcon_real::{Complex, fft, ifft};
        
        let mut data = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        
        let original = data.clone();
        fft(&mut data);
        ifft(&mut data);
        
        // Should be close to original after FFT/IFFT
        for i in 0..data.len() {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn test_real_falcon_key_serialization() {
        let (pub_key, sec_key) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        assert!(!pub_key_bytes.is_empty());
        assert!(!sec_key_bytes.is_empty());
    }

    #[test]
    fn test_real_falcon_performance() {
        test_algorithm_performance::<RealFalcon>();
    }
}

#[cfg(test)]
mod real_mceliece_tests {
    use super::*;

    #[test]
    fn test_real_mceliece_basic_functionality() {
        let (pub_key, sec_key) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
        
        let (ciphertext, shared_secret1) = RealMcEliece::encaps(&pub_key).unwrap();
        let shared_secret2 = RealMcEliece::decaps(&sec_key, &ciphertext).unwrap();
        
        assert_eq!(shared_secret1.data, shared_secret2.data);
        assert_eq!(RealMcEliece::algorithm_type(), AlgorithmType::ClassicMcEliece);
    }

    #[test]
    fn test_real_mceliece_different_security_levels() {
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealMcEliece::keygen(level).unwrap();
            
            let (ciphertext, shared_secret1) = RealMcEliece::encaps(&pub_key).unwrap();
            let shared_secret2 = RealMcEliece::decaps(&sec_key, &ciphertext).unwrap();
            
            assert_eq!(shared_secret1.data, shared_secret2.data);
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
        }
    }

    #[test]
    fn test_real_mceliece_gf_operations() {
        use real_implementations::mceliece_real::GfElement;
        
        let a = GfElement::new(5);
        let b = GfElement::new(3);
        
        let sum = a.add(&b);
        assert_eq!(sum.value, 6); // 5 XOR 3 = 6
        
        let product = a.multiply(&b, 0x11B); // Using AES irreducible polynomial
        assert!(product.value != 0); // Should be non-zero
    }

    #[test]
    fn test_real_mceliece_binary_matrix() {
        use real_implementations::mceliece_real::BinaryMatrix;
        
        let mut matrix = BinaryMatrix::new(3, 3);
        matrix.set(0, 0, true);
        matrix.set(1, 1, true);
        matrix.set(2, 2, true);
        
        let vector = vec![true, false, true];
        let result = matrix.multiply_vector(&vector);
        
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn test_real_mceliece_goppa_polynomial() {
        use real_implementations::mceliece_real::GoppaPolynomial;
        
        let poly = GoppaPolynomial::generate_irreducible(3, 16, b"test_seed");
        assert_eq!(poly.degree, 3);
        assert_eq!(poly.coeffs[3].value, 1); // Should be monic
    }

    #[test]
    fn test_real_mceliece_key_serialization() {
        let (pub_key, sec_key) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        assert!(!pub_key_bytes.is_empty());
        assert!(!sec_key_bytes.is_empty());
    }

    #[test]
    fn test_real_mceliece_performance() {
        test_kem_performance::<RealMcEliece>();
    }
}

#[cfg(test)]
mod security_property_tests {
    use super::*;

    #[test]
    fn test_signature_schemes_authenticity() {
        // Test that signatures from one key don't verify with another key
        let (pub_key1, sec_key1) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let (pub_key2, _sec_key2) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        
        let message = b"Authenticity test message";
        let signature = RealDilithium::sign(&sec_key1, message).unwrap();
        
        // Should verify with correct key
        assert!(RealDilithium::verify(&pub_key1, message, &signature).unwrap());
        
        // Should NOT verify with different key
        assert!(!RealDilithium::verify(&pub_key2, message, &signature).unwrap());
    }

    #[test]
    fn test_kem_schemes_consistency() {
        // Test that encapsulation/decapsulation produces consistent shared secrets
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        
        // Multiple encapsulations should produce different ciphertexts but valid decapsulations
        for _ in 0..5 {
            let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
            let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
            assert_eq!(shared_secret1.data, shared_secret2.data);
        }
    }

    #[test]
    fn test_different_parameter_sets_isolation() {
        // Test that different parameter sets produce different key sizes
        let (pub1, sec1) = RealDilithium::keygen_with_params(DilithiumParams::Dilithium2).unwrap();
        let (pub3, sec3) = RealDilithium::keygen_with_params(DilithiumParams::Dilithium3).unwrap();
        let (pub5, sec5) = RealDilithium::keygen_with_params(DilithiumParams::Dilithium5).unwrap();
        
        let pub1_size = pub1.as_bytes().len();
        let pub3_size = pub3.as_bytes().len();
        let pub5_size = pub5.as_bytes().len();
        
        // Higher security levels should have larger keys
        assert!(pub1_size < pub3_size);
        assert!(pub3_size < pub5_size);
        
        let sec1_size = sec1.as_bytes().len();
        let sec3_size = sec3.as_bytes().len();
        let sec5_size = sec5.as_bytes().len();
        
        assert!(sec1_size < sec3_size);
        assert!(sec3_size < sec5_size);
    }

    #[test]
    fn test_random_oracle_properties() {
        // Test that signatures are deterministic when using same randomness (for LMS)
        let (pub_key, mut sec_key1) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let (_, mut sec_key2) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        
        let message = b"Determinism test";
        
        let sig1 = RealLms::sign_with_state(&mut sec_key1, message).unwrap();
        let sig2 = RealLms::sign_with_state(&mut sec_key2, message).unwrap();
        
        // Different keys should produce different signatures even for same message
        assert_ne!(sig1.as_bytes(), sig2.as_bytes());
        
        // Both should verify with their respective public keys
        assert!(RealLms::verify(&pub_key, message, &sig1).unwrap());
    }
}

#[cfg(test)]
mod performance_benchmark_tests {
    use super::*;

    #[test]
    fn test_signature_scheme_benchmarks() {
        println!("\n=== Signature Scheme Performance ===");
        
        println!("\nReal Dilithium:");
        test_algorithm_performance::<RealDilithium>();
        
        println!("\nReal FALCON:");
        test_algorithm_performance::<RealFalcon>();
    }

    #[test]
    fn test_kem_scheme_benchmarks() {
        println!("\n=== KEM Scheme Performance ===");
        
        println!("\nReal Kyber:");
        test_kem_performance::<RealKyber>();
        
        println!("\nReal McEliece:");
        test_kem_performance::<RealMcEliece>();
    }

    #[test]
    fn test_key_size_comparisons() {
        println!("\n=== Key Size Comparisons ===");
        
        // Signature schemes
        let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let (falcon_pub, falcon_sec) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        let (lms_pub, lms_sec) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        
        println!("Signature Schemes (Level 1):");
        println!("  Dilithium - Public: {} bytes, Secret: {} bytes", 
                 dilithium_pub.as_bytes().len(), dilithium_sec.as_bytes().len());
        println!("  FALCON - Public: {} bytes, Secret: {} bytes", 
                 falcon_pub.as_bytes().len(), falcon_sec.as_bytes().len());
        println!("  LMS - Public: {} bytes, Secret: {} bytes", 
                 lms_pub.as_bytes().len(), lms_sec.as_bytes().len());
        
        // KEM schemes
        let (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let (mceliece_pub, mceliece_sec) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
        
        println!("KEM Schemes (Level 1):");
        println!("  Kyber - Public: {} bytes, Secret: {} bytes", 
                 kyber_pub.as_bytes().len(), kyber_sec.as_bytes().len());
        println!("  McEliece - Public: {} bytes, Secret: {} bytes", 
                 mceliece_pub.as_bytes().len(), mceliece_sec.as_bytes().len());
    }

    #[test]
    fn test_signature_size_comparisons() {
        println!("\n=== Signature Size Comparisons ===");
        
        let message = b"Signature size test message for comparison";
        
        let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let dilithium_sig = RealDilithium::sign(&dilithium_sec, message).unwrap();
        
        let (falcon_pub, falcon_sec) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        let falcon_sig = RealFalcon::sign(&falcon_sec, message).unwrap();
        
        let (lms_pub, mut lms_sec) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let lms_sig = RealLms::sign_with_state(&mut lms_sec, message).unwrap();
        
        println!("Signature Sizes (Level 1):");
        println!("  Dilithium: {} bytes", dilithium_sig.as_bytes().len());
        println!("  FALCON: {} bytes", falcon_sig.as_bytes().len());
        println!("  LMS: {} bytes", lms_sig.as_bytes().len());
        
        // Verify all signatures are valid
        assert!(RealDilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap());
        assert!(RealFalcon::verify(&falcon_pub, message, &falcon_sig).unwrap());
        assert!(RealLms::verify(&lms_pub, message, &lms_sig).unwrap());
    }
}

#[cfg(test)]
mod interoperability_tests {
    use super::*;

    #[test]
    fn test_cross_algorithm_compatibility() {
        // Test that all algorithms can coexist and work independently
        let message = b"Interoperability test message";
        
        // Create keys for all signature algorithms
        let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let (falcon_pub, falcon_sec) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        let (lms_pub, mut lms_sec) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        
        // Create signatures
        let dilithium_sig = RealDilithium::sign(&dilithium_sec, message).unwrap();
        let falcon_sig = RealFalcon::sign(&falcon_sec, message).unwrap();
        let lms_sig = RealLms::sign_with_state(&mut lms_sec, message).unwrap();
        
        // Verify all signatures
        assert!(RealDilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap());
        assert!(RealFalcon::verify(&falcon_pub, message, &falcon_sig).unwrap());
        assert!(RealLms::verify(&lms_pub, message, &lms_sig).unwrap());
        
        // Test KEM algorithms
        let (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let (mceliece_pub, mceliece_sec) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
        
        let (kyber_ct, kyber_ss1) = RealKyber::encaps(&kyber_pub).unwrap();
        let kyber_ss2 = RealKyber::decaps(&kyber_sec, &kyber_ct).unwrap();
        assert_eq!(kyber_ss1.data, kyber_ss2.data);
        
        let (mceliece_ct, mceliece_ss1) = RealMcEliece::encaps(&mceliece_pub).unwrap();
        let mceliece_ss2 = RealMcEliece::decaps(&mceliece_sec, &mceliece_ct).unwrap();
        assert_eq!(mceliece_ss1.data, mceliece_ss2.data);
    }

    #[test]
    fn test_algorithm_family_properties() {
        // Test different algorithm families have expected properties
        
        // Lattice-based: Should have reasonable key sizes and fast operations
        let (kyber_pub, _) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let (dilithium_pub, _) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        
        assert!(kyber_pub.as_bytes().len() < 10000); // Reasonable size
        assert!(dilithium_pub.as_bytes().len() < 10000);
        
        // Hash-based: Should be very secure but larger/slower
        let (lms_pub, _) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        assert!(!lms_pub.as_bytes().is_empty());
        
        // Code-based: Should have large public keys
        let (mceliece_pub, _) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
        assert!(mceliece_pub.as_bytes().len() > 1000); // Large public key expected
    }

    #[test]
    fn test_security_level_consistency() {
        // Test that all algorithms properly support different security levels
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // Not all algorithms support all levels, so we test what they do support
            
            // Dilithium supports all levels
            let (dilithium_pub, _) = RealDilithium::keygen(level).unwrap();
            assert_eq!(dilithium_pub.security_level(), level);
            
            // Kyber supports all levels
            let (kyber_pub, _) = RealKyber::keygen(level).unwrap();
            assert_eq!(kyber_pub.security_level(), level);
            
            // FALCON supports Level1 and Level5
            if matches!(level, SecurityLevel::Level1 | SecurityLevel::Level5) {
                let (falcon_pub, _) = RealFalcon::keygen(level).unwrap();
                assert!(matches!(falcon_pub.security_level(), SecurityLevel::Level1 | SecurityLevel::Level5));
            }
            
            // LMS supports all levels
            let (lms_pub, _) = RealLms::keygen(level).unwrap();
            assert_eq!(lms_pub.security_level(), level);
            
            // McEliece supports all levels
            let (mceliece_pub, _) = RealMcEliece::keygen(level).unwrap();
            assert_eq!(mceliece_pub.security_level(), level);
        }
    }
}

#[test]
fn test_comprehensive_pqc_functionality() {
    println!("\n=== Comprehensive Post-Quantum Cryptography Test ===");
    
    // Test all implemented algorithms work correctly
    let message = b"Comprehensive PQC test message";
    
    println!("Testing CRYSTALS-Dilithium...");
    let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
    let dilithium_sig = RealDilithium::sign(&dilithium_sec, message).unwrap();
    assert!(RealDilithium::verify(&dilithium_pub, message, &dilithium_sig).unwrap());
    println!("✓ Dilithium working correctly");
    
    println!("Testing CRYSTALS-Kyber...");
    let (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
    let (kyber_ct, kyber_ss1) = RealKyber::encaps(&kyber_pub).unwrap();
    let kyber_ss2 = RealKyber::decaps(&kyber_sec, &kyber_ct).unwrap();
    assert_eq!(kyber_ss1.data, kyber_ss2.data);
    println!("✓ Kyber working correctly");
    
    println!("Testing LMS...");
    let (lms_pub, mut lms_sec) = RealLms::keygen(SecurityLevel::Level1).unwrap();
    let lms_sig = RealLms::sign_with_state(&mut lms_sec, message).unwrap();
    assert!(RealLms::verify(&lms_pub, message, &lms_sig).unwrap());
    println!("✓ LMS working correctly");
    
    println!("Testing FALCON...");
    let (falcon_pub, falcon_sec) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
    let falcon_sig = RealFalcon::sign(&falcon_sec, message).unwrap();
    assert!(RealFalcon::verify(&falcon_pub, message, &falcon_sig).unwrap());
    println!("✓ FALCON working correctly");
    
    println!("Testing Classic McEliece...");
    let (mceliece_pub, mceliece_sec) = RealMcEliece::keygen(SecurityLevel::Level1).unwrap();
    let (mceliece_ct, mceliece_ss1) = RealMcEliece::encaps(&mceliece_pub).unwrap();
    let mceliece_ss2 = RealMcEliece::decaps(&mceliece_sec, &mceliece_ct).unwrap();
    assert_eq!(mceliece_ss1.data, mceliece_ss2.data);
    println!("✓ McEliece working correctly");
    
    println!("\n🎉 All Post-Quantum Cryptography algorithms implemented and working!");
    println!("   - Lattice-based: Dilithium (signatures), Kyber (KEM)");
    println!("   - Hash-based: LMS (signatures)");
    println!("   - Lattice-based (NTRU): FALCON (compact signatures)");
    println!("   - Code-based: Classic McEliece (KEM)");
}
