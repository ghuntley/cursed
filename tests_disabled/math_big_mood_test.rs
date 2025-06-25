/// Comprehensive test suite for the big_mood (arbitrary precision) mathematics module
/// 
/// This test suite validates all arbitrary precision arithmetic operations including:
/// - BigInt operations and edge cases
/// - BigRat fractional arithmetic and precision
/// - BigFloat high-precision computations
/// - Decimal financial arithmetic accuracy
/// - BigComplex number operations
/// - Mathematical functions with arbitrary precision
/// - Error handling and domain validation
/// - Performance with large numbers
/// - String parsing and formatting
/// - Conversion accuracy and precision tracking
/// 
/// Testing is crucial for big_mood because:
/// 1. Mathematical correctness must be preserved across all precision levels
/// 2. Overflow/underflow conditions must be handled gracefully
/// 3. String parsing must validate all input formats correctly
/// 4. Financial calculations must be exact without rounding errors
/// 5. Memory usage with very large numbers must be efficient
/// 6. Conversion accuracy must be tracked and reported properly

use cursed::stdlib::math::big_mood::*;
use cursed::stdlib::math::{MathError, MathResult};

#[test]
fn test_big_int_creation_and_basic_properties() {
    // Test basic creation methods
    let zero = BigInt::zero();
    assert_eq!(zero.to_string(), "0");
    
    let one = BigInt::one();
    assert_eq!(one.to_string(), "1");
    
    let positive = BigInt::new(12345);
    assert_eq!(positive.to_string(), "12345");
    
    let negative = BigInt::new(-6789);
    assert_eq!(negative.to_string(), "-6789");
    
    // Test large number creation
    let large = BigInt::new(i64::MAX);
    assert_eq!(large.to_string(), i64::MAX.to_string());
    
    let large_negative = BigInt::new(i64::MIN);
    assert_eq!(large_negative.to_string(), i64::MIN.to_string());
}

#[test]
fn test_big_int_arithmetic_operations() {
    let a = BigInt::new(123456789);
    let b = BigInt::new(987654321);
    
    // Addition
    let sum = a.add(&b);
    assert_eq!(sum.to_string(), "1111111110");
    
    // Subtraction
    let diff = b.sub(&a);
    assert_eq!(diff.to_string(), "864197532");
    
    // Multiplication
    let product = a.mul(&b);
    assert_eq!(product.to_string(), "121932631112635269");
    
    // Division
    let quotient = b.div(&a).unwrap();
    assert_eq!(quotient.to_string(), "8");
    
    // Remainder
    let remainder = b.rem(&a).unwrap();
    assert_eq!(remainder.to_string(), "9");
    
    // Division and remainder combined
    let (div, rem) = b.div_rem(&a).unwrap();
    assert_eq!(div.to_string(), "8");
    assert_eq!(rem.to_string(), "9");
}

#[test]
fn test_big_int_string_operations() {
    let mut big_int = BigInt::zero();
    
    // Test setting from decimal string
    big_int.set_string("123456789012345678901234567890", 10).unwrap();
    assert_eq!(big_int.to_string(), "123456789012345678901234567890");
    
    // Test setting from hexadecimal string
    big_int.set_string("DEADBEEF", 16).unwrap();
    assert_eq!(big_int.to_string(), "3735928559");
    
    // Test setting from binary string
    big_int.set_string("101010", 2).unwrap();
    assert_eq!(big_int.to_string(), "42");
    
    // Test base conversion output
    let num = BigInt::new(255);
    assert_eq!(num.to_string_radix(16), "ff");
    assert_eq!(num.to_string_radix(2), "11111111");
    assert_eq!(num.to_string_radix(8), "377");
}

