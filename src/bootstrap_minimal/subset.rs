//! Bootstrap subset definition for the CURSED language
//!
//! This module defines what language features are included in the minimal
//! bootstrap subset that can be used for self-hosting compilation.

use std::collections::HashSet;
use crate::lexer::Token;
use crate::ast::traits::{Expression, Statement};

/// Defines the minimal bootstrap subset of CURSED language features
pub struct BootstrapSubset {
    /// Allowed tokens in the bootstrap subset
    allowed_tokens: HashSet<std::mem::Discriminant<Token>>,
    /// Allowed expression types
    allowed_expressions: HashSet<String>,
    /// Allowed statement types
    allowed_statements: HashSet<String>,
}

impl Default for BootstrapSubset {
    fn default() -> Self {
        Self::new()
    }
}

impl BootstrapSubset {
    /// Creates a new bootstrap subset definition with the minimal required features
    pub fn new() -> Self {
        let mut subset = BootstrapSubset {
            allowed_tokens: HashSet::new(),
            allowed_expressions: HashSet::new(),
            allowed_statements: HashSet::new(),
        };
        
        subset.initialize_allowed_tokens();
        subset.initialize_allowed_expressions();
        subset.initialize_allowed_statements();
        
        subset
    }
    
    /// Initialize the set of allowed tokens for the bootstrap subset
    fn initialize_allowed_tokens(&mut self) {
        // Use discriminant to compare token types without values
        let allowed = vec![
            // Basic tokens
            std::mem::discriminant(&Token::Eof),
            std::mem::discriminant(&Token::Identifier(String::new())),
            std::mem::discriminant(&Token::String(String::new())),
            std::mem::discriminant(&Token::Int(0)),
            std::mem::discriminant(&Token::Float(0.0)),
            
            // Basic operators
            std::mem::discriminant(&Token::Assign),
            std::mem::discriminant(&Token::Plus),
            std::mem::discriminant(&Token::Minus),
            std::mem::discriminant(&Token::Asterisk),
            std::mem::discriminant(&Token::Slash),
            std::mem::discriminant(&Token::Percent),
            std::mem::discriminant(&Token::Lt),
            std::mem::discriminant(&Token::Gt),
            std::mem::discriminant(&Token::Eq),
            std::mem::discriminant(&Token::NotEq),
            std::mem::discriminant(&Token::LtEq),
            std::mem::discriminant(&Token::GtEq),
            std::mem::discriminant(&Token::And),
            std::mem::discriminant(&Token::Or),
            std::mem::discriminant(&Token::Bang),
            
            // Delimiters
            std::mem::discriminant(&Token::Comma),
            std::mem::discriminant(&Token::Semicolon),
            std::mem::discriminant(&Token::Colon),
            std::mem::discriminant(&Token::LParen),
            std::mem::discriminant(&Token::RParen),
            std::mem::discriminant(&Token::LBrace),
            std::mem::discriminant(&Token::RBrace),
            std::mem::discriminant(&Token::LBracket),
            std::mem::discriminant(&Token::RBracket),
            std::mem::discriminant(&Token::Dot),
            
            // Essential keywords
            std::mem::discriminant(&Token::Vibe),      // package
            std::mem::discriminant(&Token::Yeet),      // import
            std::mem::discriminant(&Token::Slay),      // func
            std::mem::discriminant(&Token::Sus),       // var
            std::mem::discriminant(&Token::Facts),     // const
            std::mem::discriminant(&Token::Lowkey),    // if
            std::mem::discriminant(&Token::Highkey),   // else
            std::mem::discriminant(&Token::Bestie),    // for
            std::mem::discriminant(&Token::Yolo),      // return
            std::mem::discriminant(&Token::Based),     // true
            std::mem::discriminant(&Token::Cap),       // nil
            std::mem::discriminant(&Token::Ghosted),   // break
            std::mem::discriminant(&Token::Simp),      // continue
            
            // Basic types
            std::mem::discriminant(&Token::Normie),    // int32
            std::mem::discriminant(&Token::Thicc),     // int64
            std::mem::discriminant(&Token::Lit),       // bool
            std::mem::discriminant(&Token::Snack),     // float32
            std::mem::discriminant(&Token::Meal),      // float64
            
            // Assignment operators
            std::mem::discriminant(&Token::DeclAssign), // :=
            std::mem::discriminant(&Token::PlusAssign),
            std::mem::discriminant(&Token::MinusAssign),
            std::mem::discriminant(&Token::AsteriskAssign),
            std::mem::discriminant(&Token::SlashAssign),
            
            // Comments
            std::mem::discriminant(&Token::LineComment),
        ];
        
        self.allowed_tokens.extend(allowed);
    }
    
