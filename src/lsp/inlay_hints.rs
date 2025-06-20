//! Inlay hints implementation for CURSED language
//! 
//! Provides contextual inline information including type hints, parameter names,
//! return types, and implicit conversions.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::*;
use crate::type_system::{Type, TypeChecker};

/// Types of inlay hints available
#[derive(Debug, Clone, PartialEq)]
pub enum InlayHintType {
    /// Type information for variables and expressions
    TypeHint,
    /// Parameter names in function calls
    ParameterName,
    /// Return types for functions
    ReturnType,
    /// Implicit conversions and casts
    Conversion,
    /// Generic type parameters
    GenericParameter,
    /// Error propagation information
    ErrorPropagation,
    /// Channel direction hints
    ChannelDirection,
    /// Goroutine information
    GoroutineInfo,
    /// Performance hints
    Performance,
    /// Memory allocation hints
    MemoryAllocation,
}

/// Configuration for inlay hints
#[derive(Debug, Clone)]
pub struct InlayHintConfig {
    /// Show type hints for variables
    pub show_type_hints: bool,
    /// Show parameter names in function calls
    pub show_parameter_names: bool,
    /// Show return types
    pub show_return_types: bool,
    /// Show implicit conversions
    pub show_conversions: bool,
    /// Show generic parameters
    pub show_generic_parameters: bool,
    /// Show error propagation
    pub show_error_propagation: bool,
    /// Show channel directions
    pub show_channel_hints: bool,
    /// Show goroutine information
    pub show_goroutine_hints: bool,
    /// Show performance hints
    pub show_performance_hints: bool,
    /// Show memory allocation hints
    pub show_memory_hints: bool,
    /// Maximum hint length before truncation
    pub max_hint_length: usize,
    /// Show hints only when types are complex
    pub only_complex_types: bool,
}

impl Default for InlayHintConfig {
    fn default() -> Self {
        Self {
            show_type_hints: true,
            show_parameter_names: true,
            show_return_types: false,
            show_conversions: true,
            show_generic_parameters: true,
            show_error_propagation: true,
            show_channel_hints: true,
            show_goroutine_hints: true,
            show_performance_hints: false,
            show_memory_hints: false,
            max_hint_length: 50,
            only_complex_types: false,
        }
    }
}

/// Custom inlay hint with CURSED-specific information
#[derive(Debug, Clone)]
pub struct CursedInlayHint {
    pub position: Position,
    pub hint_type: InlayHintType,
    pub label: String,
    pub tooltip: Option<String>,
    pub text_edits: Option<Vec<TextEdit>>,
    pub padding_left: bool,
    pub padding_right: bool,
}

impl CursedInlayHint {
    /// Create a new inlay hint
    pub fn new(
        position: Position,
        hint_type: InlayHintType,
        label: String,
        tooltip: Option<String>,
    ) -> Self {
        Self {
            position,
            hint_type,
            label,
            tooltip,
            text_edits: None,
            padding_left: false,
            padding_right: false,
        }
    }
    
    /// Create a type hint
    pub fn type_hint(position: Position, type_name: String, tooltip: Option<String>) -> Self {
        Self {
            position,
            hint_type: InlayHintType::TypeHint,
            label: format!(": {}", type_name),
            tooltip,
            text_edits: None,
            padding_left: false,
            padding_right: true,
        }
    }
    
    /// Create a parameter name hint
    pub fn parameter_hint(position: Position, param_name: String, tooltip: Option<String>) -> Self {
        Self {
            position,
            hint_type: InlayHintType::ParameterName,
            label: format!("{}:", param_name),
            tooltip,
            text_edits: None,
            padding_left: false,
            padding_right: true,
        }
    }
    
    /// Create a return type hint
    pub fn return_type_hint(position: Position, return_type: String, tooltip: Option<String>) -> Self {
        Self {
            position,
            hint_type: InlayHintType::ReturnType,
            label: format!(" -> {}", return_type),
            tooltip,
            text_edits: None,
            padding_left: true,
            padding_right: false,
        }
    }
    