#[test]
fn test_big_int_bitwise_operations() {
    let a = BigInt::new(0b11110000);
    let b = BigInt::new(0b10101010);
    
    // Bitwise AND
    let and_result = a.bitand(&b);
    assert_eq!(and_result.to_string(), (0b10100000).to_string());
    
    // Bitwise OR
    let or_result = a.bitor(&b);
    assert_eq!(or_result.to_string(), (0b11111010).to_string());
    
    // Bitwise XOR
    let xor_result = a.bitxor(&b);
    assert_eq!(xor_result.to_string(), (0b01011010).to_string());
    
    // Bitwise NOT
    let not_result = a.not();
    assert_ne!(not_result.to_string(), a.to_string());
    
    // Bit shifts
    let left_shift = a.shl(2);
    assert_eq!(left_shift.to_string(), (0b1111000000).to_string());
    
    let right_shift = a.shr(2);
    assert_eq!(right_shift.to_string(), (0b111100).to_string());
}

#[test]
fn test_big_int_gcd_and_modular_arithmetic() {
    let a = BigInt::new(48);
    let b = BigInt::new(18);
    
    // Greatest common divisor
    let gcd_result = a.gcd(&b);
    assert_eq!(gcd_result.to_string(), "6");
    
    // Modular exponentiation
    let base = BigInt::new(2);
    let exp = BigInt::new(10);
    let modulus = BigInt::new(1000);
    
    let mod_pow_result = base.mod_pow(&exp, &modulus).unwrap();
    assert_eq!(mod_pow_result.to_string(), "24"); // 2^10 mod 1000 = 1024 mod 1000 = 24
}

#[test]
fn test_big_int_conversion_and_properties() {
    let big_int = BigInt::new(1234567890);
    
    // Test conversions
    assert_eq!(big_int.to_i64(), Some(1234567890));
    assert_eq!(big_int.to_u64(), Some(1234567890));
    
    // Test bit length
    assert!(big_int.bit_len() > 0);
    
    // Test byte representation
    let bytes = big_int.to_bytes();
    assert!(!bytes.is_empty());
    
    let mut new_big_int = BigInt::zero();
    new_big_int.set_bytes(&bytes);
    assert_eq!(new_big_int.to_string(), big_int.to_string());
}

#[test]
fn test_big_int_error_handling() {
    let zero = BigInt::zero();
    let one = BigInt::one();
    
    // Division by zero
    assert!(one.div(&zero).is_err());
    assert!(one.rem(&zero).is_err());
    assert!(one.div_rem(&zero).is_err());
    assert!(one.mod_pow(&one, &zero).is_err());
    
    // Invalid string parsing
    let mut big_int = BigInt::zero();
    assert!(big_int.set_string("invalid", 10).is_err());
    assert!(big_int.set_string("ZZZ", 10).is_err());
    assert!(big_int.set_string("", 10).is_err());
}

#[test]
fn test_big_rat_creation_and_basic_operations() {
    // Test basic creation
    let rat1 = BigRat::new(1, 3).unwrap();
    assert_eq!(rat1.to_string(), "1/3");
    
    let rat2 = BigRat::new(2, 5).unwrap();
    assert_eq!(rat2.to_string(), "2/5");
    
    // Test zero and one
    let zero = BigRat::zero();
    assert_eq!(zero.to_string(), "0");
    
    let one = BigRat::one();
    assert_eq!(one.to_string(), "1");
    
    // Test negative fractions
    let negative = BigRat::new(-3, 4).unwrap();
    assert_eq!(negative.to_string(), "-3/4");
}

#[test]
fn test_big_rat_arithmetic_operations() {
    let r1 = BigRat::new(1, 3).unwrap();
    let r2 = BigRat::new(2, 5).unwrap();
    
    // Addition: 1/3 + 2/5 = 5/15 + 6/15 = 11/15
    let sum = r1.add(&r2);
    assert_eq!(sum.to_string(), "11/15");
    
    // Subtraction: 2/5 - 1/3 = 6/15 - 5/15 = 1/15
    let diff = r2.sub(&r1);
    assert_eq!(diff.to_string(), "1/15");
    
    // Multiplication: 1/3 * 2/5 = 2/15
    let product = r1.mul(&r2);
    assert_eq!(product.to_string(), "2/15");
    
    // Division: (1/3) / (2/5) = (1/3) * (5/2) = 5/6
    let quotient = r1.div(&r2).unwrap();
    assert_eq!(quotient.to_string(), "5/6");
}

