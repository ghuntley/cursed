/// Enhanced Unicode-aware string operations
// use crate::stdlib::glyph_gang::error::{GlyphGangResult, normalization_error, unicode_error};
// use crate::stdlib::glyph_gang::core::{to_upper, to_lower, to_title, simple_fold};
use std::collections::HashMap;

/// Normalization forms for Unicode text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationForm {
    /// Canonical Decomposition followed by Canonical Composition
    NFC,
    /// Canonical Decomposition
    NFD,
    /// Compatibility Decomposition followed by Canonical Composition
    NFKC,
    /// Compatibility Decomposition
    NFKD,
}

/// Constants for normalization forms
pub const NFC: NormalizationForm = NormalizationForm::NFC;
pub const NFD: NormalizationForm = NormalizationForm::NFD;
pub const NFKC: NormalizationForm = NormalizationForm::NFKC;
pub const NFKD: NormalizationForm = NormalizationForm::NFKD;

/// Convert a string to uppercase
pub fn to_upper_string(s: &str) -> String {
    s.chars().map(to_upper).collect()
}

/// Convert a string to lowercase
pub fn to_lower_string(s: &str) -> String {
    s.chars().map(to_lower).collect()
}

/// Convert a string to titlecase
pub fn to_title_string(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in s.chars() {
        if ch.is_whitespace() {
            result.push(ch);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(to_title(ch));
            capitalize_next = false;
        } else {
            result.push(to_lower(ch));
        }
    }
    
    result
}

/// Normalize a string using the specified normalization form
pub fn normalize_string(s: &str, form: NormalizationForm) -> GlyphGangResult<String> {
    // For now, this is a simplified implementation
    // In a production system, this would use proper Unicode normalization algorithms
    match form {
        NormalizationForm::NFC => {
            // Canonical Decomposition followed by Canonical Composition
            // For simplicity, return the string as-is for now
            Ok(s.to_string())
        }
        NormalizationForm::NFD => {
            // Canonical Decomposition
            // For simplicity, return the string as-is for now
            Ok(s.to_string())
        }
        NormalizationForm::NFKC => {
            // Compatibility Decomposition followed by Canonical Composition
            // For simplicity, return the string as-is for now
            Ok(s.to_string())
        }
        NormalizationForm::NFKD => {
            // Compatibility Decomposition
            // For simplicity, return the string as-is for now
            Ok(s.to_string())
        }
    }
}

/// Count the number of Unicode code points (runes) in a string
pub fn rune_count(s: &str) -> usize {
    s.chars().count()
}

/// Get the first rune and its byte size from a string
pub fn first_rune(s: &str) -> (char, usize) {
    if let Some(ch) = s.chars().next() {
        (ch, ch.len_utf8())
    } else {
        ('\0', 0)
    }
}

/// Get the last rune and its byte size from a string
pub fn last_rune(s: &str) -> (char, usize) {
    if let Some(ch) = s.chars().last() {
        (ch, ch.len_utf8())
    } else {
        ('\0', 0)
    }
}

/// Get the rune at the specified index (by rune position, not byte position)
pub fn rune_at(s: &str, index: usize) -> char {
    s.chars().nth(index).unwrap_or('\0')
}

/// Get the byte indices of all runes in a string
pub fn rune_indices(s: &str) -> Vec<usize> {
    s.char_indices().map(|(i, _)| i).collect()
}

/// Get the display width of a string (accounting for double-width characters)
pub fn string_width(s: &str) -> usize {
    s.chars().map(get_char_width).sum()
}

/// Get the display width of a single character
pub fn get_char_width(ch: char) -> usize {
    let code_point = ch as u32;
    
    // ASCII characters have width 1
    if code_point < 0x80 {
        if ch.is_control() {
            return 0; // Control characters have no visible width
        }
        return 1;
    }
    
    // Zero-width characters
    if is_zero_width(ch) {
        return 0;
    }
    
    // Double-width characters (East Asian)
    if is_double_width(ch) {
        return 2;
    }
    
    // Default to width 1
    1
}

/// Check if a character has zero display width
fn is_zero_width(ch: char) -> bool {
    let code_point = ch as u32;
    
    // Combining marks
    if (0x0300..=0x036F).contains(&code_point) {
        return true;
    }
    
    // Zero-width characters
    match code_point {
        0x200B | // Zero Width Space
        0x200C | // Zero Width Non-Joiner
        0x200D | // Zero Width Joiner
        0xFEFF   // Zero Width No-Break Space
        => true,
        _ => false,
    }
}