    /// Create a conversion hint
    pub fn conversion_hint(position: Position, conversion: String, tooltip: Option<String>) -> Self {
        Self {
            position,
            hint_type: InlayHintType::Conversion,
            label: format!("({})", conversion),
            tooltip,
            text_edits: None,
            padding_left: true,
            padding_right: false,
        }
    }
    
    /// Convert to LSP InlayHint
    pub fn to_lsp_inlay_hint(&self) -> InlayHint {
        InlayHint {
            position: self.position,
            label: InlayHintLabel::String(self.label.clone()),
            kind: Some(match self.hint_type {
                InlayHintType::TypeHint => InlayHintKind::TYPE,
                InlayHintType::ParameterName => InlayHintKind::PARAMETER,
                _ => InlayHintKind::TYPE,
            }),
            text_edits: self.text_edits.clone(),
            tooltip: self.tooltip.as_ref().map(|t| {
                InlayHintTooltip::String(t.clone())
            }),
            padding_left: Some(self.padding_left),
            padding_right: Some(self.padding_right),
            data: None,
        }
    }
}

/// Inlay hints provider for CURSED language
pub struct InlayHintsProvider {
    /// Configuration
    config: InlayHintConfig,
    /// Type checker for type inference
    type_checker: TypeChecker,
    /// Cache for expensive computations
    type_cache: HashMap<String, String>,
}

