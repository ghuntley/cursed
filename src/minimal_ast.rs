//! Minimal AST for CURSED - just enough to parse basic programs

/// Identifier module for compatibility
pub mod identifiers {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Identifier {
    impl Identifier {
        pub fn new(name: String, value: String) -> Self {
            Self { name, value }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
#[derive(Debug, Clone)]
pub enum Statement {
    Facts(String, Expression), // facts x = expression
    Slay(String, Vec<String>, Vec<Statement>), // slay function(args) { body }
#[derive(Debug, Clone)]
pub enum Expression {
}
