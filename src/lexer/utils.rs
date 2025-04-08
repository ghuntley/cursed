/// Utility functions for the lexer

/// Check if a character is a letter
pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

/// Check if a character is a digit
pub fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

/// Check if a character is a hexadecimal digit
pub fn is_hex_digit(ch: char) -> bool {
    (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
}

/// Check if a character is an octal digit
pub fn is_octal_digit(ch: char) -> bool {
    ch >= '0' && ch <= '7'
}

/// Check for a specific sequence in the input
pub fn peek_sequence(input: &str, current_pos: usize, read_pos: usize, sequence: &str) -> bool {
    if current_pos == 0 || read_pos < sequence.len() || input.len() < read_pos + sequence.len() - 1 {
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