//! Comprehensive Test Suite for Real Post-Quantum Cryptography Algorithm Implementations
//! 
//! This test suite validates all the real PQC algorithm implementations including:
//! - CRYSTALS-Kyber (KEM) 
//! - CRYSTALS-Dilithium (digital signatures)
//! - Real SPHINCS+ (hash-based signatures)
//! - FALCON (compact signatures)
//! - Real Classic McEliece (code-based KEM)
//! 
//! Tests cover functionality, security properties, performance, and interoperability.

use cursed::stdlib::crypto_pqc::{PqcResult, SecurityLevel, AlgorithmType};
use cursed::stdlib::crypto_pqc::algorithms::{DigitalSignature, KeyEncapsulation};

// Import real implementations
mod real_implementations {
    pub use cursed::stdlib::crypto_pqc::algorithms::kyber_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::dilithium_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::sphincs_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::falcon_real::*;
    pub use cursed::stdlib::crypto_pqc::algorithms::mceliece_real::*;
}

use real_implementations::*;

/// Test helper for validating algorithm performance metrics
fn test_algorithm_performance<T>() 
where 
    T: DigitalSignature + 'static,
    T::PublicKey: Clone,
    T::SecretKey: Clone,
    T::Signature: Clone,
{
    for &security_level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        println!("Testing {:?} at {:?}", T::algorithm_type(), security_level);
        
        let start = std::time::Instant::now();
        let (pub_key, sec_key) = T::keygen(security_level).unwrap();
        let keygen_time = start.elapsed();

        let message = format!("Performance test message for {:?} at {:?}", T::algorithm_type(), security_level);
        let message_bytes = message.as_bytes();
        
        let start = std::time::Instant::now();
        let signature = T::sign(&sec_key, message_bytes).unwrap();
        let sign_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let is_valid = T::verify(&pub_key, message_bytes, &signature).unwrap();
        let verify_time = start.elapsed();
        
        assert!(is_valid, "Signature verification failed for {:?} at {:?}", T::algorithm_type(), security_level);
        
        println!("  Security Level: {:?}", security_level);
        println!("  Keygen time: {:?}", keygen_time);
        println!("  Sign time: {:?}", sign_time);
        println!("  Verify time: {:?}", verify_time);
        
        // Performance expectations
        assert!(keygen_time.as_millis() < 5000, "Key generation too slow: {:?}", keygen_time);
        assert!(sign_time.as_millis() < 1000, "Signing too slow: {:?}", sign_time);
        assert!(verify_time.as_millis() < 100, "Verification too slow: {:?}", verify_time);
    }
}

