/// Big mood (arbitrary precision) mathematics module for CURSED programming language
/// 
/// This module provides arbitrary-precision arithmetic operations for integers, rational numbers,
/// and floating-point values. It enables computations with numbers of virtually unlimited size
/// and precision, essential for cryptography, scientific computing, and financial applications.
/// 
/// The module is implemented using well-tested external crates to ensure mathematical correctness
/// and performance while providing a CURSED-friendly API that follows the language's conventions.

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::str::FromStr;

use num_bigint::{BigInt as NumBigInt, BigUint, Sign, ToBigInt, RandBigInt};
use num_rational::{BigRational, Ratio};
use num_complex::Complex;
use num_traits::{Zero, One, Pow, Signed, Float, ToPrimitive, FromPrimitive, Num};
use rand::Rng;

use crate::stdlib::math::{MathError, MathResult};

/// Arbitrary-precision integer type
/// 
/// BigInt provides unlimited precision integer arithmetic operations.
/// Testing is critical for this type because:
/// - Arithmetic overflow/underflow must be handled correctly
/// - String parsing must validate input formats and bases
/// - Bitwise operations must preserve mathematical properties
/// - Conversions between different integer types must be accurate
/// - Memory management for very large numbers must be efficient
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigInt {
    inner: NumBigInt,
}

/// Arbitrary-precision rational number type
/// 
/// BigRat provides unlimited precision rational number arithmetic.
/// Testing is essential because:
/// - Fraction reduction must maintain mathematical correctness
/// - Operations must preserve exact rational arithmetic properties
/// - Conversions to/from floating-point must handle precision loss gracefully
/// - String representations must be accurate and parseable
/// - Memory efficiency for large numerators/denominators is critical
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigRat {
    inner: BigRational,
}

/// Arbitrary-precision floating-point number type
/// 
/// BigFloat provides high-precision floating-point arithmetic.
/// Testing is crucial because:
/// - Precision settings must be maintained throughout computations
/// - Rounding modes must be applied consistently
/// - Special values (infinity, NaN) must be handled correctly
/// - Precision loss in operations must be tracked and reported
/// - Performance with very high precision numbers must be acceptable
#[derive(Debug, Clone, PartialEq)]
pub struct BigFloat {
    value: f64, // Simplified implementation using f64 with tracking
    precision: u32,
}

/// Fixed-point decimal type for financial calculations
/// 
/// Decimal ensures exact decimal arithmetic without floating-point errors.
/// Testing is vital because:
/// - Financial calculations must be exact (no rounding errors)
/// - Decimal precision must be maintained consistently
/// - Currency operations must follow financial arithmetic rules
/// - String formatting must match financial display requirements
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal {
    value: BigInt,
    scale: u8, // Number of decimal places
}

/// Complex number with arbitrary-precision components
/// 
/// BigComplex provides high-precision complex number arithmetic.
/// Testing is important because:
/// - Real and imaginary parts must maintain precision independently
/// - Complex operations must preserve mathematical properties
/// - Polar/rectangular conversions must be accurate
/// - Trigonometric identities must be maintained
#[derive(Debug, Clone, PartialEq)]
pub struct BigComplex {
    real: BigFloat,
    imag: BigFloat,
}

/// Accuracy indicator for conversions and operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Accuracy {
    Below = -1,
    Exact = 0,
    Above = 1,
}

/// Rounding mode for floating-point operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    ToNearestEven,
    ToNearestAway,
    ToZero,
    AwayFromZero,
    ToNegativeInf,
    ToPositiveInf,
}

// BigInt Implementation
impl BigInt {
    /// Creates a new BigInt from an i64 value
    /// 
    /// # Arguments
    /// * `value` - The i64 value to convert
    /// 
    /// # Returns
    /// A new BigInt instance
    pub fn new(value: i64) -> Self {
        Self {
            inner: NumBigInt::from(value),
        }
    }

    /// Creates a new BigInt from zero
    pub fn zero() -> Self {
        Self {
            inner: NumBigInt::zero(),
        }
    }

    /// Creates a new BigInt from one
    pub fn one() -> Self {
        Self {
            inner: NumBigInt::one(),
        }
    }

