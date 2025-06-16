/// LLVM function compilation for the CURSED programming language
use crate::ast::declarations::FunctionStatement;
use crate::ast::calls::CallExpression;
use crate::ast::statements::ReturnStatement;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::Error;
use crate::codegen::llvm::function_registry::{FunctionRegistry, FunctionSignature, SharedFunctionRegistry};
use crate::codegen::llvm::expression_compiler::LlvmType;
use std::collections::HashMap;
use tracing::{debug, info, warn, error, instrument};

/// Function compilation trait for LLVM code generation
pub trait FunctionCompilation {
    /// Compile a function declaration (slay keyword)
    fn compile_function_declaration(&mut self, func: &FunctionStatement) -> Result<String, Error>;
    
    /// Compile a function call expression
    fn compile_function_call(&mut self, call: &CallExpression) -> Result<String, Error>;
    
    /// Compile a return statement (yolo keyword)
    fn compile_return_statement(&mut self, ret: &ReturnStatement) -> Result<String, Error>;
    
    /// Generate function type from parameters and return type
    fn generate_function_type(&self, params: &[crate::ast::expressions::Parameter], return_type: Option<&Box<dyn Expression>>) -> String;
    
    /// Generate function arguments list
    fn generate_function_arguments(&self, params: &[crate::ast::expressions::Parameter]) -> String;
    
    /// Generate function call arguments
    fn generate_call_arguments(&mut self, args: &[Box<dyn Expression>]) -> Result<String, Error>;
    
    /// Manage local variable scope and stack allocation
    fn allocate_local_variable(&mut self, name: &str, var_type: &str) -> Result<String, Error>;
    
    /// Get or create function in module
    fn get_or_create_function(&mut self, name: &str, func_type: &str) -> String;
}

/// LLVM function context for managing local variables and scopes
#[derive(Debug, Clone)]
pub struct FunctionContext {
    /// Function name
    pub name: String,
    /// Local variables mapping (name -> LLVM value)
    pub locals: HashMap<String, String>,
    /// Parameter list
    pub parameters: Vec<String>,
    /// Return type
    pub return_type: String,
    /// Function LLVM type
    pub function_type: String,
    /// Current basic block
    pub current_block: String,
    /// Function entry block
    pub entry_block: String,
    /// Next temporary variable counter
    pub temp_counter: usize,
}

impl FunctionContext {
    /// Create new function context
    pub fn new(name: String, return_type: String) -> Self {
        Self {
            name: name.clone(),
            locals: HashMap::new(),
            parameters: Vec::new(),
            return_type,
            function_type: String::new(),
            current_block: format!("{}_entry", name),
            entry_block: format!("{}_entry", name),
            temp_counter: 0,
        }
    }
    
    /// Add local variable
    pub fn add_local(&mut self, name: String, llvm_value: String) {
        self.locals.insert(name, llvm_value);
    }
    
    /// Get local variable
    pub fn get_local(&self, name: &str) -> Option<&String> {
        self.locals.get(name)
    }
    
