//! Utility functions for the CURSED lexer
//!
//! This module provides helper functions for character classification and
//! sequence detection used by the lexer during tokenization.

/// Determines if a character is a letter (alphabetic or underscore)
///
/// Used for identifying characters that can be part of identifiers.
///
/// # Arguments
///
/// * `ch` - The character to check
///
/// # Returns
///
/// `true` if the character is a letter or underscore, `false` otherwise
pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

/// Determines if a character is a decimal digit (0-9)
///
/// Used for identifying characters that can be part of numeric literals.
///
/// # Arguments
///
/// * `ch` - The character to check
///
/// # Returns
///
/// `true` if the character is a digit (0-9), `false` otherwise
pub fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

/// Determines if a character is a hexadecimal digit (0-9, a-f, A-F)
///
/// Used for identifying characters that can be part of hexadecimal literals.
///
/// # Arguments
///
/// * `ch` - The character to check
///
/// # Returns
///
/// `true` if the character is a hex digit, `false` otherwise
pub fn is_hex_digit(ch: char) -> bool {
    (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
}

/// Determines if a character is an octal digit (0-7)
///
/// Used for identifying characters that can be part of octal literals.
///
/// # Arguments
///
/// * `ch` - The character to check
///
/// # Returns
///
/// `true` if the character is an octal digit (0-7), `false` otherwise
pub fn is_octal_digit(ch: char) -> bool {
    ch >= '0' && ch <= '7'
}

/// Checks if a specific character sequence exists at a position in the input
///
/// Used for identifying multi-character tokens like comments and operators.
///
/// # Arguments
///
/// * `input` - The input string to check against
/// * `current_pos` - The current position in the input
/// * `read_pos` - The read position (usually current_pos + 1)
/// * `sequence` - The sequence to look for
///
/// # Returns
///
/// `true` if the sequence exists at the specified position, `false` otherwise
pub fn peek_sequence(input: &str, current_pos: usize, read_pos: usize, sequence: &str) -> bool {
    if current_pos == 0 || read_pos < sequence.len() || input.len() < read_pos + sequence.len() - 1
    {
        return false;
    }

    let start = read_pos - 1; // Include the current character
    let end = start + sequence.len();

    if end > input.len() {
        return false;
    }

    let slice = &input[start..end];
    slice == sequence
}
