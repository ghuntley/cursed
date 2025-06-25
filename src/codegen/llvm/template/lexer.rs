// Template lexer for LLVM code generation
use crate::error_types::CursedError;

/// Template lexer for processing LLVM code templates
#[derive(Debug)]
pub struct TemplateLexer {
impl TemplateLexer {
    pub fn new(source: String) -> Self {
        Self {
        }
    }

    pub fn tokenize(&mut self) -> crate::error_types::Result<Vec<TemplateToken>> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            let token = self.next_token()?;
            tokens.push(token);
        tokens.push(TemplateToken::Eof);
        Ok(tokens)
    fn next_token(&mut self) -> crate::error_types::Result<TemplateToken> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(TemplateToken::Eof);
        let start = self.position;
        let ch = self.advance();

        match ch {
            '{' if self.peek() == Some('{') => {
                self.advance(); // consume second '{'
                Ok(TemplateToken::OpenExpression)
            }
            '}' if self.peek() == Some('}') => {
                self.advance(); // consume second '}'
                Ok(TemplateToken::CloseExpression)
            }
            '{' if self.peek() == Some('%') => {
                self.advance(); // consume '%'
                Ok(TemplateToken::OpenStatement)
            }
            '%' if self.peek() == Some('}') => {
                self.advance(); // consume '}'
                Ok(TemplateToken::CloseStatement)
            }
            _ => {
                let text = self.source[start..self.position].to_string();
                Ok(TemplateToken::Text(text))
            }
        }
    fn string_literal(&mut self) -> crate::error_types::Result<TemplateToken> {
        let mut value = String::new();
        
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // consume closing quote
                break;
            if ch == '\\' {
                self.advance(); // consume backslash
                if let Some(escaped) = self.peek() {
                    match escaped {
                        _ => {
                            value.push('\\');
                            value.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }
        
        Ok(TemplateToken::String(value))
    fn identifier(&mut self) -> crate::error_types::Result<TemplateToken> {
        let start = self.position - 1;
        
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let text = self.source[start..self.position].to_string();
        
        // Check for keywords
        let token = match text.as_str() {
        
        Ok(token)
    fn number(&mut self) -> crate::error_types::Result<TemplateToken> {
        let start = self.position - 1;
        
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        
        let text = self.source[start..self.position].to_string();
        let value = text.parse::<i64>()
            .map_err(|_| CursedError::Parse(format!("Invalid number: {}", text)))?;
        
        Ok(TemplateToken::Number(value))
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.position).unwrap_or('\0');
        self.position += 1;
        self.column += 1;
        ch
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

/// Template token types
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateToken {
    // Delimiters
    OpenExpression,  // {{
    CloseExpression, // }}
    OpenStatement,   // {%
    CloseStatement,  // %}

    // Keywords

    // Literals

    // Special
impl std::fmt::Display for TemplateToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
}
