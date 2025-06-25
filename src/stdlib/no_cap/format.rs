/// Formatting functions for converting values to strings
use super::{Tea, Lit, Normie};

/// Convert a lit (boolean) to a tea (string)
/// 
/// Equivalent to Go's strconv.FormatBool with Gen Z slang
/// Returns "facts" for true, "cap" for false
pub fn YeetBool(b: Lit) -> Tea {
    if b {
        "facts".to_string()
    } else {
        "cap".to_string()
    }
}

/// Convert an integer to a tea (string)
/// 
/// Equivalent to Go's strconv.FormatInt
/// 
/// Parameters:
/// - i: Integer value to convert
/// - base: Number base (2-36)
pub fn YeetInt(i: i64, base: Normie) -> Tea {
    if base < 2 || base > 36 {
        return format!("invalid base: {}", base);
    if base == 10 {
        i.to_string()
    } else {
        format_int_with_base(i, base)
    }
}

/// Convert an unsigned integer to a tea (string)
/// 
/// Equivalent to Go's strconv.FormatUint
pub fn YeetUint(i: u64, base: Normie) -> Tea {
    if base < 2 || base > 36 {
        return format!("invalid base: {}", base);
    if base == 10 {
        i.to_string()
    } else {
        format_uint_with_base(i, base)
    }
}

/// Convert a floating-point number to a tea (string)
/// 
/// Equivalent to Go's strconv.FormatFloat
/// 
/// Parameters:
/// - f: Float value to convert
/// - fmt: Format type ('e', 'E', 'f', 'F', 'g', 'G')
/// - prec: Precision (-1 for default)
/// - bit_size: Bit size (32 or 64)
pub fn YeetFloat(f: f64, fmt: u8, prec: Normie, bit_size: Normie) -> Tea {
    // Handle special values first
    if f.is_nan() {
        return "NaN".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_positive() {
            "+Inf".to_string()
        } else {
            "-Inf".to_string()
    // Validate bit_size
    if ![32, 64].contains(&bit_size) {
        return format!("invalid bit size: {}", bit_size);
    // Convert to appropriate precision for 32-bit
    let value = if bit_size == 32 {
        f as f32 as f64
    } else {
        f
    
    match fmt as char {
    }
}

/// Specialized formatter for floating-point numbers with Gen Z slang
/// 
/// Returns:
/// - "sus" for NaN
/// - "bussin" for +Inf
/// - "busted" for -Inf
/// - Regular string representation otherwise
pub fn SussyFloat(f: f64) -> Tea {
    if f.is_nan() {
        "sus".to_string()
    } else if f.is_infinite() {
        if f.is_sign_positive() {
            "bussin".to_string()
        } else {
            "busted".to_string()
        }
    } else {
        // Use default formatting for normal values
        format!("{}", f)
    }
}

// Helper function to format integers with different bases
fn format_int_with_base(mut i: i64, base: Normie) -> Tea {
    if i == 0 {
        return "0".to_string();
    let is_negative = i < 0;
    if is_negative {
        i = -i;
    let mut result = String::new();
    let base = base as u64;
    let mut val = i as u64;
    
    while val > 0 {
        let digit = val % base;
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        result.push(char);
        val /= base;
    if is_negative {
        result.push('-');
    result.chars().rev().collect()
// Helper function to format unsigned integers with different bases
fn format_uint_with_base(mut i: u64, base: Normie) -> Tea {
    if i == 0 {
        return "0".to_string();
    let mut result = String::new();
    let base = base as u64;
    
    while i > 0 {
        let digit = i % base;
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        result.push(char);
        i /= base;
    result.chars().rev().collect()
// Helper function for scientific notation formatting
fn format_float_scientific(f: f64, prec: Normie, uppercase: bool) -> Tea {
    let precision = if prec < 0 { 6 } else { prec as usize };
    
    if uppercase {
        format!("{:.prec$E}", f, prec = precision)
    } else {
        format!("{:.prec$e}", f, prec = precision)
    }
}

// Helper function for fixed-point formatting
fn format_float_fixed(f: f64, prec: Normie) -> Tea {
    let precision = if prec < 0 { 6 } else { prec as usize };
    format!("{:.prec$}", f, prec = precision)
// Helper function for general formatting
fn format_float_general(f: f64, prec: Normie, uppercase: bool) -> Tea {
    let precision = if prec < 0 { 6 } else { prec as usize };
    
    // Choose between fixed and scientific notation based on magnitude
    let abs_f = f.abs();
    if abs_f != 0.0 && (abs_f < 1e-4 || abs_f >= 10f64.powi(precision as i32)) {
        format_float_scientific(f, prec, uppercase)
    } else {
        // Use fixed notation but remove trailing zeros
        let fixed = format_float_fixed(f, prec);
        if fixed.contains('.') {
            fixed.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            fixed
        }
    }
