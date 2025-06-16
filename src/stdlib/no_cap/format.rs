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
    }
    
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
    }
    
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
        };
    }
    
    // Validate bit_size
    if ![32, 64].contains(&bit_size) {
        return format!("invalid bit size: {}", bit_size);
    }
    
    // Convert to appropriate precision for 32-bit
    let value = if bit_size == 32 {
        f as f32 as f64
    } else {
        f
    };
    
    match fmt as char {
        'e' => format_float_scientific(value, prec, false),
        'E' => format_float_scientific(value, prec, true),
        'f' | 'F' => format_float_fixed(value, prec),
        'g' => format_float_general(value, prec, false),
        'G' => format_float_general(value, prec, true),
        _ => format!("invalid format: {}", fmt as char),
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
    }
    
    let is_negative = i < 0;
    if is_negative {
        i = -i;
    }
    
    let mut result = String::new();
    let base = base as u64;
    let mut val = i as u64;
    
    while val > 0 {
        let digit = val % base;
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        };
        result.push(char);
        val /= base;
    }
    
    if is_negative {
        result.push('-');
    }
    
    result.chars().rev().collect()
}

// Helper function to format unsigned integers with different bases
fn format_uint_with_base(mut i: u64, base: Normie) -> Tea {
    if i == 0 {
        return "0".to_string();
    }
    
    let mut result = String::new();
    let base = base as u64;
    
    while i > 0 {
        let digit = i % base;
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        };
        result.push(char);
        i /= base;
    }
    
    result.chars().rev().collect()
}

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
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yeet_bool() {
        assert_eq!(YeetBool(true), "facts");
        assert_eq!(YeetBool(false), "cap");
    }

    #[test]
    fn test_yeet_int_decimal() {
        assert_eq!(YeetInt(123, 10), "123");
        assert_eq!(YeetInt(-456, 10), "-456");
        assert_eq!(YeetInt(0, 10), "0");
    }

    #[test]
    fn test_yeet_int_different_bases() {
        // Test binary
        assert_eq!(YeetInt(10, 2), "1010");
        assert_eq!(YeetInt(-10, 2), "-1010");
        
        // Test hex
        assert_eq!(YeetInt(255, 16), "ff");
        assert_eq!(YeetInt(-255, 16), "-ff");
        
        // Test octal
        assert_eq!(YeetInt(64, 8), "100");
        
        // Test base 36
        assert_eq!(YeetInt(35, 36), "z");
    }

    #[test]
    fn test_yeet_int_invalid_base() {
        let result = YeetInt(123, 1);
        assert!(result.contains("invalid base"));
        
        let result = YeetInt(123, 37);
        assert!(result.contains("invalid base"));
    }

    #[test]
    fn test_yeet_uint() {
        assert_eq!(YeetUint(123, 10), "123");
        assert_eq!(YeetUint(0, 10), "0");
        
        // Test hex
        assert_eq!(YeetUint(255, 16), "ff");
        
        // Test binary
        assert_eq!(YeetUint(10, 2), "1010");
    }

    #[test]
    fn test_yeet_float_basic() {
        // Test fixed notation
        assert_eq!(YeetFloat(123.45, b'f', 2, 64), "123.45");
        assert_eq!(YeetFloat(123.456, b'f', 2, 64), "123.46"); // Rounding
        
        // Test scientific notation
        let result = YeetFloat(1234.5, b'e', 2, 64);
        assert!(result.starts_with("1.23e"));
        
        let result = YeetFloat(1234.5, b'E', 2, 64);
        assert!(result.starts_with("1.23E"));
    }

    #[test]
    fn test_yeet_float_special_values() {
        // Test NaN
        assert_eq!(YeetFloat(f64::NAN, b'f', 2, 64), "NaN");
        
        // Test infinity
        assert_eq!(YeetFloat(f64::INFINITY, b'f', 2, 64), "+Inf");
        assert_eq!(YeetFloat(f64::NEG_INFINITY, b'f', 2, 64), "-Inf");
    }

    #[test]
    fn test_yeet_float_general_format() {
        // Small numbers should use scientific notation
        let result = YeetFloat(0.0001, b'g', 6, 64);
        assert!(result.contains("e") || result == "0.0001");
        
        // Normal numbers should use fixed notation
        let result = YeetFloat(123.45, b'g', 6, 64);
        assert_eq!(result, "123.45");
    }

    #[test]
    fn test_yeet_float_invalid_format() {
        let result = YeetFloat(123.45, b'x', 2, 64);
        assert!(result.contains("invalid format"));
    }

    #[test]
    fn test_yeet_float_invalid_bit_size() {
        let result = YeetFloat(123.45, b'f', 2, 16);
        assert!(result.contains("invalid bit size"));
    }

    #[test]
    fn test_sussy_float() {
        // Test NaN
        assert_eq!(SussyFloat(f64::NAN), "sus");
        
        // Test positive infinity
        assert_eq!(SussyFloat(f64::INFINITY), "bussin");
        
        // Test negative infinity
        assert_eq!(SussyFloat(f64::NEG_INFINITY), "busted");
        
        // Test normal values
        assert_eq!(SussyFloat(123.45), "123.45");
        assert_eq!(SussyFloat(-67.89), "-67.89");
        assert_eq!(SussyFloat(0.0), "0");
    }

    #[test]
    fn test_sussy_float_edge_cases() {
        // Test zero values
        assert_eq!(SussyFloat(0.0), "0");
        assert_eq!(SussyFloat(-0.0), "-0");
        
        // Test very small and large values
        assert_eq!(SussyFloat(1e-10), "0.0000000001");
        assert_eq!(SussyFloat(1e10), "10000000000");
    }

    #[test]
    fn test_format_bases_edge_cases() {
        // Test base 2
        assert_eq!(YeetInt(1, 2), "1");
        assert_eq!(YeetUint(1, 2), "1");
        
        // Test base 36
        assert_eq!(YeetInt(35, 36), "z");
        assert_eq!(YeetUint(35, 36), "z");
        
        // Test large numbers
        assert_eq!(YeetInt(1000000, 10), "1000000");
        assert_eq!(YeetUint(1000000, 10), "1000000");
    }
}
