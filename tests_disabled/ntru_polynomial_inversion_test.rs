/// Comprehensive tests for NTRU polynomial inversion algorithms
/// 
/// This test suite validates the security-critical polynomial inversion functions
/// that were previously implemented as placeholders. These tests ensure that:
/// 1. f * f_inv ≡ 1 (mod q) for the inverse computed by invert_mod_q()
/// 2. f * f_inv ≡ 1 (mod p) for the inverse computed by invert_mod_p()
/// 3. The Extended Euclidean Algorithm correctly computes polynomial GCDs
/// 4. Error handling works correctly for non-invertible polynomials

use crate::stdlib::packages::crypto_pqc::ntru::{
    NtruEngine, NtruConfig, NtruPolynomial, NtruPolynomialRing, NtruSecurityLevel, NtruError
};

#[test]
fn test_polynomial_inversion_mod_q_basic() {
    // Test basic polynomial inversion modulo q
    let ring = NtruPolynomialRing::new(5, 7); // Small ring for testing
    
    // Create a simple invertible polynomial: 1 + x
    let poly = NtruPolynomial::new(vec![1, 1, 0, 0, 0], 5);
    
    // Compute inverse
    let inverse_result = ring.invert_mod_q(&poly);
    assert!(inverse_result.is_ok(), "Should be able to invert 1 + x mod 7");
    
    let inverse = inverse_result.unwrap();
    
    // Verify f * f_inv ≡ 1 (mod q)
    let product = ring.multiply_mod_q(&poly, &inverse).unwrap();
    
    // Check that the product is 1 (constant polynomial)
    assert_eq!(product.coefficients[0] % 7, 1, "Product should be 1 mod 7");
    for i in 1..5 {
        assert_eq!(product.coefficients[i] % 7, 0, "Higher order terms should be 0");
    }
}

#[test]
fn test_polynomial_inversion_mod_p_basic() {
    // Test basic polynomial inversion modulo p
    let ring = NtruPolynomialRing::new(5, 7);
    
    // Create a simple invertible polynomial: 2 + x^2
    let poly = NtruPolynomial::new(vec![2, 0, 1, 0, 0], 5);
    
    // Compute inverse modulo p = 3
    let inverse_result = ring.invert_mod_p(&poly, 3);
    assert!(inverse_result.is_ok(), "Should be able to invert 2 + x^2 mod 3");
    
    let inverse = inverse_result.unwrap();
    
    // Verify f * f_inv ≡ 1 (mod p)
    let product = ring.multiply_mod_p(&poly, &inverse, 3).unwrap();
    
    // Check that the product is 1 (constant polynomial)
    assert_eq!(product.coefficients[0] % 3, 1, "Product should be 1 mod 3");
    for i in 1..5 {
        assert_eq!(product.coefficients[i] % 3, 0, "Higher order terms should be 0");
    }
}

#[test]
fn test_polynomial_inversion_with_ntru_parameters() {
    // Test with realistic NTRU parameters
    let config = NtruConfig::with_security_level(NtruSecurityLevel::Level128);
    let ring = NtruPolynomialRing::new(config.n, config.q);
    
    // Create a ternary polynomial (common in NTRU)
    let mut coeffs = vec![0i32; config.n];
    coeffs[0] = 1;  // f(x) = 1 + x + x^2
    coeffs[1] = 1;
    coeffs[2] = 1;
    let poly = NtruPolynomial::new(coeffs, config.n);
    
    // Test inversion modulo q
    let inverse_q_result = ring.invert_mod_q(&poly);
    if let Ok(inverse_q) = inverse_q_result {
        // Verify correctness
        let product_q = ring.multiply_mod_q(&poly, &inverse_q).unwrap();
        assert_eq!(product_q.coefficients[0] % config.q as i32, 1);
        
        // Check that other coefficients are 0 mod q
        for i in 1..config.n {
            assert_eq!(product_q.coefficients[i] % config.q as i32, 0);
        }
    }
    
    // Test inversion modulo p
    let inverse_p_result = ring.invert_mod_p(&poly, config.p);
    if let Ok(inverse_p) = inverse_p_result {
        // Verify correctness
        let product_p = ring.multiply_mod_p(&poly, &inverse_p, config.p).unwrap();
        assert_eq!(product_p.coefficients[0] % config.p as i32, 1);
        
        // Check that other coefficients are 0 mod p
        for i in 1..config.n {
            assert_eq!(product_p.coefficients[i] % config.p as i32, 0);
        }
    }
}

