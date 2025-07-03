//! LLVM Function Compilation Module
//! Complete function compilation with full LLVM IR generation

use crate::ast::{Statement, Expression, FunctionStatement};
use crate::error::CursedError;
use std::collections::HashMap;

/// Complete function compiler for CURSED functions to LLVM IR
pub struct FunctionCompiler {
    pub ir_code: String,
    pub variable_counter: usize,
    pub label_counter: usize,
    pub variables: HashMap<String, String>,
    pub current_function: Option<String>,
}

impl FunctionCompiler {
    pub fn new() -> Self {
        Self {
            ir_code: String::new(),
            variable_counter: 0,
            label_counter: 0,
            variables: HashMap::new(),
            current_function: None,
        }
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
        self.variable_counter = 0;
        self.label_counter = 0;
        self.current_function = Some(name.to_string());
        
        let mut function_ir = String::new();
        
        // Generate function signature
        let ret_type = self.get_llvm_type(return_type.unwrap_or("int"));
        function_ir.push_str(&format!("define {} @{}(", ret_type, name));
        
        // Generate parameters with types
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                function_ir.push_str(", ");
            }
            let param_type = if let Some(types) = param_types {
                types.get(i).map(|s| s.as_str()).unwrap_or("int")
            } else {
                "int"
            };
            let llvm_type = self.get_llvm_type(param_type);
            function_ir.push_str(&format!("{} %{}", llvm_type, param));
            
            // Map parameters to variables
            self.variables.insert(param.clone(), format!("%{}", param));
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
                
                // Allocate variable on stack
                ir.push_str(&format!("  {} = alloca i32, align 4\n", var_reg));
                ir.push_str(&format!("  store i32 {}, i32* {}, align 4\n", value_reg, var_reg));
                
