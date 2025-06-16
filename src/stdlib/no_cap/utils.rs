/// Utility functions for common conversions
use super::error::{NoCapResult, syntax_error};
use super::{Tea, Normie};
use super::parse::YoinkInt;
use super::format::YeetInt;

/// Convert a tea (string) to an int (equivalent to strconv.Atoi)
/// 
/// This is a convenience function that parses a decimal string to a 32-bit integer.
/// It's equivalent to YoinkInt(s, 10, 32) but returns just the int value.
pub fn Atoi(s: Tea) -> NoCapResult<(Normie, Tea)> {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return Err(syntax_error("empty string"));
    }
    
    // Use YoinkInt with base 10 and 32-bit size
    match YoinkInt(trimmed.to_string(), 10, 32) {
        Ok((value, err_str)) => {
            // Convert i64 to i32, checking range
            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                Ok((value as i32, err_str))
            } else {
                Err(syntax_error(&format!("value {} out of range for 32-bit integer", value)))
            }
        }
        Err(e) => Err(e),
    }
}

/// Convert an int to a tea (string) (equivalent to strconv.Itoa)
/// 
/// This is a convenience function that formats a 32-bit integer as a decimal string.
/// It's equivalent to YeetInt(i as i64, 10).
pub fn Itoa(i: Normie) -> Tea {
    YeetInt(i as i64, 10)
}

/// Enhanced convenience function for parsing integers with automatic base detection
pub fn AutoYoinkInt(s: Tea) -> NoCapResult<(i64, Tea)> {
    YoinkInt(s, 0, 64) // Base 0 for auto-detection, 64-bit for maximum range
}

/// Enhanced convenience function for parsing unsigned integers with automatic base detection
pub fn AutoYoinkUint(s: Tea) -> NoCapResult<(u64, Tea)> {
    super::parse::YoinkUint(s, 0, 64) // Base 0 for auto-detection, 64-bit for maximum range
}

/// Parse a string as a float with automatic precision detection
pub fn AutoYoinkFloat(s: Tea) -> NoCapResult<(f64, Tea)> {
    super::parse::YoinkFloat(s, 64) // 64-bit for maximum precision
}

/// Validate if a string represents a valid integer
pub fn IsValidInt(s: &Tea) -> bool {
    Atoi(s.clone()).is_ok()
}

/// Validate if a string represents a valid float
pub fn IsValidFloat(s: &Tea) -> bool {
    AutoYoinkFloat(s.clone()).is_ok()
}

/// Validate if a string represents a valid boolean (using FactsCheck)
pub fn IsValidBool(s: &Tea) -> bool {
    super::parse::FactsCheck(s.clone()).is_ok()
}

/// Convert between different number bases
pub fn ConvertBase(s: Tea, from_base: Normie, to_base: Normie) -> NoCapResult<Tea> {
    // Parse the number in the source base
    let (value, _) = YoinkInt(s, from_base, 64)?;
    
    // Convert to the target base
    Ok(YeetInt(value, to_base))
}

/// Format a number with thousand separators
pub fn FormatWithSeparators(i: i64, separator: char) -> Tea {
    let mut result = YeetInt(i, 10);
    let is_negative = result.starts_with('-');
    
    if is_negative {
        result = result[1..].to_string(); // Remove the negative sign temporarily
    }
    
    if result.len() <= 3 {
        if is_negative {
            format!("-{}", result)
        } else {
            result
        }
    } else {
        let mut formatted = String::new();
        let chars: Vec<char> = result.chars().collect();
        
        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                formatted.push(separator);
            }
            formatted.push(*ch);
        }
        
        if is_negative {
            format!("-{}", formatted)
        } else {
            formatted
        }
    }
}

/// Parse a string with thousand separators
pub fn ParseWithSeparators(s: Tea, separator: char) -> NoCapResult<(i64, Tea)> {
    let cleaned = s.replace(separator, "");
    YoinkInt(cleaned, 10, 64)
}

/// Convert a boolean value using various output formats
pub fn FormatBoolCustom(b: bool, true_value: &str, false_value: &str) -> Tea {
    if b {
        true_value.to_string()
    } else {
        false_value.to_string()
    }
}

/// Helper function to determine if a string looks like a number
pub fn LooksLikeNumber(s: &Tea) -> bool {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return false;
    }
    
    // Check for common number patterns
    if IsValidInt(s) || IsValidFloat(s) {
        return true;
    }
    
    // Check for hex, binary, octal patterns
    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        return trimmed[2..].chars().all(|c| c.is_ascii_hexdigit());
    }
    
    if trimmed.starts_with("0b") || trimmed.starts_with("0B") {
        return trimmed[2..].chars().all(|c| c == '0' || c == '1');
    }
    
    false
}

/// Get the numeric type of a string
#[derive(Debug, PartialEq, Clone)]
pub enum NumberType {
    Integer,
    UnsignedInteger,
    Float,
    Boolean,
    NotANumber,
}

