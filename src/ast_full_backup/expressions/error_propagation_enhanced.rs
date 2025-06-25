
use std::any::Any;
use crate::ast::traits::{Expression, Node};
use crate::error::{CursedError, SourceLocation};
use std::fmt;

/// Enhanced error propagation AST node for the `?` operator
/// 
/// This AST node represents error propagation in CURSED using the `?` operator.
/// It provides automatic error handling by:
/// - Evaluating the inner expression
/// - If the result is an error, propagating it by returning early
/// - If the result is successful, unwrapping and continuing with the inner value
/// 
/// The `?` operator is essential for ergonomic error handling in CURSED,
/// following Rust's model but adapted for CURSED's type system.
#[derive(Debug, Clone)]
pub struct ErrorPropagation {
    /// The expression to evaluate and potentially propagate errors from
    pub expression: Box<dyn Expression>,
    
    /// Source location information for error reporting
    pub location: SourceLocation,
    
    /// Expected result type (for type checking)
    pub expected_type: Option<String>,
    
    /// Whether this propagation is in a tail position
    pub is_tail_position: bool,
}

impl ErrorPropagation {
    /// Create a new error propagation expression
    pub fn new(
        expression: Box<dyn Expression>, 
        location: SourceLocation,
    ) -> Self {
        Self {
            expression,
            location,
            expected_type: None,
            is_tail_position: false,
        }
    }
    
    /// Create error propagation with type information
    pub fn with_type(
        expression: Box<dyn Expression>,
        location: SourceLocation,
        expected_type: String,
    ) -> Self {
        Self {
            expression,
            location,
            expected_type: Some(expected_type),
            is_tail_position: false,
        }
    }
    
    /// Mark this propagation as being in tail position
    pub fn set_tail_position(mut self, is_tail: bool) -> Self {
        self.is_tail_position = is_tail;
        self
    }
    
    /// Get the inner expression being evaluated
    pub fn inner_expression(&self) -> &dyn Expression {
        &*self.expression
    }
    
    /// Get source location for error reporting
    pub fn get_location(&self) -> &SourceLocation {
        &self.location
    }
    
    /// Get expected type if available
    pub fn get_expected_type(&self) -> Option<&str> {
        self.expected_type.as_deref()
    }
    
    /// Check if this is a tail position propagation
    pub fn is_in_tail_position(&self) -> bool {
        self.is_tail_position
    }
}

impl Node for ErrorPropagation {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    }
    
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for ErrorPropagation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

// impl fmt::Display for ErrorPropagation {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}?", self.expression.string())
//     }
// }

/// Enhanced error propagation context for tracking propagation chains
#[derive(Debug, Clone)]
pub struct PropagationContext {
    /// Stack of propagation sites for error traces
    pub propagation_stack: Vec<SourceLocation>,
    
    /// Current function name for context
    pub current_function: Option<String>,
    
    /// Whether we're in a try block or similar error handling context
    pub in_error_context: bool,
    
    /// Expected return type for the current function
    pub expected_return_type: Option<String>,
}

impl PropagationContext {
    /// Create a new propagation context
    pub fn new() -> Self {
        Self {
            propagation_stack: Vec::new(),
            current_function: None,
            in_error_context: false,
            expected_return_type: None,
        }
    }
    
    /// Push a propagation site onto the stack
    pub fn push_propagation(&mut self, location: SourceLocation) {
        self.propagation_stack.push(location);
    }
    
    /// Pop a propagation site from the stack
    pub fn pop_propagation(&mut self) -> Option<SourceLocation> {
        self.propagation_stack.pop()
    }
    
    /// Set the current function context
    pub fn set_function(&mut self, function_name: String, return_type: Option<String>) {
        self.current_function = Some(function_name);
        self.expected_return_type = return_type;
    }
    
    /// Enter an error handling context
    pub fn enter_error_context(&mut self) {
        self.in_error_context = true;
    }
    
    /// Exit an error handling context
    pub fn exit_error_context(&mut self) {
        self.in_error_context = false;
    }
    
    /// Get the current propagation depth
    pub fn propagation_depth(&self) -> usize {
        self.propagation_stack.len()
    }
    
    /// Get the stack trace for error reporting
    pub fn get_stack_trace(&self) -> Vec<SourceLocation> {
        self.propagation_stack.clone()
    }
}

impl Default for PropagationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// CursedError propagation validation helper
pub struct PropagationValidator;

impl PropagationValidator {
    /// Validate that error propagation is allowed in the current context
    pub fn validate_propagation(
        expr: &ErrorPropagation,
        context: &PropagationContext,
    ) -> crate::error::Result<()> {
        // Check if we're in a valid function context
        if context.current_function.is_none() {
        }
        
        // Check for excessive propagation depth (potential infinite recursion)
        if context.propagation_depth() > 100 {
            return Err(CursedError::Runtime("CursedError propagation depth exceeded maximum limit (100)".to_string()));
        }
        
        // Validate type compatibility if available
        if let (Some(expected), Some(current)) = (&expr.expected_type, &context.expected_return_type) {
            if !Self::types_compatible(expected, current) {
                return Err(CursedError::Runtime(format!(
                    "CursedError propagation type mismatch: expected '{}', found '{}'",
                    current, expected
                )));
            }
        }
        
        Ok(())
    }
    
    /// Check if two types are compatible for error propagation
    fn types_compatible(expected: &str, actual: &str) -> bool {
        // Simplified type compatibility check
        // In a full implementation, this would use the type system
        expected == actual || 
        expected.starts_with("Result<") && actual.starts_with("Result<") ||
        expected.starts_with("Option<") && actual.starts_with("Option<")
    }
}

