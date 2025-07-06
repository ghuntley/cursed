//! LLVM Expression Compiler Module  
//! Complete expression compilation with proper register management and type handling

use crate::ast::{Expression, Literal, BinaryOperator, UnaryOperator};
use crate::error::CursedError;
use std::collections::HashMap;

/// Complete expression compiler for CURSED expressions to LLVM IR
pub struct ExpressionCompiler {
    pub variable_counter: usize,
    pub string_constants: Vec<String>,
    pub variables: HashMap<String, String>,
    pub ir_buffer: String,
}

impl ExpressionCompiler {
    pub fn new() -> Self {
        Self {
            variable_counter: 0,
            string_constants: Vec::new(),
            variables: HashMap::new(),
            ir_buffer: String::new(),
        }
    }

    /// Set the starting variable counter to synchronize with main generator
    pub fn set_variable_counter(&mut self, counter: usize) {
        self.variable_counter = counter;
    }

    /// Get the current variable counter value
    pub fn get_variable_counter(&self) -> usize {
        self.variable_counter
    }

    /// Compile any expression to LLVM IR with complete register handling
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<String, CursedError> {
        match expression {
            Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Integer(val) => Ok(val.to_string()),
            Expression::Float(val) => Ok(val.to_string()),
            Expression::String(val) => self.compile_string_literal(val),
            Expression::Boolean(val) => Ok(if *val { "1" } else { "0" }.to_string()),
            Expression::Identifier(name) => self.compile_identifier(name),
            Expression::Binary(binary_expr) => {
                self.compile_binary_expression(&binary_expr.left, &binary_expr.operator, &binary_expr.right)
            },
            Expression::Unary(unary_expr) => {
                self.compile_unary_expression(&unary_expr.operator, &unary_expr.operand)
            },
            Expression::Call(call_expr) => {
                self.compile_function_call(&call_expr.function, &call_expr.arguments)
            },
            Expression::MemberAccess(member_expr) => {
                self.compile_member_access(&member_expr.object, &member_expr.property)
            },
            Expression::Array(elements) => {
                self.compile_array_expression(elements)
            },
            Expression::Map(pairs) => {
                self.compile_map_expression(pairs)
            },
            Expression::ChannelSend(send_expr) => {
                self.compile_channel_send(&send_expr.channel, &send_expr.value)
            },
            Expression::ChannelReceive(recv_expr) => {
                self.compile_channel_receive(&recv_expr.channel)
            },
            Expression::ChannelCreation(create_expr) => {
                self.compile_channel_creation(&create_expr.element_type, &create_expr.capacity)
            },
            Expression::StructLiteral(struct_literal) => {
                // Convert StructFieldAssignment to (String, Expression) tuples
                let field_tuples: Vec<(String, Expression)> = struct_literal.fields.iter()
                    .map(|field| (field.field_name.clone(), field.value.clone()))
                    .collect();
                self.compile_struct_literal(&struct_literal.struct_name, &field_tuples)
            },
            Expression::Lambda(lambda_expr) => {
                self.compile_lambda_expression(&lambda_expr.parameters, &lambda_expr.body)
            },
            Expression::Tuple(tuple_expr) => {
                self.compile_tuple_expression(&tuple_expr.elements)
            },
            Expression::TupleAccess(tuple_access) => {
                self.compile_tuple_access(&tuple_access.tuple, tuple_access.index)
            },

        }
    }

    /// Compile literal values with proper type handling
    fn compile_literal(&mut self, literal: &Literal) -> Result<String, CursedError> {
        match literal {
            Literal::Integer(val) => Ok(val.to_string()),
            Literal::Float(val) => Ok(val.to_string()),
            Literal::String(val) => self.compile_string_literal(val),
            Literal::Boolean(val) => Ok(if *val { "1" } else { "0" }.to_string()),
            Literal::Nil | Literal::Null => Ok("null".to_string()),
        }
    }

