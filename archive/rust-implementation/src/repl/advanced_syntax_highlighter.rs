//! Advanced syntax highlighter for CURSED REPL
//! Provides real-time syntax highlighting with colors and error indicators

use crate::error::CursedError;
use colored::*;
use std::collections::HashMap;

/// Color scheme for syntax highlighting
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub keyword: Color,
    pub type_name: Color,
    pub string: Color,
    pub number: Color,
    pub comment: Color,
    pub operator: Color,
    pub identifier: Color,
    pub builtin: Color,
    pub error: Color,
    pub warning: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            keyword: Color::Cyan,
            type_name: Color::Blue,
            string: Color::Green,
            number: Color::Yellow,
            comment: Color::BrightBlack,
            operator: Color::Magenta,
            identifier: Color::White,
            builtin: Color::BrightCyan,
            error: Color::Red,
            warning: Color::BrightYellow,
        }
    }
}

/// Token type for syntax highlighting
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    TypeName,
    String,
    Number,
    Comment,
    Operator,
    Identifier,
    Builtin,
    Error,
    Warning,
    Whitespace,
    Unknown,
}

/// A highlighted token
#[derive(Debug, Clone)]
pub struct HighlightedToken {
    pub text: String,
    pub token_type: TokenType,
    pub start_pos: usize,
    pub end_pos: usize,
}

/// Advanced syntax highlighter for CURSED language
pub struct CursedSyntaxHighlighter {
    color_scheme: ColorScheme,
    keywords: HashMap<String, TokenType>,
    operators: Vec<String>,
    builtins: HashMap<String, TokenType>,
}

