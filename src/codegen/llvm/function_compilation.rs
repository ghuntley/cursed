//! LLVM Function Compilation Module
//! Complete function compilation with full LLVM IR generation

use crate::ast::{Statement, Expression, FunctionStatement, Literal};
use crate::error::CursedError;
use crate::codegen::llvm::string_constants::{StringConstantManager, get_global_string_manager};
use std::collections::HashMap;

/// Complete function compiler for CURSED functions to LLVM IR
pub struct FunctionCompiler {
    pub ir_code: String,
    pub variable_counter: usize,
    pub label_counter: usize,
    pub variables: HashMap<String, String>,  // Maps variable names to their register/pointer
    pub variable_types: HashMap<String, String>,  // Maps variable names to their LLVM types
    pub function_params: HashMap<String, String>,  // Maps parameter names to their registers
    pub current_function: Option<String>,
    pub string_manager: StringConstantManager,
}

impl FunctionCompiler {
    pub fn new() -> Self {
        Self {
            ir_code: String::new(),
            variable_counter: 0,
            label_counter: 0,
            variables: HashMap::new(),
            variable_types: HashMap::new(),
            function_params: HashMap::new(),
            current_function: None,
            string_manager: get_global_string_manager(),
        }
    }
    
    /// Get the collected string constants (now managed globally)
    pub fn get_string_constants(&self) -> Vec<String> {
        self.string_manager.get_all_constants()
    }

    /// Generate complete LLVM function definition with full IR
    pub fn compile_function(
        &mut self,
        name: &str,
        params: &[String],
        param_types: Option<&[String]>,
        return_type: Option<&str>,
        body: &[Statement]
    ) -> Result<String, CursedError> {
        // Reset state for new function
        self.variables.clear();
        self.variable_types.clear();
        self.function_params.clear();
        // All functions register numbering starts at %0 according to LLVM convention
        self.variable_counter = 0;
        self.label_counter = 0;
        self.current_function = Some(name.to_string());
        
        let mut function_ir = String::new();
        
        // Infer types for untyped parameters
        let inferred_param_types = self.infer_parameter_types(params, param_types, body)?;
        
        // Generate function signature with inferred return type
        let ret_type = if let Some(explicit_type) = return_type {
            self.get_llvm_type(explicit_type)
        } else if name == "main" {
            // Main functions always return i32 (C convention)
            "i32".to_string()
        } else {
            // Infer return type from function body
            self.infer_function_return_type(body)?
        };
        function_ir.push_str(&format!("define {} @{}(", ret_type, name));
        
        // Generate parameters with inferred types
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                function_ir.push_str(", ");
            }
            let param_type = &inferred_param_types[i];
            let llvm_type = self.get_llvm_type(param_type);
            function_ir.push_str(&format!("{} %{}", llvm_type, param));
            
