use crate::error::CursedError;
/// String splitting and joining operations
use super::{StringError, StringResult};

/// Split string by delimiter
pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    if delimiter.is_empty() {
        // Split into individual characters
        return s.chars().map(|c| c.to_string()).collect();
    }
    s.split(delimiter).map(|s| s.to_string()).collect()
}

/// Split string by delimiter, limiting to N parts
pub fn split_n(s: &str, delimiter: &str, n: usize) -> Vec<String> {
    if delimiter.is_empty() || n == 0 {
        return vec![s.to_string()];
    }
    s.splitn(n, delimiter).map(|s| s.to_string()).collect()
}

/// Split string by delimiter from the right
pub fn rsplit(s: &str, delimiter: &str) -> Vec<String> {
    if delimiter.is_empty() {
        return s.chars().rev().map(|c| c.to_string()).collect();
    }
    s.rsplit(delimiter).map(|s| s.to_string()).collect()
}

/// Split string by delimiter from the right, limiting to N parts
pub fn rsplit_n(s: &str, delimiter: &str, n: usize) -> Vec<String> {
    if delimiter.is_empty() || n == 0 {
        return vec![s.to_string()];
    }
    s.rsplitn(n, delimiter).map(|s| s.to_string()).collect()
}

/// Split string by line breaks (handles \n, \r\n, \r)
pub fn split_lines(s: &str) -> Vec<String> {
    s.split("\n").map(|line| line.to_string()).collect()
}

/// Split string by whitespace (spaces, tabs, newlines)
pub fn split_whitespace(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

/// Split string by any of the given characters
pub fn split_any(s: &str, delimiters: &[char]) -> Vec<String> {
    s.split(delimiters).map(|s| s.to_string()).collect()
}

/// Split string by predicate function
pub fn split_by<F>(s: &str, predicate: F) -> Vec<String>
where
    F: Fn(char) -> bool,
{
    s.split(predicate).map(|s| s.to_string()).collect()
}

/// Join strings with separator
pub fn join(strings: &[&str], separator: &str) -> String {
    strings.join(separator)
}

/// Join owned strings with separator
pub fn join_owned(strings: &[String], separator: &str) -> String {
    strings.join(separator)
}

/// Join strings with different separators for different positions
pub fn join_with_separators(strings: &[&str], separators: &[&str]) -> StringResult<String> {
    if strings.is_empty() {
        return Ok(String::new());
    }
    
    if strings.len() == 1 {
        return Ok(strings[0].to_string());
    }
    
    if separators.len() != strings.len() - 1 {
        return Err(StringError::InvalidParameter {
            param: "separators".to_string(),
            value: format!("Expected {} separators for {} strings", strings.len() - 1, strings.len()),
        });
    }
    
    let mut result = String::new();
    result.push_str(strings[0]);
    
    for i in 1..strings.len() {
        result.push_str(separators[i - 1]);
        result.push_str(strings[i]);
    }
    
    Ok(result)
}

/// Partition string into three parts: before delimiter, delimiter, after delimiter
pub fn partition(s: &str, delimiter: &str) -> (String, String, String) {
    if let Some(pos) = s.find(delimiter) {
        (
            s[..pos].to_string(),
            delimiter.to_string(),
            s[pos + delimiter.len()..].to_string(),
        )
    } else {
        (s.to_string(), String::new(), String::new())
    }
}

/// Partition string from the right
pub fn rpartition(s: &str, delimiter: &str) -> (String, String, String) {
    if let Some(pos) = s.rfind(delimiter) {
        (
            s[..pos].to_string(),
            delimiter.to_string(),
            s[pos + delimiter.len()..].to_string(),
        )
    } else {
        (String::new(), String::new(), s.to_string())
    }
}

/// Split string into chunks of specified size
pub fn chunk(s: &str, size: usize) -> StringResult<Vec<String>> {
    if size == 0 {
        return Err(StringError::InvalidParameter {
            param: "size".to_string(),
            value: "cannot be zero".to_string(),
        });
    }
    
    let chars: Vec<char> = s.chars().collect();
    let mut chunks = Vec::new();
    
    for chunk in chars.chunks(size) {
        chunks.push(chunk.iter().collect::<String>());
    }
    
    Ok(chunks)
}

/// Split string into exactly N equal parts (last part may be longer)
pub fn split_into_n_parts(s: &str, n: usize) -> StringResult<Vec<String>> {
    if n == 0 {
        return Err(StringError::InvalidParameter {
            param: "n".to_string(),
            value: "cannot be zero".to_string(),
        });
    }
    
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    if n >= len {
        // Each character becomes its own part, plus empty strings if needed
        let mut parts: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
        while parts.len() < n {
            parts.push(String::new());
        }
        return Ok(parts);
    }
    
    let part_size = len / n;
    let remainder = len % n;
    let mut parts = Vec::new();
    let mut start = 0;
    
    for i in 0..n {
        let end = if i < n - 1 {
            start + part_size
        } else {
            len // Last part gets all remaining characters
        };
        
        parts.push(chars[start..end].iter().collect::<String>());
        start = end;
    }
    
    Ok(parts)
}