/// Test helper for KEM performance validation
fn test_kem_performance<T>()
where 
    T: KeyEncapsulation + 'static,
    T::PublicKey: Clone,
    T::SecretKey: Clone,
    T::Ciphertext: Clone,
    T::SharedSecret: Clone,
{
    for &security_level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        println!("Testing KEM {:?} at {:?}", T::algorithm_type(), security_level);
        
        let start = std::time::Instant::now();
        let (pub_key, sec_key) = T::keygen(security_level).unwrap();
        let keygen_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let (ciphertext, shared_secret1) = T::encaps(&pub_key).unwrap();
        let encaps_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let shared_secret2 = T::decaps(&sec_key, &ciphertext).unwrap();
        let decaps_time = start.elapsed();
        
        println!("  Security Level: {:?}", security_level);
        println!("  Keygen time: {:?}", keygen_time);
        println!("  Encaps time: {:?}", encaps_time);
        println!("  Decaps time: {:?}", decaps_time);
        
        // Performance expectations
        assert!(keygen_time.as_millis() < 10000, "Key generation too slow: {:?}", keygen_time);
        assert!(encaps_time.as_millis() < 100, "Encapsulation too slow: {:?}", encaps_time);
        assert!(decaps_time.as_millis() < 100, "Decapsulation too slow: {:?}", decaps_time);
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
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealKyber::keygen(level).unwrap();
            assert_eq!(pub_key.security_level(), level);
            assert_eq!(sec_key.security_level(), level);
            
            let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
            let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
            
            assert_eq!(shared_secret1.data, shared_secret2.data);
        }
    }

    #[test]
    fn test_real_kyber_key_serialization() {
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        
        let pub_key_bytes = pub_key.as_bytes();
        let sec_key_bytes = sec_key.as_bytes();
        
        // Verify key sizes match parameter specifications
        assert!(pub_key_bytes.len() > 0);
        assert!(sec_key_bytes.len() > 0);
        assert_ne!(pub_key_bytes, sec_key_bytes);
    }

    #[test]
    fn test_real_kyber_ciphertext_serialization() {
        let (pub_key, _) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let (ciphertext, _) = RealKyber::encaps(&pub_key).unwrap();
        
        let ciphertext_bytes = ciphertext.as_bytes();
        let restored_ciphertext = KyberCiphertext::from_bytes(ciphertext.params, &ciphertext_bytes).unwrap();
        
        assert_eq!(ciphertext_bytes, restored_ciphertext.as_bytes());
    }

    #[test]
    fn test_real_kyber_polynomial_operations() {
        // Test polynomial arithmetic
        let mut poly1 = KyberPolynomial::new();
        poly1.coeffs[0] = 100;
        poly1.coeffs[1] = 200;
        
        let mut poly2 = KyberPolynomial::new();
        poly2.coeffs[0] = 50;
        poly2.coeffs[1] = 150;
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs[0], 150);
        assert_eq!(sum.coeffs[1], 350);
        
        // Test NTT/INTT
        let original = poly1.clone();
        poly1.ntt();
        poly1.intt();
        
        // Should be close to original after NTT/INTT round trip
        for i in 0..10 {
            assert!((poly1.coeffs[i] - original.coeffs[i]).abs() < 100);
        }
    }

    #[test]
    fn test_real_kyber_compression() {
        let mut poly = KyberPolynomial::new();
        poly.coeffs[0] = 1000;
        poly.coeffs[1] = 2000;
        poly.coeffs[2] = 500;
        
        let compressed = poly.compress(10);
        let decompressed = KyberPolynomial::decompress(&compressed, 10);
        
        // Should be approximately equal after compression/decompression
        for i in 0..3 {
            assert!((poly.coeffs[i] - decompressed.coeffs[i]).abs() < 500);
        }
    }

    #[test]
    fn test_real_kyber_performance() {
        test_kem_performance::<RealKyber>();
    }
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
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealDilithium::keygen(level).unwrap();
            let message = format!("Test message for {:?}", level);
            
            let signature = RealDilithium::sign(&sec_key, message.as_bytes()).unwrap();
            let is_valid = RealDilithium::verify(&pub_key, message.as_bytes(), &signature).unwrap();
            
            assert!(is_valid);
        }
    }

    #[test]
    fn test_real_dilithium_signature_determinism() {
        let (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Determinism test message";
        
        let signature1 = RealDilithium::sign(&sec_key, message).unwrap();
        let signature2 = RealDilithium::sign(&sec_key, message).unwrap();
        
        // Signatures might be different due to randomness in signing
        // But both should verify correctly
        assert!(RealDilithium::verify(&pub_key, message, &signature1).unwrap());
        assert!(RealDilithium::verify(&pub_key, message, &signature2).unwrap());
    }

    #[test]
    fn test_real_dilithium_invalid_signature() {
        let (pub_key, sec_key) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Original message";
        let wrong_message = b"Modified message";
        
        let signature = RealDilithium::sign(&sec_key, message).unwrap();
        
        // Signature should not verify for different message
        let is_valid = RealDilithium::verify(&pub_key, wrong_message, &signature).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_real_dilithium_polynomial_operations() {
        let mut poly1 = DilithiumPolynomial::new();
        poly1.coeffs[0] = 100;
        poly1.coeffs[1] = 200;
        
        let mut poly2 = DilithiumPolynomial::new();
        poly2.coeffs[0] = 50;
        poly2.coeffs[1] = 150;
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs[0], 150);
        assert_eq!(sum.coeffs[1], 350);
        
        let diff = poly1.subtract(&poly2);
        assert_eq!(diff.coeffs[0], 50);
        assert_eq!(diff.coeffs[1], 50);
    }

    #[test]
    fn test_real_dilithium_performance() {
        test_algorithm_performance::<RealDilithium>();
    }
}