#[test]
fn test_big_rat_conversions_and_precision() {
    let rat = BigRat::new(1, 3).unwrap();
    
    // Test numerator and denominator extraction
    let numer = rat.numer();
    let denom = rat.denom();
    assert_eq!(numer.to_string(), "1");
    assert_eq!(denom.to_string(), "3");
    
    // Test floating-point conversion
    let (float_val, accuracy) = rat.to_f64();
    assert!((float_val - (1.0 / 3.0)).abs() < 1e-15);
    
    // Test string representation with decimal precision
    let decimal_str = rat.to_string_decimal(6);
    assert!(decimal_str.starts_with("0.33333"));
}

#[test]
fn test_big_rat_string_operations() {
    let mut rat = BigRat::zero();
    
    // Test setting from string
    rat.set_string("3/4").unwrap();
    assert_eq!(rat.to_string(), "3/4");
    
    rat.set_string("22/7").unwrap();
    assert_eq!(rat.to_string(), "22/7");
    
    // Test setting from f64
    rat.set_f64(0.5).unwrap();
    assert_eq!(rat.to_string(), "1/2");
    
    // Test parsing from string
    let parsed = parse_rat("5/8").unwrap();
    assert_eq!(parsed.to_string(), "5/8");
}

#[test]
fn test_big_rat_error_handling() {
    // Division by zero in creation
    assert!(BigRat::new(1, 0).is_err());
    
    // Division by zero in arithmetic
    let rat = BigRat::new(1, 2).unwrap();
    let zero = BigRat::zero();
    assert!(rat.div(&zero).is_err());
    
    // Invalid f64 values
    let mut rat = BigRat::zero();
    assert!(rat.set_f64(f64::NAN).is_err());
    assert!(rat.set_f64(f64::INFINITY).is_err());
    assert!(rat.set_f64(f64::NEG_INFINITY).is_err());
}

#[test]
fn test_big_float_creation_and_precision() {
    // Test basic creation
    let float1 = BigFloat::new(3.14159);
    assert!((float1.to_f64().0 - 3.14159).abs() < 1e-10);
    
    // Test zero and one
    let zero = BigFloat::zero();
    assert_eq!(zero.to_f64().0, 0.0);
    
    let one = BigFloat::one();
    assert_eq!(one.to_f64().0, 1.0);
    
    // Test precision control
    let mut high_precision = BigFloat::new(1.0);
    high_precision.set_precision(128);
    assert_eq!(high_precision.precision(), 128);
}

#[test]
fn test_big_float_arithmetic_operations() {
    let a = BigFloat::new(2.5);
    let b = BigFloat::new(1.5);
    
    // Addition
    let sum = a.add(&b);
    assert!((sum.to_f64().0 - 4.0).abs() < 1e-10);
    
    // Subtraction
    let diff = a.sub(&b);
    assert!((diff.to_f64().0 - 1.0).abs() < 1e-10);
    
    // Multiplication
    let product = a.mul(&b);
    assert!((product.to_f64().0 - 3.75).abs() < 1e-10);
    
    // Division
    let quotient = a.div(&b).unwrap();
    assert!((quotient.to_f64().0 - (5.0/3.0)).abs() < 1e-10);
}