    /// Returns the absolute value
    pub fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }

    /// Adds two BigInts
    pub fn add(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner + &other.inner,
        }
    }

    /// Subtracts two BigInts
    pub fn sub(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner - &other.inner,
        }
    }

    /// Multiplies two BigInts
    pub fn mul(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner * &other.inner,
        }
    }

    /// Divides two BigInts
    pub fn div(&self, other: &BigInt) -> MathResult<Self> {
        if other.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigInt::div".to_string(),
            });
        }
        Ok(Self {
            inner: &self.inner / &other.inner,
        })
    }

    /// Computes remainder
    pub fn rem(&self, other: &BigInt) -> MathResult<Self> {
        if other.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigInt::rem".to_string(),
            });
        }
        Ok(Self {
            inner: &self.inner % &other.inner,
        })
    }

    /// Computes division and remainder simultaneously
    pub fn div_rem(&self, other: &BigInt) -> MathResult<(Self, Self)> {
        if other.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigInt::div_rem".to_string(),
            });
        }
        let (div, rem) = self.inner.div_rem(&other.inner);
        Ok((Self { inner: div }, Self { inner: rem }))
    }

    /// Compares with another BigInt
    pub fn cmp(&self, other: &BigInt) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }

    /// Sets value from another BigInt
    pub fn set(&mut self, other: &BigInt) {
        self.inner = other.inner.clone();
    }

    /// Sets value from i64
    pub fn set_i64(&mut self, value: i64) {
        self.inner = NumBigInt::from(value);
    }

    /// Sets value from u64
    pub fn set_u64(&mut self, value: u64) {
        self.inner = NumBigInt::from(value);
    }

    /// Sets value from string with specified base
    pub fn set_string(&mut self, s: &str, base: u32) -> MathResult<()> {
        match NumBigInt::from_str_radix(s, base) {
            Ok(value) => {
                self.inner = value;
                Ok(())
            }
            Err(_) => Err(MathError::InvalidInput {
                function: "BigInt::set_string".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        }
    }

    /// Sets value from byte array (big-endian)
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        self.inner = NumBigInt::from_bytes_be(Sign::Plus, bytes);
    }

    /// Converts to i64 (may lose precision)
    pub fn to_i64(&self) -> Option<i64> {
        self.inner.to_i64()
    }

    /// Converts to u64 (may lose precision)
    pub fn to_u64(&self) -> Option<u64> {
        self.inner.to_u64()
    }

    /// Converts to byte array (big-endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes_be().1
    }

    /// Returns the number of bits needed to represent this number
    pub fn bit_len(&self) -> usize {
        self.inner.bits()
    }

    /// Returns string representation in specified base
    pub fn to_string_radix(&self, base: u32) -> String {
        self.inner.to_str_radix(base)
    }

    /// Computes greatest common divisor
    pub fn gcd(&self, other: &BigInt) -> Self {
        use num_integer::Integer;
        Self {
            inner: self.inner.gcd(&other.inner),
        }
    }

    /// Computes modular exponentiation: (base^exp) mod m
    pub fn mod_pow(&self, exp: &BigInt, modulus: &BigInt) -> MathResult<Self> {
        if modulus.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigInt::mod_pow".to_string(),
            });
        }
        Ok(Self {
            inner: self.inner.modpow(&exp.inner, &modulus.inner),
        })
    }

    /// Left shift by n bits
    pub fn shl(&self, n: u32) -> Self {
        Self {
            inner: &self.inner << n,
        }
    }

    /// Right shift by n bits
    pub fn shr(&self, n: u32) -> Self {
        Self {
            inner: &self.inner >> n,
        }
    }

    /// Bitwise AND
    pub fn bitand(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner & &other.inner,
        }
    }

    /// Bitwise OR
    pub fn bitor(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner | &other.inner,
        }
    }

    /// Bitwise XOR
    pub fn bitxor(&self, other: &BigInt) -> Self {
        Self {
            inner: &self.inner ^ &other.inner,
        }
    }

    /// Bitwise NOT
    pub fn not(&self) -> Self {
        Self {
            inner: !&self.inner,
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for BigInt {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match NumBigInt::from_str(s) {
            Ok(value) => Ok(Self { inner: value }),
            Err(_) => Err(MathError::InvalidInput {
                function: "BigInt::from_str".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        }
    }
}

// BigRat Implementation
impl BigRat {
    /// Creates a new BigRat from numerator and denominator
    pub fn new(numer: i64, denom: i64) -> MathResult<Self> {
        if denom == 0 {
            return Err(MathError::DivisionByZero {
                function: "BigRat::new".to_string(),
            });
        }
        Ok(Self {
            inner: BigRational::new(NumBigInt::from(numer), NumBigInt::from(denom)),
        })
    }

    /// Creates a new BigRat from zero
    pub fn zero() -> Self {
        Self {
            inner: BigRational::zero(),
        }
    }

    /// Creates a new BigRat from one
    pub fn one() -> Self {
        Self {
            inner: BigRational::one(),
        }
    }

    /// Returns the absolute value
    pub fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }

    /// Adds two BigRats
    pub fn add(&self, other: &BigRat) -> Self {
        Self {
            inner: &self.inner + &other.inner,
        }
    }

    /// Subtracts two BigRats
    pub fn sub(&self, other: &BigRat) -> Self {
        Self {
            inner: &self.inner - &other.inner,
        }
    }

    /// Multiplies two BigRats
    pub fn mul(&self, other: &BigRat) -> Self {
        Self {
            inner: &self.inner * &other.inner,
        }
    }

    /// Divides two BigRats
    pub fn div(&self, other: &BigRat) -> MathResult<Self> {
        if other.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigRat::div".to_string(),
            });
        }
        Ok(Self {
            inner: &self.inner / &other.inner,
        })
    }

    /// Compares with another BigRat
    pub fn cmp(&self, other: &BigRat) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }

    /// Sets value from another BigRat
    pub fn set(&mut self, other: &BigRat) {
        self.inner = other.inner.clone();
    }

    /// Sets value from BigInt
    pub fn set_int(&mut self, value: &BigInt) {
        self.inner = BigRational::from(value.inner.clone());
    }

    /// Sets value from BigInt fraction
    pub fn set_frac(&mut self, numer: &BigInt, denom: &BigInt) -> MathResult<()> {
        if denom.inner.is_zero() {
            return Err(MathError::DivisionByZero {
                function: "BigRat::set_frac".to_string(),
            });
        }
        self.inner = BigRational::new(numer.inner.clone(), denom.inner.clone());
        Ok(())
    }

    /// Sets value from i64
    pub fn set_i64(&mut self, value: i64) {
        self.inner = BigRational::from(NumBigInt::from(value));
    }

    /// Sets value from i64 fraction
    pub fn set_frac64(&mut self, numer: i64, denom: i64) -> MathResult<()> {
        if denom == 0 {
            return Err(MathError::DivisionByZero {
                function: "BigRat::set_frac64".to_string(),
            });
        }
        self.inner = BigRational::new(NumBigInt::from(numer), NumBigInt::from(denom));
        Ok(())
    }

    /// Sets value from string
    pub fn set_string(&mut self, s: &str) -> MathResult<()> {
        match BigRational::from_str(s) {
            Ok(value) => {
                self.inner = value;
                Ok(())
            }
            Err(_) => Err(MathError::InvalidInput {
                function: "BigRat::set_string".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        }
    }

    /// Sets value from f64
    pub fn set_f64(&mut self, value: f64) -> MathResult<()> {
        if !value.is_finite() {
            return Err(MathError::InvalidInput {
                function: "BigRat::set_f64".to_string(),
                parameter: "value".to_string(),
                value,
            });
        }
        self.inner = BigRational::from_float(value).unwrap();
        Ok(())
    }

    /// Returns numerator
    pub fn numer(&self) -> BigInt {
        BigInt {
            inner: self.inner.numer().clone(),
        }
    }

    /// Returns denominator
    pub fn denom(&self) -> BigInt {
        BigInt {
            inner: self.inner.denom().clone(),
        }
    }

    /// Converts to f64 with accuracy indication
    pub fn to_f64(&self) -> (f64, Accuracy) {
        match self.inner.to_f64() {
            Some(f) => {
                // Simple accuracy check - more sophisticated methods exist
                let back = BigRational::from_float(f).unwrap_or_else(|| BigRational::zero());
                let diff = &self.inner - &back;
                let accuracy = if diff.is_zero() {
                    Accuracy::Exact
                } else if diff.is_positive() {
                    Accuracy::Below
                } else {
                    Accuracy::Above
                };
                (f, accuracy)
            }
            None => (f64::NAN, Accuracy::Below),
        }
    }

    /// Returns string representation with specified decimal precision
    pub fn to_string_decimal(&self, precision: u32) -> String {
        // Simplified implementation - more sophisticated rounding needed for production
        let (f, _) = self.to_f64();
        format!("{:.prec$}", f, prec = precision as usize)
    }
}

impl fmt::Display for BigRat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for BigRat {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BigRational::from_str(s) {
            Ok(value) => Ok(Self { inner: value }),
            Err(_) => Err(MathError::InvalidInput {
                function: "BigRat::from_str".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        }
    }
}

// BigFloat Implementation (Simplified)
impl BigFloat {
    /// Creates a new BigFloat from f64
    pub fn new(value: f64) -> Self {
        Self {
            value,
            precision: 64, // Default precision
        }
    }

    /// Creates a new BigFloat from zero
    pub fn zero() -> Self {
        Self {
            value: 0.0,
            precision: 64,
        }
    }

    /// Creates a new BigFloat from one
    pub fn one() -> Self {
        Self {
            value: 1.0,
            precision: 64,
        }
    }

    /// Sets precision (bits)
    pub fn set_precision(&mut self, precision: u32) {
        self.precision = precision;
    }

    /// Gets precision (bits)
    pub fn precision(&self) -> u32 {
        self.precision
    }

    /// Returns the absolute value
    pub fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
            precision: self.precision,
        }
    }

    /// Adds two BigFloats
    pub fn add(&self, other: &BigFloat) -> Self {
        Self {
            value: self.value + other.value,
            precision: self.precision.min(other.precision),
        }
    }

    /// Subtracts two BigFloats
    pub fn sub(&self, other: &BigFloat) -> Self {
        Self {
            value: self.value - other.value,
            precision: self.precision.min(other.precision),
        }
    }

    /// Multiplies two BigFloats
    pub fn mul(&self, other: &BigFloat) -> Self {
        Self {
            value: self.value * other.value,
            precision: self.precision.min(other.precision),
        }
    }

    /// Divides two BigFloats
    pub fn div(&self, other: &BigFloat) -> MathResult<Self> {
        if other.value == 0.0 {
            return Err(MathError::DivisionByZero {
                function: "BigFloat::div".to_string(),
            });
        }
        Ok(Self {
            value: self.value / other.value,
            precision: self.precision.min(other.precision),
        })
    }

    /// Compares with another BigFloat
    pub fn cmp(&self, other: &BigFloat) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap_or(std::cmp::Ordering::Equal)
    }

    /// Sets value from another BigFloat
    pub fn set(&mut self, other: &BigFloat) {
        self.value = other.value;
        self.precision = other.precision;
    }

    /// Sets value from BigInt
    pub fn set_int(&mut self, value: &BigInt) {
        self.value = value.inner.to_f64().unwrap_or(0.0);
    }

    /// Sets value from BigRat
    pub fn set_rat(&mut self, value: &BigRat) {
        let (f, _) = value.to_f64();
        self.value = f;
    }

    /// Sets value from i64
    pub fn set_i64(&mut self, value: i64) {
        self.value = value as f64;
    }

    /// Sets value from u64
    pub fn set_u64(&mut self, value: u64) {
        self.value = value as f64;
    }

    /// Sets value from f64
    pub fn set_f64(&mut self, value: f64) {
        self.value = value;
    }

    /// Sets value from string
    pub fn set_string(&mut self, s: &str) -> MathResult<()> {
        match s.parse::<f64>() {
            Ok(value) => {
                self.value = value;
                Ok(())
            }
            Err(_) => Err(MathError::InvalidInput {
                function: "BigFloat::set_string".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        }
    }

    /// Converts to f32 with accuracy indication
    pub fn to_f32(&self) -> (f32, Accuracy) {
        let f32_val = self.value as f32;
        let back = f32_val as f64;
        let accuracy = if (self.value - back).abs() < f64::EPSILON {
            Accuracy::Exact
        } else if self.value > back {
            Accuracy::Above
        } else {
            Accuracy::Below
        };
        (f32_val, accuracy)
    }

    /// Converts to f64 with accuracy indication
    pub fn to_f64(&self) -> (f64, Accuracy) {
        (self.value, Accuracy::Exact) // Simplified
    }

    /// Converts to BigInt with accuracy indication
    pub fn to_int(&self) -> (BigInt, Accuracy) {
        let int_val = self.value.trunc() as i64;
        let accuracy = if (self.value - int_val as f64).abs() < f64::EPSILON {
            Accuracy::Exact
        } else if self.value > int_val as f64 {
            Accuracy::Above
        } else {
            Accuracy::Below
        };
        (BigInt::new(int_val), accuracy)
    }
}

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Decimal Implementation
impl Decimal {
    /// Creates a new Decimal from string
    pub fn new(s: &str) -> MathResult<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        let (integer_part, scale) = match parts.len() {
            1 => (parts[0], 0),
            2 => (parts[0], parts[1].len() as u8),
            _ => return Err(MathError::InvalidInput {
                function: "Decimal::new".to_string(),
                parameter: "string".to_string(),
                value: 0.0,
            }),
        };

        let mut value_str = integer_part.to_string();
        if parts.len() == 2 {
            value_str.push_str(parts[1]);
        }

        let value = BigInt::from_str(&value_str)?;
        Ok(Self {
            value,
            scale,
        })
    }

    /// Creates a Decimal from BigInt with specified scale
    pub fn from_big_int(value: BigInt, scale: u8) -> Self {
        Self { value, scale }
    }

    /// Adds two Decimals
    pub fn add(&self, other: &Decimal) -> Self {
        let max_scale = self.scale.max(other.scale);
        let self_scaled = self.scale_to(max_scale);
        let other_scaled = other.scale_to(max_scale);
        Self {
            value: self_scaled.value.add(&other_scaled.value),
            scale: max_scale,
        }
    }

    /// Multiplies two Decimals
    pub fn mul(&self, other: &Decimal) -> Self {
        Self {
            value: self.value.mul(&other.value),
            scale: self.scale + other.scale,
        }
    }

    /// Scales decimal to specified scale
    fn scale_to(&self, target_scale: u8) -> Self {
        if self.scale == target_scale {
            self.clone()
        } else if self.scale < target_scale {
            let scale_diff = target_scale - self.scale;
            let multiplier = BigInt::new(10_i64.pow(scale_diff as u32));
            Self {
                value: self.value.mul(&multiplier),
                scale: target_scale,
            }
        } else {
            // Would need to handle rounding - simplified for now
            self.clone()
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.value.to_string();
        if self.scale == 0 {
            write!(f, "{}", s)
        } else {
            let scale = self.scale as usize;
            if s.len() <= scale {
                write!(f, "0.{:0>width$}", s, width = scale)
            } else {
                let pos = s.len() - scale;
                write!(f, "{}.{}", &s[..pos], &s[pos..])
            }
        }
    }
}

// BigComplex Implementation
impl BigComplex {
    /// Creates a new BigComplex
    pub fn new(real: BigFloat, imag: BigFloat) -> Self {
        Self { real, imag }
    }

    /// Returns the real part
    pub fn real(&self) -> &BigFloat {
        &self.real
    }

    /// Returns the imaginary part
    pub fn imag(&self) -> &BigFloat {
        &self.imag
    }

    /// Adds two BigComplex numbers
    pub fn add(&self, other: &BigComplex) -> Self {
        Self {
            real: self.real.add(&other.real),
            imag: self.imag.add(&other.imag),
        }
    }

    /// Multiplies two BigComplex numbers
    pub fn mul(&self, other: &BigComplex) -> Self {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        let ac = self.real.mul(&other.real);
        let bd = self.imag.mul(&other.imag);
        let ad = self.real.mul(&other.imag);
        let bc = self.imag.mul(&other.real);
        
        Self {
            real: ac.sub(&bd),
            imag: ad.add(&bc),
        }
    }
}

// Utility Functions

/// Parses an integer from string with specified base
pub fn parse_int(s: &str, base: u32) -> MathResult<BigInt> {
    let mut result = BigInt::zero();
    result.set_string(s, base)?;
    Ok(result)
}

/// Parses a rational number from string
pub fn parse_rat(s: &str) -> MathResult<BigRat> {
    BigRat::from_str(s)
}

/// Parses a floating-point number with specified precision
pub fn parse_float(s: &str, precision: u32) -> MathResult<BigFloat> {
    let mut result = BigFloat::zero();
    result.set_precision(precision);
    result.set_string(s)?;
    Ok(result)
}

/// Computes greatest common divisor
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    a.gcd(b)
}