/// Check if a character has double display width
fn is_double_width(ch: char) -> bool {
    let code_point = ch as u32;
    
    // CJK characters
    if (0x1100..=0x115F).contains(&code_point) || // Hangul Jamo
       (0x2329..=0x232A).contains(&code_point) || // Left/Right-Pointing Angle Brackets
       (0x2E80..=0x2EFF).contains(&code_point) || // CJK Radicals Supplement
       (0x2F00..=0x2FDF).contains(&code_point) || // Kangxi Radicals
       (0x2FF0..=0x2FFF).contains(&code_point) || // Ideographic Description Characters
       (0x3000..=0x303E).contains(&code_point) || // CJK Symbols and Punctuation
       (0x3041..=0x3096).contains(&code_point) || // Hiragana
       (0x3099..=0x30FF).contains(&code_point) || // Katakana
       (0x3105..=0x312D).contains(&code_point) || // Bopomofo
       (0x3131..=0x318E).contains(&code_point) || // Hangul Compatibility Jamo
       (0x3190..=0x31BA).contains(&code_point) || // Kanbun
       (0x31C0..=0x31E3).contains(&code_point) || // CJK Strokes
       (0x31F0..=0x321E).contains(&code_point) || // Katakana Phonetic Extensions
       (0x3220..=0x3247).contains(&code_point) || // Enclosed CJK Letters and Months
       (0x3250..=0x32FE).contains(&code_point) || // Enclosed CJK Letters and Months
       (0x3300..=0x4DBF).contains(&code_point) || // CJK Compatibility to Extension A
       (0x4E00..=0xA48C).contains(&code_point) || // CJK Unified Ideographs to Yi Syllables
       (0xA490..=0xA4C6).contains(&code_point) || // Yi Radicals
       (0xAC00..=0xD7A3).contains(&code_point) || // Hangul Syllables
       (0xF900..=0xFAFF).contains(&code_point) || // CJK Compatibility Ideographs
       (0xFE10..=0xFE19).contains(&code_point) || // Vertical Forms
       (0xFE30..=0xFE6F).contains(&code_point) || // CJK Compatibility Forms
       (0xFF00..=0xFF60).contains(&code_point) || // Fullwidth Forms
       (0xFFE0..=0xFFE6).contains(&code_point) || // Fullwidth Forms
       (0x20000..=0x2FFFD).contains(&code_point) || // CJK Extensions
       (0x30000..=0x3FFFD).contains(&code_point)    // CJK Extensions
    {
        return true;
    }
    
    false
}

/// Get the total display width of a string
pub fn get_string_width(s: &str) -> usize {
    string_width(s)
}

/// Truncate a string to a maximum display width
pub fn truncate_string(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;
    
    for ch in s.chars() {
        let char_width = get_char_width(ch);
        if current_width + char_width > max_width {
            break;
        }
        result.push(ch);
        current_width += char_width;
    }
    
    result
}

/// Truncate a string with ellipsis to fit within the specified width
pub fn truncate_with_ellipsis(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    
    if string_width(s) <= max_width {
        return s.to_string();
    }
    
    if max_width < 3 {
        // Not enough space for ellipsis
        return truncate_string(s, max_width);
    }
    
    // Reserve space for ellipsis (3 characters)
    let truncated = truncate_string(s, max_width - 3);
    format!("{}...", truncated)
}

/// Wrap text to fit within the specified width
pub fn wrap_text(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![String::new()];
    }
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for word in s.split_whitespace() {
        let word_width = string_width(word);
        
        // Check if adding this word would exceed the width
        if current_width + word_width + (if current_line.is_empty() { 0 } else { 1 }) > width {
            // Start a new line
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }
            
            // If the word itself is too long, we might need to break it
            if word_width > width {
                // For now, just add it as-is (word breaking is complex)
                lines.push(word.to_string());
            } else {
                current_line = word.to_string();
                current_width = word_width;
            }
        } else {
            // Add word to current line
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(word);
            current_width += word_width;
        }
    }
    
    // Add the last line if it's not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

/// Reverse a string while preserving Unicode grapheme clusters
pub fn reverse_string(s: &str) -> String {
    // For now, just reverse by characters
    // In a production implementation, this would use proper grapheme cluster segmentation
    s.chars().rev().collect()
}

/// Find word boundaries in a string (simplified implementation)
pub fn word_boundaries(s: &str) -> Vec<usize> {
    let mut boundaries = vec![0];
    let mut byte_pos = 0;
    let mut in_word = false;
    
    for ch in s.chars() {
        let is_word_char = ch.is_alphanumeric();
        
        if in_word != is_word_char {
            boundaries.push(byte_pos);
            in_word = is_word_char;
        }
        
        byte_pos += ch.len_utf8();
    }
    
    if boundaries.last() != Some(&s.len()) {
        boundaries.push(s.len());
    }
    
    boundaries
}

/// Find sentence boundaries in a string (simplified implementation)
pub fn sentence_boundaries(s: &str) -> Vec<usize> {
    let mut boundaries = vec![0];
    let mut byte_pos = 0;
    let mut last_char = '\0';
    
    for ch in s.chars() {
        if matches!(last_char, '.' | '!' | '?') && ch.is_whitespace() {
            boundaries.push(byte_pos);
        }
        
        last_char = ch;
        byte_pos += ch.len_utf8();
    }
    
    if boundaries.last() != Some(&s.len()) {
        boundaries.push(s.len());
    }
    
    boundaries
}

/// Find line break opportunities in a string (simplified implementation)
pub fn line_break_opportunities(s: &str) -> Vec<usize> {
    let mut opportunities = vec![0];
    let mut byte_pos = 0;
    
    for ch in s.chars() {
        if ch.is_whitespace() || ch == '-' {
            opportunities.push(byte_pos + ch.len_utf8());
        }
        
        byte_pos += ch.len_utf8();
    }
    
    if opportunities.last() != Some(&s.len()) {
        opportunities.push(s.len());
    }
    
    opportunities
}

/// Case-insensitive string folding for comparison
pub fn fold_string(s: &str) -> String {
    s.chars().map(simple_fold).collect()
}

/// Case-insensitive string equality comparison
pub fn equal_fold(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        // Quick check - if byte lengths differ, check character by character
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        
        if chars1.len() != chars2.len() {
            return false;
        }
        
        for (c1, c2) in chars1.iter().zip(chars2.iter()) {
            if simple_fold(*c1) != simple_fold(*c2) {
                return false;
            }
        }
        
        true
    } else {
        // Same byte length, can compare directly
        fold_string(s1) == fold_string(s2)
    }
}