impl CursedSyntaxHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();
        
        // CURSED keywords
        keywords.insert("sus".to_string(), TokenType::Keyword);
        keywords.insert("slay".to_string(), TokenType::Keyword);
        keywords.insert("damn".to_string(), TokenType::Keyword);
        keywords.insert("vibez".to_string(), TokenType::Keyword);
        keywords.insert("yeet".to_string(), TokenType::Keyword);
        keywords.insert("ready".to_string(), TokenType::Keyword);
        keywords.insert("otherwise".to_string(), TokenType::Keyword);
        keywords.insert("bestie".to_string(), TokenType::Keyword);
        keywords.insert("squad".to_string(), TokenType::Keyword);
        keywords.insert("collab".to_string(), TokenType::Keyword);
        keywords.insert("periodt".to_string(), TokenType::Keyword);
        keywords.insert("based".to_string(), TokenType::Keyword);
        keywords.insert("cringe".to_string(), TokenType::Keyword);
        keywords.insert("nocap".to_string(), TokenType::Keyword);
        keywords.insert("lowkey".to_string(), TokenType::Keyword);
        keywords.insert("highkey".to_string(), TokenType::Keyword);
        keywords.insert("deadass".to_string(), TokenType::Keyword);
        keywords.insert("onfr".to_string(), TokenType::Keyword);
        keywords.insert("bet".to_string(), TokenType::Keyword);
        keywords.insert("finna".to_string(), TokenType::Keyword);
        keywords.insert("stan".to_string(), TokenType::Keyword);
        
        // Type names
        keywords.insert("drip".to_string(), TokenType::TypeName);
        keywords.insert("tea".to_string(), TokenType::TypeName);
        keywords.insert("lit".to_string(), TokenType::TypeName);
        keywords.insert("flex".to_string(), TokenType::TypeName);
        keywords.insert("vibe".to_string(), TokenType::TypeName);
        
        let mut builtins = HashMap::new();
        
        // Built-in functions
        builtins.insert("spill".to_string(), TokenType::Builtin);
        builtins.insert("len".to_string(), TokenType::Builtin);
        builtins.insert("push".to_string(), TokenType::Builtin);
        builtins.insert("pop".to_string(), TokenType::Builtin);
        builtins.insert("slice".to_string(), TokenType::Builtin);
        builtins.insert("print".to_string(), TokenType::Builtin);
        
        let operators = vec![
            "==".to_string(), "!=".to_string(), "<=".to_string(), ">=".to_string(),
            "&&".to_string(), "||".to_string(), "++".to_string(), "--".to_string(),
            "+=".to_string(), "-=".to_string(), "*=".to_string(), "/=".to_string(),
            "=>".to_string(), "->".to_string(), "..".to_string(), "...".to_string(),
            "+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(),
            "%".to_string(), "^".to_string(), "&".to_string(), "|".to_string(),
            "!".to_string(), "~".to_string(), "<".to_string(), ">".to_string(),
            "=".to_string(), "?".to_string(), ":".to_string(), ";".to_string(),
            ",".to_string(), ".".to_string(),
        ];
        
        Self {
            color_scheme: ColorScheme::default(),
            keywords,
            operators,
            builtins,
        }
    }
    
    /// Set a custom color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
    }
    
    /// Tokenize and highlight a line of CURSED code
    pub fn highlight_line(&self, line: &str) -> Result<String, CursedError> {
        let tokens = self.tokenize(line)?;
        let highlighted = self.apply_highlighting(&tokens);
        Ok(highlighted)
    }
    
    /// Tokenize a line into highlighted tokens
    fn tokenize(&self, line: &str) -> Result<Vec<HighlightedToken>, CursedError> {
        let mut tokens = Vec::new();
        let mut chars = line.char_indices().peekable();
        let mut current_pos = 0;
        
        while let Some((pos, ch)) = chars.next() {
            current_pos = pos;
            
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' => {
                    let start = pos;
                    let mut end = pos + ch.len_utf8();
                    
                    // Consume consecutive whitespace
                    while let Some((next_pos, next_ch)) = chars.peek() {
                        if next_ch.is_whitespace() {
                            let (_, consumed_ch) = chars.next().unwrap();
                            end = next_pos + consumed_ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    
                    tokens.push(HighlightedToken {
                        text: line[start..end].to_string(),
                        token_type: TokenType::Whitespace,
                        start_pos: start,
                        end_pos: end,
                    });
                }
                
                // Comments (fr fr)
                'f' if line[pos..].starts_with("fr fr") => {
                    // Comment extends to end of line
                    let comment_text = &line[pos..];
                    tokens.push(HighlightedToken {
                        text: comment_text.to_string(),
                        token_type: TokenType::Comment,
                        start_pos: pos,
                        end_pos: line.len(),
                    });
                    break; // Rest of line is comment
                }
                
                // String literals
                '"' | '\'' => {
                    let delimiter = ch;
                    let start = pos;
                    let mut end = pos + 1;
                    let mut escaped = false;
                    
                    while let Some((str_pos, str_ch)) = chars.next() {
                        end = str_pos + str_ch.len_utf8();
                        
                        if escaped {
                            escaped = false;
                            continue;
                        }
                        
                        if str_ch == '\\' {
                            escaped = true;
                        } else if str_ch == delimiter {
                            break;
                        }
                    }
                    
                    tokens.push(HighlightedToken {
                        text: line[start..end].to_string(),
                        token_type: TokenType::String,
                        start_pos: start,
                        end_pos: end,
                    });
                }
                
                // Numbers
                '0'..='9' => {
                    let start = pos;
                    let mut end = pos + ch.len_utf8();
                    let mut has_dot = false;
                    
                    // Consume digits and at most one decimal point
                    while let Some((num_pos, num_ch)) = chars.peek() {
                        match num_ch {
                            '0'..='9' => {
                                let (_, consumed_ch) = chars.next().unwrap();
                                end = num_pos + consumed_ch.len_utf8();
                            }
                            '.' if !has_dot => {
                                has_dot = true;
                                let (_, consumed_ch) = chars.next().unwrap();
                                end = num_pos + consumed_ch.len_utf8();
                            }
                            _ => break,
                        }
                    }
                    
                    tokens.push(HighlightedToken {
                        text: line[start..end].to_string(),
                        token_type: TokenType::Number,
                        start_pos: start,
                        end_pos: end,
                    });
                }
                
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = pos;
                    let mut end = pos + ch.len_utf8();
                    
                    // Consume alphanumeric characters and underscores
                    while let Some((id_pos, id_ch)) = chars.peek() {
                        if id_ch.is_alphanumeric() || *id_ch == '_' {
                            let (_, consumed_ch) = chars.next().unwrap();
                            end = id_pos + consumed_ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    
                    let text = line[start..end].to_string();
                    let token_type = if let Some(keyword_type) = self.keywords.get(&text) {
                        keyword_type.clone()
                    } else if let Some(builtin_type) = self.builtins.get(&text) {
                        builtin_type.clone()
                    } else {
                        TokenType::Identifier
                    };
                    
                    tokens.push(HighlightedToken {
                        text,
                        token_type,
                        start_pos: start,
                        end_pos: end,
                    });
                }
                
                // Operators and punctuation
                _ => {
                    let start = pos;
                    let mut end = pos + ch.len_utf8();
                    let mut found_operator = false;
                    
                    // Check for multi-character operators
                    for op in &self.operators {
                        if line[pos..].starts_with(op) && op.len() > 1 {
                            // Consume the operator
                            for _ in 1..op.len() {
                                if chars.next().is_some() {
                                    // Position updated by iterator
                                }
                            }
                            end = pos + op.len();
                            found_operator = true;
                            break;
                        }
                    }
                    
                    // Single character operator/punctuation
                    if !found_operator {
                        for op in &self.operators {
                            if op.len() == 1 && op.starts_with(ch) {
                                found_operator = true;
                                break;
                            }
                        }
                    }
                    
                    let token_type = if found_operator {
                        TokenType::Operator
                    } else {
                        TokenType::Unknown
                    };
                    
                    tokens.push(HighlightedToken {
                        text: line[start..end].to_string(),
                        token_type,
                        start_pos: start,
                        end_pos: end,
                    });
                }
            }
        }
        
        Ok(tokens)
    }
    
    /// Apply color highlighting to tokens
    fn apply_highlighting(&self, tokens: &[HighlightedToken]) -> String {
        let mut result = String::new();
        
        for token in tokens {
            let colored_text = match token.token_type {
                TokenType::Keyword => token.text.color(self.color_scheme.keyword).bold(),
                TokenType::TypeName => token.text.color(self.color_scheme.type_name).bold(),
                TokenType::String => token.text.color(self.color_scheme.string),
                TokenType::Number => token.text.color(self.color_scheme.number),
                TokenType::Comment => token.text.color(self.color_scheme.comment).italic(),
                TokenType::Operator => token.text.color(self.color_scheme.operator),
                TokenType::Identifier => token.text.color(self.color_scheme.identifier),
                TokenType::Builtin => token.text.color(self.color_scheme.builtin).bold(),
                TokenType::Error => token.text.color(self.color_scheme.error).on_red(),
                TokenType::Warning => token.text.color(self.color_scheme.warning).on_yellow(),
                TokenType::Whitespace => token.text.normal(),
                TokenType::Unknown => token.text.color(self.color_scheme.error),
            };
            
            result.push_str(&colored_text.to_string());
        }
        
        result
    }
    
    /// Highlight syntax errors in a line
    pub fn highlight_errors(&self, line: &str, errors: &[(usize, usize, String)]) -> String {
        let mut tokens = self.tokenize(line).unwrap_or_default();
        
        // Mark tokens as errors based on error positions
        for (start, end, _message) in errors {
            for token in &mut tokens {
                if token.start_pos >= *start && token.end_pos <= *end {
                    token.token_type = TokenType::Error;
                }
            }
        }
        
        self.apply_highlighting(&tokens)
    }
    
    /// Get a preview of highlighted code
    pub fn preview_highlighting(&self, code: &str) -> String {
        code.lines()
            .map(|line| self.highlight_line(line).unwrap_or_else(|_| line.to_string()))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Validate CURSED syntax and return errors
    pub fn validate_syntax(&self, line: &str) -> Vec<(usize, usize, String)> {
        let mut errors = Vec::new();
        
        // Basic syntax validation
        let tokens = match self.tokenize(line) {
            Ok(tokens) => tokens,
            Err(_) => {
                errors.push((0, line.len(), "Failed to tokenize line".to_string()));
                return errors;
            }
        };
        
        // Check for unmatched brackets/braces/parentheses
        let mut brace_depth = 0;
        let mut bracket_depth = 0;
        let mut paren_depth = 0;
        
        for token in &tokens {
            match token.text.as_str() {
                "{" => brace_depth += 1,
                "}" => {
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        errors.push((token.start_pos, token.end_pos, "Unmatched closing brace".to_string()));
                    }
                }
                "[" => bracket_depth += 1,
                "]" => {
                    bracket_depth -= 1;
                    if bracket_depth < 0 {
                        errors.push((token.start_pos, token.end_pos, "Unmatched closing bracket".to_string()));
                    }
                }
                "(" => paren_depth += 1,
                ")" => {
                    paren_depth -= 1;
                    if paren_depth < 0 {
                        errors.push((token.start_pos, token.end_pos, "Unmatched closing parenthesis".to_string()));
                    }
                }
                _ => {}
            }
        }
        
        // Check for incomplete strings
        for token in &tokens {
            if token.token_type == TokenType::String {
                let text = &token.text;
                if text.len() < 2 || !text.ends_with(&text.chars().next().unwrap().to_string()) {
                    errors.push((token.start_pos, token.end_pos, "Unterminated string literal".to_string()));
                }
            }
        }
        
        errors
    }
    
    /// Create a custom color scheme
    pub fn create_color_scheme(
        keyword: Color,
        type_name: Color,
        string: Color,
        number: Color,
        comment: Color,
        operator: Color,
        identifier: Color,
        builtin: Color,
        error: Color,
        warning: Color,
    ) -> ColorScheme {
        ColorScheme {
            keyword,
            type_name,
            string,
            number,
            comment,
            operator,
            identifier,
            builtin,
            error,
            warning,
        }
    }
}