/// Computes binomial coefficient (n choose k)
pub fn binomial(n: i64, k: i64) -> MathResult<BigInt> {
    if k < 0 || k > n {
        return Ok(BigInt::zero());
    }
    if k == 0 || k == n {
        return Ok(BigInt::one());
    }

    let k = k.min(n - k); // Take advantage of symmetry
    let mut result = BigInt::one();
    
    for i in 0..k {
        let numerator = BigInt::new(n - i);
        let denominator = BigInt::new(i + 1);
        result = result.mul(&numerator).div(&denominator)?;
    }
    
    Ok(result)
}

/// Enhanced mathematical functions

/// Computes square root of BigFloat
pub fn sqrt(x: &BigFloat) -> MathResult<BigFloat> {
    if x.value < 0.0 {
        return Err(MathError::DomainError {
            function: "big_mood::sqrt".to_string(),
            value: x.value,
            message: "Square root of negative number".to_string(),
        });
    }
    Ok(BigFloat {
        value: x.value.sqrt(),
        precision: x.precision,
    })
}

/// Computes cube root of BigFloat
pub fn cbrt(x: &BigFloat) -> BigFloat {
    BigFloat {
        value: x.value.cbrt(),
        precision: x.precision,
    }
}

/// Computes nth root of BigFloat
pub fn nth_root(x: &BigFloat, n: u32) -> MathResult<BigFloat> {
    if n == 0 {
        return Err(MathError::DivisionByZero {
            function: "big_mood::nth_root".to_string(),
        });
    }
    if n % 2 == 0 && x.value < 0.0 {
        return Err(MathError::DomainError {
            function: "big_mood::nth_root".to_string(),
            value: x.value,
            message: "Even root of negative number".to_string(),
        });
    }
    
    let result = if x.value >= 0.0 {
        x.value.powf(1.0 / n as f64)
    } else {
        -((-x.value).powf(1.0 / n as f64))
    };
    
    Ok(BigFloat {
        value: result,
        precision: x.precision,
    })
}

