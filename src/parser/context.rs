//! Context-aware parsing framework for the CURSED language parser.
//!
//! This module provides a mechanism for tracking parsing context, which helps
//! the parser to disambiguate between similar syntactic constructs based on
//! the current context (e.g., block statements vs. hash literals).

/// Parsing context types to track what kind of construct we're currently parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingContext {
    /// Parsing a statement
    Statement,
    /// Parsing an expression
    Expression,
    /// Parsing a block statement
    BlockStatement,
    /// Parsing a hash literal
    HashLiteral,
    /// Parsing a switch statement
    SwitchStatement,
    /// Parsing the body of a switch statement
    SwitchBody,
    /// Parsing a case clause in a switch statement
    CaseClause,
    /// Parsing the default clause in a switch statement
    DefaultClause,
    /// Parsing a function declaration
    FunctionDeclaration,
    /// Parsing function parameters
    FunctionParameters,
    /// Parsing a for loop
    ForLoop,
    /// Parsing a while loop
    WhileLoop,
    /// Parsing an if statement
    IfStatement,
    /// Parsing an else clause
    ElseClause,
    /// Parsing a struct declaration
    StructDeclaration,
    /// Parsing a struct field
    StructField,
    /// Parsing a type parameter list
    TypeParameters,
    /// Parsing a type instantiation
    TypeInstantiation,
    /// Parsing a generic function call
    GenericFunctionCall,
}

/// Trait for providing context-aware parsing capabilities
pub trait ContextAwareParsing {
    /// Push a new context onto the context stack
    fn push_context(&mut self, context: ParsingContext);
    
    /// Pop the most recent context from the context stack
    fn pop_context(&mut self) -> Option<ParsingContext>;
    
    /// Get the current parsing context
    fn current_context(&self) -> Option<&ParsingContext>;
    
    /// Check if we're currently in a specific context
    fn in_context(&self, context: ParsingContext) -> bool;
    
    /// Check if we're in any of the specified contexts
    fn in_any_context(&self, contexts: &[ParsingContext]) -> bool;
    
    /// Check if the current token is in a particular context
    fn current_token_is_in_context(&self, token_predicate: fn(&Self) -> bool, context: ParsingContext) -> bool;
}