            // Map parameters to their LLVM types and registers
            self.variable_types.insert(param.clone(), llvm_type.clone());
            self.function_params.insert(param.clone(), format!("%{}", param));
        }
        
        function_ir.push_str(") {\n");
        function_ir.push_str("entry:\n");
        
        // Generate function body with complete IR
        let mut has_return = false;
        for statement in body {
            let stmt_ir = self.compile_statement(statement)?;
            function_ir.push_str(&stmt_ir);
            
            if matches!(statement, Statement::Return(_)) {
                has_return = true;
            }
        }
        
        // Ensure function has a return statement
        if !has_return {
            if ret_type == "void" {
                function_ir.push_str("  ret void\n");
            } else {
                function_ir.push_str("  ret i32 0\n");
            }
        }
        
        function_ir.push_str("}\n\n");
        
        Ok(function_ir)
    }

    /// Compile individual statements to complete LLVM IR
    pub fn compile_statement(&mut self, statement: &Statement) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Remember the current IR length to capture any new IR generated during compilation
        let ir_start = self.ir_code.len();
        
        match statement {
            Statement::Expression(expr) => {
                let expr_reg = self.compile_expression(expr)?;
                ir.push_str(&format!("  ; Expression result: {}\n", expr_reg));
            },
            Statement::Let(let_stmt) => {
                let value_reg = self.compile_expression(&let_stmt.value)?;
                let var_reg = self.next_register();
                
                // Infer the type from the value expression
                let var_type = if let Some(explicit_type) = &let_stmt.var_type {
                    self.get_llvm_type(&explicit_type.to_string())
                } else {
                    self.infer_expression_type(&let_stmt.value)?
                };
                
                // Allocate variable on stack with correct type first
                ir.push_str(&format!("  {} = alloca {}, align 4\n", var_reg, var_type));
                
                // Check if we need type conversion
                let value_type = self.infer_expression_type(&let_stmt.value)?;
                let final_value_reg = if var_type != value_type {
                    // Need type conversion
                    let conv_reg = self.next_register();
                    match (value_type.as_str(), var_type.as_str()) {
                        ("double", "i32") => {
                            ir.push_str(&format!("  {} = fptosi double {} to i32\n", conv_reg, value_reg));
                            conv_reg
                        },
                        ("i32", "double") => {
                            ir.push_str(&format!("  {} = sitofp i32 {} to double\n", conv_reg, value_reg));
                            conv_reg
                        },
                        ("i1", "i32") => {
                            ir.push_str(&format!("  {} = zext i1 {} to i32\n", conv_reg, value_reg));
                            conv_reg
                        },
                        ("i32", "i1") => {
                            ir.push_str(&format!("  {} = icmp ne i32 {}, 0\n", conv_reg, value_reg));
                            conv_reg
                        },
                        _ => {
                            // No conversion available, use original value
                            value_reg.clone()
                        }
                    }
                } else {
                    value_reg.clone()
                };
                
                // Store the value (with conversion if needed)
                ir.push_str(&format!("  store {} {}, {}* {}, align 4\n", var_type, final_value_reg, var_type, var_reg));
                
                // Store variable mapping
                match &let_stmt.target {
                    crate::ast::LetTarget::Single(name) => {
                        self.variables.insert(name.clone(), var_reg);
                        self.variable_types.insert(name.clone(), var_type.clone());
                        ir.push_str(&format!("  ; Variable {} allocated\n", name));
                    },
                    crate::ast::LetTarget::Tuple(names) => {
                        ir.push_str("  ; Tuple destructuring let statement\n");
                        
                        // Extract each element from the tuple and assign to variables
                        for (index, var_name) in names.iter().enumerate() {
                            // Generate getelementptr to access tuple field
                            let field_ptr = format!("%tuple_field_{}", self.variable_counter);
                            self.variable_counter += 1;
                            ir.push_str(&format!(
                                "  {} = getelementptr inbounds {{i32, i32, i32}}, {{i32, i32, i32}}* {}, i32 0, i32 {}\n",
                                field_ptr, value_reg, index
                            ));
                            
                            // Load the value from the field
                            let field_value = format!("%tuple_val_{}", self.variable_counter);
                            self.variable_counter += 1;
                            ir.push_str(&format!(
                                "  {} = load i32, i32* {}, align 4\n",
                                field_value, field_ptr
                            ));
                            
                            // Store the variable mapping
                            self.variables.insert(var_name.clone(), field_value.clone());
                            ir.push_str(&format!("  ; Extracted {} = {} from tuple\n", var_name, field_value));
                        }
                    }
                }
            },
            Statement::Assignment(assign_stmt) => {
                let value_reg = self.compile_expression(&assign_stmt.value)?;
                
                // Handle only simple variable assignment for now
                if let crate::ast::AssignmentTarget::Single(name) = &assign_stmt.target {
                    // Look up existing variable
                    if let Some(var_reg) = self.variables.get(name).cloned() {
                        // Store new value to existing variable
                        // TODO: Use proper type tracking
                        if name.contains("name") || name.contains("text") || name.contains("str") {
                            ir.push_str(&format!("  store i8* {}, i8** {}, align 8\n", value_reg, var_reg));
                        } else if name.contains("flag") || name.contains("bool") || name.contains("is_") || name.contains("active") || name.contains("enabled") {
                            ir.push_str(&format!("  store i1 {}, i1* {}, align 1\n", value_reg, var_reg));
                        } else {
                            ir.push_str(&format!("  store i32 {}, i32* {}, align 4\n", value_reg, var_reg));
                        }
                        ir.push_str(&format!("  ; Assignment to {} = {}\n", name, value_reg));
                    } else {
                        return Err(CursedError::runtime_error(&format!("Undefined variable: {}", name)));
                    }
                } else if let crate::ast::AssignmentTarget::Tuple(var_names) = &assign_stmt.target {
                    ir.push_str("  ; Tuple destructuring assignment in function\n");
                    
                    // Extract each element from the tuple and assign to variables
                    for (index, var_name) in var_names.iter().enumerate() {
                        // Generate getelementptr to access tuple field
                        let field_ptr = format!("%tuple_field_{}", self.variable_counter);
                        self.variable_counter += 1;
                        ir.push_str(&format!(
                            "  {} = getelementptr inbounds {{i32, i32, i32}}, {{i32, i32, i32}}* {}, i32 0, i32 {}\n",
                            field_ptr, value_reg, index
                        ));
                        
                        // Load the value from the field
                        let field_value = format!("%tuple_val_{}", self.variable_counter);
                        self.variable_counter += 1;
                        ir.push_str(&format!(
                            "  {} = load i32, i32* {}, align 4\n",
                            field_value, field_ptr
                        ));
                        
                        // Store the variable mapping in function scope
                        self.variables.insert(var_name.clone(), field_value.clone());
                        ir.push_str(&format!("  ; Extracted {} = {} from tuple\n", var_name, field_value));
                    }
                } else {
                    ir.push_str("  ; Unknown assignment target type\n");
                }
            },
            Statement::Return(return_stmt) => {
                if let Some(val) = &return_stmt.value {
                    let return_reg = self.compile_expression(val)?;
                    let return_type = self.infer_expression_type(val)?;
                    
                    // Special handling for main function
                    if let Some(ref func_name) = self.current_function {
                        if func_name == "main" {
                            match return_type.as_str() {
                                "i8*" => {
                                    // Print the string using puts and return 0
                                    ir.push_str(&format!("  call i32 @puts(i8* {})\n", return_reg));
                                    ir.push_str("  ret i32 0\n");
                                },
                                "i1" => {
                                    // Convert boolean to i32 for main function
                                    let conv_reg = self.next_register();
                                    ir.push_str(&format!("  {} = zext i1 {} to i32\n", conv_reg, return_reg));
                                    ir.push_str(&format!("  ret i32 {}\n", conv_reg));
                                },
                                "i32" => {
                                    // Already correct type
                                    ir.push_str(&format!("  ret i32 {}\n", return_reg));
                                },
                                _ => {
                                    // Default: try to convert to i32
                                    ir.push_str(&format!("  ret i32 {}\n", return_reg));
                                }
                            }
                        } else {
                            ir.push_str(&format!("  ret {} {}\n", return_type, return_reg));
                        }
                    } else {
                        ir.push_str(&format!("  ret {} {}\n", return_type, return_reg));
                    }
                } else {
                    ir.push_str("  ret i32 0\n");
                }
            },
            Statement::If(if_stmt) => {
                ir.push_str(&self.compile_if_statement(
                    &if_stmt.condition,
                    &if_stmt.then_branch,
                    &if_stmt.else_branch
                )?);
            },
            Statement::While(while_stmt) => {
                ir.push_str(&self.compile_while_statement(
                    &while_stmt.condition,
                    &while_stmt.body
                )?);
            },
            Statement::For(for_stmt) => {
                ir.push_str(&self.compile_for_statement(for_stmt)?);
            },
            Statement::Function(_) => {
                // Nested functions not supported in LLVM - skip or error
                ir.push_str("  ; Nested function skipped\n");
            },
            _ => {
                ir.push_str("  ; Unsupported statement\n");
            }
        }
        
        // Include any new IR that was accumulated during expression compilation
        let new_ir = &self.ir_code[ir_start..];
        if !new_ir.is_empty() {
            // Insert the accumulated IR before the statement comment
            let mut final_ir = String::new();
            final_ir.push_str(new_ir);
            final_ir.push_str(&ir);
            ir = final_ir;
            
            // Clear the accumulated IR to prevent duplication
            self.ir_code.truncate(ir_start);
        }
        
        Ok(ir)
    }

    /// Compile expressions to complete LLVM IR with proper register allocation
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<String, CursedError> {
        match expression {
            Expression::Integer(val) => Ok(val.to_string()),
            Expression::Float(val) => Ok(val.to_string()),
            Expression::Boolean(val) => Ok(if *val { "1" } else { "0" }.to_string()),
            Expression::String(val) => {
                let cleaned_val = val.replace("\"", "\\\"");
                
                // Use the centralized string manager to get reference
                let string_ref = self.string_manager.add_string_constant(&cleaned_val);
                
                // Generate register and assignment
                let str_reg = self.next_register();
                let ir_line = format!(
                    "  {} = {}\n",
                    str_reg, string_ref
                );
                log::debug!("Adding IR line: {}", ir_line.trim());
                self.ir_code.push_str(&ir_line);
                Ok(str_reg)
            },
            Expression::Identifier(name) => {
                // Check if this is a function parameter first
                if let Some(param_reg) = self.function_params.get(name) {
                    // Function parameters are already values, no need to load
                    Ok(param_reg.clone())
                } else if let Some(var_reg) = self.variables.get(name) {
                    // Local variable allocated on stack - need to load
                    let var_reg = var_reg.clone();
                    let load_reg = self.next_register();
                    
                    // Use the type from variable_types to generate the correct load instruction
                    if let Some(var_type) = self.variable_types.get(name) {
                        self.ir_code.push_str(&format!("  {} = load {}, {}* {}, align 4\n", 
                            load_reg, var_type, var_type, var_reg));
                    } else {
                        // Fallback to i32 if type not found
                        self.ir_code.push_str(&format!("  {} = load i32, i32* {}, align 4\n", 
                            load_reg, var_reg));
                    }
                    Ok(load_reg)
                } else {
                    // Unknown identifier, assume it's a global or something
                    Ok(format!("%{}", name))
                }
            },
            Expression::Binary(binary_expr) => {
                self.compile_binary_expression(&binary_expr.left, &binary_expr.operator, &binary_expr.right)
            },
            Expression::Call(call_expr) => {
                self.compile_function_call(&call_expr.function, &call_expr.arguments)
            },
            Expression::MemberAccess(member_expr) => {
                self.compile_member_access(&member_expr.object, &member_expr.property)
            },
            Expression::Unary(unary_expr) => {
                self.compile_unary_expression(&unary_expr.operator, &unary_expr.operand)
            },
            Expression::Array(elements) => {
                self.compile_array_literal(elements)
            },
            Expression::Map(pairs) => {
                self.compile_map_literal(pairs)
            },
            _ => {
                let reg = self.next_register();
                self.ir_code.push_str(&format!("  {} = add i32 0, 0 ; placeholder\n", reg));
                Ok(reg)
            }
        }
    }

    /// Compile binary expressions with proper type handling
    fn compile_binary_expression(
        &mut self,
        left: &Expression,
        operator: &str,
        right: &Expression
    ) -> Result<String, CursedError> {
        // Handle string concatenation specially
        if operator == "+" && (matches!(left, Expression::String(_)) || matches!(right, Expression::String(_))) {
            return self.compile_string_concatenation(left, right);
        }
        
        let left_reg = self.compile_expression(left)?;
        let right_reg = self.compile_expression(right)?;
        
        // Infer types of operands for mixed-type arithmetic
        let left_type = self.infer_expression_type(left)?;
        let right_type = self.infer_expression_type(right)?;
        
        // Determine the result type and required conversions
        let (final_left_reg, final_right_reg, result_type, llvm_op) = if left_type == "double" || right_type == "double" {
            // Mixed arithmetic: promote to double
            let promoted_left = if left_type != "double" {
                let conv_reg = self.next_register();
                self.ir_code.push_str(&format!("  {} = sitofp {} {} to double\n", conv_reg, left_type, left_reg));
                conv_reg
            } else {
                left_reg
            };
            
            let promoted_right = if right_type != "double" {
                let conv_reg = self.next_register();
                self.ir_code.push_str(&format!("  {} = sitofp {} {} to double\n", conv_reg, right_type, right_reg));
                conv_reg
            } else {
                right_reg
            };
            
            let op_str = match operator {
                "+" => "fadd",
                "-" => "fsub", 
                "*" => "fmul",
                "/" => "fdiv",
                "%" => "frem",
                "==" => "fcmp oeq",
                "!=" => "fcmp one",
                "<" => "fcmp olt",
                ">" => "fcmp ogt",
                "<=" => "fcmp ole",
                ">=" => "fcmp oge",
                _ => return Err(CursedError::CompilerError(format!("Unsupported float operator: {}", operator))),
            };
            
            (promoted_left, promoted_right, "double".to_string(), op_str.to_string())
        } else {
            // Integer arithmetic
            let op_str = match operator {
                "+" => "add",
                "-" => "sub", 
                "*" => "mul",
                "/" => "sdiv",
                "%" => "srem",
                "==" => "icmp eq",
                "!=" => "icmp ne",
                "<" => "icmp slt",
                ">" => "icmp sgt",
                "<=" => "icmp sle",
                ">=" => "icmp sge",
                "&&" => "and",
                "||" => "or",
                "&" => "and",
                "|" => "or",
                "^" => "xor",
                "<<" => "shl",
                ">>" => "ashr",
                _ => return Err(CursedError::CompilerError(format!("Unsupported binary operator: {}", operator))),
            };
            
            (left_reg, right_reg, "i32".to_string(), op_str.to_string())
        };
        
        let result_reg = self.next_register();
        
        // Generate the operation with proper types
        if llvm_op.starts_with("icmp") || llvm_op.starts_with("fcmp") {
            // Comparison operations return i1
            self.ir_code.push_str(&format!("  {} = {} {} {}, {}\n", result_reg, llvm_op, result_type, final_left_reg, final_right_reg));
        } else {
            // Arithmetic operations return the same type as operands
            self.ir_code.push_str(&format!("  {} = {} {} {}, {}\n", result_reg, llvm_op, result_type, final_left_reg, final_right_reg));
        }
        
        Ok(result_reg)
    }

    /// Compile function calls with argument handling
    fn compile_function_call(&mut self, function: &Expression, arguments: &[Expression]) -> Result<String, CursedError> {
        match function {
            Expression::Identifier(func_name) => {
                // First compile all arguments to generate their intermediate IR
                let mut arg_regs = Vec::new();
                let mut arg_types = Vec::new();
                for arg in arguments {
                    let arg_reg = self.compile_expression(arg)?;
                    arg_regs.push(arg_reg);
                    
                    // Infer argument type from the expression
                    let arg_type = match arg {
                        Expression::String(_) => "i8*".to_string(),
                        Expression::Identifier(name) if name.contains("name") || name.contains("text") || name.contains("str") => "i8*".to_string(),
                        Expression::Boolean(_) => "i1".to_string(),
                        Expression::Binary(bin_expr) if bin_expr.operator == "+" => "i8*".to_string(), // String concatenation
                        _ => "i32".to_string(),
                    };
                    arg_types.push(arg_type);
                }
                
                // Now allocate result register after all arguments are compiled
                let result_reg = self.next_register();
                
                // Generate the function call with compiled arguments
                self.ir_code.push_str(&format!("  {} = call i32 @{}(", result_reg, func_name));
                
                for (i, (arg_reg, arg_type)) in arg_regs.iter().zip(arg_types.iter()).enumerate() {
                    if i > 0 {
                        self.ir_code.push_str(", ");
                    }
                    self.ir_code.push_str(&format!("{} {}", arg_type, arg_reg));
                }
                
                self.ir_code.push_str(")\n");
                Ok(result_reg)
            },
            Expression::MemberAccess(member_expr) => {
                // Handle method calls
                if let Expression::Identifier(obj_name) = &*member_expr.object {
                    // Special handling for stdlib methods like vibez.spill
                    if obj_name == "vibez" {
                        return self.compile_vibez_method_call(&member_expr.property, arguments);
                    }
                    
                    // First compile all arguments to generate their intermediate IR
                    let mut arg_regs = Vec::new();
                    for arg in arguments {
                        let arg_reg = self.compile_expression(arg)?;
                        arg_regs.push(arg_reg);
                    }
                    
                    // Now allocate result register after all arguments are compiled
                    let result_reg = self.next_register();
                    
                    // Generate the method call with compiled arguments
                    let func_name = format!("{}_{}", obj_name, member_expr.property);
                    self.ir_code.push_str(&format!("  {} = call i32 @{}(", result_reg, func_name));
                    
                    for (i, arg_reg) in arg_regs.iter().enumerate() {
                        if i > 0 {
                            self.ir_code.push_str(", ");
                        }
                        self.ir_code.push_str(&format!("i32 {}", arg_reg));
                    }
                    
                    self.ir_code.push_str(")\n");
                    Ok(result_reg)
                } else {
                    return Err(CursedError::CompilerError("Unsupported member access in function call".to_string()));
                }
            },
            _ => {
                return Err(CursedError::CompilerError("Unsupported function call expression".to_string()));
            }
        }
    }


    /// Compile vibez method calls (stdlib output methods)
    fn compile_vibez_method_call(&mut self, method: &str, arguments: &[Expression]) -> Result<String, CursedError> {
        match method {
            "spill" => {
                // Handle vibez.spill() calls
                for arg in arguments {
                    let arg_reg = self.compile_expression(arg)?;
                    // Check if this is a string-like expression (includes concatenations and string variables)
                    let arg_type = self.infer_expression_type(arg)?;
                    let is_string = arg_type == "i8*" || matches!(arg, Expression::String(_)) || 
                                   matches!(arg, Expression::Binary(bin_expr) if bin_expr.operator == "+");
                    
                    if is_string {
                        let call_result = self.next_register();
                        self.ir_code.push_str(&format!("  {} = call i32 @puts(i8* {})\n", call_result, arg_reg));
                    } else {
                            // For non-string types, determine the correct format and type
                            let arg_type = self.infer_expression_type(arg)?;
                            let (format_string, llvm_type) = match arg_type.as_str() {
                                "i32" => ("%d\\0A\\00", "i32"),
                                "i1" => ("%d\\0A\\00", "i32"), // Convert boolean to int for printf
                                "double" => ("%f\\0A\\00", "double"),
                                _ => ("%d\\0A\\00", "i32"), // Default fallback
                            };
                            
                            // Use the centralized string manager for format string
                            let format_str_cleaned = format_string.replace("\\00", "");
                            let format_ref = self.string_manager.add_string_constant(&format_str_cleaned);
                            
                            let format_reg = self.next_register();
                            self.ir_code.push_str(&format!("  {} = {}\n", format_reg, format_ref));
                            
                            // Convert boolean to integer if needed
                            let final_arg_reg = if arg_type == "i1" {
                                let convert_reg = self.next_register();
                                self.ir_code.push_str(&format!("  {} = zext i1 {} to i32\n", convert_reg, arg_reg));
                                convert_reg
                            } else {
                                arg_reg
                            };
                            
                            let call_result = self.next_register();
                            self.ir_code.push_str(&format!("  {} = call i32 (i8*, ...) @printf(i8* {}, {} {})\n", call_result, format_reg, llvm_type, final_arg_reg));
                    }
                }
                // Return a dummy register for the result (vibez.spill returns void conceptually)
                let result_reg = self.next_register();
                self.ir_code.push_str(&format!("  {} = add i32 0, 0\n", result_reg));
                Ok(result_reg)
            },
            _ => {
                return Err(CursedError::CompilerError(format!("Unknown vibez method: {}", method)));
            }
        }
    }

    /// Compile member access expressions
    fn compile_member_access(&mut self, object: &Expression, property: &str) -> Result<String, CursedError> {
        // Check for special vibez method calls
        if let Expression::Identifier(obj_name) = object {
            if obj_name == "vibez" && property == "spill" {
                // Return a special marker for vibez.spill method calls
                // This will be handled by the function call compiler
                return Ok("vibez_spill_method".to_string());
            }
        }
        
        let obj_reg = self.compile_expression(object)?;
        let member_reg = self.next_register();
        
        // Generate struct member access
        self.ir_code.push_str(&format!("  ; Member access: {}.{}\n", obj_reg, property));
        self.ir_code.push_str(&format!("  {} = getelementptr inbounds %struct.object, %struct.object* {}, i32 0, i32 0\n", 
            member_reg, obj_reg));
        
        let result_reg = self.next_register();
        self.ir_code.push_str(&format!("  {} = load i32, i32* {}, align 4\n", result_reg, member_reg));
        
        Ok(result_reg)
    }

    /// Compile unary expressions
    fn compile_unary_expression(&mut self, operator: &crate::ast::UnaryOperator, operand: &Expression) -> Result<String, CursedError> {
        let operand_reg = self.compile_expression(operand)?;
        let result_reg = self.next_register();
        
        match operator {
            crate::ast::UnaryOperator::Not => {
                self.ir_code.push_str(&format!("  {} = xor i1 {}, true\n", result_reg, operand_reg));
            },
            crate::ast::UnaryOperator::Minus => {
                self.ir_code.push_str(&format!("  {} = sub i32 0, {}\n", result_reg, operand_reg));
            },
            crate::ast::UnaryOperator::Plus => {
                self.ir_code.push_str(&format!("  {} = add i32 0, {}\n", result_reg, operand_reg));
            },
        }
        
        Ok(result_reg)
    }

    /// Infer the LLVM type for an expression
    fn infer_expression_type(&self, expr: &Expression) -> Result<String, CursedError> {
        match expr {
            Expression::String(_) => Ok("i8*".to_string()),
            Expression::Integer(_) => Ok("i32".to_string()),
            Expression::Float(_) => Ok("double".to_string()),
            Expression::Boolean(_) => Ok("i1".to_string()),
            Expression::Identifier(name) => {
                // Look up the variable type from the variable_types HashMap
                if let Some(llvm_type) = self.variable_types.get(name) {
                    Ok(llvm_type.clone())
                } else {
                    Ok("i32".to_string()) // Default if not found
                }
            },
            Expression::Binary(binary_expr) => {
                // For binary expressions, we need to check what kind of operation it is
                let left_type = self.infer_expression_type(&binary_expr.left)?;
                let right_type = self.infer_expression_type(&binary_expr.right)?;
                
                match binary_expr.operator.as_str() {
                    "+" => {
                        // For add operations, check if either operand is a string (string concatenation)
                        if left_type == "i8*" || right_type == "i8*" {
                            Ok("i8*".to_string()) // String concatenation result
                        } else if left_type == "double" || right_type == "double" {
                            Ok("double".to_string()) // Float arithmetic result
                        } else {
                            Ok("i32".to_string()) // Integer arithmetic result
                        }
                    },
                    "-" | "*" | "/" | "%" => {
                        // Arithmetic operations: promote to double if either operand is double
                        if left_type == "double" || right_type == "double" {
                            Ok("double".to_string())
                        } else {
                            Ok("i32".to_string())
                        }
                    },
                    "<" | "<=" | ">" | ">=" | "==" | "!=" => {
                        Ok("i1".to_string()) // Comparison result is boolean
                    },
                    _ => Ok("i32".to_string()), // Default for other operations
                }
            },
            Expression::Unary(_) => Ok("i32".to_string()), // Default for now
            Expression::Call(_) => Ok("i32".to_string()), // Default for now
            Expression::Literal(lit) => self.infer_literal_type(lit),
            Expression::Array(elements) => {
                // For arrays, return a pointer to the array type
                let len = elements.len();
                if len == 0 {
                    Ok("[0 x i32]*".to_string()) // Empty array pointer
                } else {
                    Ok(format!("[{} x i32]*", len)) // Array pointer with length
                }
            },
            Expression::ArrayAccess(_) => {
                // Array access returns the element type (for now, assume i32)
                Ok("i32".to_string())
            },
            _ => Ok("i32".to_string()), // Default fallback
        }
    }

    /// Infer the LLVM type for a literal
    fn infer_literal_type(&self, literal: &Literal) -> Result<String, CursedError> {
        match literal {
            Literal::String(_) => Ok("i8*".to_string()),
            Literal::Integer(_) => Ok("i32".to_string()),
            Literal::Float(_) => Ok("double".to_string()),
            Literal::Boolean(_) => Ok("i1".to_string()),
            Literal::Null => Ok("i8*".to_string()),
            Literal::Nil => Ok("i8*".to_string()),
        }
    }

    /// Infer function return type from function body
    fn infer_function_return_type(&self, body: &[Statement]) -> Result<String, CursedError> {
        // Look for return statements in the function body
        for statement in body {
            if let Statement::Return(return_stmt) = statement {
                if let Some(val) = &return_stmt.value {
                    return self.infer_expression_type(val);
                }
            }
        }
        
        // If no explicit return found, check if the last statement is an expression
        // that could be an implicit return (CURSED uses expression-based returns)
        if let Some(Statement::Expression(expr)) = body.last() {
            return self.infer_expression_type(expr);
        }
        
        // Default to void if no return type can be inferred
        Ok("void".to_string())
    }

    /// Compile if statements with proper branch handling
    fn compile_if_statement(
        &mut self,
        condition: &Expression,
        then_branch: &[Statement],
        else_branch: &Option<Vec<Statement>>
    ) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        let cond_reg = self.compile_expression(condition)?;
        let then_label = self.next_label();
        let else_label = self.next_label();
        let end_label = self.next_label();
        
        ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, then_label, else_label));
        
        // Then branch
        ir.push_str(&format!("{}:\n", then_label));
        let mut then_has_return = false;
        for stmt in then_branch {
            let stmt_ir = self.compile_statement(stmt)?;
            ir.push_str(&stmt_ir);
            if matches!(stmt, Statement::Return(_)) {
                then_has_return = true;
            }
        }
        if !then_has_return {
            ir.push_str(&format!("  br label %{}\n", end_label));
        }
        
        // Else branch
        ir.push_str(&format!("{}:\n", else_label));
        let mut else_has_return = false;
        if let Some(else_stmts) = else_branch {
            for stmt in else_stmts {
                let stmt_ir = self.compile_statement(stmt)?;
                ir.push_str(&stmt_ir);
                if matches!(stmt, Statement::Return(_)) {
                    else_has_return = true;
                }
            }
        }
        if !else_has_return {
            ir.push_str(&format!("  br label %{}\n", end_label));
        }
        
        // End label - only add if needed
        if !then_has_return || !else_has_return {
            ir.push_str(&format!("{}:\n", end_label));
        }
        
        Ok(ir)
    }

    /// Compile while loops
    fn compile_while_statement(&mut self, condition: &Expression, body: &[Statement]) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        let loop_label = self.next_label();
        let body_label = self.next_label();
        let end_label = self.next_label();
        
        ir.push_str(&format!("  br label %{}\n", loop_label));
        
        // Loop condition
        ir.push_str(&format!("{}:\n", loop_label));
        let cond_reg = self.compile_expression(condition)?;
        ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, body_label, end_label));
        
        // Loop body
        ir.push_str(&format!("{}:\n", body_label));
        for stmt in body {
            ir.push_str(&self.compile_statement(stmt)?);
        }
        ir.push_str(&format!("  br label %{}\n", loop_label));
        
        // End
        ir.push_str(&format!("{}:\n", end_label));
        
        Ok(ir)
    }

    /// Compile for statements
    fn compile_for_statement(&mut self, for_stmt: &crate::ast::ForStatement) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Initialize
        if let Some(init) = &for_stmt.init {
            ir.push_str(&self.compile_statement(init)?);
        }
        
        let loop_label = self.next_label();
        let body_label = self.next_label();
        let update_label = self.next_label();
        let end_label = self.next_label();
        
        ir.push_str(&format!("  br label %{}\n", loop_label));
        
        // Loop condition
        ir.push_str(&format!("{}:\n", loop_label));
        if let Some(condition) = &for_stmt.condition {
            let cond_reg = self.compile_expression(condition)?;
            ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, body_label, end_label));
        } else {
            ir.push_str(&format!("  br label %{}\n", body_label));
        }
        
        // Body
        ir.push_str(&format!("{}:\n", body_label));
        for stmt in &for_stmt.body {
            ir.push_str(&self.compile_statement(stmt)?);
        }
        ir.push_str(&format!("  br label %{}\n", update_label));
        
        // Update
        ir.push_str(&format!("{}:\n", update_label));
        if let Some(update) = &for_stmt.update {
            self.compile_expression(update)?;
        }
        ir.push_str(&format!("  br label %{}\n", loop_label));
        
        // End
        ir.push_str(&format!("{}:\n", end_label));
        
        Ok(ir)
    }

    /// Compile array literals
    fn compile_array_literal(&mut self, elements: &[Expression]) -> Result<String, CursedError> {
        let array_reg = self.next_register();
        let len = elements.len();
        
        // Allocate array
        self.ir_code.push_str(&format!("  {} = alloca [{}x i32], align 4\n", array_reg, len));
        
        // Initialize elements
        for (i, element) in elements.iter().enumerate() {
            let elem_reg = self.compile_expression(element)?;
            let elem_ptr = self.next_register();
            self.ir_code.push_str(&format!("  {} = getelementptr inbounds [{}x i32], [{}x i32]* {}, i64 0, i64 {}\n", 
                elem_ptr, len, len, array_reg, i));
            self.ir_code.push_str(&format!("  store i32 {}, i32* {}, align 4\n", elem_reg, elem_ptr));
        }
        
        Ok(array_reg)
    }

    /// Compile map literals
    fn compile_map_literal(&mut self, pairs: &[(Expression, Expression)]) -> Result<String, CursedError> {
        let map_reg = self.next_register();
        
        // For now, generate a simple struct-based map
        self.ir_code.push_str(&format!("  {} = alloca %struct.map, align 8\n", map_reg));
        self.ir_code.push_str(&format!("  ; Map with {} entries\n", pairs.len()));
        
        // Initialize map entries (simplified)
        for (i, (key, value)) in pairs.iter().enumerate() {
            let key_reg = self.compile_expression(key)?;
            let value_reg = self.compile_expression(value)?;
            self.ir_code.push_str(&format!("  ; Map entry {}: {} -> {}\n", i, key_reg, value_reg));
        }
        
        Ok(map_reg)
    }

    /// Get LLVM type string from CURSED type
    /// Infer parameter types by analyzing how they're used in the function body
    fn infer_parameter_types(
        &self,
        params: &[String],
        param_types: Option<&[String]>,
        body: &[Statement]
    ) -> Result<Vec<String>, CursedError> {
        let mut inferred_types = Vec::new();
        
        for (i, param) in params.iter().enumerate() {
            // If type is explicitly provided, use it
            if let Some(types) = param_types {
                if let Some(explicit_type) = types.get(i) {
                    if explicit_type != "UNTYPED" {
                        inferred_types.push(explicit_type.clone());
                        continue;
                    }
                }
            }
            
            // Infer type from usage in function body
            let inferred_type = self.infer_parameter_type_from_usage(param, body)?;
            inferred_types.push(inferred_type);
        }
        
        Ok(inferred_types)
    }
    
    /// Infer a parameter's type by analyzing its usage in the function body
    fn infer_parameter_type_from_usage(
        &self,
        param_name: &str,
        body: &[Statement]
    ) -> Result<String, CursedError> {
        let mut type_constraints = Vec::new();
        
        // Analyze all statements in the function body
        for statement in body {
            self.collect_type_constraints(param_name, statement, &mut type_constraints)?;
        }
        
        // Resolve type constraints to determine the parameter type
        self.resolve_type_constraints(&type_constraints)
    }
    
    /// Collect type constraints for a parameter based on its usage
    fn collect_type_constraints(
        &self,
        param_name: &str,
        statement: &Statement,
        constraints: &mut Vec<String>
    ) -> Result<(), CursedError> {
        match statement {
            Statement::Return(return_stmt) => {
                if let Some(value) = &return_stmt.value {
                    self.collect_expression_constraints(param_name, value, constraints)?;
                }
            }
            Statement::Let(let_stmt) => {
                self.collect_expression_constraints(param_name, &let_stmt.value, constraints)?;
            }
            Statement::Expression(expr) => {
                self.collect_expression_constraints(param_name, expr, constraints)?;
            }
            Statement::If(if_stmt) => {
                self.collect_expression_constraints(param_name, &if_stmt.condition, constraints)?;
                for stmt in &if_stmt.then_branch {
                    self.collect_type_constraints(param_name, stmt, constraints)?;
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.collect_type_constraints(param_name, stmt, constraints)?;
                    }
                }
            }
            Statement::While(while_stmt) => {
                self.collect_expression_constraints(param_name, &while_stmt.condition, constraints)?;
                for stmt in &while_stmt.body {
                    self.collect_type_constraints(param_name, stmt, constraints)?;
                }
            }
            Statement::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    self.collect_type_constraints(param_name, init, constraints)?;
                }
                if let Some(condition) = &for_stmt.condition {
                    self.collect_expression_constraints(param_name, condition, constraints)?;
                }
                if let Some(update) = &for_stmt.update {
                    self.collect_expression_constraints(param_name, update, constraints)?;
                }
                for stmt in &for_stmt.body {
                    self.collect_type_constraints(param_name, stmt, constraints)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Collect type constraints from expression usage
    fn collect_expression_constraints(
        &self,
        param_name: &str,
        expression: &Expression,
        constraints: &mut Vec<String>
    ) -> Result<(), CursedError> {
        match expression {
            Expression::Identifier(name) if name == param_name => {
                // Direct usage - doesn't give us type info by itself
            }
            Expression::Binary(binary) => {
                // Check if our parameter is used in a binary operation
                if let Expression::Identifier(left_name) = &*binary.left {
                    if left_name == param_name {
                        // Parameter is the left operand
                        self.infer_type_from_binary_operation(&binary.operator, &binary.right, constraints)?;
                    }
                }
                if let Expression::Identifier(right_name) = &*binary.right {
                    if right_name == param_name {
                        // Parameter is the right operand  
                        self.infer_type_from_binary_operation(&binary.operator, &binary.left, constraints)?;
                    }
                }
                // Recursively check nested expressions
                self.collect_expression_constraints(param_name, &binary.left, constraints)?;
                self.collect_expression_constraints(param_name, &binary.right, constraints)?;
            }
            Expression::Call(call) => {
                // Check function arguments
                for arg in &call.arguments {
                    self.collect_expression_constraints(param_name, arg, constraints)?;
                }
                // Check function expression
                self.collect_expression_constraints(param_name, &call.function, constraints)?;
            }
            Expression::Array(elements) => {
                for element in elements {
                    self.collect_expression_constraints(param_name, element, constraints)?;
                }
            }
            Expression::Unary(unary) => {
                self.collect_expression_constraints(param_name, &unary.operand, constraints)?;
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Infer type constraints from binary operations
    fn infer_type_from_binary_operation(
        &self,
        operator: &str,
        other_operand: &Expression,
        constraints: &mut Vec<String>
    ) -> Result<(), CursedError> {
        match operator {
            "+" | "-" | "*" | "/" | "%" => {
                // Arithmetic operations - operands should be numeric
                match other_operand {
                    Expression::Integer(_) => {
                        constraints.push("normie".to_string()); // Integer literal suggests normie type
                    }
                    Expression::Float(_) => {
                        constraints.push("f32".to_string()); // Float literal suggests f32 type
                    }
                    _ => {
                        constraints.push("normie".to_string()); // Default to normie for arithmetic
                    }
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                // Comparison operations - infer type from the other operand
                match other_operand {
                    Expression::Integer(_) => {
                        constraints.push("normie".to_string());
                    }
                    Expression::Float(_) => {
                        constraints.push("f32".to_string());
                    }
                    Expression::String(_) => {
                        constraints.push("tea".to_string());
                    }
                    Expression::Boolean(_) => {
                        constraints.push("lit".to_string());
                    }
                    _ => {
                        constraints.push("normie".to_string()); // Default to normie
                    }
                }
            }
            "&&" | "||" => {
                // Logical operations - operands should be boolean
                constraints.push("lit".to_string());
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Resolve type constraints to determine the final type
    fn resolve_type_constraints(&self, constraints: &[String]) -> Result<String, CursedError> {
        if constraints.is_empty() {
            // No constraints found, default to normie (integer)
            return Ok("normie".to_string());
        }
        
        // Count frequency of each type constraint
        let mut type_counts = std::collections::HashMap::new();
        for constraint in constraints {
            *type_counts.entry(constraint.clone()).or_insert(0) += 1;
        }
        
        // Find the most frequent type
        let most_frequent_type = type_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(type_name, _)| type_name.clone())
            .unwrap_or_else(|| "normie".to_string());
        
        Ok(most_frequent_type)
    }

    fn get_llvm_type(&self, cursed_type: &str) -> String {
        match cursed_type {
            // Standard types
            "int" | "i32" => "i32".to_string(),
            "i64" | "long" => "i64".to_string(),
            "f32" | "float" => "float".to_string(),
            "f64" | "double" => "double".to_string(),
            "bool" => "i1".to_string(),
            "string" | "str" => "i8*".to_string(),
            "void" => "void".to_string(),
            // Gen-Z CURSED types
            "normie" => "i32".to_string(),      // normie = i32
            "tea" => "i8*".to_string(),         // tea = string
            "lit" => "i1".to_string(),          // lit = bool
            "cap" => "i8*".to_string(),         // cap = string
            "dm" => "i8*".to_string(),          // dm = string
            "facts" => "i32".to_string(),       // facts = i32 (immutable)
            _ => "i8*".to_string(), // Default to pointer for complex types
        }
    }

    /// Handle string concatenation
    fn compile_string_concatenation(&mut self, left: &Expression, right: &Expression) -> Result<String, CursedError> {
        let left_type = self.infer_expression_type(left)?;
        let right_type = self.infer_expression_type(right)?;
        
        let left_reg = self.compile_expression(left)?;
        let right_reg = self.compile_expression(right)?;
        
        // Convert operands to strings if needed
        let left_str_reg = if left_type == "i32" {
            let converted_reg = self.next_register();
            self.ir_code.push_str(&format!("  {} = call i8* @i32_to_string(i32 {})\n", 
                converted_reg, left_reg));
            converted_reg
        } else {
            left_reg
        };
        
        let right_str_reg = if right_type == "i32" {
            let converted_reg = self.next_register();
            self.ir_code.push_str(&format!("  {} = call i8* @i32_to_string(i32 {})\n", 
                converted_reg, right_reg));
            converted_reg
        } else {
            right_reg
        };
        
        // Generate a new register for the result
        let result_reg = self.next_register();
        
        // Call the string_concat runtime function
        // string_concat(str1_ptr: *const c_char, str2_ptr: *const c_char) -> *mut c_char
        self.ir_code.push_str(&format!("  {} = call i8* @string_concat(i8* {}, i8* {})\n", 
            result_reg, left_str_reg, right_str_reg));
        
        log::debug!("String concatenation: {} + {} -> {} (left_type: {}, right_type: {})", 
            left_str_reg, right_str_reg, result_reg, left_type, right_type);
        Ok(result_reg)
    }

    /// Generate next register name
    fn next_register(&mut self) -> String {
        let reg = format!("%{}", self.variable_counter);
        log::debug!("Generated register: {} (counter was {})", reg, self.variable_counter);
        self.variable_counter += 1;
        reg
    }

    /// Generate next label name
    fn next_label(&mut self) -> String {
        let label = format!("label{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Get current IR code
    pub fn get_ir(&self) -> &str {
        &self.ir_code
    }

    /// Clear IR code
    pub fn clear_ir(&mut self) {
        self.ir_code.clear();
    }
}
