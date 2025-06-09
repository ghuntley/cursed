//! Syntax Highlighting for CURSED REPL
//! 
//! Provides real-time syntax highlighting for CURSED keywords,
//! operators, literals, and comments using ANSI color codes.

use std::collections::HashSet;
use once_cell::sync::Lazy;

/// ANSI color codes for syntax highlighting
pub struct Colors {
    pub reset: &'static str,
    pub keyword: &'static str,
    pub operator: &'static str,
    pub string: &'static str,
    pub number: &'static str,
    pub comment: &'static str,
    pub type_name: &'static str,
    pub function: &'static str,
    pub variable: &'static str,
    pub error: &'static str,
}

impl Colors {
    pub const fn new() -> Self {
        Self {
            reset: "\x1b[0m",
            keyword: "\x1b[35m",      // Magenta
            operator: "\x1b[33m",     // Yellow
            string: "\x1b[32m",       // Green
            number: "\x1b[36m",       // Cyan
            comment: "\x1b[90m",      // Bright black (gray)
            type_name: "\x1b[34m",    // Blue
            function: "\x1b[96m",     // Bright cyan
            variable: "\x1b[37m",     // White
            error: "\x1b[31m",        // Red
        }
    }
}

static COLORS: Colors = Colors::new();

/// CURSED language keywords for highlighting
static CURSED_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // Gen Z slang keywords
        "slay", "yolo", "sus", "facts", "lowkey", "highkey", "periodt",
        "bestie", "flex", "vibe_check", "mood", "basic", "no_cap",
        "straight_fire", "it_hits_different", "absolute_unit", "send_it",
        "that_slaps", "bussin", "fire", "slaps", "bet", "say_less",
        "cap", "no_printer", "frfr", "ong", "sheesh", "valid", "slick",
        
        // Traditional keywords
        "func", "var", "const", "type", "interface", "struct", "enum",
        "if", "else", "for", "while", "switch", "case", "default",
        "break", "continue", "return", "defer", "go", "chan", "select",
        "package", "import", "nil", "true", "false",
        
        // CURSED-specific
        "squad", "collab", "vibes", "energy", "aura", "rizz",
        "main_character", "side_character", "npc",
    ].into_iter().collect()
});

/// CURSED operators for highlighting
static CURSED_OPERATORS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=",
        "&&", "||", "!", "&", "|", "^", "<<", ">>", "++", "--",
        "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=",
        "->", "=>", ":=", "...", "..", "?", ":", ";", ",", ".",
        "(", ")", "[", "]", "{", "}", "<-",
    ].into_iter().collect()
});

/// CURSED type names for highlighting
static CURSED_TYPES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "int", "int8", "int16", "int32", "int64",
        "uint", "uint8", "uint16", "uint32", "uint64",
        "float32", "float64", "bool", "string", "byte", "rune",
        "error", "interface{}", "chan", "map", "slice",
        "Energy", "Vibes", "Aura", "Rizz",
    ].into_iter().collect()
});

/// Syntax highlighter for CURSED code
pub struct SyntaxHighlighter {
    enable_colors: bool,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter
    pub fn new() -> Self {
        Self {
            enable_colors: atty::is(atty::Stream::Stdout),
        }
    }

    /// Create a syntax highlighter with explicit color support
    pub fn with_colors(enable_colors: bool) -> Self {
        Self { enable_colors }
    }

