use crate::error::CursedError;
/// String validation and character checking operations
use super::{StringError, StringResult};

/// Check if string represents a numeric value
pub fn is_numeric(s: &str) -> bool {
    if s.is_empty() {
        return false;
    // Handle optional sign
    let trimmed = s.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    let start_idx = if chars[0] == '+' || chars[0] == '-' { 1 } else { 0 };
    
    if start_idx >= chars.len() {
        return false;
    let mut has_dot = false;
    let mut has_digit = false;
    
    for (i, &ch) in chars.iter().enumerate().skip(start_idx) {
        if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch == '.' && !has_dot {
            has_dot = true;
        } else {
            return false;
        }
    }
    
    has_digit
/// Check if string represents an integer
pub fn is_integer(s: &str) -> bool {
    if s.is_empty() {
        return false;
    let trimmed = s.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    let start_idx = if chars[0] == '+' || chars[0] == '-' { 1 } else { 0 };
    
    if start_idx >= chars.len() {
        return false;
    chars[start_idx..].iter().all(|&ch| ch.is_ascii_digit())
/// Check if string contains only alphabetic characters
pub fn is_alphabetic(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
/// Check if string contains only alphanumeric characters
pub fn is_alphanumeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
/// Check if string contains only whitespace characters
pub fn is_whitespace(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_whitespace())
/// Check if string contains only uppercase characters (ignoring non-alphabetic)
pub fn is_uppercase(s: &str) -> bool {
    if s.is_empty() {
        return false;
    let alphabetic_chars: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
    !alphabetic_chars.is_empty() && alphabetic_chars.iter().all(|c| c.is_uppercase())
/// Check if string contains only lowercase characters (ignoring non-alphabetic)
pub fn is_lowercase(s: &str) -> bool {
    if s.is_empty() {
        return false;
    let alphabetic_chars: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
    !alphabetic_chars.is_empty() && alphabetic_chars.iter().all(|c| c.is_lowercase())
/// Check if string is in title case (first letter of each word capitalized)
pub fn is_title_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    for word in s.split_whitespace() {
        let chars: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
        if chars.is_empty() {
            continue;
        if !chars[0].is_uppercase() {
            return false;
        for &ch in &chars[1..] {
            if !ch.is_lowercase() {
                return false;
            }
        }
    true
/// Check if string contains only hexadecimal characters
pub fn is_hex(s: &str) -> bool {
    if s.is_empty() {
        return false;
    // Handle optional 0x prefix
    let content = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    
    !content.is_empty() && content.chars().all(|c| c.is_ascii_hexdigit())
/// Check if string is a valid email address (basic validation)
pub fn is_email(s: &str) -> bool {
    if s.is_empty() {
        return false;
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    let (local, domain) = (parts[0], parts[1]);
    
    // Basic local part validation
    if local.is_empty() || local.len() > 64 {
        return false;
    // Basic domain validation
    if domain.is_empty() || !domain.contains('.') {
        return false;
    // Check for valid characters in local part
    let valid_local_chars = |c: char| {
        c.is_alphanumeric() || ".!#$%&'*+/=?^_`{|}~-".contains(c)
    
    if !local.chars().all(valid_local_chars) {
        return false;
    // Check domain parts
    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|part| part.is_empty()) {
        return false;
    // Basic domain character validation
    let valid_domain_chars = |c: char| c.is_alphanumeric() || c == '-';
    domain.chars().all(|c| valid_domain_chars(c) || c == '.')
/// Check if string is a valid URL (basic validation)
pub fn is_url(s: &str) -> bool {
    if s.is_empty() {
        return false;
    // Check for scheme
    if let Some(scheme_end) = s.find("://") {
        let scheme = &s[..scheme_end];
        if scheme.is_empty() || !scheme.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '-' || c == '.') {
            return false;
        let rest = &s[scheme_end + 3..];
        return !rest.is_empty();
    false
/// Check if string matches a basic phone number pattern
pub fn is_phone_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    // Remove common phone number separators
    let cleaned: String = s.chars()
        .filter(|&c| c != ' ' && c != '-' && c != '(' && c != ')' && c != '.')
        .collect();
    
    // Check if it starts with + (international) and has digits
    if cleaned.starts_with('+') {
        let rest = &cleaned[1..];
        return rest.len() >= 7 && rest.len() <= 15 && rest.chars().all(|c| c.is_ascii_digit());
    // Check for domestic number (7-15 digits)
    cleaned.len() >= 7 && cleaned.len() <= 15 && cleaned.chars().all(|c| c.is_ascii_digit())
/// Check if string contains balanced parentheses
pub fn has_balanced_parentheses(s: &str) -> bool {
    let mut count = 0i32;
    
    for ch in s.chars() {
        match ch {
            ')' => {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
            _ => {}
        }
    count == 0
/// Check if string contains balanced brackets (multiple types)
pub fn has_balanced_brackets(s: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in s.chars() {
        match ch {
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    stack.is_empty()
/// Check if string is a palindrome (ignoring case and whitespace)
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap_or(c))
        .collect();
    
    let chars: Vec<char> = cleaned.chars().collect();
    let len = chars.len();
    
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    
    true
