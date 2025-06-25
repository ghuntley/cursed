use crate::error::CursedError;
/// Utility functions for regex operations
use super::error::{RegexVibesResult, RegexVibesError, invalid_input_error, encoding_error};
use super::pattern::VibePattern;
use std::collections::HashMap;
use std::io::Read;

/// Validate if a string is a valid regex pattern
pub fn is_valid_pattern(pattern: &str) -> bool {
    regex::Regex::new(pattern).is_ok()
}

/// Get detailed information about why a pattern is invalid
pub fn validate_pattern(pattern: &str) -> RegexVibesResult<()> {
    regex::Regex::new(pattern)
        .map(|_| ())
        .map_err(RegexVibesError::from)
}

/// Count the number of capturing groups in a pattern
pub fn count_capture_groups(pattern: &str) -> RegexVibesResult<i32> {
    let regex = regex::Regex::new(pattern)?;
    Ok(regex.captures_len() as i32)
}

/// Extract all literal strings from a pattern (simplified)
pub fn extract_literals(pattern: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let mut current_literal = String::new();
    let mut chars = pattern.chars().peekable();
    let mut in_escape = false;
    let mut in_brackets = false;
    let mut in_group = false;

    while let Some(ch) = chars.next() {
        if in_escape {
            current_literal.push(ch);
            in_escape = false;
            continue;
        }

        match ch {
            '\\' => in_escape = true,
            '[' => {
                if !current_literal.is_empty() {
                    literals.push(current_literal.clone());
                    current_literal.clear();
                }
                in_brackets = true;
            }
            ']' if in_brackets => in_brackets = false,
            '(' => {
                if !current_literal.is_empty() {
                    literals.push(current_literal.clone());
                    current_literal.clear();
                }
                in_group = true;
            }
            ')' if in_group => in_group = false,
            '.' | '*' | '+' | '?' | '^' | '$' | '|' if !in_brackets => {
                if !current_literal.is_empty() {
                    literals.push(current_literal.clone());
                    current_literal.clear();
                }
            }
            _ if !in_brackets => current_literal.push(ch),
            _ => {}
        }
    }

    if !current_literal.is_empty() {
        literals.push(current_literal);
    }

    literals.into_iter().filter(|s| s.len() > 1).collect()
}

/// Find common prefix in a list of strings that could be optimized in regex
pub fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    for string in strings.iter().skip(1) {
        let mut common_len = 0;
        for (i, (a, b)) in prefix.chars().zip(string.chars()).enumerate() {
            if a == b {
                common_len = i + 1;
            } else {
                break;
            }
        }
        prefix.truncate(common_len);
    }

    prefix
}

/// Find common suffix in a list of strings
pub fn find_common_suffix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let reversed: Vec<String> = strings.iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    
    let common_prefix = find_common_prefix(&reversed);
    common_prefix.chars().rev().collect()
}

/// Generate a regex pattern that matches any of the given strings
pub fn strings_to_alternation(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    if strings.len() == 1 {
        return regex::escape(&strings[0]);
    }

    let escaped: Vec<String> = strings.iter()
        .map(|s| regex::escape(s))
        .collect();

    format!("(?:{})", escaped.join("|"))
}

/// Optimize a list of literal strings into a more efficient regex
pub fn optimize_string_list(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    if strings.len() == 1 {
        return regex::escape(&strings[0]);
    }

    let prefix = find_common_prefix(strings);
    let suffix = find_common_suffix(strings);

    if prefix.is_empty() && suffix.is_empty() {
        return strings_to_alternation(strings);
    }

    let mut middle_parts = Vec::new();
    for string in strings {
        let mut middle = string.clone();
        if !prefix.is_empty() {
            middle = middle.strip_prefix(&prefix).unwrap_or(&middle).to_string();
        }
        if !suffix.is_empty() {
            middle = middle.strip_suffix(&suffix).unwrap_or(&middle).to_string();
        }
        middle_parts.push(middle);
    }

    let prefix_escaped = if prefix.is_empty() { String::new() } else { regex::escape(&prefix) };
    let suffix_escaped = if suffix.is_empty() { String::new() } else { regex::escape(&suffix) };
    let middle_pattern = strings_to_alternation(&middle_parts);

    format!("{}{}{}", prefix_escaped, middle_pattern, suffix_escaped)
}

/// Test multiple patterns against a string and return which ones match
pub fn test_patterns(text: &str, patterns: &[&str]) -> RegexVibesResult<Vec<(String, bool)>> {
    let mut results = Vec::new();
    
    for pattern in patterns {
        match VibePattern::compile(pattern) {
            Ok(p) => results.push((pattern.to_string(), p.match_string(text))),
            Err(_) => results.push((pattern.to_string(), false)),
        }
    }
    
    Ok(results)
}