    /// Highlight CURSED code with syntax coloring
    pub fn highlight(&self, code: &str) -> String {
        if !self.enable_colors {
            return code.to_string();
        }

        let mut result = String::new();
        let mut chars = code.chars().peekable();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut string_char = '"';
        let mut in_comment = false;
        let mut in_number = false;

        while let Some(ch) = chars.next() {
            match ch {
                // String literals
                '"' | '\'' if !in_comment => {
                    if in_string && ch == string_char {
                        // End of string
                        current_token.push(ch);
                        result.push_str(&self.colorize(&current_token, TokenType::String));
                        current_token.clear();
                        in_string = false;
                    } else if !in_string {
                        // Start of string
                        if !current_token.is_empty() {
                            result.push_str(&self.highlight_token(&current_token));
                            current_token.clear();
                        }
                        current_token.push(ch);
                        in_string = true;
                        string_char = ch;
                    } else {
                        current_token.push(ch);
                    }
                }
                
                // Comments
                '/' if !in_string && !in_comment => {
                    if let Some(&'/') = chars.peek() {
                        // Line comment
                        if !current_token.is_empty() {
                            result.push_str(&self.highlight_token(&current_token));
                            current_token.clear();
                        }
                        current_token.push(ch);
                        current_token.push(chars.next().unwrap());
                        in_comment = true;
                    } else {
                        current_token.push(ch);
                    }
                }
                
                '\n' if in_comment => {
                    // End of line comment
                    result.push_str(&self.colorize(&current_token, TokenType::Comment));
                    result.push(ch);
                    current_token.clear();
                    in_comment = false;
                }
                
                // Numbers
                c if c.is_ascii_digit() && !in_string && !in_comment => {
                    if current_token.is_empty() {
                        in_number = true;
                    }
                    current_token.push(ch);
                }
                
                '.' if in_number && !in_string && !in_comment => {
                    current_token.push(ch);
                }
                
                // Operators and delimiters
                c if self.is_operator_char(c) && !in_string && !in_comment && !in_number => {
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                        in_number = false;
                    }
                    
                    // Handle multi-character operators
                    let mut operator = String::new();
                    operator.push(c);
                    
                    // Look ahead for multi-character operators
                    while let Some(&next_char) = chars.peek() {
                        let potential_op = format!("{}{}", operator, next_char);
                        if CURSED_OPERATORS.contains(potential_op.as_str()) {
                            operator.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    result.push_str(&self.colorize(&operator, TokenType::Operator));
                }
                
                // Whitespace and other characters
                c if c.is_whitespace() => {
                    if !current_token.is_empty() {
                        if in_comment {
                            current_token.push(ch);
                        } else {
                            result.push_str(&self.highlight_token(&current_token));
                            current_token.clear();
                            in_number = false;
                        }
                    }
                    
                    if !in_comment {
                        result.push(ch);
                    } else {
                        current_token.push(ch);
                    }
                }
                
                // Default case
                _ => {
                    current_token.push(ch);
                    if in_string || in_comment {
                        // Continue collecting
                    } else if in_number && !ch.is_ascii_alphanumeric() && ch != '.' {
                        // End of number
                        let num_token = current_token[..current_token.len()-1].to_string();
                        if !num_token.is_empty() {
                            result.push_str(&self.colorize(&num_token, TokenType::Number));
                        }
                        current_token = ch.to_string();
                        in_number = false;
                    }
                }
            }
        }

        // Handle remaining token
        if !current_token.is_empty() {
            if in_comment {
                result.push_str(&self.colorize(&current_token, TokenType::Comment));
            } else {
                result.push_str(&self.highlight_token(&current_token));
            }
        }

        result
    }

    /// Highlight a single token
    fn highlight_token(&self, token: &str) -> String {
        let token_type = self.classify_token(token);
        self.colorize(token, token_type)
    }

    /// Classify a token to determine its type
    fn classify_token(&self, token: &str) -> TokenType {
        if CURSED_KEYWORDS.contains(token) {
            TokenType::Keyword
        } else if CURSED_OPERATORS.contains(token) {
            TokenType::Operator
        } else if CURSED_TYPES.contains(token) {
            TokenType::TypeName
        } else if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
            TokenType::Number
        } else if token.starts_with('"') || token.starts_with('\'') {
            TokenType::String
        } else if token.starts_with("//") {
            TokenType::Comment
        } else if self.is_function_name(token) {
            TokenType::Function
        } else {
            TokenType::Variable
        }
    }

    /// Check if a character is part of an operator
    fn is_operator_char(&self, ch: char) -> bool {
        matches!(ch, '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | 
                     '&' | '|' | '^' | '?' | ':' | ';' | ',' | '.' | 
                     '(' | ')' | '[' | ']' | '{' | '}')
    }

    /// Check if a token is likely a function name
    fn is_function_name(&self, token: &str) -> bool {
        // Simple heuristic: if it's followed by '(' in context
        // For now, just check common function patterns
        token.ends_with("_func") || 
        token.starts_with("get_") || 
        token.starts_with("set_") ||
        token.starts_with("is_") ||
        token.starts_with("has_")
    }

    /// Apply color to a token based on its type
    fn colorize(&self, token: &str, token_type: TokenType) -> String {
        if !self.enable_colors {
            return token.to_string();
        }

        let color = match token_type {
            TokenType::Keyword => COLORS.keyword,
            TokenType::Operator => COLORS.operator,
            TokenType::String => COLORS.string,
            TokenType::Number => COLORS.number,
            TokenType::Comment => COLORS.comment,
            TokenType::TypeName => COLORS.type_name,
            TokenType::Function => COLORS.function,
            TokenType::Variable => COLORS.variable,
            TokenType::Error => COLORS.error,
        };

        format!("{}{}{}", color, token, COLORS.reset)
    }
}

/// Token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    Keyword,
    Operator,
    String,
    Number,
    Comment,
    TypeName,
    Function,
    Variable,
    Error,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_highlighting() {
        let highlighter = SyntaxHighlighter::with_colors(true);
        let code = "slay main_character() {";
        let highlighted = highlighter.highlight(code);
        
        // Should contain color codes for keywords
        assert!(highlighted.contains('\x1b'));
    }

    #[test]
    fn test_string_highlighting() {
        let highlighter = SyntaxHighlighter::with_colors(true);
        let code = r#"facts message = "Hello, world!""#;
        let highlighted = highlighter.highlight(code);
        
        // Should contain color codes
        assert!(highlighted.contains('\x1b'));
    }

    #[test]
    fn test_comment_highlighting() {
        let highlighter = SyntaxHighlighter::with_colors(true);
        let code = "// This is a comment\nfacts x = 42";
        let highlighted = highlighter.highlight(code);
        
        // Should contain color codes
        assert!(highlighted.contains('\x1b'));
    }

    #[test]
    fn test_no_colors() {
        let highlighter = SyntaxHighlighter::with_colors(false);
        let code = "slay main() { facts x = 42; }";
        let highlighted = highlighter.highlight(code);
        
        // Should not contain color codes
        assert_eq!(highlighted, code);
    }

    #[test]
    fn test_number_highlighting() {
        let highlighter = SyntaxHighlighter::with_colors(true);
        let code = "facts x = 42.5";
        let highlighted = highlighter.highlight(code);
        
        // Should contain color codes
        assert!(highlighted.contains('\x1b'));
    }
}