    /// Initialize the set of allowed expression types
    fn initialize_allowed_expressions(&mut self) {
        let allowed = vec![
            "IntegerLiteral",
            "FloatLiteral", 
            "StringLiteral",
            "BooleanLiteral",
            "Identifier",
            "InfixExpression",
            "PrefixExpression", 
            "CallExpression",
            "ParenthesizedExpression",
            "ArrayLiteral",      // Basic array literals only
            "IndexExpression",   // Array/slice indexing
            "DotExpression",     // For standard library access
        ];
        
        self.allowed_expressions.extend(allowed.into_iter().map(String::from));
    }
    
    /// Initialize the set of allowed statement types
    fn initialize_allowed_statements(&mut self) {
        let allowed = vec![
            "PackageStatement",
            "ImportStatement", 
            "VarStatement",
            "ConstStatement",
            "FunctionStatement",
            "ExpressionStatement",
            "BlockStatement",
            "IfStatement",
            "ForStatement",
            "ReturnStatement",
            "BreakStatement",
            "ContinueStatement",
            "AssignmentStatement",
        ];
        
        self.allowed_statements.extend(allowed.into_iter().map(String::from));
    }
    
    /// Check if a token is allowed in the bootstrap subset
    pub fn is_token_allowed(&self, token: &Token) -> bool {
        let token_discriminant = std::mem::discriminant(token);
        self.allowed_tokens.contains(&token_discriminant)
    }
    
    /// Check if an expression type is allowed in the bootstrap subset
    pub fn is_expression_allowed(&self, expr_type: &str) -> bool {
        self.allowed_expressions.contains(expr_type)
    }
    
    /// Check if a statement type is allowed in the bootstrap subset
    pub fn is_statement_allowed(&self, stmt_type: &str) -> bool {
        self.allowed_statements.contains(stmt_type)
    }
    
    /// Get a list of all allowed tokens for documentation/debugging
    pub fn get_allowed_tokens(&self) -> Vec<String> {
        // This is a simplified representation - in practice we'd need more sophisticated token listing
        vec![
            "Identifiers".to_string(), "Strings".to_string(), "Integers".to_string(), "Floats".to_string(), "Booleans".to_string(),
            "Basic operators (+, -, *, /, %, ==, !=, <, >, <=, >=, &&, ||, !)".to_string(),
            "Assignment operators (=, :=, +=, -=, *=, /=)".to_string(),
            "Delimiters (, ), {, }, [, ], ., ,, ;, :)".to_string(),
            "Keywords (vibe, yeet, slay, sus, facts, lowkey, highkey, bestie, yolo)".to_string(),
            "Types (normie, thicc, lit, snack, meal)".to_string(),
            "Literals (based, cap)".to_string(),
            "Control flow (ghosted, simp)".to_string(),
        ]
    }
    
    /// Get a list of all allowed expression types
    pub fn get_allowed_expressions(&self) -> Vec<String> {
        self.allowed_expressions.iter().cloned().collect()
    }
    
    /// Get a list of all allowed statement types  
    pub fn get_allowed_statements(&self) -> Vec<String> {
        self.allowed_statements.iter().cloned().collect()
    }
    
    /// Check if the subset includes essential standard library access
    pub fn allows_stdlib_access(&self) -> bool {
        // Must allow dot expressions for vibez.spill, mathz.add, etc.
        self.is_expression_allowed("DotExpression") && 
        self.is_expression_allowed("CallExpression")
    }
    
