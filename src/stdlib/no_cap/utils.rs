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
    }
}

/// Convert an int to a tea (string) (equivalent to strconv.Itoa)
/// 
/// This is a convenience function that formats a 32-bit integer as a decimal string.
/// It's equivalent to YeetInt(i as i64, 10).
pub fn Itoa(i: Normie) -> Tea {
    YeetInt(i as i64, 10)
/// Enhanced convenience function for parsing integers with automatic base detection
pub fn AutoYoinkInt(s: Tea) -> NoCapResult<(i64, Tea)> {
    YoinkInt(s, 0, 64) // Base 0 for auto-detection, 64-bit for maximum range
/// Enhanced convenience function for parsing unsigned integers with automatic base detection
pub fn AutoYoinkUint(s: Tea) -> NoCapResult<(u64, Tea)> {
    super::parse::YoinkUint(s, 0, 64) // Base 0 for auto-detection, 64-bit for maximum range
/// Parse a string as a float with automatic precision detection
pub fn AutoYoinkFloat(s: Tea) -> NoCapResult<(f64, Tea)> {
    super::parse::YoinkFloat(s, 64) // 64-bit for maximum precision
/// Validate if a string represents a valid integer
pub fn IsValidInt(s: &Tea) -> bool {
    Atoi(s.clone()).is_ok()
/// Validate if a string represents a valid float
pub fn IsValidFloat(s: &Tea) -> bool {
    AutoYoinkFloat(s.clone()).is_ok()
/// Validate if a string represents a valid boolean (using FactsCheck)
pub fn IsValidBool(s: &Tea) -> bool {
    super::parse::FactsCheck(s.clone()).is_ok()
/// Convert between different number bases
pub fn ConvertBase(s: Tea, from_base: Normie, to_base: Normie) -> NoCapResult<Tea> {
    // Parse the number in the source base
    let (value, _) = YoinkInt(s, from_base, 64)?;
    
    // Convert to the target base
    Ok(YeetInt(value, to_base))
/// Format a number with thousand separators
pub fn FormatWithSeparators(i: i64, separator: char) -> Tea {
    let mut result = YeetInt(i, 10);
    let is_negative = result.starts_with('-');
    
    if is_negative {
        result = result[1..].to_string(); // Remove the negative sign temporarily
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
        if is_negative {
            format!("-{}", formatted)
        } else {
            formatted
        }
    }
/// Parse a string with thousand separators
pub fn ParseWithSeparators(s: Tea, separator: char) -> NoCapResult<(i64, Tea)> {
    let cleaned = s.replace(separator, "");
    YoinkInt(cleaned, 10, 64)
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
    // Check for common number patterns
    if IsValidInt(s) || IsValidFloat(s) {
        return true;
    // Check for hex, binary, octal patterns
    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        return trimmed[2..].chars().all(|c| c.is_ascii_hexdigit());
    if trimmed.starts_with("0b") || trimmed.starts_with("0B") {
        return trimmed[2..].chars().all(|c| c == '0' || c == '1');
    false
/// Get the numeric type of a string
#[derive(Debug, PartialEq, Clone)]
pub enum NumberType {
pub fn GetNumberType(s: &Tea) -> NumberType {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return NumberType::NotANumber;
    // Check boolean first
    if IsValidBool(s) {
        return NumberType::Boolean;
    // Check integer
    if IsValidInt(s) {
        // Check if it could be unsigned (no negative sign)
        if !trimmed.starts_with('-') && AutoYoinkUint(s.clone()).is_ok() {
            return NumberType::UnsignedInteger;
        }
        return NumberType::Integer;
    // Check float
    if IsValidFloat(s) {
        return NumberType::Float;
    NumberType::NotANumber