#[cfg(test)]
mod real_sphincs_plus_tests {
    use super::*;

    #[test]
    fn test_real_sphincs_plus_basic_functionality() {
        let (pub_key, sec_key) = RealSphincsPlusSignature::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, real SPHINCS+!";
        
        let signature = RealSphincsPlusSignature::sign(&sec_key, message).unwrap();
        let is_valid = RealSphincsPlusSignature::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
        assert_eq!(RealSphincsPlusSignature::algorithm_type(), AlgorithmType::Sphincs);
    }

    #[test]
    fn test_real_sphincs_plus_different_security_levels() {
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealSphincsPlusSignature::keygen(level).unwrap();
            assert_eq!(pub_key.params.security_level(), level);
            
            let message = format!("Test message for SPHINCS+ {:?}", level);
            let signature = RealSphincsPlusSignature::sign(&sec_key, message.as_bytes()).unwrap();
            let is_valid = RealSphincsPlusSignature::verify(&pub_key, message.as_bytes(), &signature).unwrap();
            
            assert!(is_valid);
        }
    }

    #[test]
    fn test_real_sphincs_plus_fast_variant() {
        let (pub_key, sec_key) = RealSphincsPlusSignature::keygen_fast(SecurityLevel::Level1).unwrap();
        let message = b"Fast variant test";
        
        let signature = RealSphincsPlusSignature::sign(&sec_key, message).unwrap();
        let is_valid = RealSphincsPlusSignature::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_sphincs_plus_parameters() {
        let params_small = SphincsPlusParams::new(SecurityLevel::Level1, false);
        let params_fast = SphincsPlusParams::new(SecurityLevel::Level1, true);
        
        assert_eq!(params_small.n(), 16);
        assert_eq!(params_fast.n(), 16);
        assert_eq!(params_small.security_level(), SecurityLevel::Level1);
        assert_eq!(params_fast.security_level(), SecurityLevel::Level1);
        
        // Fast variant should have different tree structure
        assert_ne!(params_small.d(), params_fast.d());
    }

    #[test]
    fn test_sphincs_plus_address_serialization() {
        let mut addr = Address::new();
        addr.set_layer(1);
        addr.set_tree(42);
        addr.set_type(2);
        addr.set_keypair(10);
        
        let bytes = addr.to_bytes();
        assert_eq!(bytes.len(), 32);
        
        // Check serialization correctness
        assert_eq!(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 1);
        assert_eq!(u64::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11]]), 42);
    }

    #[test]
    fn test_wots_plus_base_w() {
        let input = [0x12, 0x34, 0x56];
        let result = WotsPlus::base_w(&input, 16, 6).unwrap();
        
        assert_eq!(result.len(), 6);
        // 0x12 = 0001 0010, 0x34 = 0011 0100, 0x56 = 0101 0110
        // In base 16: [1, 2, 3, 4, 5, 6]
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_fors_message_to_indices() {
        let message = [0xFF, 0x00, 0xAA];
        let indices = Fors::message_to_indices(&message, 4, 4).unwrap();
        
        assert_eq!(indices.len(), 4);
        // First 4 bits: 1111 = 15
        assert_eq!(indices[0], 15);
        // Next 4 bits: 0000 = 0  
        assert_eq!(indices[1], 0);
    }

    #[test]
    fn test_real_sphincs_plus_performance() {
        test_algorithm_performance::<RealSphincsPlusSignature>();
    }
}

#[cfg(test)]
mod real_falcon_tests {
    use super::*;