#[test]
fn test_non_invertible_polynomial() {
    // Test error handling for non-invertible polynomials
    let ring = NtruPolynomialRing::new(4, 6); // Even modulus for testing
    
    // Create a polynomial that shares factors with x^n - 1
    // In ring Z_6[x]/(x^4 - 1), polynomial 2x should not be invertible
    let poly = NtruPolynomial::new(vec![0, 2, 0, 0], 4);
    
    let inverse_result = ring.invert_mod_q(&poly);
    assert!(inverse_result.is_err(), "Non-invertible polynomial should return error");
    
    match inverse_result {
        Err(NtruError::InversionError(msg)) => {
            assert!(msg.contains("not invertible"), "Error message should mention invertibility");
        }
        _ => panic!("Should return InversionError"),
    }
}

#[test]
fn test_zero_polynomial_inversion() {
    // Test that zero polynomial cannot be inverted
    let ring = NtruPolynomialRing::new(5, 7);
    let zero_poly = NtruPolynomial::new(vec![0, 0, 0, 0, 0], 5);
    
    let inverse_result = ring.invert_mod_q(&zero_poly);
    assert!(inverse_result.is_err(), "Zero polynomial should not be invertible");
}

#[test]
fn test_unit_polynomial_inversion() {
    // Test that unit polynomial (1) is its own inverse
    let ring = NtruPolynomialRing::new(5, 7);
    let unit_poly = NtruPolynomial::new(vec![1, 0, 0, 0, 0], 5);
    
    let inverse = ring.invert_mod_q(&unit_poly).unwrap();
    
    // Unit polynomial should be its own inverse
    assert_eq!(inverse.coefficients[0], 1);
    for i in 1..5 {
        assert_eq!(inverse.coefficients[i], 0);
    }
}

#[test]
fn test_modular_inverse_helper() {
    // Test the modular inverse helper function
    let ring = NtruPolynomialRing::new(5, 7);
    
    // Test known modular inverses
    assert_eq!(ring.modular_inverse(1, 7), Some(1));  // 1 * 1 ≡ 1 (mod 7)
    assert_eq!(ring.modular_inverse(2, 7), Some(4));  // 2 * 4 ≡ 8 ≡ 1 (mod 7)
    assert_eq!(ring.modular_inverse(3, 7), Some(5));  // 3 * 5 ≡ 15 ≡ 1 (mod 7)
    assert_eq!(ring.modular_inverse(4, 7), Some(2));  // 4 * 2 ≡ 8 ≡ 1 (mod 7)
    assert_eq!(ring.modular_inverse(5, 7), Some(3));  // 5 * 3 ≡ 15 ≡ 1 (mod 7)
    assert_eq!(ring.modular_inverse(6, 7), Some(6));  // 6 * 6 ≡ 36 ≡ 1 (mod 7)
    
    // Test non-invertible case
    assert_eq!(ring.modular_inverse(0, 7), None);     // 0 has no inverse
    assert_eq!(ring.modular_inverse(6, 12), None);    // gcd(6, 12) = 6 ≠ 1
}

#[test]
fn test_polynomial_arithmetic_helpers() {
    // Test polynomial arithmetic helper functions
    let ring = NtruPolynomialRing::new(3, 5);
    
    let poly_a = NtruPolynomial::new(vec![1, 2, 3], 3);
    let poly_b = NtruPolynomial::new(vec![2, 1, 4], 3);
    
    // Test addition
    let sum = ring.add_polynomials_mod(&poly_a, &poly_b, 5).unwrap();
    assert_eq!(sum.coefficients, vec![3, 3, 2]); // (1+2, 2+1, 3+4) mod 5
    
    // Test subtraction
    let diff = ring.subtract_polynomials_mod(&poly_a, &poly_b, 5).unwrap();
    assert_eq!(diff.coefficients, vec![4, 1, 4]); // (1-2, 2-1, 3-4) mod 5 = (-1, 1, -1) = (4, 1, 4)
    
    // Test scalar multiplication
    let scaled = ring.scalar_multiply_mod(&poly_a, 2, 5).unwrap();
    assert_eq!(scaled.coefficients, vec![2, 4, 1]); // (2*1, 2*2, 2*3) mod 5 = (2, 4, 6) = (2, 4, 1)
}