    /// Check if the subset supports basic control flow
    pub fn supports_control_flow(&self) -> bool {
        self.is_statement_allowed("IfStatement") &&
        self.is_statement_allowed("ForStatement") &&
        self.is_statement_allowed("BlockStatement")
    }
    
    /// Check if the subset supports function definitions
    pub fn supports_functions(&self) -> bool {
        self.is_statement_allowed("FunctionStatement") &&
        self.is_expression_allowed("CallExpression")
    }
    
    /// Check if the subset supports variable declarations  
    pub fn supports_variables(&self) -> bool {
        self.is_statement_allowed("VarStatement") &&
        self.is_statement_allowed("AssignmentStatement")
    }
    
    /// Validate that the subset is self-consistent and complete
    pub fn validate_subset(&self) -> Result<(), String> {
        if !self.allows_stdlib_access() {
            return Err("Bootstrap subset must allow standard library access".to_string());
        }
        
        if !self.supports_control_flow() {
            return Err("Bootstrap subset must support basic control flow".to_string());
        }
        
        if !self.supports_functions() {
            return Err("Bootstrap subset must support function definitions and calls".to_string());
        }
        
        if !self.supports_variables() {
            return Err("Bootstrap subset must support variable declarations and assignments".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_subset_creation() {
        let subset = BootstrapSubset::new();
        assert!(subset.validate_subset().is_ok());
    }
    
    #[test]
    fn test_essential_tokens_allowed() {
        let subset = BootstrapSubset::new();
        
        // Test essential keywords
        assert!(subset.is_token_allowed(&Token::Vibe));      // package
        assert!(subset.is_token_allowed(&Token::Slay));      // func  
        assert!(subset.is_token_allowed(&Token::Sus));       // var
        assert!(subset.is_token_allowed(&Token::Lowkey));    // if
        assert!(subset.is_token_allowed(&Token::Bestie));    // for
        
        // Test basic types
        assert!(subset.is_token_allowed(&Token::Normie));    // int32
        assert!(subset.is_token_allowed(&Token::Lit));       // bool
        
        // Test operators
        assert!(subset.is_token_allowed(&Token::Plus));
        assert!(subset.is_token_allowed(&Token::Assign));
    }
    
    #[test]
    fn test_advanced_features_excluded() {
        let subset = BootstrapSubset::new();
        
        // Advanced keywords should not be allowed
        assert!(!subset.is_token_allowed(&Token::Squad));     // struct
        assert!(!subset.is_token_allowed(&Token::Collab));    // interface
        assert!(!subset.is_token_allowed(&Token::Dm));        // chan
        assert!(!subset.is_token_allowed(&Token::Stan));      // go
        assert!(!subset.is_token_allowed(&Token::VibeCheck)); // switch
        assert!(!subset.is_token_allowed(&Token::Choose));    // select
    }
    
    #[test]
    fn test_expression_types() {
        let subset = BootstrapSubset::new();
        
        // Basic expressions should be allowed
        assert!(subset.is_expression_allowed("IntegerLiteral"));
        assert!(subset.is_expression_allowed("CallExpression"));
        assert!(subset.is_expression_allowed("DotExpression"));
        
        // Advanced expressions should not be allowed by default
        assert!(!subset.is_expression_allowed("StructLiteral"));
        assert!(!subset.is_expression_allowed("ChannelExpression"));
    }
    
    #[test]
    fn test_statement_types() {
        let subset = BootstrapSubset::new();
        
        // Basic statements should be allowed
        assert!(subset.is_statement_allowed("FunctionStatement"));
        assert!(subset.is_statement_allowed("IfStatement"));
        assert!(subset.is_statement_allowed("ForStatement"));
        
        // Advanced statements should not be allowed
        assert!(!subset.is_statement_allowed("SwitchStatement"));
        assert!(subset.is_statement_allowed("SelectStatement") == false);
    }
    
    #[test]
    fn test_subset_capabilities() {
        let subset = BootstrapSubset::new();
        
        assert!(subset.allows_stdlib_access());
        assert!(subset.supports_control_flow());
        assert!(subset.supports_functions());
        assert!(subset.supports_variables());
    }
}