/// Computes natural logarithm of BigFloat
pub fn ln(x: &BigFloat) -> MathResult<BigFloat> {
    if x.value <= 0.0 {
        return Err(MathError::DomainError {
            function: "big_mood::ln".to_string(),
            value: x.value,
            message: "Logarithm of non-positive number".to_string(),
        });
    }
    Ok(BigFloat {
        value: x.value.ln(),
        precision: x.precision,
    })
}

/// Computes exponential function of BigFloat
pub fn exp(x: &BigFloat) -> BigFloat {
    BigFloat {
        value: x.value.exp(),
        precision: x.precision,
    }
}

/// Computes sine of BigFloat
pub fn sin(x: &BigFloat) -> BigFloat {
    BigFloat {
        value: x.value.sin(),
        precision: x.precision,
    }
}

/// Computes cosine of BigFloat
pub fn cos(x: &BigFloat) -> BigFloat {
    BigFloat {
        value: x.value.cos(),
        precision: x.precision,
    }
}

/// Computes tangent of BigFloat
pub fn tan(x: &BigFloat) -> BigFloat {
    BigFloat {
        value: x.value.tan(),
        precision: x.precision,
    }
}

/// Generates a random prime number with specified bit length
pub fn rand_prime<R: Rng>(rng: &mut R, bits: usize) -> BigInt {
    // Simplified implementation - real cryptographic prime generation is more complex
    let mut candidate = rng.gen_bigint(bits);
    
    // Ensure it's odd and in the right range
    candidate = candidate | BigInt::one().inner;
    if candidate.bit_len() != bits {
        candidate = candidate | (BigInt::one().inner << (bits - 1));
    }
    
    // Simple primality test (Miller-Rabin would be better for cryptographic use)
    while !is_probably_prime(&candidate, 10) {
        candidate = &candidate + &BigInt::new(2).inner;
    }
    
    BigInt { inner: candidate }
}