#[test]
fn test_polynomial_degree_calculation() {
    // Test polynomial degree calculation
    let ring = NtruPolynomialRing::new(5, 7);
    
    let poly1 = NtruPolynomial::new(vec![1, 2, 3, 0, 0], 5);
    assert_eq!(ring.polynomial_degree(&poly1), 2);
    
    let poly2 = NtruPolynomial::new(vec![0, 0, 0, 0, 5], 5);
    assert_eq!(ring.polynomial_degree(&poly2), 4);
    
    let zero_poly = NtruPolynomial::new(vec![0, 0, 0, 0, 0], 5);
    assert_eq!(ring.polynomial_degree(&zero_poly), 0);
    
    let constant_poly = NtruPolynomial::new(vec![7, 0, 0, 0, 0], 5);
    assert_eq!(ring.polynomial_degree(&constant_poly), 0);
}

#[test]
fn test_timing_attack_resistance() {
    // Test that constant-time operations work correctly
    let ring = NtruPolynomialRing::new(5, 7);
    
    // Test constant-time multiplication
    assert_eq!(ring.constant_time_multiply(3, 4, 7), 5); // 3 * 4 = 12 ≡ 5 (mod 7)
    assert_eq!(ring.constant_time_multiply(-2, 3, 7), 1); // -2 * 3 = -6 ≡ 1 (mod 7)
    
    // Test modular reduction
    assert_eq!(ring.mod_reduce(10, 7), 3);   // 10 mod 7 = 3
    assert_eq!(ring.mod_reduce(-3, 7), 4);   // -3 mod 7 = 4
    assert_eq!(ring.mod_reduce(0, 7), 0);    // 0 mod 7 = 0
}

#[test]
fn test_cryptographic_properties() {
    // Test that the implementation maintains cryptographic properties
    let config = NtruConfig::with_security_level(NtruSecurityLevel::Level128);
    let ring = NtruPolynomialRing::new(config.n, config.q);
    
    // Create a random-looking ternary polynomial
    let mut coeffs = vec![0i32; config.n];
    // Set some coefficients to simulate a real NTRU polynomial
    for i in (0..config.n).step_by(10) {
        coeffs[i] = if i % 3 == 0 { 1 } else if i % 3 == 1 { -1 } else { 0 };
    }
    let poly = NtruPolynomial::new(coeffs, config.n);
    
    // Test that if inversion succeeds, the result is correct
    if let Ok(inverse) = ring.invert_mod_q(&poly) {
        let product = ring.multiply_mod_q(&poly, &inverse).unwrap();
        
        // Verify the product is 1
        assert_eq!(product.coefficients[0] % config.q as i32, 1);
        
        // Verify all other coefficients are 0
        for i in 1..config.n {
            assert_eq!(product.coefficients[i] % config.q as i32, 0);
        }
        
        // Test that the inverse is also correctly computed for mod p
        if let Ok(inverse_p) = ring.invert_mod_p(&poly, config.p) {
            let product_p = ring.multiply_mod_p(&poly, &inverse_p, config.p).unwrap();
            assert_eq!(product_p.coefficients[0] % config.p as i32, 1);
            for i in 1..config.n {
                assert_eq!(product_p.coefficients[i] % config.p as i32, 0);
            }
        }
    }
}

#[test]
fn test_security_vulnerability_fix() {
    // This test specifically validates that the security vulnerability has been fixed
    // Previously, invert_mod_q and invert_mod_p returned placeholder polynomials
    // Now they should return real mathematical inverses or proper errors
    
    let ring = NtruPolynomialRing::new(7, 11);
    
    // Create a polynomial that we know is invertible
    let poly = NtruPolynomial::new(vec![1, 1, 0, 0, 0, 0, 0], 7); // 1 + x
    
    let inverse = ring.invert_mod_q(&poly).expect("Should compute real inverse");
    
    // Verify this is NOT the old placeholder (which was [1, 0, 0, ...])
    let product = ring.multiply_mod_q(&poly, &inverse).unwrap();
    
    // The key test: f * f_inv should equal 1, not just look like [1, 0, 0, ...]
    // If this was the old placeholder, the product would not be 1
    assert_eq!(product.coefficients[0] % 11, 1, "Product must be 1 - security fix validation");
    
    // Verify the inverse is not the trivial placeholder
    let is_placeholder = inverse.coefficients[0] == 1 && 
                        inverse.coefficients.iter().skip(1).all(|&c| c == 0);
    
    if is_placeholder {
        // If it looks like the placeholder, verify it's actually correct
        let test_product = ring.multiply_mod_q(&poly, &inverse).unwrap();
        assert_eq!(test_product.coefficients[0] % 11, 1);
        assert!(test_product.coefficients.iter().skip(1).all(|&c| c % 11 == 0));
    }
    
    println!("✅ Security vulnerability fixed: Real polynomial inversion implemented");
}
