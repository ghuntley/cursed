//! CURSED Code Formatter
//! 
//! A comprehensive formatter for CURSED source code that handles Gen Z slang syntax
//! and produces well-formatted, readable code with configurable indentation and style options.

use crate::error_types::{Error, Result};
use std::collections::HashMap;

/// Main formatter for CURSED source code
#[derive(Debug, Clone)]
pub struct CursedFormatter {
    config: FormatterConfig,
    indentation_level: usize,
    line_buffer: Vec<String>,
    current_line: String,
}

/// Configuration options for the CURSED formatter
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Formatting style options
    pub options: FormattingOptions,
    /// Whether to preserve comments
    pub preserve_comments: bool,
    /// Whether to sort imports alphabetically
    pub sort_imports: bool,
    /// Maximum line length before wrapping
    pub max_line_length: usize,
    /// Whether to add trailing commas
    pub trailing_commas: bool,
}

/// Specific formatting style options
#[derive(Debug, Clone)]
pub struct FormattingOptions {
    /// Indentation style (spaces or tabs)
    pub indent_style: IndentStyle,
    /// Number of spaces for indentation (when using spaces)
    pub indent_size: usize,
    /// Whether to add spaces around operators
    pub spaces_around_operators: bool,
    /// Whether to add spaces after commas
    pub spaces_after_commas: bool,
    /// Whether to add spaces inside parentheses
    pub spaces_inside_parentheses: bool,
    /// Whether to add spaces inside braces
    pub spaces_inside_braces: bool,
    /// Brace placement style
    pub brace_style: BraceStyle,
    /// How to handle blank lines
    pub blank_lines: BlankLinePolicy,
}

/// Indentation style options
#[derive(Debug, Clone, PartialEq)]
pub enum IndentStyle {
    Spaces,
    Tabs,
}

/// Brace placement style
#[derive(Debug, Clone, PartialEq)]
pub enum BraceStyle {
    /// Opening brace on same line
    SameLine,
    /// Opening brace on next line
    NextLine,
    /// Opening brace on next line with extra indent
    NextLineIndented,
}

/// Blank line handling policy
#[derive(Debug, Clone)]
pub struct BlankLinePolicy {
    /// Max blank lines between statements
    pub max_blank_lines: usize,
    /// Blank lines before function declarations
    pub before_functions: usize,
    /// Blank lines after imports
    pub after_imports: usize,
    /// Blank lines around control structures
    pub around_control_structures: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            options: FormattingOptions::default(),
            preserve_comments: true,
            sort_imports: true,
            max_line_length: 100,
            trailing_commas: false,
        }
    }
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces,
            indent_size: 4,
            spaces_around_operators: true,
            spaces_after_commas: true,
            spaces_inside_parentheses: false,
            spaces_inside_braces: true,
            brace_style: BraceStyle::SameLine,
            blank_lines: BlankLinePolicy::default(),
        }
    }
}

impl Default for BlankLinePolicy {
    fn default() -> Self {
        Self {
            max_blank_lines: 2,
            before_functions: 1,
            after_imports: 1,
            around_control_structures: 0,
        }
    }
}

impl CursedFormatter {
    /// Create a new formatter with default configuration
    pub fn new() -> Self {
        Self::with_config(FormatterConfig::default())
    }

    /// Create a new formatter with the given configuration
    pub fn with_config(config: FormatterConfig) -> Self {
        Self {
            config,
            indentation_level: 0,
            line_buffer: Vec::new(),
            current_line: String::new(),
        }
    }

    /// Format CURSED source code
    pub fn format(&mut self, source: &str) -> Result<String> {
        self.reset();
        
        let tokens = self.tokenize_source(source)?;
        self.format_tokens(&tokens)?;
        
        Ok(self.finalize_output())
    }

    /// Reset the formatter state
    fn reset(&mut self) {
        self.indentation_level = 0;
        self.line_buffer.clear();
        self.current_line.clear();
    }