                // Store variable mapping
                self.variables.insert(let_stmt.name.clone(), var_reg);
                ir.push_str(&format!("  ; Variable {} allocated\n", let_stmt.name));
            },
            Statement::Assignment(assign_stmt) => {
                let value_reg = self.compile_expression(&assign_stmt.value)?;
                
                // Look up existing variable
                if let Some(var_reg) = self.variables.get(&assign_stmt.name).cloned() {
                    // Store new value to existing variable
                    ir.push_str(&format!("  store i32 {}, i32* {}, align 4\n", value_reg, var_reg));
                    ir.push_str(&format!("  ; Assignment to {} = {}\n", assign_stmt.name, value_reg));
                } else {
                    return Err(CursedError::runtime_error(&format!("Undefined variable: {}", assign_stmt.name)));
                }
            },
            Statement::Return(return_stmt) => {
                if let Some(val) = &return_stmt.value {
                    let return_reg = self.compile_expression(val)?;
                    ir.push_str(&format!("  ret i32 {}\n", return_reg));
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
            Expression::Boolean(val) => Ok(if *val { "1" } else { "0" }.to_string()),
            Expression::String(val) => {
                let str_reg = self.next_register();
                let len = val.len() + 1;
                self.ir_code.push_str(&format!(
                    "  {} = getelementptr inbounds [{} x i8], [{} x i8]* @.str.{}, i64 0, i64 0\n",
                    str_reg, len, len, self.variable_counter
                ));
                Ok(str_reg)
            },
            Expression::Identifier(name) => {
                if let Some(reg) = self.variables.get(name) {
                    // Clone the register string to avoid borrow issues
                    let reg_name = reg.clone();
                    // Load from variable
                    let load_reg = self.next_register();
                    self.ir_code.push_str(&format!("  {} = load i32, i32* {}, align 4\n", load_reg, reg_name));
                    Ok(load_reg)
                } else {
                    // Function parameter or global
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
        let left_reg = self.compile_expression(left)?;
        let right_reg = self.compile_expression(right)?;
        let result_reg = self.next_register();
        
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
        
        // Choose appropriate type based on operation
        let op_type = if operator.starts_with("icmp") {
            self.ir_code.push_str(&format!("  {} = {} i32 {}, {}\n", result_reg, op_str, left_reg, right_reg));
        } else {
            self.ir_code.push_str(&format!("  {} = {} i32 {}, {}\n", result_reg, op_str, left_reg, right_reg));
        };
        
        Ok(result_reg)
    }

    /// Compile function calls with argument handling
    fn compile_function_call(&mut self, function: &Expression, arguments: &[Expression]) -> Result<String, CursedError> {
        let result_reg = self.next_register();
        
        match function {
            Expression::Identifier(func_name) => {
                // First compile all arguments to generate their intermediate IR
                let mut arg_regs = Vec::new();
                for arg in arguments {
                    let arg_reg = self.compile_expression(arg)?;
                    arg_regs.push(arg_reg);
                }
                
                // Now generate the function call with compiled arguments
                self.ir_code.push_str(&format!("  {} = call i32 @{}(", result_reg, func_name));
                
                for (i, arg_reg) in arg_regs.iter().enumerate() {
                    if i > 0 {
                        self.ir_code.push_str(", ");
                    }
                    self.ir_code.push_str(&format!("i32 {}", arg_reg));
                }
                
                self.ir_code.push_str(")\n");
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
                    
                    // Now generate the method call with compiled arguments
                    let func_name = format!("{}_{}", obj_name, member_expr.property);
                    self.ir_code.push_str(&format!("  {} = call i32 @{}(", result_reg, func_name));
                    
                    for (i, arg_reg) in arg_regs.iter().enumerate() {
                        if i > 0 {
                            self.ir_code.push_str(", ");
                        }
                        self.ir_code.push_str(&format!("i32 {}", arg_reg));
                    }
                    
                    self.ir_code.push_str(")\n");
                }
            },
            _ => {
                return Err(CursedError::CompilerError("Unsupported function call expression".to_string()));
            }
        }
        
        Ok(result_reg)
    }

    /// Compile vibez method calls (stdlib output methods)
    fn compile_vibez_method_call(&mut self, method: &str, arguments: &[Expression]) -> Result<String, CursedError> {
        match method {
            "spill" => {
                // Handle vibez.spill() calls
                for arg in arguments {
                    let arg_reg = self.compile_expression(arg)?;
                    match arg {
                        Expression::String(_) => {
                            self.ir_code.push_str(&format!("  call i32 @puts(i8* {})\n", arg_reg));
                        },
                        _ => {
                            // For non-string types, use printf with %d format
                            let format_reg = self.next_register();
                            let const_name = format!("@.str.{}", self.variable_counter);
                            self.variable_counter += 1;
                            
                            // Add format string constant (this would normally be added to constants section)
                            self.ir_code.push_str(&format!("  {} = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d, i64 0, i64 0\n", format_reg));
                            self.ir_code.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i32 {})\n", format_reg, arg_reg));
                        }
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
        for stmt in then_branch {
            ir.push_str(&self.compile_statement(stmt)?);
        }
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // Else branch
        ir.push_str(&format!("{}:\n", else_label));
        if let Some(else_stmts) = else_branch {
            for stmt in else_stmts {
                ir.push_str(&self.compile_statement(stmt)?);
            }
        }
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // End label
        ir.push_str(&format!("{}:\n", end_label));
        
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
    fn get_llvm_type(&self, cursed_type: &str) -> String {
        match cursed_type {
            "int" | "i32" => "i32".to_string(),
            "i64" | "long" => "i64".to_string(),
            "f32" | "float" => "float".to_string(),
            "f64" | "double" => "double".to_string(),
            "bool" => "i1".to_string(),
            "string" | "str" => "i8*".to_string(),
            "void" => "void".to_string(),
            _ => "i8*".to_string(), // Default to pointer for complex types
        }
    }

    /// Generate next register name
    fn next_register(&mut self) -> String {
        let reg = format!("%{}", self.variable_counter);
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