/// Helper function to create a dark theme
pub fn create_dark_theme() -> ColorScheme {
    CursedSyntaxHighlighter::create_color_scheme(
        Color::BrightCyan,    // keyword
        Color::BrightBlue,    // type_name
        Color::BrightGreen,   // string
        Color::BrightYellow,  // number
        Color::BrightBlack,   // comment
        Color::BrightMagenta, // operator
        Color::White,         // identifier
        Color::Cyan,          // builtin
        Color::BrightRed,     // error
        Color::Yellow,        // warning
    )
}

/// Helper function to create a light theme
pub fn create_light_theme() -> ColorScheme {
    CursedSyntaxHighlighter::create_color_scheme(
        Color::Blue,          // keyword
        Color::BrightBlue,    // type_name
        Color::Green,         // string
        Color::BrightYellow,  // number
        Color::Black,         // comment
        Color::Magenta,       // operator
        Color::Black,         // identifier
        Color::Cyan,          // builtin
        Color::Red,           // error
        Color::BrightYellow,  // warning
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_highlighting() {
        let highlighter = CursedSyntaxHighlighter::new();
        let tokens = highlighter.tokenize("sus x drip = 42").unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Keyword); // sus
        assert_eq!(tokens[2].token_type, TokenType::Identifier); // x
        assert_eq!(tokens[4].token_type, TokenType::TypeName); // drip
        assert_eq!(tokens[6].token_type, TokenType::Operator); // =
        assert_eq!(tokens[8].token_type, TokenType::Number); // 42
    }

    #[test]
    fn test_string_highlighting() {
        let highlighter = CursedSyntaxHighlighter::new();
        let tokens = highlighter.tokenize("\"hello world\"").unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].text, "\"hello world\"");
    }

    #[test]
    fn test_comment_highlighting() {
        let highlighter = CursedSyntaxHighlighter::new();
        let tokens = highlighter.tokenize("sus x drip = 42 fr fr this is a comment").unwrap();
        
        // Find the comment token
        let comment_token = tokens.iter().find(|t| t.token_type == TokenType::Comment);
        assert!(comment_token.is_some());
        assert!(comment_token.unwrap().text.contains("fr fr"));
    }

    #[test]
    fn test_syntax_validation() {
        let highlighter = CursedSyntaxHighlighter::new();
        
        // Valid syntax
        let errors = highlighter.validate_syntax("sus x drip = 42");
        assert!(errors.is_empty());
        
        // Invalid syntax - unmatched brace
        let errors = highlighter.validate_syntax("ready (x > 0) { vibez.spill(x)");
        assert!(!errors.is_empty());
    }
}
