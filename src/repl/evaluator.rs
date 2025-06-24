use crate::error::Error;
//! REPL Evaluation Engine for CURSED
//! 
//! Provides incremental compilation and execution capabilities
//! for the CURSED REPL, managing execution context and state.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::repl::{ReplResult, SessionManager, ReplOutput};
use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::Program;
use crate::codegen::LlvmCodeGenerator;
use crate::codegen::llvm::{JitCompilationInterface, create_optimized_jit_interface};
use inkwell::context::Context;

/// Execution context for REPL evaluations
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub variables: HashMap<String, ContextValue>,
    pub functions: HashMap<String, ContextFunction>,
    pub imports: Vec<String>,
    pub current_module: Option<String>,
}

/// Value stored in REPL context
#[derive(Debug, Clone)]
pub struct ContextValue {
    pub name: String,
    pub type_info: String,
    pub value: String,
    pub is_mutable: bool,
}

/// Function stored in REPL context
#[derive(Debug, Clone)]
pub struct ContextFunction {
    pub name: String,
    pub signature: String,
    pub return_type: String,
    pub body: String,
    pub is_external: bool,
}

/// REPL evaluation engine
pub struct ReplEvaluator {
    context: Arc<Mutex<ExecutionContext>>,
    codegen: Option<LlvmCodeGenerator>,
    jit_interface: Option<JitCompilationInterface<'static>>,
    llvm_context: Option<Context>,
    session_code: Vec<String>,
    last_expression: Option<String>,
    evaluation_timeout: Duration,
}

impl ReplEvaluator {
    /// Create a new REPL evaluator
    pub fn new() -> ReplResult<Self> {
        let context = ExecutionContext {
            variables: HashMap::new(),
            functions: HashMap::new(),
            imports: Vec::new(),
            current_module: None,
        };

        Ok(Self {
            context: Arc::new(Mutex::new(context)),
            codegen: None,
            jit_interface: None,
            llvm_context: None,
            session_code: Vec::new(),
            last_expression: None,
            evaluation_timeout: Duration::from_secs(30),
        })
    }

    /// Initialize the LLVM code generator
    pub fn initialize_codegen(&mut self) -> ReplResult<()> {
        match LlvmCodeGenerator::new() {
            Ok(codegen) => {
                self.codegen = Some(codegen);
                
                // Initialize JIT compilation interface
                if let Err(e) = self.initialize_jit_interface() {
                    tracing::warn!("JIT compilation not available: {}", e);
                    tracing::info!("Falling back to interpretation mode");
                }
                
                Ok(())
            }
            Err(e) => {
                // Fall back to interpreter mode if LLVM is not available
                eprintln!("Warning: LLVM code generation not available: {}", e);
                eprintln!("Falling back to interpreter mode");
                Ok(())
            }
        }
    }

    /// Initialize JIT compilation interface
    fn initialize_jit_interface(&mut self) -> ReplResult<()> {
        // Create LLVM context - in production this would be managed properly
        // For now, we'll use a leaked static context to satisfy lifetime requirements
        let context = Box::leak(Box::new(Context::create()));
        
        match create_optimized_jit_interface(context) {
            Ok(jit_interface) => {
                self.jit_interface = Some(jit_interface);
                tracing::info!("JIT compilation interface initialized for REPL");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to initialize JIT interface: {}", e);
                Err(CursedError::repl_error(format!(
                    "Failed to initialize JIT compilation: {}", e
                )))
            }
        }
    }

    /// Evaluate CURSED code in the current context
    pub fn evaluate(&mut self, code: &str, session_manager: &mut SessionManager) -> ReplResult<ReplOutput> {
        let start_time = Instant::now();
        
        // Parse the input
        let ast = self.parse_code(code)?;
        
        // Check if this is an expression or statement
        let result = if self.is_expression(&ast) {
            self.evaluate_expression(code, &ast, session_manager)?
        } else {
            self.execute_statement(code, &ast, session_manager)?
        };

        let execution_time = start_time.elapsed();
        
        // Update session manager
        session_manager.add_to_history(code.to_string(), true, execution_time);
        
        Ok(ReplOutput::success(result).with_timing(execution_time))
    }