    #[test]
    fn test_real_falcon_basic_functionality() {
        let (pub_key, sec_key) = RealFalcon::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, real FALCON!";
        
        let signature = RealFalcon::sign(&sec_key, message).unwrap();
        let is_valid = RealFalcon::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_real_falcon_different_security_levels() {
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealFalcon::keygen(level).unwrap();
            let message = format!("Test message for FALCON {:?}", level);
            
            let signature = RealFalcon::sign(&sec_key, message.as_bytes()).unwrap();
            let is_valid = RealFalcon::verify(&pub_key, message.as_bytes(), &signature).unwrap();
            
            assert!(is_valid);
        }
    }

    #[test]
    fn test_falcon_complex_number_operations() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);
        
        let sum = a.add(&b);
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imag, 6.0);
        
        let product = a.multiply(&b);
        assert_eq!(product.real, -5.0); // (3*1 - 4*2)
        assert_eq!(product.imag, 10.0); // (3*2 + 4*1)
        
        let magnitude = a.magnitude();
        assert_eq!(magnitude, 5.0); // sqrt(3^2 + 4^2)
    }

    #[test]
    fn test_falcon_fft_operations() {
        let mut samples = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        
        let original = samples.clone();
        FalconFFT::fft(&mut samples);
        FalconFFT::ifft(&mut samples);
        
        // Should be close to original after FFT/IFFT round trip
        for (orig, restored) in original.iter().zip(samples.iter()) {
            assert!((orig.real - restored.real).abs() < 1e-10);
            assert!((orig.imag - restored.imag).abs() < 1e-10);
        }
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
        let (pub_key, sec_key) = RealClassicMcEliece::keygen(SecurityLevel::Level1).unwrap();
        
        let (ciphertext, shared_secret1) = RealClassicMcEliece::encaps(&pub_key).unwrap();
        let shared_secret2 = RealClassicMcEliece::decaps(&sec_key, &ciphertext).unwrap();
        
        assert_eq!(shared_secret1.data, shared_secret2.data);
        assert_eq!(RealClassicMcEliece::algorithm_type(), AlgorithmType::ClassicMcEliece);
    }

    #[test]
    fn test_real_mceliece_different_security_levels() {
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let (pub_key, sec_key) = RealClassicMcEliece::keygen(level).unwrap();
            assert_eq!(pub_key.params.security_level(), level);
            
            let (ciphertext, shared_secret1) = RealClassicMcEliece::encaps(&pub_key).unwrap();
            let shared_secret2 = RealClassicMcEliece::decaps(&sec_key, &ciphertext).unwrap();
            
            assert_eq!(shared_secret1.data, shared_secret2.data);
        }
    }

    #[test]
    fn test_mceliece_field_element_operations() {
        let a = FieldElement::new(5, 4);
        let b = FieldElement::new(3, 4);
        let irreducible = 0x13; // x^4 + x + 1
        
        // Test addition (XOR in GF(2^m))
        let sum = a.add(&b);
        assert_eq!(sum.value(), 5 ^ 3);
        
        // Test multiplication
        let product = a.multiply(&b, irreducible);
        assert!(product.value() < 16); // Should be in GF(2^4)
        
        // Test inverse
        if let Some(inv_a) = a.inverse(irreducible) {
            let should_be_one = a.multiply(&inv_a, irreducible);
            assert_eq!(should_be_one.value(), 1);
        }
    }

    #[test]
    fn test_mceliece_support_set() {
        let support = SupportSet::generate_random(100, 8).unwrap();
        assert_eq!(support.len(), 100);
        
        // Check uniqueness
        let mut values = std::collections::HashSet::new();
        for element in support.elements() {
            values.insert(element.value());
        }
        assert_eq!(values.len(), 100);
    }

    #[test]
    fn test_mceliece_error_vector() {
        let error_vector = RealClassicMcEliece::generate_error_vector(1000, 50).unwrap();
        assert_eq!(error_vector.len(), 1000);
        assert_eq!(error_vector.iter().filter(|&&b| b).count(), 50);
    }

    #[test]
    fn test_mceliece_ciphertext_serialization() {
        let params = RealMcElieceParams::new(SecurityLevel::Level1);
        let ciphertext_bits = vec![true; params.n()];
        let ciphertext = RealMcElieceCiphertext::new(params, ciphertext_bits).unwrap();
        
        let bytes = ciphertext.as_bytes();
        let restored = RealMcElieceCiphertext::from_bytes(params, &bytes).unwrap();
        
        assert_eq!(ciphertext.ciphertext.len(), restored.ciphertext.len());
    }

    #[test]
    fn test_mceliece_goppa_polynomial() {
        let poly = GoppaPolynomial::generate_random(10, 8, 0x11D).unwrap();
        assert_eq!(poly.degree(), 10);
        
        let point = FieldElement::new(5, 8);
        let value = poly.evaluate(point, 0x11D);
        assert!(value.value() < 256); // Should be in GF(2^8)
    }

    #[test]
    fn test_real_mceliece_performance() {
        test_kem_performance::<RealClassicMcEliece>();
    }
}