/// Benchmark pattern matching performance
pub fn benchmark_pattern(pattern: &str, texts: &[&str], iterations: usize) -> RegexVibesResult<BenchmarkResult> {
    let compiled_pattern = VibePattern::compile(pattern)?;
    
    let start = std::time::Instant::now();
    let mut total_matches = 0;
    
    for _ in 0..iterations {
        for text in texts {
            if compiled_pattern.match_string(text) {
                total_matches += 1;
            }
        }
    }
    
    let duration = start.elapsed();
    let total_operations = iterations * texts.len();
    
    Ok(BenchmarkResult {
        pattern: pattern.to_string(),
        total_operations,
        total_matches,
        duration_micros: duration.as_micros() as u64,
        operations_per_second: (total_operations as f64 / duration.as_secs_f64()) as u64,
    })
}

/// Result of pattern benchmarking
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub pattern: String,
    pub total_operations: usize,
    pub total_matches: usize,
    pub duration_micros: u64,
    pub operations_per_second: u64,
}

impl BenchmarkResult {
    /// Get the average microseconds per operation
    pub fn avg_micros_per_operation(&self) -> f64 {
        self.duration_micros as f64 / self.total_operations as f64
    }

    /// Get the match rate as a percentage
    pub fn match_rate(&self) -> f64 {
        (self.total_matches as f64 / self.total_operations as f64) * 100.0
    }
}

/// Escape special characters for use in replacement strings
pub fn escape_replacement(s: &str) -> String {
    s.replace('$', "$$")
}

/// Parse replacement string and extract group references
pub fn parse_replacement_references(replacement: &str) -> Vec<String> {
    let mut references = Vec::new();
    let mut chars = replacement.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    let mut num = String::new();
                    while let Some(&digit) = chars.peek() {
                        if digit.is_ascii_digit() {
                            num.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    references.push(format!("${}", num));
                } else if next_ch == '{' {
                    chars.next(); // consume '{'
                    let mut name = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                        name.push(ch);
                    }
                    if !name.is_empty() {
                        references.push(format!("${{{}}}", name));
                    }
                }
            }
        }
    }
    
    references
}

/// Convert a glob pattern to a regex pattern
pub fn glob_to_regex(glob: &str) -> String {
    let mut regex = String::new();
    regex.push('^');
    
    let mut chars = glob.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                    if chars.peek() == Some(&'/') {
                        chars.next(); // consume /
                        regex.push_str("(?:.*/)?");
                    } else {
                        regex.push_str(".*");
                    }
                } else {
                    regex.push_str("[^/]*");
                }
            }
            '?' => regex.push_str("[^/]"),
            '[' => {
                regex.push('[');
                let mut in_bracket = true;
                while let Some(ch) = chars.next() {
                    regex.push(ch);
                    if ch == ']' {
                        in_bracket = false;
                        break;
                    }
                }
                if in_bracket {
                    regex.push(']');
                }
            }
            ch if "^$(){}+|\\".contains(ch) => {
                regex.push('\\');
                regex.push(ch);
            }
            ch => regex.push(ch),
        }
    }
    
    regex.push('$');
    regex
}

/// Check if a string matches a glob pattern
pub fn glob_match(pattern: &str, text: &str) -> RegexVibesResult<bool> {
    let regex_pattern = glob_to_regex(pattern);
    let compiled = VibePattern::compile(&regex_pattern)?;
    Ok(compiled.match_string(text))
}

/// Find all regex patterns in a string (patterns enclosed in forward slashes)
pub fn find_regex_patterns(text: &str) -> Vec<String> {
    let pattern = VibePattern::compile(r"/([^/\\]+(?:\\.[^/\\]*)*)/").unwrap();
    pattern.find_all_string_submatch(text, -1)
        .into_iter()
        .filter_map(|matches| matches.get(1).cloned())
        .collect()
}

/// Create a pattern that matches any line containing the given patterns
pub fn create_line_filter(patterns: &[&str]) -> RegexVibesResult<VibePattern> {
    if patterns.is_empty() {
        return Err(invalid_input_error("No patterns provided"));
    }

    let escaped_patterns: Vec<String> = patterns.iter()
        .map(|p| regex::escape(p))
        .collect();

    let combined = format!("^.*(?:{}).*$", escaped_patterns.join("|"));
    VibePattern::compile(&combined)
}

/// Split text by regex and keep the delimiters
pub fn split_keep_delimiter(text: &str, pattern: &str) -> RegexVibesResult<Vec<String>> {
    let regex_pattern = VibePattern::compile(pattern)?;
    let mut result = Vec::new();
    let mut last_end = 0;

    let indexes = regex_pattern.find_all_string_index(text, -1);
    
    for index_pair in indexes {
        if index_pair.len() >= 2 {
            let start = index_pair[0] as usize;
            let end = index_pair[1] as usize;
            
            // Add text before delimiter
            if start > last_end {
                result.push(text[last_end..start].to_string());
            }
            
            // Add delimiter
            result.push(text[start..end].to_string());
            
            last_end = end;
        }
    }
    
    // Add remaining text
    if last_end < text.len() {
        result.push(text[last_end..].to_string());
    }
    
    Ok(result)
}