    /// Evaluate code using JIT compilation if available
    pub fn evaluate_with_jit(&mut self, code: &str) -> ReplResult<String> {
        if let Some(ref mut jit_interface) = self.jit_interface {
            tracing::debug!("Attempting JIT evaluation of: {}", code);
            
            match jit_interface.execute_repl_code(code) {
                Ok(result) => {
                    tracing::debug!("JIT execution succeeded with result: {}", result);
                    Ok(result.to_string())
                }
                Err(e) => {
                    tracing::debug!("JIT execution failed: {}", e);
                    Err(CursedError::repl_error(format!("JIT execution failed: {}", e)))
                }
            }
        } else {
            Err(CursedError::repl_error("JIT interface not initialized".to_string()))
        }
    }

    /// Check if JIT compilation is available
    pub fn has_jit_support(&self) -> bool {
        self.jit_interface.is_some()
    }

    /// Get JIT performance report
    pub fn get_jit_performance_report(&self) -> Option<String> {
        self.jit_interface.as_ref().map(|jit| jit.generate_performance_report())
    }

    /// Compile and cache a function in JIT engine
    pub fn compile_function(&mut self, name: &str, source: &str) -> ReplResult<()> {
        if let Some(ref mut jit_interface) = self.jit_interface {
            jit_interface.compile_and_cache_function(name, source)
                .map_err(|e| CursedError::repl_error(format!("Function compilation failed: {}", e)))
        } else {
            Err(CursedError::repl_error("JIT interface not initialized".to_string()))
        }
    }

    /// List all available JIT-compiled functions
    pub fn list_jit_functions(&self) -> Vec<String> {
        self.jit_interface.as_ref()
            .map(|jit| jit.list_functions())
            .unwrap_or_default()
    }

    /// Execute a previously compiled function
    pub fn execute_jit_function(&mut self, function_name: &str) -> ReplResult<String> {
        if let Some(ref mut jit_interface) = self.jit_interface {
            match jit_interface.execute_function(function_name) {
                Ok(result) => Ok(result.to_string()),
                Err(e) => Err(CursedError::repl_error(format!("Function execution failed: {}", e)))
            }
        } else {
            Err(CursedError::repl_error("JIT interface not initialized".to_string()))
        }
    }

    /// Parse CURSED code into AST
    fn parse_code(&self, code: &str) -> ReplResult<Program> {
        let lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer)
            .map_err(|e| CursedError::repl_error(format!("Failed to create parser: {}", e)))?;
        
