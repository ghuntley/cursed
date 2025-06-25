/// Parsing functions for converting strings to values
use super::error::{NoCapResult, syntax_error, range_error};
use super::{Tea, Lit, Normie};

/// Parse a tea (string) as a lit (boolean) value
/// 
/// Equivalent to Go's strconv.ParseBool with Gen Z slang support.
/// 
/// Values for true: "1", "t", "T", "based", "TRUE", "True", "facts", "FACTS", "Facts", "no cap", "fr fr"
/// Values for false: "0", "f", "F", "false", "FALSE", "False", "cap", "CAP", "idk"
pub fn FactsCheck(s: Tea) -> NoCapResult<(Lit, Tea)> {
    let trimmed = s.trim();
    
    // Check for true values
    match trimmed {
        "1" | "t" | "T" | "based" | "TRUE" | "True" | "facts" | "FACTS" | "Facts" | "no cap" | "fr fr" => {
            Ok((true, String::new()))
        }
        "0" | "f" | "F" | "false" | "FALSE" | "False" | "cap" | "CAP" | "idk" => {
            Ok((false, String::new()))
        }
        _ => Err(syntax_error(&format!("invalid boolean value: '{}'", trimmed)))
    }
}

/// Parse a tea (string) as an integer
/// 
/// Equivalent to Go's strconv.ParseInt
/// 
/// Parameters:
/// - s: String to parse
/// - base: Number base (2-36, or 0 for auto-detection)
/// - bit_size: Bit size for range checking (8, 16, 32, 64)
pub fn YoinkInt(s: Tea, base: Normie, bit_size: Normie) -> NoCapResult<(i64, Tea)> {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return Err(syntax_error("empty string"));
    }
    
    // Validate base
    if base != 0 && (base < 2 || base > 36) {
        return Err(syntax_error(&format!("invalid base: {}", base)));
    }
    
    // Validate bit_size
    if ![8, 16, 32, 64].contains(&bit_size) {
        return Err(syntax_error(&format!("invalid bit size: {}", bit_size)));
    }
    
    // Handle sign
    let (is_negative, number_part) = if trimmed.starts_with('-') {
        (true, &trimmed[1..])
    } else if trimmed.starts_with('+') {
        (false, &trimmed[1..])
    } else {
        (false, trimmed)
    };
    
    if number_part.is_empty() {
        return Err(syntax_error("invalid number format"));
    }
    
    // Determine actual base
    let actual_base = if base == 0 {
        if number_part.starts_with("0x") || number_part.starts_with("0X") {
            16
        } else if number_part.starts_with("0b") || number_part.starts_with("0B") {
            2
        } else if number_part.starts_with('0') && number_part.len() > 1 {
            8
        } else {
            10
        }
    } else {
        base
    };
    
    // Remove prefix for hex and binary
    let clean_number = if actual_base == 16 && (number_part.starts_with("0x") || number_part.starts_with("0X")) {
        &number_part[2..]
    } else if actual_base == 2 && (number_part.starts_with("0b") || number_part.starts_with("0B")) {
        &number_part[2..]
    } else {
        number_part
    };
    
    if clean_number.is_empty() {
        return Err(syntax_error("invalid number format"));
    }
    
    // Parse the number
    let result = i64::from_str_radix(clean_number, actual_base as u32)
        .map_err(|_| syntax_error(&format!("invalid number format: '{}'", trimmed)))?;
    
    let final_result = if is_negative { -result } else { result };
    
    // Check range based on bit_size
    let (min_val, max_val) = match bit_size {
        8 => (i8::MIN as i64, i8::MAX as i64),
        16 => (i16::MIN as i64, i16::MAX as i64),
        32 => (i32::MIN as i64, i32::MAX as i64),
        64 => (i64::MIN, i64::MAX),
        _ => unreachable!(),
    };
    
    if final_result < min_val || final_result > max_val {
        return Err(range_error(&format!("value {} out of range for {}-bit signed integer", final_result, bit_size)));
    }
    
    Ok((final_result, String::new()))
}