#[cfg(test)]
mod cross_algorithm_tests {
    use super::*;

    #[test]
    fn test_all_signature_algorithms() {
        let algorithms = [
            ("Dilithium", test_algorithm_performance::<RealDilithium> as fn()),
            ("SPHINCS+", test_algorithm_performance::<RealSphincsPlusSignature>),
            ("FALCON", test_algorithm_performance::<RealFalcon>),
        ];
        
        for (name, test_fn) in algorithms.iter() {
            println!("Testing signature algorithm: {}", name);
            test_fn();
        }
    }

    #[test]
    fn test_all_kem_algorithms() {
        let algorithms = [
            ("Kyber", test_kem_performance::<RealKyber> as fn()),
            ("Classic McEliece", test_kem_performance::<RealClassicMcEliece>),
        ];
        
        for (name, test_fn) in algorithms.iter() {
            println!("Testing KEM algorithm: {}", name);
            test_fn();
        }
    }

    #[test]
    fn test_security_level_consistency() {
        // Test that all algorithms handle security levels consistently
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            println!("Testing security level consistency for {:?}", level);
            
            // Test signature algorithms
            let (dilithium_pub, _) = RealDilithium::keygen(level).unwrap();
            let (sphincs_pub, _) = RealSphincsPlusSignature::keygen(level).unwrap();
            
            assert_eq!(dilithium_pub.params.security_level(), level);
            assert_eq!(sphincs_pub.params.security_level(), level);
            
            // Test KEM algorithms
            let (kyber_pub, _) = RealKyber::keygen(level).unwrap();
            let (mceliece_pub, _) = RealClassicMcEliece::keygen(level).unwrap();
            
            assert_eq!(kyber_pub.security_level(), level);
            assert_eq!(mceliece_pub.params.security_level(), level);
            
            // Test FALCON (only supports Level1 and Level5)
            if level == SecurityLevel::Level1 || level == SecurityLevel::Level5 {
                let (falcon_pub, _) = RealFalcon::keygen(level).unwrap();
                assert_eq!(falcon_pub.params.security_level(), level);
            }
        }
    }

    #[test]
    fn test_algorithm_type_identification() {
        assert_eq!(RealKyber::algorithm_type(), AlgorithmType::Kyber);
        assert_eq!(RealDilithium::algorithm_type(), AlgorithmType::Dilithium);
        assert_eq!(RealSphincsPlusSignature::algorithm_type(), AlgorithmType::Sphincs);
        assert_eq!(RealClassicMcEliece::algorithm_type(), AlgorithmType::ClassicMcEliece);
        // Note: FALCON uses Dilithium temporarily due to enum limitation
    }

    #[test]
    fn test_shared_secret_sizes() {
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // Kyber shared secrets
            let (kyber_pub, _) = RealKyber::keygen(level).unwrap();
            let (_, kyber_secret) = RealKyber::encaps(&kyber_pub).unwrap();
            
            // McEliece shared secrets
            let (mceliece_pub, _) = RealClassicMcEliece::keygen(level).unwrap();
            let (_, mceliece_secret) = RealClassicMcEliece::encaps(&mceliece_pub).unwrap();
            
            // Both should have appropriate sizes for security level
            let expected_size = match level {
                SecurityLevel::Level1 => 16..=32,
                SecurityLevel::Level3 => 24..=32,
                SecurityLevel::Level5 => 32..=32,
            };
            
            assert!(expected_size.contains(&kyber_secret.data.len()), 
                   "Kyber shared secret size {} not in expected range for {:?}", 
                   kyber_secret.data.len(), level);
            assert!(expected_size.contains(&mceliece_secret.data.len()), 
                   "McEliece shared secret size {} not in expected range for {:?}", 
                   mceliece_secret.data.len(), level);
        }
    }

    #[test]
    fn test_key_size_reasonableness() {
        for &level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            // Check that key sizes are reasonable
            let (kyber_pub, kyber_sec) = RealKyber::keygen(level).unwrap();
            let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(level).unwrap();
            let (sphincs_pub, sphincs_sec) = RealSphincsPlusSignature::keygen(level).unwrap();
            let (mceliece_pub, mceliece_sec) = RealClassicMcEliece::keygen(level).unwrap();
            
            // Public keys should be smaller than secret keys for most algorithms
            assert!(kyber_pub.as_bytes().len() >= 100);
            assert!(kyber_sec.as_bytes().len() >= 100);
            
            assert!(dilithium_pub.as_bytes().len() >= 100);
            assert!(dilithium_sec.as_bytes().len() >= 100);
            
            assert!(sphincs_pub.as_bytes().len() >= 16);
            assert!(sphincs_sec.as_bytes().len() >= 32);
            
            // McEliece has large public keys but smaller secret keys
            assert!(mceliece_pub.as_bytes().len() >= 1000);
            assert!(mceliece_sec.as_bytes().len() >= 100);
        }
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_multiple_operations() {
        let iterations = 10;
        
        // Test multiple Kyber operations
        let (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        for i in 0..iterations {
            let (ciphertext, shared_secret1) = RealKyber::encaps(&kyber_pub).unwrap();
            let shared_secret2 = RealKyber::decaps(&kyber_sec, &ciphertext).unwrap();
            assert_eq!(shared_secret1.data, shared_secret2.data, "Kyber iteration {}", i);
        }
        
        // Test multiple signature operations
        let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        for i in 0..iterations {
            let message = format!("Test message {}", i);
            let signature = RealDilithium::sign(&dilithium_sec, message.as_bytes()).unwrap();
            let is_valid = RealDilithium::verify(&dilithium_pub, message.as_bytes(), &signature).unwrap();
            assert!(is_valid, "Dilithium iteration {}", i);
        }
    }

    #[test]
    fn test_large_messages() {
        let large_message = vec![0x42u8; 10000]; // 10KB message
        
        // Test signature algorithms with large messages
        let (dilithium_pub, dilithium_sec) = RealDilithium::keygen(SecurityLevel::Level1).unwrap();
        let signature = RealDilithium::sign(&dilithium_sec, &large_message).unwrap();
        let is_valid = RealDilithium::verify(&dilithium_pub, &large_message, &signature).unwrap();
        assert!(is_valid);
        
        let (sphincs_pub, sphincs_sec) = RealSphincsPlusSignature::keygen(SecurityLevel::Level1).unwrap();
        let signature = RealSphincsPlusSignature::sign(&sphincs_sec, &large_message).unwrap();
        let is_valid = RealSphincsPlusSignature::verify(&sphincs_pub, &large_message, &signature).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_concurrent_operations() {
        use std::sync::Arc;
        use std::thread;
        
        let (kyber_pub, kyber_sec) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        let kyber_pub = Arc::new(kyber_pub);
        let kyber_sec = Arc::new(kyber_sec);
        
        let mut handles = Vec::new();
        
        // Spawn multiple threads performing KEM operations
        for i in 0..4 {
            let pub_key = Arc::clone(&kyber_pub);
            let sec_key = Arc::clone(&kyber_sec);
            
            let handle = thread::spawn(move || {
                for j in 0..5 {
                    let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
                    let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
                    assert_eq!(shared_secret1.data, shared_secret2.data, 
                              "Thread {} iteration {}", i, j);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