        let program = parser.parse_program()
            .map_err(|e| CursedError::repl_error(format!("Parse error: {}", e)))?;
        
        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(CursedError::repl_error(format!("Parse errors: {}", errors.join(", "))));
        }
        
        Ok(program)
    }

    /// Check if the AST represents an expression (returns a value)
    fn is_expression(&self, ast: &Program) -> bool {
        if ast.statements.len() != 1 {
            return false;
        }
        
        // Check if the single statement is an expression statement
        if let Some(expr_stmt) = ast.statements[0].as_any().downcast_ref::<crate::ast::ExpressionStatement>() {
            true
        } else {
            false
        }
    }

    /// Evaluate an expression and return its value
    fn evaluate_expression(&mut self, code: &str, ast: &Program, session_manager: &mut SessionManager) -> ReplResult<String> {
        // Add to session code for context
        self.session_code.push(format!("// Expression: {}", code));
        self.last_expression = Some(code.to_string());
        
        // Try to compile and execute with LLVM if available
        if let Some(ref mut codegen) = self.codegen {
            match self.compile_and_execute_with_llvm(code, codegen) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    eprintln!("LLVM execution failed, falling back to interpreter: {}", e);
                }
            }
        }
        
        // Fall back to interpreter-based evaluation
        self.interpret_expression(code, ast, session_manager)
    }

    /// Execute a statement (doesn't return a value)
    fn execute_statement(&mut self, code: &str, ast: &Program, session_manager: &mut SessionManager) -> ReplResult<String> {
        // Add to session code
        self.session_code.push(code.to_string());
        
        // Analyze the statement for variable/function definitions
        self.analyze_definitions(ast)?;
        
        // Try to compile and execute with LLVM if available
        if let Some(ref mut codegen) = self.codegen {
            match self.compile_and_execute_with_llvm(code, codegen) {
                Ok(_) => return Ok("".to_string()), // Statements don't return values
                Err(e) => {
                    eprintln!("LLVM execution failed, falling back to interpreter: {}", e);
                }
            }
        }
        
        // Fall back to interpreter-based execution
        self.interpret_statement(code, ast, session_manager)
    }

    /// Compile and execute code using LLVM
    fn compile_and_execute_with_llvm(&mut self, code: &str, codegen: &mut LlvmCodeGenerator) -> ReplResult<String> {
        // Create a complete program with session context
        let full_program = self.build_complete_program(code);
        
        // Compile to LLVM IR
        let _ir = crate::compile_to_ir(&full_program)
            .map_err(|e| CursedError::repl_error(format!("Compilation failed: {}", e)))?;
        
        // For now, return a placeholder since actual execution would require
        // LLVM JIT compilation which is more complex
        Ok("(compiled with LLVM)".to_string())
    }

    /// Build a complete program including session context
    fn build_complete_program(&self, new_code: &str) -> String {
        let mut program = String::new();
        
        // Add imports
        if let Ok(context) = self.context.lock() {
            for import in &context.imports {
                program.push_str(&format!("import \"{}\"\n", import));
            }
            program.push('\n');
            
            // Add function definitions
            for func in context.functions.values() {
                if !func.is_external {
                    program.push_str(&func.body);
                    program.push('\n');
                }
            }
        }
        
        // Add session code
        for line in &self.session_code {
            if !line.starts_with("//") {
                program.push_str(line);
                program.push('\n');
            }
        }
        
        // Add new code
        program.push_str(new_code);
        
        program
    }

    /// Interpret an expression using a simple evaluator
    fn interpret_expression(&mut self, code: &str, _ast: &Program, _session_manager: &mut SessionManager) -> ReplResult<String> {
        let trimmed = code.trim();
        
        // Handle literals
        if trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Ok(trimmed.to_string());
        }
        
        if let Ok(float_val) = trimmed.parse::<f64>() {
            return Ok(float_val.to_string());
        }
        
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            return Ok(trimmed.to_string());
        }
        
        if trimmed == "true" || trimmed == "false" {
            return Ok(trimmed.to_string());
        }
        
        // Handle variable references
        if let Ok(context) = self.context.lock() {
            if let Some(var) = context.variables.get(trimmed) {
                return Ok(var.value.clone());
            }
        }
        
        // Handle simple arithmetic
        if let Some(result) = self.evaluate_simple_arithmetic(trimmed) {
            return Ok(result);
        }
        
        // Handle function calls
        if trimmed.contains('(') && trimmed.contains(')') {
            return self.evaluate_function_call(trimmed);
        }
        
        // Default case
        Ok(format!("(expression: {})", trimmed))
    }

    /// Interpret a statement using a simple evaluator
    fn interpret_statement(&mut self, code: &str, _ast: &Program, _session_manager: &mut SessionManager) -> ReplResult<String> {
        let trimmed = code.trim();
        
        // Handle variable declarations
        if trimmed.starts_with("facts ") || trimmed.starts_with("sus ") {
            return self.handle_variable_declaration(trimmed);
        }
        
        // Handle function declarations
        if trimmed.starts_with("slay ") {
            return self.handle_function_declaration(trimmed);
        }
        
        // Handle import statements
        if trimmed.starts_with("import ") {
            return self.handle_import_statement(trimmed);
        }
        
        // Default case
        Ok("".to_string())
    }

    /// Analyze AST for variable and function definitions
    fn analyze_definitions(&mut self, ast: &Program) -> ReplResult<()> {
        // This would be a more sophisticated analysis in a real implementation
        // For now, we'll rely on the simpler text-based analysis in interpret_statement
        Ok(())
    }

    /// Evaluate simple arithmetic expressions
    fn evaluate_simple_arithmetic(&self, expr: &str) -> Option<String> {
        // Very basic arithmetic evaluation
        if let Some(pos) = expr.find('+') {
            let left = expr[..pos].trim();
            let right = expr[pos+1..].trim();
            
            if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                return Some((l + r).to_string());
            }
        }
        
        if let Some(pos) = expr.find('-') {
            let left = expr[..pos].trim();
            let right = expr[pos+1..].trim();
            
            if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                return Some((l - r).to_string());
            }
        }
        
        if let Some(pos) = expr.find('*') {
            let left = expr[..pos].trim();
            let right = expr[pos+1..].trim();
            
            if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                return Some((l * r).to_string());
            }
        }
        
        if let Some(pos) = expr.find('/') {
            let left = expr[..pos].trim();
            let right = expr[pos+1..].trim();
            
            if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                if r != 0.0 {
                    return Some((l / r).to_string());
                }
            }
        }
        
        None
    }

    /// Evaluate function calls
    fn evaluate_function_call(&self, call: &str) -> ReplResult<String> {
        // Extract function name and arguments
        if let Some(paren_pos) = call.find('(') {
            let func_name = call[..paren_pos].trim();
            
            // Handle built-in functions
            match func_name {
                "len" => Ok("(length function)".to_string()),
                "print" | "println" => {
                    // Extract the argument and return empty string (side effect)
                    Ok("".to_string())
                }
                _ => {
                    // Check if it's a user-defined function
                    if let Ok(context) = self.context.lock() {
                        if context.functions.contains_key(func_name) {
                            Ok(format!("(function call: {})", func_name))
                        } else {
                            Err(CursedError::repl_error(format!("Unknown function: {}", func_name)))
                        }
                    } else {
                        Ok(format!("(function call: {})", func_name))
                    }
                }
            }
        } else {
            Err(CursedError::repl_error("Invalid function call syntax".to_string()))
        }
    }

    /// Handle variable declarations
    fn handle_variable_declaration(&mut self, decl: &str) -> ReplResult<String> {
        // Parse variable declaration: "facts name = value" or "sus name = value"
        let parts: Vec<&str> = decl.split_whitespace().collect();
        
        if parts.len() >= 4 && parts[2] == "=" {
            let is_mutable = parts[0] == "sus";
            let var_name = parts[1].to_string();
            let value_expr = parts[3..].join(" ");
            
            // Evaluate the value expression
            let value = if let Some(result) = self.evaluate_simple_arithmetic(&value_expr) {
                result
            } else if value_expr.starts_with('"') && value_expr.ends_with('"') {
                value_expr
            } else if value_expr.chars().all(|c| c.is_ascii_digit()) {
                value_expr
            } else if value_expr == "true" || value_expr == "false" {
                value_expr
            } else {
                value_expr
            };
            
            // Infer type
            let type_info = if value.chars().all(|c| c.is_ascii_digit()) {
                "int".to_string()
            } else if value.parse::<f64>().is_ok() {
                "float64".to_string()
            } else if value.starts_with('"') && value.ends_with('"') {
                "string".to_string()
            } else if value == "true" || value == "false" {
                "bool".to_string()
            } else {
                "unknown".to_string()
            };
            
            // Store in context
            if let Ok(mut context) = self.context.lock() {
                let context_value = ContextValue {
                    name: var_name.clone(),
                    type_info,
                    value,
                    is_mutable,
                };
                context.variables.insert(var_name.clone(), context_value);
            }
            
            Ok(format!("Variable {} declared", var_name))
        } else {
            Err(CursedError::repl_error("Invalid variable declaration syntax".to_string()))
        }
    }

    /// Handle function declarations
    fn handle_function_declaration(&mut self, decl: &str) -> ReplResult<String> {
        // Parse function declaration: "slay name(args) { ... }"
        if let Some(paren_pos) = decl.find('(') {
            let prefix = &decl[..paren_pos];
            let func_name = prefix.strip_prefix("slay ").unwrap_or("").trim();
            
            if !func_name.is_empty() {
                // For now, just record that the function exists
                if let Ok(mut context) = self.context.lock() {
                    let context_function = ContextFunction {
                        name: func_name.to_string(),
                        signature: decl.to_string(),
                        return_type: "unknown".to_string(),
                        body: decl.to_string(),
                        is_external: false,
                    };
                    context.functions.insert(func_name.to_string(), context_function);
                }
                
                Ok(format!("Function {} declared", func_name))
            } else {
                Err(CursedError::repl_error("Invalid function name".to_string()))
            }
        } else {
            Err(CursedError::repl_error("Invalid function declaration syntax".to_string()))
        }
    }

    /// Handle import statements
    fn handle_import_statement(&mut self, stmt: &str) -> ReplResult<String> {
        // Parse import statement: "import \"module\""
        if let Some(start) = stmt.find('"') {
            if let Some(end) = stmt.rfind('"') {
                if start < end {
                    let module_name = &stmt[start+1..end];
                    
                    if let Ok(mut context) = self.context.lock() {
                        if !context.imports.contains(&module_name.to_string()) {
                            context.imports.push(module_name.to_string());
                        }
                    }
                    
                    Ok(format!("Imported module: {}", module_name))
                } else {
                    Err(CursedError::repl_error("Invalid import syntax".to_string()))
                }
            } else {
                Err(CursedError::repl_error("Unterminated import string".to_string()))
            }
        } else {
            Err(CursedError::repl_error("Import statement must include module name in quotes".to_string()))
        }
    }

    /// Get all variables in the current context
    pub fn get_variables(&self) -> Vec<(String, String, String)> {
        if let Ok(context) = self.context.lock() {
            context.variables.values()
                .map(|var| (var.name.clone(), var.type_info.clone(), var.value.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all functions in the current context
    pub fn get_functions(&self) -> Vec<(String, String)> {
        if let Ok(context) = self.context.lock() {
            context.functions.values()
                .map(|func| (func.name.clone(), func.signature.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get the type of an expression
    pub fn get_expression_type(&self, expr: &str) -> ReplResult<String> {
        let trimmed = expr.trim();
        
        // Check variables first
        if let Ok(context) = self.context.lock() {
            if let Some(var) = context.variables.get(trimmed) {
                return Ok(var.type_info.clone());
            }
            
            if let Some(func) = context.functions.get(trimmed) {
                return Ok(format!("function {}", func.signature));
            }
        }
        
        // Type inference for literals
        if trimmed.chars().all(|c| c.is_ascii_digit()) {
            Ok("int".to_string())
        } else if trimmed.parse::<f64>().is_ok() {
            Ok("float64".to_string())
        } else if trimmed.starts_with('"') && trimmed.ends_with('"') {
            Ok("string".to_string())
        } else if trimmed == "true" || trimmed == "false" {
            Ok("bool".to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// Clear the evaluation context
    pub fn clear_context(&mut self) -> ReplResult<()> {
        if let Ok(mut context) = self.context.lock() {
            context.variables.clear();
            context.functions.clear();
            context.imports.clear();
        }
        
        self.session_code.clear();
        self.last_expression = None;
        
        Ok(())
    }

    /// Get the current session code
    pub fn get_session_code(&self) -> String {
        self.session_code.join("\n")
    }
}

impl Default for ReplEvaluator {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_creation() {
        let evaluator = ReplEvaluator::new();
        assert!(evaluator.is_ok());
    }

    #[test]
    fn test_simple_arithmetic() {
        let evaluator = ReplEvaluator::new().unwrap();
        
        assert_eq!(evaluator.evaluate_simple_arithmetic("2 + 3"), Some("5".to_string()));
        assert_eq!(evaluator.evaluate_simple_arithmetic("10 - 4"), Some("6".to_string()));
        assert_eq!(evaluator.evaluate_simple_arithmetic("3 * 4"), Some("12".to_string()));
        assert_eq!(evaluator.evaluate_simple_arithmetic("8 / 2"), Some("4".to_string()));
    }

    #[test]
    fn test_variable_declaration() {
        let mut evaluator = ReplEvaluator::new().unwrap();
        let mut session = SessionManager::new();
        
        let result = evaluator.evaluate("facts x = 42", &mut session);
        assert!(result.is_ok());
        
        let vars = evaluator.get_variables();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].0, "x");
    }

    #[test]
    fn test_expression_type_inference() {
        let evaluator = ReplEvaluator::new().unwrap();
        
        assert_eq!(evaluator.get_expression_type("42").unwrap(), "int");
        assert_eq!(evaluator.get_expression_type("3.14").unwrap(), "float64");
        assert_eq!(evaluator.get_expression_type("\"hello\"").unwrap(), "string");
        assert_eq!(evaluator.get_expression_type("true").unwrap(), "bool");
    }

    #[test]
    fn test_function_declaration() {
        let mut evaluator = ReplEvaluator::new().unwrap();
        let mut session = SessionManager::new();
        
        let result = evaluator.evaluate("slay test() { }", &mut session);
        assert!(result.is_ok());
        
        let funcs = evaluator.get_functions();
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].0, "test");
    }
}
