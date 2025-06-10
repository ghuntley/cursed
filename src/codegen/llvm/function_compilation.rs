/// LLVM function compilation for the CURSED programming language
use crate::ast::declarations::FunctionStatement;
use crate::ast::calls::CallExpression;
use crate::ast::statements::ReturnStatement;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::Error;
use std::collections::HashMap;

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
    fn compile_function_declaration(&mut self, func: &FunctionStatement) -> Result<String, Error> {
        let func_name = func.name.string();
        
        // Determine return type (default to void/i32 if not specified)
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
        
        // Function declaration
        ir.push_str(&format!("\n; Function: {} (slay keyword)\n", func_name));
        ir.push_str(&format!("define {} @{}({}) {{\n", return_type, func_name, args));
        
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
        
        self.pop_function_context();
        
        Ok(ir)
    }
    
    fn compile_function_call(&mut self, call: &CallExpression) -> Result<String, Error> {
        let func_name = call.function.string();
        
        // Generate arguments
        let args_ir = self.generate_call_arguments(&call.arguments)?;
        
        // Get or generate function type (simplified for now)
        let func_type = "i32 (...)".to_string(); // Variadic for simplicity
        
        // Generate call instruction
        let temp = if let Some(context) = self.current_function() {
            context.next_temp()
        } else {
            "%temp0".to_string()
        };
        
        let mut ir = String::new();
        ir.push_str(&format!("  {} = call {} @{}({})\n", temp, func_type.split(' ').next().unwrap_or("i32"), func_name, args_ir));
        
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
    
    /// Compile a block statement (placeholder)
    fn compile_block_statement(&mut self, _block: &crate::ast::block::BlockStatement) -> Result<String, Error> {
        // This would compile all statements in the block
        // For now, return empty implementation
        Ok("  ; Block statements would be compiled here\n".to_string())
    }
    
    /// Compile an expression body (placeholder)
    fn compile_expression_body(&mut self, _expr: &Box<dyn Expression>) -> Result<String, Error> {
        // This would compile the expression and return its LLVM IR
        // For now, return a simple constant
        Ok("  ; Expression compilation placeholder\n".to_string())
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