    /// Tokenize the source code into manageable tokens
    fn tokenize_source(&self, source: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();
        let mut current_token = String::new();
        let mut line_number = 1;
        let mut column = 1;
        let mut in_string = false;
        let mut in_comment = false;
        let mut string_delimiter = '"';

        while let Some(ch) = chars.next() {
            match ch {
                '\n' => {
                    if in_comment {
                        in_comment = false;
                        if !current_token.is_empty() {
                            tokens.push(Token::new(TokenType::Comment, current_token.clone(), line_number, column));
                            current_token.clear();
                        }
                    } else if !current_token.is_empty() && !in_string {
                        tokens.push(self.classify_token(&current_token, line_number, column));
                        current_token.clear();
                    }
                    if in_string {
                        current_token.push(ch);
                    } else {
                        tokens.push(Token::new(TokenType::Newline, "\n".to_string(), line_number, column));
                    }
                    line_number += 1;
                    column = 1;
                },
                '"' | '\'' => {
                    if !in_comment {
                        if in_string && ch == string_delimiter {
                            current_token.push(ch);
                            tokens.push(Token::new(TokenType::String, current_token.clone(), line_number, column));
                            current_token.clear();
                            in_string = false;
                        } else if !in_string {
                            if !current_token.is_empty() {
                                tokens.push(self.classify_token(&current_token, line_number, column));
                                current_token.clear();
                            }
                            current_token.push(ch);
                            in_string = true;
                            string_delimiter = ch;
                        } else {
                            current_token.push(ch);
                        }
                    } else {
                        current_token.push(ch);
                    }
                    column += 1;
                },
                '/' if chars.peek() == Some(&'/') && !in_string => {
                    if !current_token.is_empty() {
                        tokens.push(self.classify_token(&current_token, line_number, column));
                        current_token.clear();
                    }
                    current_token.push(ch);
                    in_comment = true;
                    column += 1;
                },
                ' ' | '\t' | '\r' => {
                    if in_string || in_comment {
                        current_token.push(ch);
                    } else if !current_token.is_empty() {
                        tokens.push(self.classify_token(&current_token, line_number, column));
                        current_token.clear();
                    }
                    column += 1;
                },
                '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' | '=' | '+' | '-' | '*' | '/' | '%' | '!' | '<' | '>' | '&' | '|' | '^' | '?' | ':' => {
                    if in_string || in_comment {
                        current_token.push(ch);
                    } else {
                        if !current_token.is_empty() {
                            tokens.push(self.classify_token(&current_token, line_number, column));
                            current_token.clear();
                        }
                        tokens.push(self.classify_operator_or_delimiter(ch, line_number, column));
                    }
                    column += 1;
                },
                _ => {
                    current_token.push(ch);
                    column += 1;
                }
            }
        }

        if !current_token.is_empty() {
            if in_comment {
                tokens.push(Token::new(TokenType::Comment, current_token, line_number, column));
            } else {
                tokens.push(self.classify_token(&current_token, line_number, column));
            }
        }

        Ok(tokens)
    }