pub fn GetNumberType(s: &Tea) -> NumberType {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return NumberType::NotANumber;
    }
    
    // Check boolean first
    if IsValidBool(s) {
        return NumberType::Boolean;
    }
    
    // Check integer
    if IsValidInt(s) {
        // Check if it could be unsigned (no negative sign)
        if !trimmed.starts_with('-') && AutoYoinkUint(s.clone()).is_ok() {
            return NumberType::UnsignedInteger;
        }
        return NumberType::Integer;
    }
    
    // Check float
    if IsValidFloat(s) {
        return NumberType::Float;
    }
    
    NumberType::NotANumber
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi_basic() {
        let result = Atoi("123".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, 123);

        let result = Atoi("-456".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, -456);

        let result = Atoi("0".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn test_atoi_invalid() {
        let result = Atoi("abc".to_string());
        assert!(result.is_err());

        let result = Atoi("".to_string());
        assert!(result.is_err());

        let result = Atoi("123.45".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_atoi_range() {
        // Test value within range
        let result = Atoi("2147483647".to_string());
        assert!(result.is_ok());

        // Test value outside range (assuming this exceeds i32::MAX)
        let large_value = (i32::MAX as i64 + 1).to_string();
        let result = Atoi(large_value);
        assert!(result.is_err());
    }

    #[test]
    fn test_itoa() {
        assert_eq!(Itoa(123), "123");
        assert_eq!(Itoa(-456), "-456");
        assert_eq!(Itoa(0), "0");
    }

    #[test]
    fn test_auto_yoink_functions() {
        // Test AutoYoinkInt
        let result = AutoYoinkInt("0xFF".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, 255);

        // Test AutoYoinkUint
        let result = AutoYoinkUint("0b1010".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, 10);

        // Test AutoYoinkFloat
        let result = AutoYoinkFloat("123.45".to_string());
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert!((value - 123.45).abs() < f64::EPSILON);
    }

    #[test]
    fn test_validation_functions() {
        // Test IsValidInt
        assert!(IsValidInt(&"123".to_string()));
        assert!(IsValidInt(&"-456".to_string()));
        assert!(!IsValidInt(&"123.45".to_string()));
        assert!(!IsValidInt(&"abc".to_string()));

        // Test IsValidFloat
        assert!(IsValidFloat(&"123.45".to_string()));
        assert!(IsValidFloat(&"-67.89".to_string()));
        assert!(IsValidFloat(&"123".to_string())); // Integers are valid floats
        assert!(!IsValidFloat(&"abc".to_string()));

        // Test IsValidBool
        assert!(IsValidBool(&"facts".to_string()));
        assert!(IsValidBool(&"cap".to_string()));
        assert!(IsValidBool(&"true".to_string()));
        assert!(!IsValidBool(&"maybe".to_string()));
    }

    #[test]
    fn test_convert_base() {
        // Convert decimal to hex
        let result = ConvertBase("255".to_string(), 10, 16);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ff");

        // Convert hex to decimal
        let result = ConvertBase("ff".to_string(), 16, 10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "255");

        // Convert binary to decimal
        let result = ConvertBase("1010".to_string(), 2, 10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_format_with_separators() {
        assert_eq!(FormatWithSeparators(1234567, ','), "1,234,567");
        assert_eq!(FormatWithSeparators(-1234567, ','), "-1,234,567");
        assert_eq!(FormatWithSeparators(123, ','), "123");
        assert_eq!(FormatWithSeparators(0, ','), "0");
    }

    #[test]
    fn test_parse_with_separators() {
        let result = ParseWithSeparators("1,234,567".to_string(), ',');
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, 1234567);

        let result = ParseWithSeparators("-1,234,567".to_string(), ',');
        assert!(result.is_ok());
        let (value, _) = result.unwrap();
        assert_eq!(value, -1234567);
    }

    #[test]
    fn test_format_bool_custom() {
        assert_eq!(FormatBoolCustom(true, "yes", "no"), "yes");
        assert_eq!(FormatBoolCustom(false, "yes", "no"), "no");
        assert_eq!(FormatBoolCustom(true, "1", "0"), "1");
        assert_eq!(FormatBoolCustom(false, "1", "0"), "0");
    }

    #[test]
    fn test_looks_like_number() {
        assert!(LooksLikeNumber(&"123".to_string()));
        assert!(LooksLikeNumber(&"-456".to_string()));
        assert!(LooksLikeNumber(&"123.45".to_string()));
        assert!(LooksLikeNumber(&"0xFF".to_string()));
        assert!(LooksLikeNumber(&"0b1010".to_string()));
        assert!(!LooksLikeNumber(&"abc".to_string()));
        assert!(!LooksLikeNumber(&"".to_string()));
    }

    #[test]
    fn test_get_number_type() {
        assert_eq!(GetNumberType(&"123".to_string()), NumberType::UnsignedInteger);
        assert_eq!(GetNumberType(&"-123".to_string()), NumberType::Integer);
        assert_eq!(GetNumberType(&"123.45".to_string()), NumberType::Float);
        assert_eq!(GetNumberType(&"facts".to_string()), NumberType::Boolean);
        assert_eq!(GetNumberType(&"abc".to_string()), NumberType::NotANumber);
    }

    #[test]
    fn test_roundtrip_conversions() {
        // Test integer roundtrip
        let original = 12345;
        let string_form = Itoa(original);
        let result = Atoi(string_form);
        assert!(result.is_ok());
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, original);

        // Test negative integer roundtrip
        let original = -6789;
        let string_form = Itoa(original);
        let result = Atoi(string_form);
        assert!(result.is_ok());
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, original);
    }
}