/// Parse a tea (string) as an unsigned integer
/// 
/// Equivalent to Go's strconv.ParseUint
pub fn YoinkUint(s: Tea, base: Normie, bit_size: Normie) -> NoCapResult<(u64, Tea)> {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return Err(syntax_error("empty string"));
    }
    
    // No negative values allowed for unsigned
    if trimmed.starts_with('-') {
        return Err(syntax_error("negative value not allowed for unsigned integer"));
    }
    
    // Remove optional + sign
    let number_part = if trimmed.starts_with('+') {
        &trimmed[1..]
    } else {
        trimmed
    };
    
    if number_part.is_empty() {
        return Err(syntax_error("invalid number format"));
    }
    
    // Validate base
    if base != 0 && (base < 2 || base > 36) {
        return Err(syntax_error(&format!("invalid base: {}", base)));
    }
    
    // Validate bit_size
    if ![8, 16, 32, 64].contains(&bit_size) {
        return Err(syntax_error(&format!("invalid bit size: {}", bit_size)));
    }
    
    // Determine actual base
    let actual_base = if base == 0 {
        if number_part.starts_with("0x") || number_part.starts_with("0X") {
            16
        } else if number_part.starts_with("0b") || number_part.starts_with("0B") {
            2
        } else if number_part.starts_with('0') && number_part.len() > 1 {
            8
        } else {
            10
        }
    } else {
        base
    };
    
    // Remove prefix for hex and binary
    let clean_number = if actual_base == 16 && (number_part.starts_with("0x") || number_part.starts_with("0X")) {
        &number_part[2..]
    } else if actual_base == 2 && (number_part.starts_with("0b") || number_part.starts_with("0B")) {
        &number_part[2..]
    } else {
        number_part
    };
    
    if clean_number.is_empty() {
        return Err(syntax_error("invalid number format"));
    }
    
    // Parse the number
    let result = u64::from_str_radix(clean_number, actual_base as u32)
        .map_err(|_| syntax_error(&format!("invalid number format: '{}'", trimmed)))?;
    
    // Check range based on bit_size
    let max_val = match bit_size {
        8 => u8::MAX as u64,
        16 => u16::MAX as u64,
        32 => u32::MAX as u64,
        64 => u64::MAX,
        _ => unreachable!(),
    };
    
    if result > max_val {
        return Err(range_error(&format!("value {} out of range for {}-bit unsigned integer", result, bit_size)));
    }
    
    Ok((result, String::new()))
}

/// Parse a tea (string) as a floating-point number
/// 
/// Equivalent to Go's strconv.ParseFloat
pub fn YoinkFloat(s: Tea, bit_size: Normie) -> NoCapResult<(f64, Tea)> {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return Err(syntax_error("empty string"));
    }
    
    // Validate bit_size
    if ![32, 64].contains(&bit_size) {
        return Err(syntax_error(&format!("invalid bit size: {}", bit_size)));
    }
    
    // Handle special values with Gen Z slang
    match trimmed.to_lowercase().as_str() {
        "nan" | "sus" => return Ok((f64::NAN, String::new())),
        "inf" | "+inf" | "infinity" | "+infinity" | "bussin" => return Ok((f64::INFINITY, String::new())),
        "-inf" | "-infinity" | "busted" => return Ok((f64::NEG_INFINITY, String::new())),
        _ => {}
    }
    
    // Parse the float
    let result: f64 = trimmed.parse()
        .map_err(|_| syntax_error(&format!("invalid float format: '{}'", trimmed)))?;
    
    // Check range for 32-bit floats
    if bit_size == 32 {
        if result.is_finite() && (result.abs() > f32::MAX as f64) {
            return Err(range_error(&format!("value {} out of range for 32-bit float", result)));
        }
        
        // Convert to f32 and back to handle precision correctly
        let f32_result = result as f32;
        Ok((f32_result as f64, String::new()))
    } else {
        Ok((result, String::new()))
    }
}

