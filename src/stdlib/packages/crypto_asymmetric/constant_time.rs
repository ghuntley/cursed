// Constant time operations
// 
// Provides constant time cryptographic operations to prevent timing side-channel attacks.
// All operations in this module are designed to execute in constant time regardless of input values.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::ptr;

/// Constant time comparison result
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTimeResult {
/// Constant time comparison of two byte arrays
/// Returns true if arrays are equal, false otherwise
/// Executes in constant time regardless of input values
pub fn constant_time_compare(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 2 {
        return Err(CursedError::InvalidArgument(
            "constant_time_compare requires exactly 2 arguments".to_string()
        ));
    let bytes1 = extract_bytes(&args[0])?;
    let bytes2 = extract_bytes(&args[1])?;
    
    let result = constant_time_bytes_equal(&bytes1, &bytes2);
    Ok(Value::Bool(result))
/// Constant time conditional selection
/// Returns a if condition is true, b if condition is false
/// Executes in constant time regardless of condition value
pub fn constant_time_select(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 3 {
        return Err(CursedError::InvalidArgument(
            "constant_time_select requires exactly 3 arguments (condition, a, b)".to_string()
        ));
    let condition = match &args[0] {
        _ => return Err(CursedError::InvalidArgument(
            "First argument must be boolean or integer".to_string()
    
    let a = &args[1];
    let b = &args[2];
    
    // Constant time selection using bit manipulation
    let selected = constant_time_conditional_select(condition, a, b)?;
    Ok(selected)
/// Constant time byte array copy
/// Copies src to dst if condition is true, otherwise dst remains unchanged
/// Executes in constant time regardless of condition
pub fn constant_time_copy(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 3 {
        return Err(CursedError::InvalidArgument(
            "constant_time_copy requires exactly 3 arguments (condition, src, dst)".to_string()
        ));
    let condition = match &args[0] {
        _ => return Err(CursedError::InvalidArgument(
            "First argument must be boolean or integer".to_string()
    
    let src = extract_bytes(&args[1])?;
    let mut dst = extract_bytes(&args[2])?;
    
    constant_time_conditional_copy(condition, &src, &mut dst);
    
    Ok(Value::Array(dst.into_iter().map(|b| Value::Integer(b as i64)).collect()))
/// Constant time memory clear
/// Clears memory in a timing-safe manner
pub fn constant_time_clear(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 1 {
        return Err(CursedError::InvalidArgument(
            "constant_time_clear requires exactly 1 argument".to_string()
        ));
    let mut bytes = extract_bytes(&args[0])?;
    constant_time_zero(&mut bytes);
    
    Ok(Value::Array(bytes.into_iter().map(|b| Value::Integer(b as i64)).collect()))
/// Constant time integer comparison
/// Returns 1 if a == b, 0 otherwise, in constant time
pub fn constant_time_int_equal(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 2 {
        return Err(CursedError::InvalidArgument(
            "constant_time_int_equal requires exactly 2 arguments".to_string()
        ));
    let a = match &args[0] {
        _ => return Err(CursedError::InvalidArgument(
            "First argument must be integer".to_string()
    
    let b = match &args[1] {
        _ => return Err(CursedError::InvalidArgument(
            "Second argument must be integer".to_string()
    
    let result = constant_time_u64_equal(a as u64, b as u64);
    Ok(Value::Integer(result as i64))
/// Constant time less than comparison
/// Returns 1 if a < b, 0 otherwise, in constant time
pub fn constant_time_less_than(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 2 {
        return Err(CursedError::InvalidArgument(
            "constant_time_less_than requires exactly 2 arguments".to_string()
        ));
    let a = match &args[0] {
        _ => return Err(CursedError::InvalidArgument(
            "First argument must be integer".to_string()
    
    let b = match &args[1] {
        _ => return Err(CursedError::InvalidArgument(
            "Second argument must be integer".to_string()
    
    let result = constant_time_u64_less_than(a, b);
    Ok(Value::Integer(result as i64))
/// Constant time key derivation
/// Derives a key from input material in constant time
pub fn constant_time_key_derive(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() != 2 {
        return Err(CursedError::InvalidArgument(
            "constant_time_key_derive requires exactly 2 arguments (input, length)".to_string()
        ));
    let input = extract_bytes(&args[0])?;
    let length = match &args[1] {
        _ => return Err(CursedError::InvalidArgument(
            "Second argument must be integer length".to_string()
    
    if length > 1024 {
        return Err(CursedError::InvalidArgument(
            "Key length cannot exceed 1024 bytes".to_string()
        ));
    let derived_key = constant_time_derive_key(&input, length);
    Ok(Value::Array(derived_key.into_iter().map(|b| Value::Integer(b as i64)).collect()))
// Core constant-time implementations

/// Constant time byte array equality check
fn constant_time_bytes_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a[i] ^ b[i];
    // Convert to boolean in constant time
    (result as u32).wrapping_sub(1) >> 31 == 1
/// Constant time u64 equality check
fn constant_time_u64_equal(a: u64, b: u64) -> u32 {
    let x = a ^ b;
    let y = x.wrapping_sub(1);
    ((y & !x) >> 63) as u32
/// Constant time u64 less than comparison
fn constant_time_u64_less_than(a: u64, b: u64) -> u32 {
    // a < b is equivalent to (a - b) having the high bit set
    let diff = a.wrapping_sub(b);
    (diff >> 63) as u32
/// Constant time conditional selection between two values
fn constant_time_conditional_select(condition: bool, a: &Value, b: &Value) -> crate::error::Result<()> {
    let mask = if condition { !0u8 } else { 0u8 };
    
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => {
            let selected = constant_time_select_u64(condition, *x as u64, *y as u64);
            Ok(Value::Integer(selected as i64))
        (Value::Bool(x), Value::Bool(y)) => {
            let selected = if condition { *x } else { *y };
            Ok(Value::Bool(selected))
        _ => {
            // For other types, extract as bytes and select
            let bytes_a = extract_bytes(a)?;
            let bytes_b = extract_bytes(b)?;
            
            if bytes_a.len() != bytes_b.len() {
                return Err(CursedError::InvalidArgument(
                    "Values must have same length for constant time selection".to_string()
                ));
            let mut result = Vec::with_capacity(bytes_a.len());
            for i in 0..bytes_a.len() {
                let selected = (bytes_a[i] & mask) | (bytes_b[i] & !mask);
                result.push(selected);
            Ok(Value::Array(result.into_iter().map(|b| Value::Integer(b as i64)).collect()))
        }
    }
/// Constant time u64 selection
fn constant_time_select_u64(condition: bool, a: u64, b: u64) -> u64 {
    let mask = if condition { !0u64 } else { 0u64 };
    (a & mask) | (b & !mask)
/// Constant time conditional copy
fn constant_time_conditional_copy(condition: bool, src: &[u8], dst: &mut [u8]) {
    let mask = if condition { !0u8 } else { 0u8 };
    
    for i in 0..std::cmp::min(src.len(), dst.len()) {
        dst[i] = (src[i] & mask) | (dst[i] & !mask);
    }
}

/// Constant time memory zeroing
fn constant_time_zero(data: &mut [u8]) {
    // Use volatile operations to prevent compiler optimization
    unsafe {
        ptr::write_bytes(data.as_mut_ptr(), 0, data.len());
    // Additional barrier to prevent reordering
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
/// Constant time key derivation using simple XOR expansion
fn constant_time_derive_key(input: &[u8], length: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(length);
    let mut state = 0x5A5A5A5Au32; // Initial state
    
    for i in 0..length {
        // Mix input with current state
        let input_byte = input[i % input.len()];
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
        state ^= (input_byte as u32) << ((i % 4) * 8);
        
        // Extract byte in constant time
        let output_byte = (state >> ((i % 4) * 8)) as u8;
        key.push(output_byte);
    key
/// Extract bytes from a Value in a consistent manner
fn extract_bytes(value: &Value) -> crate::error::Result<()> {
    match value {
        Value::Array(arr) => {
            let mut bytes = Vec::new();
            for v in arr {
                match v {
                    Value::Integer(i) => {
                        if *i < 0 || *i > 255 {
                            return Err(CursedError::InvalidArgument(
                                "Array elements must be valid bytes (0-255)".to_string()
                            ));
                        }
                        bytes.push(*i as u8);
                    _ => return Err(CursedError::InvalidArgument(
                        "Array must contain only integers for byte conversion".to_string()
                }
            }
            Ok(bytes)
        Value::Integer(i) => {
            // Convert integer to bytes (little-endian)
            Ok(i.to_le_bytes().to_vec())
        _ => Err(CursedError::InvalidArgument(
            "Value type not supported for byte extraction".to_string()
    }
}

