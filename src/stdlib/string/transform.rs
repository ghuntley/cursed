use crate::error::CursedError;
/// String transformation operations
use super::{StringError, StringResult};

/// Extract substring from start index with given length
pub fn substring(s: &str, start: usize, length: usize) -> StringResult<String> {
    let chars: Vec<char> = s.chars().collect();
    let str_len = chars.len();
    
    if start > str_len {
        return Err(StringError::IndexOutOfBounds { 
            length: str_len 
        });
    let end = (start + length).min(str_len);
    Ok(chars[start..end].iter().collect())
/// Extract substring from start to end index (exclusive)
pub fn substring_range(s: &str, start: usize, end: usize) -> StringResult<String> {
    let chars: Vec<char> = s.chars().collect();
    let str_len = chars.len();
    
    if start > str_len || end > str_len {
        return Err(StringError::IndexOutOfBounds { 
            length: str_len 
        });
    if start > end {
        return Err(StringError::InvalidRange { 
            length: str_len 
        });
    Ok(chars[start..end].iter().collect())
/// Remove leading and trailing whitespace
pub fn trim(s: &str) -> String {
    s.trim().to_string()
/// Remove leading whitespace
pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
/// Remove trailing whitespace
pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
/// Remove leading and trailing occurrences of specified characters
pub fn trim_chars(s: &str, chars_to_trim: &[char]) -> String {
    s.trim_matches(chars_to_trim).to_string()
/// Remove leading occurrences of specified characters
pub fn trim_start_chars(s: &str, chars_to_trim: &[char]) -> String {
    s.trim_start_matches(chars_to_trim).to_string()
/// Remove trailing occurrences of specified characters
pub fn trim_end_chars(s: &str, chars_to_trim: &[char]) -> String {
    s.trim_end_matches(chars_to_trim).to_string()
/// Convert string to lowercase
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
/// Convert string to uppercase
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
/// Convert string to title case (first letter of each word capitalized)
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                for i in 1..chars.len() {
                    chars[i] = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                }
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
/// Convert string to camelCase
pub fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.is_empty() {
        return String::new();
    let mut result = words[0].to_lowercase();
    for word in &words[1..] {
        if !word.is_empty() {
            let mut chars: Vec<char> = word.chars().collect();
            chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
            for i in 1..chars.len() {
                chars[i] = chars[i].to_lowercase().next().unwrap_or(chars[i]);
            }
            result.push_str(&chars.into_iter().collect::<String>());
        }
    }
    result
/// Convert string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                for i in 1..chars.len() {
                    chars[i] = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                }
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("")
/// Convert string to snake_case
pub fn to_snake_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
/// Convert string to kebab-case
pub fn to_kebab_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
/// Capitalize the first letter of a string (leaving the rest unchanged)
pub fn capitalize(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if !chars.is_empty() {
        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
    }
    chars.into_iter().collect()
/// Insert string at specified position
pub fn insert_at(s: &str, pos: usize, insert: &str) -> StringResult<String> {
    let chars: Vec<char> = s.chars().collect();
    let str_len = chars.len();
    
    if pos > str_len {
        return Err(StringError::IndexOutOfBounds { 
            length: str_len 
        });
    let mut result = String::new();
    result.push_str(&chars[..pos].iter().collect::<String>());
    result.push_str(insert);
    result.push_str(&chars[pos..].iter().collect::<String>());
    
    Ok(result)
/// Remove characters from start to end position
pub fn remove_range(s: &str, start: usize, end: usize) -> StringResult<String> {
    let chars: Vec<char> = s.chars().collect();
    let str_len = chars.len();
    
    if start > str_len || end > str_len {
        return Err(StringError::IndexOutOfBounds { 
            length: str_len 
        });
    if start > end {
        return Err(StringError::InvalidRange { 
            length: str_len 
        });
    let mut result = String::new();
    result.push_str(&chars[..start].iter().collect::<String>());
    result.push_str(&chars[end..].iter().collect::<String>());
    
    Ok(result)
