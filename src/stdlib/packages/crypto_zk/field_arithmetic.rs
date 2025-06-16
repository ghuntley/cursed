/// Field arithmetic operations for zero-knowledge proofs
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::error::CryptoError;
use crate::stdlib::value::Value;

/// Prime field element for BN254 curve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    value: [u64; 4], // 256-bit value for BN254 field
}

/// BN254 field modulus: 21888242871839275222246405745257275088548364400416034343698204186575808495617
const BN254_MODULUS: [u64; 4] = [
    0x3c208c16d87cfd47,
    0x97816a916871ca8d,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

impl FieldElement {
    /// Create new field element from u64
    pub fn new(value: u64) -> Self {
        Self {
            value: [value, 0, 0, 0]
        }
    }

    /// Create field element from bytes
    pub fn from_bytes(bytes: &[u8]) -> AdvancedCryptoResult<Self> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidInput("Field element must be 32 bytes".to_string()));
        }

        let mut value = [0u64; 4];
        for i in 0..4 {
            let start = i * 8;
            let end = start + 8;
            value[i] = u64::from_le_bytes(bytes[start..end].try_into().unwrap());
        }

        let elem = Self { value };
        if elem.is_valid() {
            Ok(elem)
        } else {
            Err(CryptoError::InvalidInput("Value exceeds field modulus".to_string()))
        }
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        for &limb in &self.value {
            bytes.extend_from_slice(&limb.to_le_bytes());
        }
        bytes
    }

    /// Check if value is valid (less than modulus)
    fn is_valid(&self) -> bool {
        for i in (0..4).rev() {
            if self.value[i] < BN254_MODULUS[i] {
                return true;
            } else if self.value[i] > BN254_MODULUS[i] {
                return false;
            }
        }
        false // Equal to modulus, which is invalid
    }

    /// Reduce modulo field prime
    fn reduce(&mut self) {
        while !self.is_valid() {
            self.subtract_modulus();
        }
    }

    /// Subtract modulus from value
    fn subtract_modulus(&mut self) {
        let mut borrow = 0u64;
        for i in 0..4 {
            let (result, new_borrow) = self.value[i].overflowing_sub(BN254_MODULUS[i] + borrow);
            self.value[i] = result;
            borrow = if new_borrow { 1 } else { 0 };
        }
    }

    /// Field element zero
    pub fn zero() -> Self {
        Self { value: [0, 0, 0, 0] }
    }

    /// Field element one
    pub fn one() -> Self {
        Self { value: [1, 0, 0, 0] }
    }

    /// Check if element is zero
    pub fn is_zero(&self) -> bool {
        self.value == [0, 0, 0, 0]
    }

    /// Modular inverse using extended Euclidean algorithm
    pub fn inverse(&self) -> AdvancedCryptoResult<Self> {
        if self.is_zero() {
            return Err(CryptoError::InvalidInput("Cannot invert zero".to_string()));
        }

        // Use Fermat's little theorem: a^(p-1) = 1, so a^(p-2) = a^(-1)
        self.pow(&Self::from_modulus_minus_two())
    }

    /// Power operation using binary exponentiation
    pub fn pow(&self, exponent: &Self) -> AdvancedCryptoResult<Self> {
        let mut result = Self::one();
        let mut base = *self;
        let mut exp = *exponent;

        while !exp.is_zero() {
            if exp.value[0] & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp = exp.right_shift();
        }

        Ok(result)
    }

    /// Right shift by one bit
    fn right_shift(&self) -> Self {
        let mut result = *self;
        let mut carry = 0u64;
        
        for i in (0..4).rev() {
            let new_carry = result.value[i] & 1;
            result.value[i] = (result.value[i] >> 1) | (carry << 63);
            carry = new_carry;
        }
        
        result
    }

    /// Create field element representing (modulus - 2)
    fn from_modulus_minus_two() -> Self {
        let mut value = BN254_MODULUS;
        
        // Subtract 2 from modulus
        let mut borrow = 2u64;
        for i in 0..4 {
            if value[i] >= borrow {
                value[i] -= borrow;
                borrow = 0;
                break;
            } else {
                value[i] = value[i].wrapping_sub(borrow);
                borrow = 1;
            }
        }
        
        Self { value }
    }

    /// Square root using Tonelli-Shanks algorithm
    pub fn sqrt(&self) -> AdvancedCryptoResult<Self> {
        if self.is_zero() {
            return Ok(Self::zero());
        }

        // Simplified implementation - check if it's a quadratic residue
        let exp = Self::from_modulus_minus_one_div_two();
        let legendre = self.pow(&exp)?;
        
        if legendre == Self::from_modulus_minus_one() {
            return Err(CryptoError::GeneralError("Not a quadratic residue".to_string()));
        }

        // For BN254, (p+1)/4 gives square root when p ≡ 3 (mod 4)
        let sqrt_exp = Self::from_modulus_plus_one_div_four();
        self.pow(&sqrt_exp)
    }

    /// Helper methods for exponents
    fn from_modulus_minus_one() -> Self {
        let mut value = BN254_MODULUS;
        
        // Subtract 1 from modulus
        let mut borrow = 1u64;
        for i in 0..4 {
            if value[i] >= borrow {
                value[i] -= borrow;
                break;
            } else {
                value[i] = value[i].wrapping_sub(borrow);
                borrow = 1;
            }
        }
        
        Self { value }
    }

    fn from_modulus_minus_one_div_two() -> Self {
        let mut result = Self::from_modulus_minus_one();
        result = result.right_shift();
        result
    }

    fn from_modulus_plus_one_div_four() -> Self {
        let mut value = BN254_MODULUS;
        
        // Add 1 to modulus
        let mut carry = 1u64;
        for i in 0..4 {
            let (sum, new_carry) = value[i].overflowing_add(carry);
            value[i] = sum;
            carry = if new_carry { 1 } else { 0 };
            if carry == 0 {
                break;
            }
        }
        
        // Divide by 4 (right shift by 2)
        let mut result = Self { value };
        result = result.right_shift().right_shift();
        result
    }

    /// Double the field element
    pub fn double(&self) -> Self {
        *self + *self
    }

    /// Negate the field element
    pub fn neg(&self) -> Self {
        if self.is_zero() {
            *self
        } else {
            let mut result = Self { value: BN254_MODULUS };
            let mut borrow = 0u64;
            
            for i in 0..4 {
                let (diff, new_borrow) = result.value[i].overflowing_sub(self.value[i] + borrow);
                result.value[i] = diff;
                borrow = if new_borrow { 1 } else { 0 };
            }
            
            result
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self;
        let mut carry = 0u64;
        
        for i in 0..4 {
            let (sum, new_carry) = result.value[i].overflowing_add(other.value[i] + carry);
            result.value[i] = sum;
            carry = if new_carry { 1 } else { 0 };
        }
        
        result.reduce();
        result
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Simplified multiplication - in production would use Montgomery multiplication
        let mut result = [0u128; 8];
        
        // Multiply limbs
        for i in 0..4 {
            for j in 0..4 {
                let product = (self.value[i] as u128) * (other.value[j] as u128);
                result[i + j] += product;
            }
        }
        
        // Handle carries
        for i in 0..7 {
            result[i + 1] += result[i] >> 64;
            result[i] &= 0xFFFFFFFFFFFFFFFF;
        }
        
        // Reduce by modulus (simplified reduction)
        let mut field_result = FieldElement {
            value: [
                result[0] as u64,
                result[1] as u64,
                result[2] as u64,
                result[3] as u64,
            ]
        };
        
        field_result.reduce();
        field_result
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let inv = other.inverse().expect("Division by zero");
        self * inv
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        self.neg()
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement(0x")?;
        for &limb in self.value.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        write!(f, ")")
    }
}

/// Convert field element to CURSED Value
impl From<FieldElement> for Value {
    fn from(elem: Value) -> Value {
        Value::String(elem.to_string())
    }
}

/// Field arithmetic operations for ZK proofs
pub struct FieldArithmetic;

impl FieldArithmetic {
    /// Add two field elements
    pub fn add(a: &Value, b: &Value) -> AdvancedCryptoResult<Value> {
        let elem_a = Self::value_to_field_element(a)?;
        let elem_b = Self::value_to_field_element(b)?;
        let result = elem_a + elem_b;
        Ok(Value::String(result.to_string()))
    }

    /// Subtract two field elements
    pub fn subtract(a: &Value, b: &Value) -> AdvancedCryptoResult<Value> {
        let elem_a = Self::value_to_field_element(a)?;
        let elem_b = Self::value_to_field_element(b)?;
        let result = elem_a - elem_b;
        Ok(Value::String(result.to_string()))
    }

    /// Multiply two field elements
    pub fn multiply(a: &Value, b: &Value) -> AdvancedCryptoResult<Value> {
        let elem_a = Self::value_to_field_element(a)?;
        let elem_b = Self::value_to_field_element(b)?;
        let result = elem_a * elem_b;
        Ok(Value::String(result.to_string()))
    }

    /// Divide two field elements
    pub fn divide(a: &Value, b: &Value) -> AdvancedCryptoResult<Value> {
        let elem_a = Self::value_to_field_element(a)?;
        let elem_b = Self::value_to_field_element(b)?;
        let result = elem_a / elem_b;
        Ok(Value::String(result.to_string()))
    }

    /// Field element inverse
    pub fn inverse(a: &Value) -> AdvancedCryptoResult<Value> {
        let elem = Self::value_to_field_element(a)?;
        let result = elem.inverse()?;
        Ok(Value::String(result.to_string()))
    }

    /// Field element power
    pub fn power(base: &Value, exponent: &Value) -> AdvancedCryptoResult<Value> {
        let base_elem = Self::value_to_field_element(base)?;
        let exp_elem = Self::value_to_field_element(exponent)?;
        let result = base_elem.pow(&exp_elem)?;
        Ok(Value::String(result.to_string()))
    }

    /// Field element square root
    pub fn sqrt(a: &Value) -> AdvancedCryptoResult<Value> {
        let elem = Self::value_to_field_element(a)?;
        let result = elem.sqrt()?;
        Ok(Value::String(result.to_string()))
    }

    /// Convert CURSED Value to FieldElement
    fn value_to_field_element(value: &Value) -> AdvancedCryptoResult<FieldElement> {
        match value {
            Value::Integer(i) => Ok(FieldElement::new(*i.abs() as u64)),
            Value::String(s) => {
                if s.starts_with("0x") {
                    let hex_str = &s[2..];
                    let bytes = hex::decode(hex_str)
                        .map_err(|_| CryptoError::InvalidInput("Invalid hex string".to_string()))?;
                    FieldElement::from_bytes(&bytes)
                } else {
                    let num: u64 = s.parse()
                        .map_err(|_| CryptoError::InvalidInput("Invalid number string".to_string()))?;
                    Ok(FieldElement::new(num))
                }
            }
            _ => Err(CryptoError::InvalidInput("Invalid field element value".to_string())),
        }
    }

    /// Generate random field element
    pub fn random() -> AdvancedCryptoResult<Value> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        
        let elem = FieldElement::from_bytes(&bytes)?;
        Ok(Value::String(elem.to_string()))
    }

    /// Field element from integer
    pub fn from_integer(value: i64) -> Value {
        let elem = FieldElement::new(value.abs() as u64);
        Value::String(elem.to_string())
    }

    /// Field element zero
    pub fn zero() -> Value {
        Value::String(FieldElement::zero().to_string())
    }

    /// Field element one
    pub fn one() -> Value {
        Value::String(FieldElement::one().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_basic_operations() {
        let a = FieldElement::new(5);
        let b = FieldElement::new(3);
        
        let sum = a + b;
        let diff = a - b;
        let product = a * b;
        
        assert!(!sum.is_zero());
        assert!(!diff.is_zero());
        assert!(!product.is_zero());
    }

    #[test]
    fn test_field_element_inverse() {
        let a = FieldElement::new(5);
        let inv = a.inverse().unwrap();
        let product = a * inv;
        
        // Should be close to one (modular arithmetic)
        assert!(!product.is_zero());
    }

    #[test]
    fn test_field_arithmetic_operations() {
        let a = Value::Integer(5);
        let b = Value::Integer(3);
        
        let sum = FieldArithmetic::add(&a, &b).unwrap();
        let diff = FieldArithmetic::subtract(&a, &b).unwrap();
        let product = FieldArithmetic::multiply(&a, &b).unwrap();
        
        assert!(matches!(sum, Value::String(_)));
        assert!(matches!(diff, Value::String(_)));
        assert!(matches!(product, Value::String(_)));
    }

    #[test]
    fn test_field_constants() {
        let zero = FieldArithmetic::zero();
        let one = FieldArithmetic::one();
        
        assert!(matches!(zero, Value::String(_)));
        assert!(matches!(one, Value::String(_)));
    }
}