    /// Classify a token based on its content
    fn classify_token(&self, token: &str, line: usize, column: usize) -> Token {
        let token_type = match token {
            // Gen Z slang keywords
            "facts" => TokenType::FactsKeyword,
            "slay" => TokenType::SlayKeyword,
            "yeet" => TokenType::YeetKeyword,
            "yolo" => TokenType::YoloKeyword,
            "lowkey" => TokenType::LowkeyKeyword,
            "highkey" => TokenType::HighkeyKeyword,
            "stan" => TokenType::StanKeyword,
            "periodt" => TokenType::PerioddtKeyword,
            "bestie" => TokenType::BestieKeyword,
            "sus" => TokenType::SusKeyword,
            "cap" => TokenType::CapKeyword,
            "no_cap" => TokenType::NoCapKeyword,
            "vibez" => TokenType::VibezKeyword,
            "spill" => TokenType::SpillKeyword,
            "tea" => TokenType::TeaKeyword,
            "normie" => TokenType::NormieKeyword,
            
            // Control flow
            "if" => TokenType::IfKeyword,
            "else" => TokenType::ElseKeyword,
            "while" => TokenType::WhileKeyword,
            "for" => TokenType::ForKeyword,
            "match" => TokenType::MatchKeyword,
            "case" => TokenType::CaseKeyword,
            "default" => TokenType::DefaultKeyword,
            "break" => TokenType::BreakKeyword,
            "continue" => TokenType::ContinueKeyword,
            "return" => TokenType::ReturnKeyword,
            
            // Types
            "struct" => TokenType::StructKeyword,
            "enum" => TokenType::EnumKeyword,
            "trait" => TokenType::TraitKeyword,
            "impl" => TokenType::ImplKeyword,
            "type" => TokenType::TypeKeyword,
            
            // Modifiers
            "pub" => TokenType::PubKeyword,
            "mut" => TokenType::MutKeyword,
            "const" => TokenType::ConstKeyword,
            "static" => TokenType::StaticKeyword,
            
            _ => {
                if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    TokenType::Number
                } else if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    TokenType::Identifier
                } else {
                    TokenType::Unknown
                }
            }
        };

        Token::new(token_type, token.to_string(), line, column)
    }

    /// Classify operators and delimiters
    fn classify_operator_or_delimiter(&self, ch: char, line: usize, column: usize) -> Token {
        let token_type = match ch {
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '=' => TokenType::Assign,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Multiply,
            '/' => TokenType::Divide,
            '%' => TokenType::Modulo,
            '!' => TokenType::Not,
            '<' => TokenType::Less,
            '>' => TokenType::Greater,
            '&' => TokenType::And,
            '|' => TokenType::Or,
            '^' => TokenType::Xor,
            '?' => TokenType::Question,
            ':' => TokenType::Colon,
            _ => TokenType::Unknown,
        };

        Token::new(token_type, ch.to_string(), line, column)
    }

    /// Format the tokenized source code
    fn format_tokens(&mut self, tokens: &[Token]) -> Result<()> {
        let mut i = 0;
        while i < tokens.len() {
            i = self.format_token_at_index(tokens, i)?;
        }
        Ok(())
    }

    /// Format a single token at the given index
    fn format_token_at_index(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        let token = &tokens[index];
        
        match &token.token_type {
            TokenType::FactsKeyword => {
                self.format_facts_declaration(tokens, index)
            },
            TokenType::SlayKeyword => {
                self.format_slay_function(tokens, index)
            },
            TokenType::YeetKeyword => {
                self.format_yeet_import(tokens, index)
            },
            TokenType::LowkeyKeyword => {
                self.format_lowkey_if(tokens, index)
            },
            TokenType::PerioddtKeyword => {
                self.format_periodt_while(tokens, index)
            },
            TokenType::BestieKeyword => {
                self.format_bestie_for(tokens, index)
            },
            TokenType::StanKeyword => {
                self.format_stan_goroutine(tokens, index)
            },
            TokenType::LeftBrace => {
                self.format_opening_brace(tokens, index)
            },
            TokenType::RightBrace => {
                self.format_closing_brace(tokens, index)
            },
            TokenType::Newline => {
                self.format_newline(tokens, index)
            },
            TokenType::Comment => {
                self.format_comment(tokens, index)
            },
            _ => {
                self.add_token_to_current_line(&token.value);
                Ok(index + 1)
            }
        }
    }

    /// Format a facts (constant) declaration
    fn format_facts_declaration(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("facts");
        let mut next_index = index + 1;
        
        // Add space after facts
        if self.config.options.spaces_around_operators {
            self.add_token_to_current_line(" ");
        }
        
        // Handle identifier
        if next_index < tokens.len() {
            self.add_token_to_current_line(&tokens[next_index].value);
            next_index += 1;
        }
        
        // Handle assignment
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::Assign => {
                    if self.config.options.spaces_around_operators {
                        self.add_token_to_current_line(" = ");
                    } else {
                        self.add_token_to_current_line("=");
                    }
                    next_index += 1;
                    break;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        // Handle value until semicolon or newline
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::Semicolon => {
                    self.add_token_to_current_line(";");
                    next_index += 1;
                    break;
                },
                TokenType::Newline => {
                    break;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a slay (function) declaration
    fn format_slay_function(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        // Add blank line before function if configured
        if self.config.options.blank_lines.before_functions > 0 && !self.line_buffer.is_empty() {
            for _ in 0..self.config.options.blank_lines.before_functions {
                self.commit_current_line();
            }
        }
        
        self.add_token_to_current_line("slay");
        let mut next_index = index + 1;
        
        // Add space after slay
        self.add_token_to_current_line(" ");
        
        // Handle function name and parameters
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::LeftBrace => {
                    break;
                },
                TokenType::LeftParen => {
                    self.add_token_to_current_line("(");
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    next_index += 1;
                },
                TokenType::RightParen => {
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    self.add_token_to_current_line(")");
                    next_index += 1;
                },
                TokenType::Comma => {
                    self.add_token_to_current_line(",");
                    if self.config.options.spaces_after_commas {
                        self.add_token_to_current_line(" ");
                    }
                    next_index += 1;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a yeet (import) statement
    fn format_yeet_import(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("yeet");
        let mut next_index = index + 1;
        
        // Add space after yeet
        self.add_token_to_current_line(" ");
        
        // Handle import path until semicolon or newline
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::Semicolon => {
                    self.add_token_to_current_line(";");
                    next_index += 1;
                    break;
                },
                TokenType::Newline => {
                    break;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a lowkey (if) statement
    fn format_lowkey_if(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("lowkey");
        let mut next_index = index + 1;
        
        // Add space after lowkey
        self.add_token_to_current_line(" ");
        
        // Handle condition until opening brace
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::LeftBrace => {
                    break;
                },
                TokenType::LeftParen => {
                    self.add_token_to_current_line("(");
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    next_index += 1;
                },
                TokenType::RightParen => {
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    self.add_token_to_current_line(")");
                    next_index += 1;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a periodt (while) loop
    fn format_periodt_while(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("periodt");
        let mut next_index = index + 1;
        
        // Add space after periodt
        self.add_token_to_current_line(" ");
        
        // Handle condition until opening brace
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::LeftBrace => {
                    break;
                },
                TokenType::LeftParen => {
                    self.add_token_to_current_line("(");
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    next_index += 1;
                },
                TokenType::RightParen => {
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    self.add_token_to_current_line(")");
                    next_index += 1;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a bestie (for) loop
    fn format_bestie_for(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("bestie");
        let mut next_index = index + 1;
        
        // Add space after bestie
        self.add_token_to_current_line(" ");
        
        // Handle loop parameters until opening brace
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::LeftBrace => {
                    break;
                },
                TokenType::LeftParen => {
                    self.add_token_to_current_line("(");
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    next_index += 1;
                },
                TokenType::RightParen => {
                    if self.config.options.spaces_inside_parentheses {
                        self.add_token_to_current_line(" ");
                    }
                    self.add_token_to_current_line(")");
                    next_index += 1;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format a stan (goroutine) statement
    fn format_stan_goroutine(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        self.add_token_to_current_line("stan");
        let mut next_index = index + 1;
        
        // Add space after stan
        self.add_token_to_current_line(" ");
        
        // Handle function call until semicolon or newline
        while next_index < tokens.len() {
            let token = &tokens[next_index];
            match token.token_type {
                TokenType::Semicolon => {
                    self.add_token_to_current_line(";");
                    next_index += 1;
                    break;
                },
                TokenType::Newline => {
                    break;
                },
                _ => {
                    self.add_token_to_current_line(&token.value);
                    next_index += 1;
                }
            }
        }
        
        Ok(next_index)
    }

    /// Format an opening brace
    fn format_opening_brace(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        match self.config.options.brace_style {
            BraceStyle::SameLine => {
                if self.config.options.spaces_inside_braces {
                    self.add_token_to_current_line(" {");
                } else {
                    self.add_token_to_current_line("{");
                }
                self.commit_current_line();
                self.indentation_level += 1;
            },
            BraceStyle::NextLine => {
                self.commit_current_line();
                self.add_indentation();
                self.add_token_to_current_line("{");
                self.commit_current_line();
                self.indentation_level += 1;
            },
            BraceStyle::NextLineIndented => {
                self.commit_current_line();
                self.indentation_level += 1;
                self.add_indentation();
                self.add_token_to_current_line("{");
                self.commit_current_line();
            },
        }
        
        Ok(index + 1)
    }

    /// Format a closing brace
    fn format_closing_brace(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
        
        self.commit_current_line();
        self.add_indentation();
        self.add_token_to_current_line("}");
        self.commit_current_line();
        
        Ok(index + 1)
    }

    /// Format a newline
    fn format_newline(&mut self, _tokens: &[Token], index: usize) -> Result<usize> {
        self.commit_current_line();
        Ok(index + 1)
    }

    /// Format a comment
    fn format_comment(&mut self, tokens: &[Token], index: usize) -> Result<usize> {
        if self.config.preserve_comments {
            let token = &tokens[index];
            self.add_token_to_current_line(&token.value);
        }
        Ok(index + 1)
    }

    /// Add text to the current line
    fn add_token_to_current_line(&mut self, text: &str) {
        self.current_line.push_str(text);
    }

    /// Add proper indentation to the current line
    fn add_indentation(&mut self) {
        let indent = match self.config.options.indent_style {
            IndentStyle::Spaces => " ".repeat(self.config.options.indent_size * self.indentation_level),
            IndentStyle::Tabs => "\t".repeat(self.indentation_level),
        };
        self.current_line.push_str(&indent);
    }

    /// Commit the current line to the buffer
    fn commit_current_line(&mut self) {
        if !self.current_line.is_empty() || self.line_buffer.is_empty() {
            self.line_buffer.push(self.current_line.clone());
        }
        self.current_line.clear();
        
        // Add indentation for the next line if we're inside a block
        if self.indentation_level > 0 {
            self.add_indentation();
        }
    }

    /// Finalize the output and return the formatted string
    fn finalize_output(&mut self) -> String {
        if !self.current_line.is_empty() {
            self.commit_current_line();
        }
        
        // Remove excessive blank lines
        let mut result = Vec::new();
        let mut blank_line_count = 0;
        
        for line in &self.line_buffer {
            if line.trim().is_empty() {
                blank_line_count += 1;
                if blank_line_count <= self.config.options.blank_lines.max_blank_lines {
                    result.push(line.clone());
                }
            } else {
                blank_line_count = 0;
                result.push(line.clone());
            }
        }
        
        result.join("\n")
    }
}

impl Default for CursedFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Token representation for the formatter
#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
}

impl Token {
    fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
}

/// Token types for CURSED language
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // Gen Z slang keywords
    FactsKeyword,       // facts (constant)
    SlayKeyword,        // slay (function)
    YeetKeyword,        // yeet (import)
    YoloKeyword,        // yolo (return/print)
    LowkeyKeyword,      // lowkey (if)
    HighkeyKeyword,     // highkey (else)
    StanKeyword,        // stan (goroutine)
    PerioddtKeyword,    // periodt (while)
    BestieKeyword,      // bestie (for)
    SusKeyword,         // sus (mutable variable)
    CapKeyword,         // cap (boolean true)
    NoCapKeyword,       // no_cap (boolean false)
    VibezKeyword,       // vibez (utilities)
    SpillKeyword,       // spill (print/output)
    TeaKeyword,         // tea (string type)
    NormieKeyword,      // normie (integer type)
    
    // Traditional keywords
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    ForKeyword,
    MatchKeyword,
    CaseKeyword,
    DefaultKeyword,
    BreakKeyword,
    ContinueKeyword,
    ReturnKeyword,
    StructKeyword,
    EnumKeyword,
    TraitKeyword,
    ImplKeyword,
    TypeKeyword,
    PubKeyword,
    MutKeyword,
    ConstKeyword,
    StaticKeyword,
    
    // Literals
    String,
    Number,
    Identifier,
    
    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Not,
    Less,
    Greater,
    And,
    Or,
    Xor,
    Question,
    
    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Colon,
    
    // Special
    Newline,
    Comment,
    Unknown,
}

/// Convenience functions for creating formatters with common configurations
impl CursedFormatter {
    /// Create a formatter with compact style (minimal spacing)
    pub fn compact() -> Self {
        let mut config = FormatterConfig::default();
        config.options.spaces_around_operators = false;
        config.options.spaces_after_commas = false;
        config.options.spaces_inside_parentheses = false;
        config.options.spaces_inside_braces = false;
        config.options.blank_lines.max_blank_lines = 1;
        config.options.blank_lines.before_functions = 0;
        config.options.blank_lines.after_imports = 0;
        
        Self::with_config(config)
    }
    
    /// Create a formatter with verbose style (lots of spacing)
    pub fn verbose() -> Self {
        let mut config = FormatterConfig::default();
        config.options.spaces_around_operators = true;
        config.options.spaces_after_commas = true;
        config.options.spaces_inside_parentheses = true;
        config.options.spaces_inside_braces = true;
        config.options.blank_lines.max_blank_lines = 3;
        config.options.blank_lines.before_functions = 2;
        config.options.blank_lines.after_imports = 2;
        
        Self::with_config(config)
    }
    
    /// Create a formatter with tab indentation
    pub fn with_tabs() -> Self {
        let mut config = FormatterConfig::default();
        config.options.indent_style = IndentStyle::Tabs;
        
        Self::with_config(config)
    }
    
    /// Create a formatter with custom indentation size
    pub fn with_indent_size(size: usize) -> Self {
        let mut config = FormatterConfig::default();
        config.options.indent_size = size;
        
        Self::with_config(config)
    }
}

/// Public API functions
pub fn format_cursed_code(source: &str) -> Result<String> {
    let mut formatter = CursedFormatter::new();
    formatter.format(source)
}

pub fn format_cursed_code_with_config(source: &str, config: FormatterConfig) -> Result<String> {
    let mut formatter = CursedFormatter::with_config(config);
    formatter.format(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_facts_declaration() {
        let source = "facts   x=42;";
        let expected = "facts x = 42;";
        
        let mut formatter = CursedFormatter::new();
        let result = formatter.format(source).unwrap();
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_slay_function() {
        let source = "slay  main(){yolo \"Hello\"}";
        let expected = "slay main() {\n    yolo \"Hello\"\n}";
        
        let mut formatter = CursedFormatter::new();
        let result = formatter.format(source).unwrap();
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_yeet_import() {
        let source = "yeet\"std::io\";";
        let expected = "yeet \"std::io\";";
        
        let mut formatter = CursedFormatter::new();
        let result = formatter.format(source).unwrap();
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_compact_style() {
        let source = "facts x = 42; slay main() { yolo x; }";
        
        let mut formatter = CursedFormatter::compact();
        let result = formatter.format(source).unwrap();
        
        // Compact style should have minimal spacing
        assert!(!result.contains(" = "));
        assert!(!result.contains(" { "));
    }

    #[test]
    fn test_format_with_tabs() {
        let source = "slay main() { yolo \"test\"; }";
        
        let mut formatter = CursedFormatter::with_tabs();
        let result = formatter.format(source).unwrap();
        
        // Should contain tab characters for indentation
        assert!(result.contains('\t'));
    }
}