impl InlayHintsProvider {
    /// Create a new inlay hints provider
    pub fn new() -> Self {
        Self {
            config: InlayHintConfig::default(),
            type_checker: TypeChecker::new(),
            type_cache: HashMap::new(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: InlayHintConfig) -> Self {
        Self {
            config,
            type_checker: TypeChecker::new(),
            type_cache: HashMap::new(),
        }
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: InlayHintConfig) {
        self.config = config;
    }
    
    /// Get inlay hints for the given content and range
    #[instrument(skip(self, content))]
    pub async fn get_inlay_hints(
        &mut self,
        content: &str,
        range: Range,
    ) -> Result<Vec<CursedInlayHint>, String> {
        debug!("Generating inlay hints for range {:?}", range);
        
        let mut hints = Vec::new();
        
        // Parse the content
        let mut lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        
        match parser?.parse() {
            Ok(ast) => {
                // Type check the AST for type information
                if let Err(e) = self.type_checker.check(&ast) {
                    debug!("Type checking failed: {:?}", e);
                }
                
                // Generate various types of hints
                self.generate_variable_type_hints(&ast, range, &mut hints).await;
                self.generate_function_call_hints(&ast, range, &mut hints).await;
                self.generate_return_type_hints(&ast, range, &mut hints).await;
                self.generate_conversion_hints(&ast, range, &mut hints).await;
                self.generate_error_propagation_hints(&ast, range, &mut hints).await;
                self.generate_channel_hints(&ast, range, &mut hints).await;
                self.generate_goroutine_hints(&ast, range, &mut hints).await;
            }
            Err(e) => {
                debug!("Failed to parse content for inlay hints: {:?}", e);
                // Generate basic hints from lexical analysis
                self.generate_lexical_hints(content, range, &mut hints).await;
            }
        }
        
        // Filter hints by length and complexity
        hints = self.filter_hints(hints);
        
        Ok(hints)
    }
    
    /// Generate type hints for variable declarations
    async fn generate_variable_type_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_type_hints {
            return;
        }
        
        for statement in &ast.statements {
            if let Some(var_decl) = statement.as_any().downcast_ref::<VariableStatement>() {
                // For VariableStatement, we don't have line/column info in the name field
                // This is a simplified implementation
                let var_line = 0u32; // Would need to get from somewhere else
                
                // Check if variable is in range (simplified)
                if var_line >= range.start.line && var_line <= range.end.line {
                    // Only show hint if type is not explicitly declared
                    if var_decl.var_type.is_none() {
                        if let Some(inferred_type) = self.infer_variable_type(var_decl) {
                            let position = Position {
                                line: var_line,
                                character: var_decl.name.len() as u32,
                            };
                            
                            let tooltip = Some(format!(
                                "Inferred type for variable '{}'",
                                var_decl.name
                            ));
                            
                            hints.push(CursedInlayHint::type_hint(
                                position,
                                inferred_type,
                                tooltip,
                            ));
                        }
                    }
                }
            }
        }
    }
    
    /// Generate parameter name hints for function calls
    async fn generate_function_call_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_parameter_names {
            return;
        }
        
        // This is a simplified implementation
        // In a real implementation, you'd traverse the AST to find function call expressions
        for statement in &ast.statements {
            if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
                self.generate_expression_hints(&*expr_stmt.expression, range, hints).await;
            }
        }
    }
    
    /// Generate hints for expressions
    async fn generate_expression_hints(
        &mut self,
        expr: &dyn Expression,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if let Some(call_expr) = expr.as_any().downcast_ref::<CallExpression>() {
            // Get function signature and match parameters
            if let Some(signature) = self.get_function_signature(&*call_expr.function) {
                for (i, arg) in call_expr.arguments.iter().enumerate() {
                    if i < signature.parameters.len() {
                        let param_name = &signature.parameters[i].name;
                        let arg_position = self.get_expression_position(&**arg);
                        
                        if self.is_position_in_range(arg_position, range) {
                            let tooltip = Some(format!(
                                "Parameter '{}' of function '{}'",
                                param_name,
                                signature.name
                            ));
                            
                            hints.push(CursedInlayHint::parameter_hint(
                                arg_position,
                                param_name.clone(),
                                tooltip,
                            ));
                        }
                    }
                }
            }
        } else if let Some(bin_op) = expr.as_any().downcast_ref::<BinaryExpression>() {
            // Recursively process operands
            self.generate_expression_hints(&*bin_op.left, range, hints).await;
            self.generate_expression_hints(&*bin_op.right, range, hints).await;
            
            // Check for implicit conversions
            if self.config.show_conversions {
                if let Some(conversion) = self.detect_implicit_conversion(bin_op) {
                    let position = self.get_expression_position(&*bin_op.right);
                    if self.is_position_in_range(position, range) {
                        hints.push(CursedInlayHint::conversion_hint(
                            position,
                            conversion,
                            Some("Implicit type conversion".to_string()),
                        ));
                    }
                }
            }
        } else if let Some(unary_op) = expr.as_any().downcast_ref::<UnaryExpression>() {
            self.generate_expression_hints(&*unary_op.operand, range, hints).await;
        } else if let Some(array_access) = expr.as_any().downcast_ref::<IndexExpression>() {
            self.generate_expression_hints(&*array_access.left, range, hints).await;
            self.generate_expression_hints(&*array_access.index, range, hints).await;
        } else if let Some(field_access) = expr.as_any().downcast_ref::<DotExpression>() {
            self.generate_expression_hints(&*field_access.left, range, hints).await;
        }
    }
    
    /// Generate return type hints for functions
    async fn generate_return_type_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_return_types {
            return;
        }
        
        for statement in &ast.statements {
            if let Some(func_decl) = statement.as_any().downcast_ref::<FunctionStatement>() {
                // For FunctionStatement, we don't have line info readily available
                let func_line = 0u32; // Would need to get from somewhere else
                
                if func_line >= range.start.line && func_line <= range.end.line {
                    // Only show hint if return type is not explicitly declared
                    if func_decl.return_type.is_none() {
                        if let Some(inferred_return_type) = self.infer_function_return_type(func_decl) {
                            let position = Position {
                                line: func_line,
                                character: self.get_function_params_end_position(func_decl),
                            };
                            
                            let tooltip = Some(format!(
                                "Inferred return type for function '{}'",
                                func_decl.name.value
                            ));
                            
                            hints.push(CursedInlayHint::return_type_hint(
                                position,
                                inferred_return_type,
                                tooltip,
                            ));
                        }
                    }
                }
            }
        }
    }
    
    /// Generate conversion hints
    async fn generate_conversion_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_conversions {
            return;
        }
        
        // This would analyze the AST for implicit conversions
        // Implementation would be similar to expression hints
    }
    
    /// Generate error propagation hints
    async fn generate_error_propagation_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_error_propagation {
            return;
        }
        
        // Look for ? operators and functions that can fail
        for statement in &ast.statements {
            if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
                self.find_error_propagation_in_expression(&*expr_stmt.expression, range, hints).await;
            }
        }
    }
    
    /// Find error propagation in expressions
    async fn find_error_propagation_in_expression(
        &mut self,
        expr: &dyn Expression,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if let Some(error_prop) = expr.as_any().downcast_ref::<QuestionMarkExpression>() {
            let position = self.get_expression_position(&*error_prop.expression);
            if self.is_position_in_range(position, range) {
                hints.push(CursedInlayHint::new(
                    position,
                    InlayHintType::ErrorPropagation,
                    "?".to_string(),
                    Some("Error propagation operator".to_string()),
                ));
            }
        } else if let Some(call_expr) = expr.as_any().downcast_ref::<CallExpression>() {
            // Check if function can fail
            if self.function_can_fail(&*call_expr.function) {
                let position = self.get_expression_position(expr);
                if self.is_position_in_range(position, range) {
                    hints.push(CursedInlayHint::new(
                        position,
                        InlayHintType::ErrorPropagation,
                        "!".to_string(),
                        Some("Function may fail - consider using ?".to_string()),
                    ));
                }
            }
        }
        // Note: Recursively check sub-expressions could be added here
    }
    
    /// Generate channel direction hints
    async fn generate_channel_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_channel_hints {
            return;
        }
        
        // Look for channel operations
        for statement in &ast.statements {
            if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
                self.find_channel_operations(&*expr_stmt.expression, range, hints).await;
            }
        }
    }
    
    /// Find channel operations in expressions
    async fn find_channel_operations(
        &mut self,
        expr: &dyn Expression,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        // Channel operations detection would go here
        // For now, we don't have specific channel expression types defined
        // This would be expanded when channel expressions are properly implemented
    }
    
    /// Generate goroutine hints
    async fn generate_goroutine_hints(
        &mut self,
        ast: &Program,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        if !self.config.show_goroutine_hints {
            return;
        }
        
        // Look for goroutine spawning (stan keyword)
        for statement in &ast.statements {
            if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
                if let Some(spawn_expr) = expr_stmt.expression.as_any().downcast_ref::<StanExpression>() {
                    let position = self.get_expression_position(&*spawn_expr.call);
                    if self.is_position_in_range(position, range) {
                        hints.push(CursedInlayHint::new(
                            position,
                            InlayHintType::GoroutineInfo,
                            "goroutine".to_string(),
                            Some("Function will run in a new goroutine".to_string()),
                        ));
                    }
                }
            }
        }
    }
    
    /// Generate basic hints from lexical analysis when parsing fails
    async fn generate_lexical_hints(
        &mut self,
        content: &str,
        range: Range,
        hints: &mut Vec<CursedInlayHint>,
    ) {
        let mut lexer = Lexer::new(content);
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                    
                    let token_line = (token.location.line - 1) as u32;
                    if token_line >= range.start.line && token_line <= range.end.line {
                        // Generate basic hints for recognized patterns
                        if token.literal == "?" && self.config.show_error_propagation {
                            let position = Position {
                                line: token_line,
                                character: token.location.column as u32,
                            };
                            
                            hints.push(CursedInlayHint::new(
                                position,
                                InlayHintType::ErrorPropagation,
                                "error propagation".to_string(),
                                Some("Error propagation operator".to_string()),
                            ));
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    
    /// Filter hints based on configuration
    fn filter_hints(&self, hints: Vec<CursedInlayHint>) -> Vec<CursedInlayHint> {
        hints
            .into_iter()
            .filter(|hint| {
                // Filter by length
                if hint.label.len() > self.config.max_hint_length {
                    return false;
                }
                
                // Filter by complexity if enabled
                if self.config.only_complex_types {
                    match hint.hint_type {
                        InlayHintType::TypeHint => {
                            // Only show hints for complex types
                            self.is_complex_type(&hint.label)
                        }
                        _ => true,
                    }
                } else {
                    true
                }
            })
            .collect()
    }
    
    /// Check if a type is considered complex
    fn is_complex_type(&self, type_str: &str) -> bool {
        // Consider types complex if they contain generics, channels, or are multi-word
        type_str.contains('<') ||
        type_str.contains("chan") ||
        type_str.contains("map") ||
        type_str.split_whitespace().count() > 1
    }
    
    // Helper methods for type inference and AST analysis
    
    fn infer_variable_type(&mut self, var_decl: &VariableStatement) -> Option<String> {
        // Use type checker to infer variable type
        if let Some(value) = &var_decl.value {
            // Simplified type inference based on the value expression
            // This would need to use proper type inference from the type checker
            Some("inferred".to_string()) // Placeholder
        } else {
            None
        }
    }
    
    fn infer_function_return_type(&mut self, func_decl: &FunctionStatement) -> Option<String> {
        // Analyze function body to infer return type
        // This is a simplified implementation
        Some("void".to_string())
    }
    
    fn get_function_signature(&self, function: &dyn Expression) -> Option<FunctionSignature> {
        // Get function signature from type checker or symbol table
        None
    }
    
    fn detect_implicit_conversion(&self, bin_op: &BinaryOperation) -> Option<String> {
        // Detect if there's an implicit conversion in binary operation
        None
    }
    
    fn function_can_fail(&self, function: &dyn Expression) -> bool {
        // Check if function can return an error
        false
    }
    
    fn get_expression_position(&self, expr: &dyn Expression) -> Position {
        // Get position of expression start
        Position { line: 0, character: 0 }
    }
    
    fn get_function_params_end_position(&self, func_decl: &FunctionStatement) -> u32 {
        // Get position after function parameters
        0
    }
    
    fn is_position_in_range(&self, position: Position, range: Range) -> bool {
        position.line >= range.start.line &&
        position.line <= range.end.line &&
        (position.line > range.start.line || position.character >= range.start.character) &&
        (position.line < range.end.line || position.character <= range.end.character)
    }
}

/// Function signature for parameter name hints
#[derive(Debug, Clone)]
struct FunctionSignature {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Option<String>,
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    type_name: String,
}

impl Default for InlayHintsProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_inlay_hints_generation() {
        let mut provider = InlayHintsProvider::new();
        let content = r#"
            slay greet(name: string) {
                sus greeting = "Hello, " + name;
                vibez greeting;
            }
        "#;
        
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };
        
        let hints = provider.get_inlay_hints(content, range).await.unwrap();
        
        // Should have some hints
        assert!(!hints.is_empty());
    }
    
    #[test]
    fn test_hint_filtering() {
        let mut config = InlayHintConfig::default();
        config.max_hint_length = 10;
        config.only_complex_types = true;
        
        let provider = InlayHintsProvider::with_config(config);
        
        let hints = vec![
            CursedInlayHint::type_hint(
                Position { line: 0, character: 0 },
                "string".to_string(),
                None,
            ),
            CursedInlayHint::type_hint(
                Position { line: 1, character: 0 },
                "map<string, i32>".to_string(),
                None,
            ),
            CursedInlayHint::type_hint(
                Position { line: 2, character: 0 },
                "very_long_type_name_that_exceeds_limit".to_string(),
                None,
            ),
        ];
        
        let filtered = provider.filter_hints(hints);
        
        // Should keep complex type and filter out simple and too long
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].label.contains("map"));
    }
    
    #[test]
    fn test_complex_type_detection() {
        let provider = InlayHintsProvider::new();
        
        assert!(provider.is_complex_type("map<string, i32>"));
        assert!(provider.is_complex_type("chan int"));
        assert!(provider.is_complex_type("Option<T>"));
        assert!(!provider.is_complex_type("string"));
        assert!(!provider.is_complex_type("i32"));
    }
}