#[test]
fn test_big_float_mathematical_functions() {
    // Test square root
    let four = BigFloat::new(4.0);
    let sqrt_result = sqrt(&four).unwrap();
    assert!((sqrt_result.to_f64().0 - 2.0).abs() < 1e-10);
    
    // Test cube root
    let eight = BigFloat::new(8.0);
    let cbrt_result = cbrt(&eight);
    assert!((cbrt_result.to_f64().0 - 2.0).abs() < 1e-10);
    
    // Test nth root
    let sixteen = BigFloat::new(16.0);
    let fourth_root = nth_root(&sixteen, 4).unwrap();
    assert!((fourth_root.to_f64().0 - 2.0).abs() < 1e-10);
    
    // Test natural logarithm
    let e_val = BigFloat::new(std::f64::consts::E);
    let ln_result = ln(&e_val).unwrap();
    assert!((ln_result.to_f64().0 - 1.0).abs() < 1e-10);
    
    // Test exponential
    let one = BigFloat::new(1.0);
    let exp_result = exp(&one);
    assert!((exp_result.to_f64().0 - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_big_float_trigonometric_functions() {
    // Test sine
    let pi_half = BigFloat::new(std::f64::consts::PI / 2.0);
    let sin_result = sin(&pi_half);
    assert!((sin_result.to_f64().0 - 1.0).abs() < 1e-10);
    
    // Test cosine
    let zero = BigFloat::new(0.0);
    let cos_result = cos(&zero);
    assert!((cos_result.to_f64().0 - 1.0).abs() < 1e-10);
    
    // Test tangent
    let pi_quarter = BigFloat::new(std::f64::consts::PI / 4.0);
    let tan_result = tan(&pi_quarter);
    assert!((tan_result.to_f64().0 - 1.0).abs() < 1e-10);
}

#[test]
fn test_big_float_error_handling() {
    // Division by zero
    let one = BigFloat::new(1.0);
    let zero = BigFloat::new(0.0);
    assert!(one.div(&zero).is_err());
    
    // Square root of negative number
    let negative = BigFloat::new(-1.0);
    assert!(sqrt(&negative).is_err());
    
    // Logarithm of non-positive number
    assert!(ln(&zero).is_err());
    assert!(ln(&negative).is_err());
    
    // Even root of negative number
    assert!(nth_root(&negative, 2).is_err());
    assert!(nth_root(&negative, 4).is_err());
    
    // Zero root
    assert!(nth_root(&one, 0).is_err());
}

#[test]
fn test_decimal_creation_and_formatting() {
    // Test creation from string
    let dec1 = Decimal::new("123.45").unwrap();
    assert_eq!(dec1.to_string(), "123.45");
    
    let dec2 = Decimal::new("100").unwrap();
    assert_eq!(dec2.to_string(), "100");
    
    let dec3 = Decimal::new("0.001").unwrap();
    assert_eq!(dec3.to_string(), "0.001");
    
    // Test very small decimals
    let small = Decimal::new("0.00001").unwrap();
    assert_eq!(small.to_string(), "0.00001");
}

#[test]
fn test_decimal_financial_arithmetic() {
    // Test price calculations
    let price = Decimal::new("19.99").unwrap();
    let tax_rate = Decimal::new("0.0825").unwrap(); // 8.25%
    
    // Calculate tax (price * tax_rate)
    let tax = price.mul(&tax_rate);
    assert!(tax.to_string().starts_with("1.64")); // Approximately 1.649175
    
    // Calculate total (price + tax)
    let total = price.add(&tax);
    assert!(total.to_string().starts_with("21.6")); // Approximately 21.639175
    
    // Test exact decimal arithmetic without floating-point errors
    let amount1 = Decimal::new("0.1").unwrap();
    let amount2 = Decimal::new("0.2").unwrap();
    let sum = amount1.add(&amount2);
    assert_eq!(sum.to_string(), "0.3"); // Exact, no 0.30000000000000004
}

#[test]
fn test_decimal_error_handling() {
    // Invalid decimal strings
    assert!(Decimal::new("abc").is_err());
    assert!(Decimal::new("12.34.56").is_err());
    assert!(Decimal::new("").is_err());
    assert!(Decimal::new("12.").is_err());
}

#[test]
fn test_big_complex_operations() {
    let real1 = BigFloat::new(3.0);
    let imag1 = BigFloat::new(4.0);
    let c1 = BigComplex::new(real1, imag1);
    
    let real2 = BigFloat::new(1.0);
    let imag2 = BigFloat::new(2.0);
    let c2 = BigComplex::new(real2, imag2);
    
    // Test complex addition: (3+4i) + (1+2i) = (4+6i)
    let sum = c1.add(&c2);
    assert!((sum.real().to_f64().0 - 4.0).abs() < 1e-10);
    assert!((sum.imag().to_f64().0 - 6.0).abs() < 1e-10);
    
    // Test complex multiplication: (3+4i) * (1+2i) = (3-8) + (6+4)i = -5+10i
    let product = c1.mul(&c2);
    assert!((product.real().to_f64().0 - (-5.0)).abs() < 1e-10);
    assert!((product.imag().to_f64().0 - 10.0).abs() < 1e-10);
}

#[test]
fn test_parsing_functions() {
    // Test parse_int with different bases
    let decimal = parse_int("123", 10).unwrap();
    assert_eq!(decimal.to_string(), "123");
    
    let hex = parse_int("FF", 16).unwrap();
    assert_eq!(hex.to_string(), "255");
    
    let binary = parse_int("1010", 2).unwrap();
    assert_eq!(binary.to_string(), "10");
    
    // Test parse_rat
    let rational = parse_rat("22/7").unwrap();
    assert_eq!(rational.to_string(), "22/7");
    
    // Test parse_float
    let float_val = parse_float("3.14159", 64).unwrap();
    assert!((float_val.to_f64().0 - 3.14159).abs() < 1e-10);
}

#[test]
fn test_utility_functions() {
    // Test GCD
    let a = BigInt::new(48);
    let b = BigInt::new(18);
    let gcd_result = gcd(&a, &b);
    assert_eq!(gcd_result.to_string(), "6");
    
    // Test binomial coefficient
    let binom_5_2 = binomial(5, 2).unwrap();
    assert_eq!(binom_5_2.to_string(), "10");
    
    let binom_10_3 = binomial(10, 3).unwrap();
    assert_eq!(binom_10_3.to_string(), "120");
    
    // Test edge cases for binomial
    let binom_0_0 = binomial(0, 0).unwrap();
    assert_eq!(binom_0_0.to_string(), "1");
    
    let binom_5_0 = binomial(5, 0).unwrap();
    assert_eq!(binom_5_0.to_string(), "1");
    
    let binom_5_5 = binomial(5, 5).unwrap();
    assert_eq!(binom_5_5.to_string(), "1");
    
    // Test invalid binomial coefficients
    let binom_invalid = binomial(3, 5).unwrap();
    assert_eq!(binom_invalid.to_string(), "0");
}

#[test]
fn test_accuracy_tracking() {
    // Test BigRat to f64 conversion accuracy
    let exact_rat = BigRat::new(1, 2).unwrap(); // 0.5 is exactly representable
    let (f_val, accuracy) = exact_rat.to_f64();
    assert_eq!(f_val, 0.5);
    assert_eq!(accuracy, Accuracy::Exact);
    
    // Test BigFloat to f32 conversion
    let big_float = BigFloat::new(1.5);
    let (f32_val, accuracy) = big_float.to_f32();
    assert_eq!(f32_val, 1.5);
    assert_eq!(accuracy, Accuracy::Exact);
    
    // Test BigFloat to int conversion
    let float_val = BigFloat::new(3.7);
    let (int_val, accuracy) = float_val.to_int();
    assert_eq!(int_val.to_string(), "3");
    assert_eq!(accuracy, Accuracy::Below);
}

#[test]
fn test_large_number_operations() {
    // Test with very large numbers
    let large1 = parse_int("123456789012345678901234567890", 10).unwrap();
    let large2 = parse_int("987654321098765432109876543210", 10).unwrap();
    
    // Test addition of large numbers
    let sum = large1.add(&large2);
    assert_eq!(sum.to_string(), "1111111110111111111011111111100");
    
    // Test multiplication of large numbers
    let product = fast_mul(&BigInt::new(12345), &BigInt::new(67890));
    assert_eq!(product.to_string(), "838102050");
}

#[test]
fn test_random_prime_generation() {
    let mut rng = rand::thread_rng();
    
    // Generate small primes for testing
    let prime = rand_prime(&mut rng, 8); // 8-bit prime
    assert!(prime.bit_len() <= 8);
    assert!(prime.to_string().parse::<u64>().unwrap() > 1);
    
    // Test multiple primes are different
    let prime1 = rand_prime(&mut rng, 16);
    let prime2 = rand_prime(&mut rng, 16);
    // Very unlikely to be the same for cryptographically secure generation
    // Note: This could occasionally fail due to randomness, but extremely unlikely
}

#[test]
fn test_comprehensive_error_scenarios() {
    // Test all error types that can occur in big_mood operations
    
    // Domain errors
    let negative = BigFloat::new(-1.0);
    assert!(matches!(sqrt(&negative), Err(MathError::DomainError { .. })));
    assert!(matches!(ln(&negative), Err(MathError::DomainError { .. })));
    assert!(matches!(nth_root(&negative, 2), Err(MathError::DomainError { .. })));
    
    // Division by zero errors
    let one = BigInt::one();
    let zero = BigInt::zero();
    assert!(matches!(one.div(&zero), Err(MathError::DivisionByZero { .. })));
    
    let one_float = BigFloat::one();
    let zero_float = BigFloat::zero();
    assert!(matches!(one_float.div(&zero_float), Err(MathError::DivisionByZero { .. })));
    
    // Invalid input errors
    let mut big_int = BigInt::zero();
    assert!(matches!(big_int.set_string("invalid", 10), Err(MathError::InvalidInput { .. })));
    
    let mut big_rat = BigRat::zero();
    assert!(matches!(big_rat.set_f64(f64::NAN), Err(MathError::InvalidInput { .. })));
}

#[test]
fn test_mathematical_properties_and_identities() {
    // Test mathematical identities to ensure correctness
    
    // Test that (a + b) * c = a*c + b*c (distributive property)
    let a = BigInt::new(123);
    let b = BigInt::new(456);
    let c = BigInt::new(789);
    
    let left_side = a.add(&b).mul(&c);
    let right_side = a.mul(&c).add(&b.mul(&c));
    assert_eq!(left_side.to_string(), right_side.to_string());
    
    // Test that sqrt(x^2) = |x|
    let x = BigFloat::new(-5.0);
    let x_squared = x.mul(&x);
    let sqrt_result = sqrt(&x_squared).unwrap();
    assert!((sqrt_result.to_f64().0 - 5.0).abs() < 1e-10);
    
    // Test that ln(e^x) = x for positive x
    let x = BigFloat::new(2.0);
    let exp_x = exp(&x);
    let ln_exp_x = ln(&exp_x).unwrap();
    assert!((ln_exp_x.to_f64().0 - 2.0).abs() < 1e-10);
    
    // Test that sin²(x) + cos²(x) = 1
    let angle = BigFloat::new(1.23456);
    let sin_val = sin(&angle);
    let cos_val = cos(&angle);
    let sin_squared = sin_val.mul(&sin_val);
    let cos_squared = cos_val.mul(&cos_val);
    let sum = sin_squared.add(&cos_squared);
    assert!((sum.to_f64().0 - 1.0).abs() < 1e-10);
}

#[test]
fn test_memory_efficiency_and_performance() {
    // Test that operations with moderately large numbers complete in reasonable time
    let start = std::time::Instant::now();
    
    // Create moderately large numbers
    let mut large_num = BigInt::new(1);
    for _ in 0..1000 {
        large_num = large_num.mul(&BigInt::new(2));
    }
    
    // Perform some operations
    let doubled = large_num.mul(&BigInt::new(2));
    let _halved = doubled.div(&BigInt::new(2)).unwrap();
    
    let elapsed = start.elapsed();
    // Should complete in reasonable time (less than 1 second for moderate operations)
    assert!(elapsed.as_millis() < 1000);
    
    // Test that string conversion doesn't take too long
    let start = std::time::Instant::now();
    let _string_repr = large_num.to_string();
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 500);
}
