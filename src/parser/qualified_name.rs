//! Parser for qualified names (package.symbol)
//!
//! This module extends the parser to recognize and parse qualified names,
//! distinguishing them from regular dot expressions and providing proper
//! symbol kind inference where possible.

use crate::ast::expressions::{QualifiedName, QualifiedSymbolKind};
use crate::ast::Expression;
use crate::error::Error;
use crate::lexer::Token;
use crate::parser::Parser;

impl Parser {
    /// Parse a qualified name expression (package.symbol)
    /// This is called when we encounter an identifier followed by a dot
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn parse_qualified_name(&mut self, package_name: String) -> Result<Box<dyn Expression>, Error> {
        tracing::debug!(package = package_name, "Parsing qualified name");
        
        // Expect dot
        if !self.expect_peek(&Token::Dot)? {
            return Err(Error::from_str(&format!(
                "Expected '.' after package name '{}', got {:?}",
                package_name, self.peek_token
            )));
        }
        
        // Advance past the dot
        self.next_token()?;
        
        // Expect symbol name
        let symbol_name = match &self.peek_token {
            Token::Ident(name) => name.clone(),
            _ => return Err(Error::from_str(&format!(
                "Expected identifier after '{}.', got {:?}",
                package_name, self.peek_token
            ))),
        };
        
        // Advance past the symbol name
        self.next_token()?;
        
        // Infer symbol kind based on context
        let symbol_kind = self.infer_qualified_symbol_kind(&package_name, &symbol_name);
        
        let qualified = QualifiedName::new_with_kind(
            ".".to_string(),
            package_name,
            symbol_name,
            symbol_kind
        );
        
        tracing::debug!(
            package = qualified.package,
            symbol = qualified.symbol,
            kind = ?qualified.symbol_kind,
            "Parsed qualified name"
        );
        
        Ok(Box::new(qualified))
    }
    
    /// Infer the symbol kind based on context clues
    fn infer_qualified_symbol_kind(&self, package: &str, symbol: &str) -> QualifiedSymbolKind {
        // Look at the next token to infer the symbol kind
        match &self.peek_token {
            // If followed by '(', it's likely a function call
            Token::LeftParen => QualifiedSymbolKind::Function,
            
            // If followed by '{', it's likely a struct type
            Token::LeftBrace => QualifiedSymbolKind::Type,
            
            // Use naming conventions to infer
            _ => {
                // Check for common patterns
                if symbol.chars().next().unwrap_or('a').is_uppercase() {
                    // Uppercase names are typically types or constants
                    if symbol.chars().all(|c| c.is_uppercase() || c == '_') {
                        QualifiedSymbolKind::Constant // ALL_CAPS suggests constant
                    } else {
                        QualifiedSymbolKind::Type // PascalCase suggests type
                    }
                } else {
                    // Lowercase names are typically functions or variables
                    QualifiedSymbolKind::Unknown // Let runtime resolution decide
                }
            }
        }
    }
    
    /// Check if the current identifier could be a package name
    pub fn could_be_package_name(&self, name: &str) -> bool {
        // Check against known package names
        matches!(name, 
            "vibez" | "htmlrizzler" | "mathz" | "timez" | "reflectz" | 
            "cryptz" | "stringz" | "dropz" | "concurrenz" | "web_vibez" | 
            "rizztemplate" | "json_tea" | "oglogging" | "regex_vibez" |
            "vibe_life" | "core" | "math" | "http" | "strings" | "time" |
            "os" | "fmt" | "log" | "net" | "io" | "sync"
        ) || name.ends_with("z") // CURSED packages often end with 'z'
    }
    
    /// Parse a potential qualified name or dot expression
    /// This determines whether identifier.something is a qualified name or regular dot expression
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn parse_qualified_or_dot(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        // Check if the left side is an identifier that could be a package name
        if let Some(identifier) = left.as_any().downcast_ref::<crate::ast::expressions::Identifier>() {
            if self.could_be_package_name(&identifier.value) {
                tracing::debug!(identifier = identifier.value, "Treating as qualified name");
                return self.parse_qualified_name(identifier.value.clone());
            }
        }
        
        // Fall back to regular dot expression parsing
        tracing::debug!("Treating as regular dot expression");
        self.parse_dot_expression(left)
    }
    
    /// Parse a regular dot expression (for backward compatibility)
    fn parse_dot_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        // Expect dot
        if !self.expect_peek(&Token::Dot)? {
            return Err(Error::from_str("Expected '.' in dot expression"));
        }
        
        // Advance past the dot
        self.next_token()?;
        
        // Expect property name
        let property = match &self.peek_token {
            Token::Ident(name) => name.clone(),
            _ => return Err(Error::from_str(&format!(
                "Expected identifier after '.', got {:?}",
                self.peek_token
            ))),
        };
        
        // Advance past the property name
        self.next_token()?;
        
        let dot_expr = crate::ast::expressions::DotExpression {
            token: ".".to_string(),
            object: left,
            property,
        };
        
        Ok(Box::new(dot_expr))
    }
}

/// Extension methods for the Parser to support qualified name parsing
pub trait QualifiedNameParsing {
    /// Parse a qualified name with explicit package and symbol names
    fn parse_explicit_qualified_name(&mut self, package: &str, symbol: &str) -> Result<QualifiedName, Error>;
    
    /// Check if a token sequence represents a qualified name
    fn is_qualified_name_sequence(&self) -> bool;
}

impl QualifiedNameParsing for Parser {
    fn parse_explicit_qualified_name(&mut self, package: &str, symbol: &str) -> Result<QualifiedName, Error> {
        let symbol_kind = self.infer_qualified_symbol_kind(package, symbol);
        
        Ok(QualifiedName::new_with_kind(
            ".".to_string(),
            package.to_string(),
            symbol.to_string(),
            symbol_kind
        ))
    }
    
    fn is_qualified_name_sequence(&self) -> bool {
        // Check if current token is identifier, next is dot, and the one after is identifier
        if let Token::Ident(name) = &self.current_token {
            if self.could_be_package_name(name) {
                if let Token::Dot = &self.peek_token {
                    // Would need to look ahead one more token, but that's complex
                    // For now, assume it could be
                    return true;
                }
            }
        }
        false
    }
}