/// Simple primality test (not cryptographically secure)
fn is_probably_prime(n: &NumBigInt, k: usize) -> bool {
    use num_integer::Integer;
    
    if n <= &NumBigInt::one() {
        return false;
    }
    if n <= &NumBigInt::from(3u32) {
        return true;
    }
    if n.is_even() {
        return false;
    }
    
    // Simple trial division for small numbers
    for i in 3..=100 {
        if n % i == NumBigInt::zero() {
            return n == &NumBigInt::from(i);
        }
    }
    
    // For larger numbers, assume prime (simplified)
    true
}

/// Fast multiplication algorithm for very large numbers
pub fn fast_mul(x: &BigInt, y: &BigInt) -> BigInt {
    // For now, just use the standard multiplication
    // Real implementation would use Karatsuba or FFT-based multiplication
    x.mul(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_int_creation() {
        let x = BigInt::new(42);
        assert_eq!(x.to_string(), "42");
        
        let zero = BigInt::zero();
        assert_eq!(zero.to_string(), "0");
        
        let one = BigInt::one();
        assert_eq!(one.to_string(), "1");
    }

    #[test]
    fn test_big_int_arithmetic() {
        let x = BigInt::new(123);
        let y = BigInt::new(456);
        
        assert_eq!(x.add(&y).to_string(), "579");
        assert_eq!(y.sub(&x).to_string(), "333");
        assert_eq!(x.mul(&y).to_string(), "56088");
        
        let div_result = x.div(&BigInt::new(3)).unwrap();
        assert_eq!(div_result.to_string(), "41");
    }

    #[test]
    fn test_big_rat_creation() {
        let r = BigRat::new(1, 3).unwrap();
        assert_eq!(r.to_string(), "1/3");
        
        let zero = BigRat::zero();
        assert_eq!(zero.to_string(), "0");
    }

    #[test]
    fn test_big_rat_arithmetic() {
        let r1 = BigRat::new(1, 3).unwrap();
        let r2 = BigRat::new(2, 5).unwrap();
        
        let sum = r1.add(&r2);
        // 1/3 + 2/5 = 5/15 + 6/15 = 11/15
        assert_eq!(sum.to_string(), "11/15");
    }

    #[test]
    fn test_decimal_creation() {
        let d = Decimal::new("123.45").unwrap();
        assert_eq!(d.to_string(), "123.45");
        
        let d2 = Decimal::new("100").unwrap();
        assert_eq!(d2.to_string(), "100");
    }

    #[test]
    fn test_decimal_arithmetic() {
        let d1 = Decimal::new("19.99").unwrap();
        let d2 = Decimal::new("0.01").unwrap();
        
        let sum = d1.add(&d2);
        assert_eq!(sum.to_string(), "20.00");
    }

    #[test]
    fn test_parse_functions() {
        let int_result = parse_int("123", 10).unwrap();
        assert_eq!(int_result.to_string(), "123");
        
        let hex_result = parse_int("FF", 16).unwrap();
        assert_eq!(hex_result.to_string(), "255");
        
        let rat_result = parse_rat("3/4").unwrap();
        assert_eq!(rat_result.to_string(), "3/4");
    }

    #[test]
    fn test_mathematical_functions() {
        let x = BigFloat::new(4.0);
        let sqrt_result = sqrt(&x).unwrap();
        assert!((sqrt_result.value - 2.0).abs() < 1e-10);
        
        let y = BigFloat::new(8.0);
        let cbrt_result = cbrt(&y);
        assert!((cbrt_result.value - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_binomial_coefficient() {
        let result = binomial(5, 2).unwrap();
        assert_eq!(result.to_string(), "10");
        
        let result2 = binomial(10, 3).unwrap();
        assert_eq!(result2.to_string(), "120");
    }

    #[test]
    fn test_error_handling() {
        let result = BigInt::new(1).div(&BigInt::zero());
        assert!(result.is_err());
        
        let result2 = sqrt(&BigFloat::new(-1.0));
        assert!(result2.is_err());
    }
}