    /// Compile string literals with constant pool management
    fn compile_string_literal(&mut self, value: &str) -> Result<String, CursedError> {
        let const_name = format!("@.str.{}", self.string_constants.len());
        let len = value.len() + 1; // +1 for null terminator
        
        // Add to constant pool
        self.string_constants.push(format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1", 
            const_name, len, value.replace("\"", "\\\"")));
        
        // Generate getelementptr to get string pointer
        let reg = self.next_register();
        self.ir_buffer.push_str(&format!(
            "  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0\n",
            reg, len, len, const_name
        ));
        
        Ok(reg)
    }

    /// Compile identifier access with variable resolution
    fn compile_identifier(&mut self, name: &str) -> Result<String, CursedError> {
        if let Some(reg) = self.variables.get(name) {
            // Clone the register string to avoid borrow issues
            let reg_name = reg.clone();
            
            // Check if this is a function parameter (starts with PARAM:)
            if reg_name.starts_with("PARAM:") {
                // Function parameters are already values, no need to load - strip PARAM: prefix
                Ok(reg_name.strip_prefix("PARAM:").unwrap().to_string())
            } else {
                // Local variable allocated on stack - need to load
                let load_reg = self.next_register();
                self.ir_buffer.push_str(&format!("  {} = load i32, i32* {}, align 4\n", load_reg, reg_name));
                Ok(load_reg)
            }
        } else {
            // Function parameter or global variable
            Ok(format!("%{}", name))
        }
    }

    /// Compile binary expressions with complete operator support
    fn compile_binary_expression(
        &mut self,
        left: &Expression,
        operator: &str,
        right: &Expression
    ) -> Result<String, CursedError> {
        let left_reg = self.compile_expression(left)?;
        let right_reg = self.compile_expression(right)?;
        let result_reg = self.next_register();
        
        // Handle different operator types
        match operator {
            // Arithmetic operators
            "+" => {
                self.ir_buffer.push_str(&format!("  {} = add i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "-" => {
                self.ir_buffer.push_str(&format!("  {} = sub i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "*" => {
                self.ir_buffer.push_str(&format!("  {} = mul i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "/" => {
                self.ir_buffer.push_str(&format!("  {} = sdiv i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "%" => {
                self.ir_buffer.push_str(&format!("  {} = srem i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            // Comparison operators
            "==" => {
                self.ir_buffer.push_str(&format!("  {} = icmp eq i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "!=" => {
                self.ir_buffer.push_str(&format!("  {} = icmp ne i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "<" => {
                self.ir_buffer.push_str(&format!("  {} = icmp slt i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            ">" => {
                self.ir_buffer.push_str(&format!("  {} = icmp sgt i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "<=" => {
                self.ir_buffer.push_str(&format!("  {} = icmp sle i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            ">=" => {
                self.ir_buffer.push_str(&format!("  {} = icmp sge i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            // Logical operators
            "&&" => {
                // Short-circuit AND
                let true_label = format!("and_true_{}", self.variable_counter);
                let false_label = format!("and_false_{}", self.variable_counter);
                let end_label = format!("and_end_{}", self.variable_counter);
                
                self.ir_buffer.push_str(&format!("  br i1 {}, label %{}, label %{}\n", left_reg, true_label, false_label));
                self.ir_buffer.push_str(&format!("{}:\n", true_label));
                self.ir_buffer.push_str(&format!("  br i1 {}, label %{}, label %{}\n", right_reg, end_label, false_label));
                self.ir_buffer.push_str(&format!("{}:\n", false_label));
                self.ir_buffer.push_str(&format!("  br label %{}\n", end_label));
                self.ir_buffer.push_str(&format!("{}:\n", end_label));
                self.ir_buffer.push_str(&format!("  {} = phi i1 [ {}, %{} ], [ false, %{} ]\n", 
                    result_reg, right_reg, true_label, false_label));
            },
            "||" => {
                // Short-circuit OR
                let true_label = format!("or_true_{}", self.variable_counter);
                let false_label = format!("or_false_{}", self.variable_counter);
                let end_label = format!("or_end_{}", self.variable_counter);
                
                self.ir_buffer.push_str(&format!("  br i1 {}, label %{}, label %{}\n", left_reg, true_label, false_label));
                self.ir_buffer.push_str(&format!("{}:\n", true_label));
                self.ir_buffer.push_str(&format!("  br label %{}\n", end_label));
                self.ir_buffer.push_str(&format!("{}:\n", false_label));
                self.ir_buffer.push_str(&format!("  br label %{}\n", end_label));
                self.ir_buffer.push_str(&format!("{}:\n", end_label));
                self.ir_buffer.push_str(&format!("  {} = phi i1 [ true, %{} ], [ {}, %{} ]\n", 
                    result_reg, true_label, right_reg, false_label));
            },
            // Bitwise operators
            "&" => {
                self.ir_buffer.push_str(&format!("  {} = and i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "|" => {
                self.ir_buffer.push_str(&format!("  {} = or i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "^" => {
                self.ir_buffer.push_str(&format!("  {} = xor i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            "<<" => {
                self.ir_buffer.push_str(&format!("  {} = shl i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            ">>" => {
                self.ir_buffer.push_str(&format!("  {} = ashr i32 {}, {}\n", result_reg, left_reg, right_reg));
            },
            // Assignment operators (return left side)
            "=" => {
                // For now, just return the right side value
                return Ok(right_reg);
            },
            _ => {
                return Err(CursedError::CompilerError(format!("Unsupported binary operator: {}", operator)));
            }
        }
        
        Ok(result_reg)
    }

    /// Compile unary expressions
    fn compile_unary_expression(&mut self, operator: &UnaryOperator, operand: &Expression) -> Result<String, CursedError> {
        let operand_reg = self.compile_expression(operand)?;
        let result_reg = self.next_register();
        
        match operator {
            UnaryOperator::Not => {
                self.ir_buffer.push_str(&format!("  {} = icmp eq i32 {}, 0\n", result_reg, operand_reg));
            },
            UnaryOperator::Minus => {
                self.ir_buffer.push_str(&format!("  {} = sub i32 0, {}\n", result_reg, operand_reg));
            },
            UnaryOperator::Plus => {
                // Unary plus is a no-op
                return Ok(operand_reg);
            },
        }
        
        Ok(result_reg)
    }

    /// Compile function calls with proper argument handling
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
                self.ir_buffer.push_str(&format!("  {} = call i32 @{}(", result_reg, func_name));
                
                for (i, arg_reg) in arg_regs.iter().enumerate() {
                    if i > 0 {
                        self.ir_buffer.push_str(", ");
                    }
                    self.ir_buffer.push_str(&format!("i32 {}", arg_reg));
                }
                
                self.ir_buffer.push_str(")\n");
            },
            Expression::MemberAccess(member_expr) => {
                // Method call
                if let Expression::Identifier(obj_name) = &*member_expr.object {
                    // Special handling for stdlib methods
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
                    let method_name = format!("{}_{}", obj_name, member_expr.property);
                    self.ir_buffer.push_str(&format!("  {} = call i32 @{}(", result_reg, method_name));
                    
                    for (i, arg_reg) in arg_regs.iter().enumerate() {
                        if i > 0 {
                            self.ir_buffer.push_str(", ");
                        }
                        self.ir_buffer.push_str(&format!("i32 {}", arg_reg));
                    }
                    
                    self.ir_buffer.push_str(")\n");
                }
            },
            _ => {
                return Err(CursedError::CompilerError("Unsupported function call expression".to_string()));
            }
        }
        
        Ok(result_reg)
    }

    /// Compile member access expressions with proper struct handling
    fn compile_member_access(&mut self, object: &Expression, property: &str) -> Result<String, CursedError> {
        let obj_reg = self.compile_expression(object)?;
        
        // Special handling for known stdlib objects
        if let Expression::Identifier(obj_name) = object {
            if obj_name == "vibez" {
                // Return function pointer for vibez methods
                return self.compile_vibez_member_access(property);
            }
        }
        
        // General struct member access
        let member_ptr = self.next_register();
        let result_reg = self.next_register();
        
        // Assume all structs have uniform layout for now
        self.ir_buffer.push_str(&format!("  ; Member access: {}.{}\n", obj_reg, property));
        self.ir_buffer.push_str(&format!("  {} = getelementptr inbounds %struct.object, %struct.object* {}, i32 0, i32 0\n", 
            member_ptr, obj_reg));
        self.ir_buffer.push_str(&format!("  {} = load i32, i32* {}, align 4\n", result_reg, member_ptr));
        
        Ok(result_reg)
    }

    /// Compile array expressions
    fn compile_array_expression(&mut self, elements: &[Expression]) -> Result<String, CursedError> {
        let array_reg = self.next_register();
        let len = elements.len();
        
        if len == 0 {
            // Empty array
            self.ir_buffer.push_str(&format!("  {} = inttoptr i64 0 to [0 x i32]*\n", array_reg));
            return Ok(array_reg);
        }
        
        // Allocate array
        self.ir_buffer.push_str(&format!("  {} = alloca [{} x i32], align 4\n", array_reg, len));
        
        // Initialize elements
        for (i, element) in elements.iter().enumerate() {
            let elem_reg = self.compile_expression(element)?;
            let elem_ptr = self.next_register();
            self.ir_buffer.push_str(&format!("  {} = getelementptr inbounds [{} x i32], [{} x i32]* {}, i64 0, i64 {}\n", 
                elem_ptr, len, len, array_reg, i));
            self.ir_buffer.push_str(&format!("  store i32 {}, i32* {}, align 4\n", elem_reg, elem_ptr));
        }
        
        Ok(array_reg)
    }

    /// Compile map expressions
    fn compile_map_expression(&mut self, pairs: &[(Expression, Expression)]) -> Result<String, CursedError> {
        let map_reg = self.next_register();
        
        // Allocate map structure
        self.ir_buffer.push_str(&format!("  {} = alloca %struct.map, align 8\n", map_reg));
        self.ir_buffer.push_str(&format!("  ; Map with {} entries\n", pairs.len()));
        
        // Initialize map entries
        for (i, (key, value)) in pairs.iter().enumerate() {
            let key_reg = self.compile_expression(key)?;
            let value_reg = self.compile_expression(value)?;
            self.ir_buffer.push_str(&format!("  ; Map entry {}: {} -> {}\n", i, key_reg, value_reg));
            
            // For now, just generate comments - real map implementation would store in hash table
        }
        
        Ok(map_reg)
    }

    /// Compile array/map indexing
    fn compile_index_expression(&mut self, object: &Expression, index: &Expression) -> Result<String, CursedError> {
        let obj_reg = self.compile_expression(object)?;
        let index_reg = self.compile_expression(index)?;
        
        let elem_ptr = self.next_register();
        let result_reg = self.next_register();
        
        // Generate array indexing
        self.ir_buffer.push_str(&format!("  {} = getelementptr inbounds i32, i32* {}, i32 {}\n", 
            elem_ptr, obj_reg, index_reg));
        self.ir_buffer.push_str(&format!("  {} = load i32, i32* {}, align 4\n", result_reg, elem_ptr));
        
        Ok(result_reg)
    }

    /// Compile range expressions
    fn compile_range_expression(&mut self, start: &Expression, end: &Expression) -> Result<String, CursedError> {
        let start_reg = self.compile_expression(start)?;
        let end_reg = self.compile_expression(end)?;
        let range_reg = self.next_register();
        
        // For now, create a simple range struct
        self.ir_buffer.push_str(&format!("  {} = alloca %struct.range, align 8\n", range_reg));
        self.ir_buffer.push_str(&format!("  ; Range from {} to {}\n", start_reg, end_reg));
        
        Ok(range_reg)
    }

    /// Compile lambda expressions
    fn compile_lambda_expression(&mut self, parameters: &[String], body: &Expression) -> Result<String, CursedError> {
        // Generate a unique function name for this lambda
        let lambda_func_name = format!("lambda_{}", self.variable_counter);
        self.variable_counter += 1;
        
        // Save current state
        let old_variables = self.variables.clone();
        let old_ir_buffer = self.ir_buffer.clone();
        
        // Clear IR buffer for lambda function generation
        self.ir_buffer.clear();
        
        // Build lambda function signature
        let mut lambda_ir = String::new();
        lambda_ir.push_str(&format!("define i32 @{}(", lambda_func_name));
        
        // Add parameters to function signature
        for (i, param) in parameters.iter().enumerate() {
            if i > 0 {
                lambda_ir.push_str(", ");
            }
            lambda_ir.push_str(&format!("i32 %{}", param));
            
            // Map parameters to variables with PARAM: prefix
            self.variables.insert(param.clone(), format!("PARAM:%{}", param));
        }
        
        lambda_ir.push_str(") {\n");
        lambda_ir.push_str("entry:\n");
        
        // Compile the lambda body
        let body_reg = self.compile_expression(body)?;
        
        // Add any IR generated during body compilation
        if !self.ir_buffer.is_empty() {
            lambda_ir.push_str(&self.ir_buffer);
        }
        
        // Return the body result
        lambda_ir.push_str(&format!("  ret i32 {}\n", body_reg));
        lambda_ir.push_str("}\n\n");
        
        // Restore previous state
        self.variables = old_variables;
        self.ir_buffer = old_ir_buffer;
        
        // Store the lambda function definition to be emitted later
        // For now, we'll append it to the string constants as a hack
        self.string_constants.push(lambda_ir);
        
        // Generate code to create a function pointer
        let lambda_ptr_reg = self.next_register();
        self.ir_buffer.push_str(&format!("  {} = bitcast i32 (", lambda_ptr_reg));
        
        // Add parameter types to bitcast
        for (i, _) in parameters.iter().enumerate() {
            if i > 0 {
                self.ir_buffer.push_str(", ");
            }
            self.ir_buffer.push_str("i32");
        }
        
        self.ir_buffer.push_str(&format!(")* @{} to i8*\n", lambda_func_name));
        self.ir_buffer.push_str(&format!("  ; Lambda function pointer with {} parameters\n", parameters.len()));
        
        Ok(lambda_ptr_reg)
    }

    /// Compile struct literals
    fn compile_struct_literal(&mut self, struct_name: &str, fields: &[(String, Expression)]) -> Result<String, CursedError> {
        let struct_reg = self.next_register();
        
        // Allocate struct
        self.ir_buffer.push_str(&format!("  {} = alloca %struct.{}, align 8\n", struct_reg, struct_name));
        
        // Initialize fields
        for (i, (field_name, value)) in fields.iter().enumerate() {
            let value_reg = self.compile_expression(value)?;
            let field_ptr = self.next_register();
            
            self.ir_buffer.push_str(&format!("  {} = getelementptr inbounds %struct.{}, %struct.{}* {}, i32 0, i32 {}\n", 
                field_ptr, struct_name, struct_name, struct_reg, i));
            self.ir_buffer.push_str(&format!("  store i32 {}, i32* {}, align 4\n", value_reg, field_ptr));
            self.ir_buffer.push_str(&format!("  ; Field {}: {}\n", field_name, value_reg));
        }
        
        Ok(struct_reg)
    }

    /// Compile vibez method calls
    fn compile_vibez_method_call(&mut self, method: &str, arguments: &[Expression]) -> Result<String, CursedError> {
        let result_reg = self.next_register();
        
        match method {
            "spill" => {
                // Handle vibez.spill() calls
                for arg in arguments {
                    let arg_reg = self.compile_expression(arg)?;
                    match arg {
                        Expression::String(_) => {
                            self.ir_buffer.push_str(&format!("  call i32 @puts(i8* {})\n", arg_reg));
                        },
                        _ => {
                            let format_str = self.compile_string_literal("%d\\n")?;
                            self.ir_buffer.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i32 {})\n", format_str, arg_reg));
                        }
                    }
                }
                self.ir_buffer.push_str(&format!("  {} = add i32 0, 0\n", result_reg));
            },
            "spillf" => {
                // Handle vibez.spillf() calls
                if !arguments.is_empty() {
                    // First compile all arguments to generate their intermediate IR
                    let mut arg_regs = Vec::new();
                    for arg in arguments {
                        let arg_reg = self.compile_expression(arg)?;
                        arg_regs.push(arg_reg);
                    }
                    
                    // Now generate the printf call with compiled arguments
                    self.ir_buffer.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}", arg_regs[0]));
                    
                    for arg_reg in &arg_regs[1..] {
                        self.ir_buffer.push_str(&format!(", i32 {}", arg_reg));
                    }
                    
                    self.ir_buffer.push_str(")\n");
                }
                self.ir_buffer.push_str(&format!("  {} = add i32 0, 0\n", result_reg));
            },
            _ => {
                return Err(CursedError::CompilerError(format!("Unknown vibez method: {}", method)));
            }
        }
        
        Ok(result_reg)
    }

    /// Compile vibez member access
    fn compile_vibez_member_access(&mut self, property: &str) -> Result<String, CursedError> {
        let func_reg = self.next_register();
        
        match property {
            "spill" => {
                self.ir_buffer.push_str(&format!("  {} = bitcast i32 (i8**, i64)* @cursed_vibez_spill to i8*\n", func_reg));
            },
            "spillf" => {
                self.ir_buffer.push_str(&format!("  {} = bitcast i32 (i8*, i8**, i64)* @cursed_vibez_spillf to i8*\n", func_reg));
            },
            _ => {
                return Err(CursedError::CompilerError(format!("Unknown vibez property: {}", property)));
            }
        }
        
        Ok(func_reg)
    }

    /// Generate next register name
    fn next_register(&mut self) -> String {
        self.variable_counter += 1;
        let reg = format!("%{}", self.variable_counter);
        reg
    }

    /// Get string constants for global declaration
    pub fn get_string_constants(&self) -> &[String] {
        &self.string_constants
    }
    
    /// Get lambda function definitions
    pub fn get_lambda_functions(&self) -> Vec<String> {
        self.string_constants.iter()
            .filter(|s| s.starts_with("define i32 @lambda_"))
            .cloned()
            .collect()
    }
    
    /// Get only actual string constants (not lambda functions)
    pub fn get_actual_string_constants(&self) -> Vec<String> {
        self.string_constants.iter()
            .filter(|s| !s.starts_with("define i32 @lambda_"))
            .cloned()
            .collect()
    }

    /// Get generated IR code
    pub fn get_ir(&self) -> &str {
        &self.ir_buffer
    }

    /// Clear IR buffer
    pub fn clear_ir(&mut self) {
        self.ir_buffer.clear();
    }

    /// Set variable mapping
    pub fn set_variable(&mut self, name: String, register: String) {
        self.variables.insert(name, register);
    }

    /// Get variable register
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    /// Compile channel send operation
    fn compile_channel_send(&mut self, channel: &Expression, value: &Expression) -> Result<String, CursedError> {
        let channel_reg = self.compile_expression(channel)?;
        let value_reg = self.compile_expression(value)?;
        
        let result_reg = self.next_register();
        self.ir_buffer.push_str(&format!(
            "  {} = call void @cursed_channel_send(i8* {}, i64 {})\n",
            result_reg, channel_reg, value_reg
        ));
        
        Ok(result_reg)
    }

    /// Compile channel receive operation  
    fn compile_channel_receive(&mut self, channel: &Expression) -> Result<String, CursedError> {
        let channel_reg = self.compile_expression(channel)?;
        
        let result_reg = self.next_register();
        self.ir_buffer.push_str(&format!(
            "  {} = call i64 @cursed_channel_receive(i8* {})\n",
            result_reg, channel_reg
        ));
        
        Ok(result_reg)
    }

    /// Compile channel creation operation (dm type())
    fn compile_channel_creation(&mut self, element_type: &str, capacity: &Option<Box<Expression>>) -> Result<String, CursedError> {
        // Determine capacity
        let capacity_reg = if let Some(cap_expr) = capacity {
            self.compile_expression(cap_expr)?
        } else {
            "0".to_string() // Unbuffered channel
        };

        // Generate call to runtime channel creation function
        let result_reg = format!("%var{}", self.variable_counter);
        self.variable_counter += 1;

        let string_index = self.string_constants.len();
        self.ir_buffer.push_str(&format!(
            "  {} = call i8* @cursed_channel_create(i32 {}, i8* getelementptr inbounds ([{}x i8], [{}x i8]* @.str.{}, i32 0, i32 0))\n",
            result_reg, capacity_reg, element_type.len() + 1, element_type.len() + 1, string_index
        ));

        // Add the element type string to the strings section
        self.string_constants.push(element_type.to_string());

        Ok(result_reg)
    }

    /// Compile tuple expression (e.g., (1, "hello", based))
    fn compile_tuple_expression(&mut self, elements: &[Expression]) -> Result<String, CursedError> {
        // Create a struct type for the tuple with appropriate fields
        let mut element_types = Vec::new();
        let mut element_regs = Vec::new();
        
        // Compile each element and determine its type
        for element in elements {
            let element_reg = self.compile_expression(element)?;
            element_regs.push(element_reg);
            
            // Determine LLVM type based on expression
            let llvm_type = match element {
                Expression::Integer(_) => "i32",
                Expression::String(_) => "i8*",
                Expression::Boolean(_) => "i1",
                _ => "i8*", // Default to pointer for complex types
            };
            element_types.push(llvm_type);
        }
        
        // Create the tuple struct type
        let tuple_type = format!("{{ {} }}", element_types.join(", "));
        
        // Allocate memory for the tuple
        let tuple_reg = self.next_register();
        self.ir_buffer.push_str(&format!(
            "  {} = alloca {}, align 8\n",
            tuple_reg, tuple_type
        ));
        
        // Store each element in the tuple
        for (i, element_reg) in element_regs.iter().enumerate() {
            let field_ptr = self.next_register();
            self.ir_buffer.push_str(&format!(
                "  {} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}\n",
                field_ptr, tuple_type, tuple_type, tuple_reg, i
            ));
            
            self.ir_buffer.push_str(&format!(
                "  store {} {}, {}* {}, align 4\n",
                element_types[i], element_reg, element_types[i], field_ptr
            ));
        }
        
        Ok(tuple_reg)
    }

    /// Compile tuple access expression (e.g., tuple.0, tuple.1)
    fn compile_tuple_access(&mut self, tuple_expr: &Expression, index: usize) -> Result<String, CursedError> {
        let tuple_reg = self.compile_expression(tuple_expr)?;
        
        // For now, we'll assume a generic tuple type - in a real implementation,
        // we'd need proper type information from the type system
        let field_ptr = self.next_register();
        
        // Generate GEP instruction to access the field
        self.ir_buffer.push_str(&format!(
            "  {} = getelementptr inbounds %tuple_type, %tuple_type* {}, i32 0, i32 {}\n",
            field_ptr, tuple_reg, index
        ));
        
        // Load the value from the field
        let result_reg = self.next_register();
        self.ir_buffer.push_str(&format!(
            "  {} = load i32, i32* {}, align 4\n",
            result_reg, field_ptr
        ));
        
        Ok(result_reg)
    }
}