    /// Generate next temporary variable name
    pub fn next_temp(&mut self) -> String {
        let temp = format!("%temp{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }
    
    /// Add parameter
    pub fn add_parameter(&mut self, param_name: String) {
        self.parameters.push(param_name);
    }
}

/// Implementation of function compilation for LLVM code generator
impl crate::codegen::llvm::LlvmCodeGenerator {
    /// Current function context stack
    fn function_stack(&mut self) -> &mut Vec<FunctionContext> {
        // This would be added to the LlvmCodeGenerator struct
        // For now, we'll use a placeholder approach
        static mut FUNCTION_STACK: Vec<FunctionContext> = Vec::new();
        unsafe { &mut *std::ptr::addr_of_mut!(FUNCTION_STACK) }
    }
    
    /// Get current function context
    fn current_function(&mut self) -> Option<&mut FunctionContext> {
        self.function_stack().last_mut()
    }
    
    /// Push new function context
    fn push_function_context(&mut self, context: FunctionContext) {
        self.function_stack().push(context);
    }
    
    /// Pop function context
    fn pop_function_context(&mut self) -> Option<FunctionContext> {
        self.function_stack().pop()
    }
}

impl FunctionCompilation for crate::codegen::llvm::LlvmCodeGenerator {
    #[instrument(skip(self, func))]
    fn compile_function_declaration(&mut self, func: &FunctionStatement) -> Result<String, Error> {
        let func_name = func.name.string();
        
        info!("Compiling function declaration: {}", func_name);
        
        // Create function signature from AST
        let function_signature = FunctionSignature::from_function_statement(func)?;
        
        // Register function in the function registry
        self.register_function(function_signature.clone())?;
        
        // Determine return type (default to void if not specified)
        let return_type = if let Some(ret_type) = &func.return_type {
            self.map_cursed_type_to_llvm(&ret_type.string())
        } else {
            "void".to_string()
        };
        
        // Generate function type
        let func_type = self.generate_function_type(&func.parameters, func.return_type.as_ref());
        let args = self.generate_function_arguments(&func.parameters);
        
        // Create function context
        let mut context = FunctionContext::new(func_name.clone(), return_type.clone());
        context.function_type = func_type.clone();
        
        // Add parameters to context
        for (i, param) in func.parameters.iter().enumerate() {
            let param_name = format!("%{}", param.name);
            context.add_parameter(param_name.clone());
            context.add_local(param.name.clone(), param_name);
        }
        
        self.push_function_context(context);
        
        // Generate LLVM IR
        let mut ir = String::new();
        
        // Function declaration with proper linkage
        let linkage = if function_signature.is_builtin {
            "declare"
        } else {
            "define"
        };
        
        ir.push_str(&format!("\n; Function: {} (slay keyword)\n", func_name));
        ir.push_str(&format!("{} {} @{}({}) {{\n", linkage, return_type, func_name, args));
        
        // Only generate body for non-builtin functions
        if !function_signature.is_builtin {
            // Entry block
            ir.push_str(&format!("{}:\n", self.current_function().unwrap().entry_block));
            
            // Allocate space for parameters if needed
            for param in &func.parameters {
                let param_type_str = if param.param_type.is_empty() { "any" } else { &param.param_type };
                let param_type = self.map_cursed_type_to_llvm(param_type_str);
                let alloca = format!("  %{}_addr = alloca {}, align 8\n", param.name, param_type);
                ir.push_str(&alloca);
                
                let store = format!("  store {} %{}, {}* %{}_addr, align 8\n", 
                    param_type, param.name, param_type, param.name);
                ir.push_str(&store);
            }
            
            // Compile function body
            let body_ir = self.compile_block_statement(&func.body)?;
            ir.push_str(&body_ir);
            
            // Add implicit return if no explicit return at end
            if !body_ir.trim_end().ends_with("ret ") {
                if return_type == "void" {
                    ir.push_str("  ret void\n");
                } else if return_type == "i32" {
                    ir.push_str("  ret i32 0\n");
                } else {
                    ir.push_str(&format!("  ret {} zeroinitializer\n", return_type));
                }
            }
            
            ir.push_str("}\n");
        }
        
        self.pop_function_context();
        
        debug!("Function '{}' compiled successfully", func_name);
        Ok(ir)
    }
    
    #[instrument(skip(self, call))]
    fn compile_function_call(&mut self, call: &CallExpression) -> Result<String, Error> {
        let func_name = call.function.string();
        
        info!("Compiling function call: {}", func_name);
        
        // Compile arguments and determine their types
        let mut arg_types = Vec::new();
        let mut arg_llvm_names = Vec::new();
        
        for arg in &call.arguments {
            // For now, we'll use simplified expression compilation
            // In a full implementation, this would integrate with the expression compiler
            let arg_ir = self.compile_expression_body(arg)?;
            arg_types.push(LlvmType::Int32); // Simplified - would infer actual type
            arg_llvm_names.push("%temp_arg".to_string()); // Simplified
        }
        
        // Look up function signature in registry
        let function_signature = self.lookup_function_with_args(&func_name, &arg_types)
            .ok_or_else(|| {
                Error::CompilationError(format!(
                    "Function '{}' not found or no matching overload for {} arguments",
                    func_name, arg_types.len()
                ))
            })?;
        
        // Validate argument types
        function_signature.check_argument_types(&arg_types)?;
        
        // Generate call IR
        let call_result_temp = format!("%call_result_{}", self.next_temp_id());
        let args_str = function_signature.generate_call_arguments(&arg_llvm_names, &arg_types);
        
        let mut ir = String::new();
        
        // Generate the actual LLVM call instruction
        if function_signature.return_type == LlvmType::Void {
            ir.push_str(&format!("  call {} @{}({})\n", 
                function_signature.return_type.to_llvm_string(),
                func_name,
                args_str));
        } else {
            ir.push_str(&format!("  {} = call {} @{}({})\n", 
                call_result_temp,
                function_signature.return_type.to_llvm_string(),
                func_name,
                args_str));
        }
        
        debug!("Function call '{}' compiled successfully", func_name);
        Ok(ir)
    }
    
    fn compile_return_statement(&mut self, ret: &ReturnStatement) -> Result<String, Error> {
        let mut ir = String::new();
        
        if let Some(return_value) = &ret.return_value {
            // Compile the return value expression
            let value_ir = self.compile_expression_body(return_value)?;
            ir.push_str(&value_ir);
            
            // Get the value's LLVM representation
            let value_ref = "%return_val"; // Simplified
            
            // Determine return type from current function
            let return_type = if let Some(context) = self.current_function() {
                context.return_type.clone()
            } else {
                "i32".to_string()
            };
            
            ir.push_str(&format!("  ret {} {}\n", return_type, value_ref));
        } else {
            // Return void (yolo with no value)
            ir.push_str("  ret void\n");
        }
        
        Ok(ir)
    }
    
    fn generate_function_type(&self, params: &[crate::ast::expressions::Parameter], return_type: Option<&Box<dyn Expression>>) -> String {
        let param_types: Vec<String> = params.iter()
            .map(|p| {
                let param_type = if p.param_type.is_empty() { "any" } else { &p.param_type };
                self.map_cursed_type_to_llvm(param_type)
            })
            .collect();
        
        let ret_type = if let Some(rt) = return_type {
            self.map_cursed_type_to_llvm(&rt.string())
        } else {
            "void".to_string()
        };
        
        format!("{} ({})", ret_type, param_types.join(", "))
    }
    
    fn generate_function_arguments(&self, params: &[crate::ast::expressions::Parameter]) -> String {
        params.iter()
            .map(|p| {
                let param_type = if p.param_type.is_empty() { "any" } else { &p.param_type };
                let llvm_type = self.map_cursed_type_to_llvm(param_type);
                format!("{} %{}", llvm_type, p.name)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
    
    fn generate_call_arguments(&mut self, args: &[Box<dyn Expression>]) -> Result<String, Error> {
        let mut arg_strings = Vec::new();
        
        for arg in args {
            // Compile argument expression
            let arg_ir = self.compile_expression_body(arg)?;
            
            // For now, assume all arguments are i32 (simplified)
            let arg_type = "i32";
            let arg_value = "%arg_val"; // Simplified
            
            arg_strings.push(format!("{} {}", arg_type, arg_value));
        }
        
        Ok(arg_strings.join(", "))
    }
    
    fn allocate_local_variable(&mut self, name: &str, var_type: &str) -> Result<String, Error> {
        let llvm_type = self.map_cursed_type_to_llvm(var_type);
        let alloca_name = format!("%{}_addr", name);
        
        // Add to current function context
        if let Some(context) = self.current_function() {
            context.add_local(name.to_string(), alloca_name.clone());
        }
        
        let ir = format!("  {} = alloca {}, align 8\n", alloca_name, llvm_type);
        Ok(ir)
    }
    
    fn get_or_create_function(&mut self, name: &str, func_type: &str) -> String {
        // For now, just return the function name
        // In a full implementation, this would check if the function exists in the module
        format!("@{}", name)
    }
}

/// Helper methods for the LLVM code generator
impl crate::codegen::llvm::LlvmCodeGenerator {
    /// Map CURSED types to LLVM types
    fn map_cursed_type_to_llvm(&self, cursed_type: &str) -> String {
        match cursed_type {
            "int" | "i32" => "i32".to_string(),
            "i64" => "i64".to_string(),
            "float" | "f32" => "float".to_string(),
            "f64" | "double" => "double".to_string(),
            "bool" => "i1".to_string(),
            "string" | "str" => "i8*".to_string(), // Pointer to char array
            "void" => "void".to_string(),
            "any" => "i8*".to_string(), // Generic pointer for dynamic types
            _ => "i8*".to_string(), // Default to generic pointer
        }
    }
    
    /// Compile a block statement
    fn compile_block_statement(&mut self, block: &crate::ast::block::BlockStatement) -> Result<String, Error> {
        use crate::ast::traits::Statement;
        
        let mut ir = String::new();
        
        // Compile each statement in the block
        for statement in &block.statements {
            // Try to compile different statement types
            let stmt_ir = self.compile_statement_dispatch(statement.as_ref())?;
            ir.push_str(&stmt_ir);
        }
        
        // If the block is empty, add a comment
        if block.statements.is_empty() {
            ir.push_str("  ; Empty block\n");
        }
        
        Ok(ir)
    }
    
    /// Dispatch statement compilation to appropriate handler
    fn compile_statement_dispatch(&mut self, stmt: &dyn Statement) -> Result<String, Error> {
        use crate::ast::{LetStatement, FactsStatement, ExpressionStatement, ReturnStatement};
        
        // Try to downcast to specific statement types
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
            self.compile_let_statement(let_stmt)
        } else if let Some(facts_stmt) = stmt.as_any().downcast_ref::<FactsStatement>() {
            self.compile_facts_statement(facts_stmt)
        } else if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
            self.compile_expression_statement(expr_stmt)
        } else if let Some(ret_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
            self.compile_return_statement(ret_stmt)
        } else {
            // For unknown statement types, generate a comment
            Ok(format!("  ; Statement: {}\n", stmt.string()))
        }
    }
    
    /// Compile a let statement (variable declaration)
    fn compile_let_statement(&mut self, let_stmt: &crate::ast::LetStatement) -> Result<String, Error> {
        let var_name = &let_stmt.name.value;
        
        // Determine variable type
        let var_type = if let Some(type_annotation) = &let_stmt.type_annotation {
            self.map_cursed_type_to_llvm(&type_annotation.string())
        } else if let Some(value) = &let_stmt.value {
            // Infer type from initial value
            self.infer_type_from_expression(value.as_ref())?
        } else {
            "i8*".to_string() // Default to generic pointer
        };
        
        let mut ir = String::new();
        
        // Allocate local variable
        let alloca_ir = self.allocate_local_variable(var_name, &var_type)?;
        ir.push_str(&alloca_ir);
        
        // If there's an initial value, compile and store it
        if let Some(value) = &let_stmt.value {
            let value_ir = self.compile_expression_body(value)?;
            ir.push_str(&value_ir);
            
            // Store the value
            let store_ir = format!("  store {} %temp_val, {}* %{}_addr, align 8\n", 
                var_type, var_type, var_name);
            ir.push_str(&store_ir);
        }
        
        Ok(ir)
    }
    
    /// Compile a facts statement (constant declaration)
    fn compile_facts_statement(&mut self, facts_stmt: &crate::ast::FactsStatement) -> Result<String, Error> {
        let const_name = &facts_stmt.name.value;
        
        // Determine constant type
        let const_type = if let Some(type_annotation) = &facts_stmt.type_annotation {
            self.map_cursed_type_to_llvm(&type_annotation.string())
        } else {
            self.infer_type_from_expression(facts_stmt.value.as_ref())?
        };
        
        let mut ir = String::new();
        
        // Compile the constant value
        let value_ir = self.compile_expression_body(&facts_stmt.value)?;
        ir.push_str(&value_ir);
        
        // Create a local constant (using alloca + store for simplicity)
        let alloca_ir = format!("  %{}_addr = alloca {}, align 8\n", const_name, const_type);
        ir.push_str(&alloca_ir);
        
        let store_ir = format!("  store {} %temp_val, {}* %{}_addr, align 8\n", 
            const_type, const_type, const_name);
        ir.push_str(&store_ir);
        
        Ok(ir)
    }
    
    /// Compile an expression statement
    fn compile_expression_statement(&mut self, expr_stmt: &crate::ast::ExpressionStatement) -> Result<String, Error> {
        // Compile the expression and discard the result
        let expr_ir = self.compile_expression_body(&expr_stmt.expression)?;
        Ok(expr_ir)
    }
    
    /// Infer LLVM type from expression
    fn infer_type_from_expression(&self, expr: &dyn crate::ast::traits::Expression) -> Result<String, Error> {
        use crate::ast::expressions::{Literal, LiteralValue};
        
        // Try to infer type from literal values
        if let Some(literal) = expr.as_any().downcast_ref::<Literal>() {
            match &literal.value {
                LiteralValue::Integer(_) => Ok("i64".to_string()),
                LiteralValue::Float(_) => Ok("double".to_string()),
                LiteralValue::String(_) => Ok("i8*".to_string()),
                LiteralValue::Boolean(_) => Ok("i1".to_string()),
                LiteralValue::Nil => Ok("i8*".to_string()),
            }
        } else {
            // For complex expressions, default to generic pointer
            Ok("i8*".to_string())
        }
    }
    
    /// Compile an expression body
    fn compile_expression_body(&mut self, expr: &Box<dyn Expression>) -> Result<String, Error> {
        use crate::ast::expressions::{Literal, LiteralValue};
        use crate::ast::identifiers::Identifier;
        use crate::ast::operators::BinaryExpression;
        use crate::ast::calls::CallExpression;
        
        let mut ir = String::new();
        
        // Try to downcast to specific expression types
        if let Some(literal) = expr.as_any().downcast_ref::<Literal>() {
            ir.push_str(&self.compile_literal_expression(literal)?);
        } else if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            ir.push_str(&self.compile_identifier_expression(identifier)?);
        } else if let Some(binary) = expr.as_any().downcast_ref::<BinaryExpression>() {
            ir.push_str(&self.compile_binary_expression(binary)?);
        } else if let Some(call) = expr.as_any().downcast_ref::<CallExpression>() {
            ir.push_str(&self.compile_function_call(call)?);
        } else {
            // For unknown expression types, generate a placeholder
            ir.push_str(&format!("  ; Expression: {} (placeholder)\n  %temp_val = add i32 0, 0\n", expr.string()));
        }
        
        Ok(ir)
    }
    
    /// Compile a literal expression
    fn compile_literal_expression(&mut self, literal: &crate::ast::expressions::Literal) -> Result<String, Error> {
        use crate::ast::expressions::LiteralValue;
        
        match &literal.value {
            LiteralValue::Integer(val) => {
                Ok(format!("  ; Integer literal: {}\n  %temp_val = add i64 {}, 0\n", val, val))
            },
            LiteralValue::Float(val) => {
                Ok(format!("  ; Float literal: {}\n  %temp_val = fadd double {}, 0.0\n", val, val))
            },
            LiteralValue::String(val) => {
                // Generate a global string constant
                let string_id = format!("str_{}", self.get_next_string_id());
                let escaped_string = val.escape_default().to_string();
                
                Ok(format!(
                    "  ; String literal: \"{}\"\n  %temp_val = getelementptr inbounds [{} x i8], [{} x i8]* @{}, i32 0, i32 0\n",
                    val, val.len() + 1, val.len() + 1, string_id
                ))
            },
            LiteralValue::Boolean(val) => {
                let bool_val = if *val { 1 } else { 0 };
                Ok(format!("  ; Boolean literal: {}\n  %temp_val = add i1 {}, 0\n", val, bool_val))
            },
            LiteralValue::Nil => {
                Ok("  ; Nil literal\n  %temp_val = add i8* null, 0\n".to_string())
            },
        }
    }
    
    /// Compile an identifier expression (variable reference)
    fn compile_identifier_expression(&mut self, identifier: &crate::ast::identifiers::Identifier) -> Result<String, Error> {
        let var_name = &identifier.value;
        
        // Check if it's a local variable
        if let Some(context) = self.current_function() {
            if let Some(llvm_value) = context.get_local(var_name) {
                return Ok(format!("  ; Load variable: {}\n  %temp_val = load i8*, i8** {}\n", var_name, llvm_value));
            }
        }
        
        // Check if it's a parameter
        if let Some(context) = self.current_function() {
            for (i, param) in context.parameters.iter().enumerate() {
                if param == &format!("%{}", var_name) {
                    return Ok(format!("  ; Parameter reference: {}\n  %temp_val = add i8* %{}, 0\n", var_name, var_name));
                }
            }
        }
        
        // Default to global variable
        Ok(format!("  ; Global variable: {}\n  %temp_val = load i8*, i8** @{}\n", var_name, var_name))
    }
    
    /// Compile a binary expression
    fn compile_binary_expression(&mut self, binary: &crate::ast::operators::BinaryExpression) -> Result<String, Error> {
        let mut ir = String::new();
        
        // Compile left operand
        let left_ir = self.compile_expression_body(&binary.left)?;
        ir.push_str(&left_ir);
        ir.push_str("  %left_val = add i32 %temp_val, 0\n"); // Store left value
        
        // Compile right operand
        let right_ir = self.compile_expression_body(&binary.right)?;
        ir.push_str(&right_ir);
        ir.push_str("  %right_val = add i32 %temp_val, 0\n"); // Store right value
        
        // Generate operation based on operator
        let op_ir = match binary.operator.as_str() {
            "+" => "  %temp_val = add i32 %left_val, %right_val\n",
            "-" => "  %temp_val = sub i32 %left_val, %right_val\n",
            "*" => "  %temp_val = mul i32 %left_val, %right_val\n",
            "/" => "  %temp_val = sdiv i32 %left_val, %right_val\n",
            "%" => "  %temp_val = srem i32 %left_val, %right_val\n",
            "==" => "  %temp_val = icmp eq i32 %left_val, %right_val\n",
            "!=" => "  %temp_val = icmp ne i32 %left_val, %right_val\n",
            "<" => "  %temp_val = icmp slt i32 %left_val, %right_val\n",
            ">" => "  %temp_val = icmp sgt i32 %left_val, %right_val\n",
            "<=" => "  %temp_val = icmp sle i32 %left_val, %right_val\n",
            ">=" => "  %temp_val = icmp sge i32 %left_val, %right_val\n",
            "&&" => "  %temp_val = and i1 %left_val, %right_val\n",
            "||" => "  %temp_val = or i1 %left_val, %right_val\n",
            _ => "  ; Unknown operator\n  %temp_val = add i32 0, 0\n",
        };
        
        ir.push_str(&format!("  ; Binary operation: {}\n", binary.operator));
        ir.push_str(op_ir);
        
        Ok(ir)
    }
    
    /// Compile a call expression
    fn compile_call_expression(&mut self, call: &crate::ast::calls::CallExpression) -> Result<String, Error> {
        let func_name = call.function.string();
        
        // Compile arguments
        let mut args_ir = String::new();
        let mut arg_values = Vec::new();
        
        for (i, arg) in call.arguments.iter().enumerate() {
            let arg_ir = self.compile_expression_body(arg)?;
            args_ir.push_str(&arg_ir);
            
            let arg_temp = format!("%arg_{}", i);
            args_ir.push_str(&format!("  {} = add i32 %temp_val, 0\n", arg_temp));
            arg_values.push(format!("i32 {}", arg_temp));
        }
        
        let args_str = arg_values.join(", ");
        
        let mut ir = String::new();
        ir.push_str(&args_ir);
        ir.push_str(&format!("  ; Function call: {}\n", func_name));
        ir.push_str(&format!("  %temp_val = call i32 @{}({})\n", func_name, args_str));
        
        Ok(ir)
    }
    
    /// Get next string ID for global string constants
    fn get_next_string_id(&mut self) -> u64 {
        if let Some(context) = self.current_function() {
            context.temp_counter += 1;
            context.temp_counter as u64
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;
    use crate::ast::expressions::Parameter;
    use crate::ast::block::BlockStatement;
    
    #[test]
    fn test_function_context_creation() {
        let context = FunctionContext::new("test_func".to_string(), "i32".to_string());
        assert_eq!(context.name, "test_func");
        assert_eq!(context.return_type, "i32");
        assert_eq!(context.current_block, "test_func_entry");
        assert_eq!(context.entry_block, "test_func_entry");
    }
    
    #[test]
    fn test_function_context_locals() {
        let mut context = FunctionContext::new("test".to_string(), "void".to_string());
        context.add_local("x".to_string(), "%x_addr".to_string());
        
        assert_eq!(context.get_local("x"), Some(&"%x_addr".to_string()));
        assert_eq!(context.get_local("y"), None);
    }
    
    #[test]
    fn test_temp_variable_generation() {
        let mut context = FunctionContext::new("test".to_string(), "void".to_string());
        
        assert_eq!(context.next_temp(), "%temp0");
        assert_eq!(context.next_temp(), "%temp1");
        assert_eq!(context.next_temp(), "%temp2");
    }
    
    #[test]
    fn test_type_mapping() {
        let generator = crate::codegen::llvm::LlvmCodeGenerator::new().unwrap();
        
        assert_eq!(generator.map_cursed_type_to_llvm("int"), "i32");
        assert_eq!(generator.map_cursed_type_to_llvm("float"), "float");
        assert_eq!(generator.map_cursed_type_to_llvm("bool"), "i1");
        assert_eq!(generator.map_cursed_type_to_llvm("string"), "i8*");
        assert_eq!(generator.map_cursed_type_to_llvm("void"), "void");
        assert_eq!(generator.map_cursed_type_to_llvm("unknown"), "i8*");
    }
    
    #[test]
    fn test_function_type_generation() {
        let generator = crate::codegen::llvm::LlvmCodeGenerator::new().unwrap();
        
        let params = vec![
            Parameter::new("x".to_string(), "int".to_string()),
            Parameter::new("y".to_string(), "float".to_string()),
        ];
        
        let func_type = generator.generate_function_type(&params, None);
        assert_eq!(func_type, "void (i32, float)");
    }
    
    #[test]
    fn test_function_arguments_generation() {
        let generator = crate::codegen::llvm::LlvmCodeGenerator::new().unwrap();
        
        let params = vec![
            Parameter::new("x".to_string(), "int".to_string()),
            Parameter::new("name".to_string(), "string".to_string()),
        ];
        
        let args = generator.generate_function_arguments(&params);
        assert_eq!(args, "i32 %x, i8* %name");
    }
    
    #[test]
    fn test_simple_function_compilation() {
        let mut generator = crate::codegen::llvm::LlvmCodeGenerator::new().unwrap();
        
        // Create a simple function: slay main() { }
        let func = FunctionStatement::new(
            "slay".to_string(),
            Identifier::new("main".to_string(), "main".to_string()),
            vec![],
            None,
            BlockStatement::new("main_block".to_string(), vec![]),
        );
        
        let result = generator.compile_function_declaration(&func);
        assert!(result.is_ok());
        
        let ir = result.unwrap();
        assert!(ir.contains("define void @main()"));
        assert!(ir.contains("ret void"));
    }
}
