//! LLVM code generation for expressions

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use inkwell::FloatPredicate;
use crate::ast::*;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles an AST Expression node into an LLVM value.
    pub fn compile_expression<'expr>(
        &mut self, 
        expression: &'expr dyn Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        if let Some(type_conv_expr) = expression.as_any().downcast_ref::<TypeConversionExpression>() {
            // Handle type conversion
            return self.compile_type_conversion(type_conv_expr);
        } else if let Some(be_like_expr) = expression.as_any().downcast_ref::<BeLikeExpression>() {
            // Handle struct instantiation
            return self.compile_struct_instantiation(be_like_expr);
        } else if let Some(channel_expr) = expression.as_any().downcast_ref::<ChannelExpression>() {
            // Handle channel creation
            return self.compile_channel_creation(channel_expr);
        } else if let Some(send_expr) = expression.as_any().downcast_ref::<SendExpression>() {
            // Handle send to channel
            return self.compile_send_expression(send_expr);
        } else if let Some(recv_expr) = expression.as_any().downcast_ref::<ReceiveExpression>() {
            // Handle receive from channel
            return self.compile_receive_expression(recv_expr);
        } else if let Some(pointer_type) = expression.as_any().downcast_ref::<PointerType>() {
            // Handle pointer type expressions
            return self.compile_pointer_type(pointer_type);
        } else if let Some(pointer_deref) = expression.as_any().downcast_ref::<PointerDereference>() {
            // Handle pointer dereference expressions
            return self.compile_pointer_dereference(pointer_deref);
        } else if let Some(stan_expr) = expression.as_any().downcast_ref::<StanExpression>() {
            // Handle goroutine (stan) expression
            return self.compile_stan_expression(stan_expr);
        }
        
        // Handle basic expression types
        if let Some(lit) = expression.as_any().downcast_ref::<IntegerLiteral>() {
            Ok(self.context.i64_type().const_int(lit.value as u64, false).into())
        } else if let Some(lit) = expression.as_any().downcast_ref::<BooleanLiteral>() {
            Ok(self.context.bool_type().const_int(lit.value as u64, false).into())
        } else if let Some(lit) = expression.as_any().downcast_ref::<FloatLiteral>() {
            Ok(self.context.f64_type().const_float(lit.value).into())
        } else if let Some(lit) = expression.as_any().downcast_ref::<ByteLiteral>() {
            // Byte literals are represented as 8-bit integers in LLVM IR
            Ok(self.context.i8_type().const_int(lit.value as u64, false).into())
        } else if let Some(lit) = expression.as_any().downcast_ref::<RuneLiteral>() {
            // Rune literals are represented as 32-bit integers (Unicode code points) in LLVM IR
            Ok(self.context.i32_type().const_int(lit.value as u32 as u64, false).into())
        } else if let Some(lit) = expression.as_any().downcast_ref::<StringLiteral>() {
            // Create a constant global string
            let string_value = self.builder.build_global_string_ptr(&lit.value, "str").unwrap();
            
            // Return a pointer to the string data
            Ok(string_value.as_pointer_value().into())
        } else if let Some(ident) = expression.as_any().downcast_ref::<Identifier>() {
            self.compile_identifier(ident)
        } else if let Some(prefix) = expression.as_any().downcast_ref::<PrefixExpression>() {
            self.compile_prefix_expression(prefix)
        } else if let Some(infix) = expression.as_any().downcast_ref::<InfixExpression>() {
            self.compile_infix_expression(infix)
        } else if let Some(if_expr) = expression.as_any().downcast_ref::<IfStatement>() {
            self.compile_if_expression(if_expr)
        } else if let Some(fn_lit) = expression.as_any().downcast_ref::<FunctionLiteral>() {
            self.compile_function_literal(fn_lit)
        } else if let Some(call_expr) = expression.as_any().downcast_ref::<CallExpression>() {
            self.compile_call_expression(call_expr)
        } else if let Some(array_lit) = expression.as_any().downcast_ref::<ArrayLiteral>() {
            self.compile_array_literal(array_lit)
        } else if let Some(index_expr) = expression.as_any().downcast_ref::<IndexExpression>() {
            self.compile_index_expression(index_expr)
        } else if let Some(hash_lit) = expression.as_any().downcast_ref::<HashLiteral>() {
            self.compile_hash_literal(hash_lit)
        } else if let Some(prop_access) = expression.as_any().downcast_ref::<PropertyAccessExpression>() {
            self.compile_property_access(prop_access)
        } else if let Some(assign_expr) = expression.as_any().downcast_ref::<AssignmentExpression>() {
            self.compile_assignment(assign_expr)
        } else {
            Err(format!("Unsupported expression type: {}", expression.string()))
        }
    }
    
    // Forward declarations of expression compilation methods
    // These will be implemented in separate modules
    
    /// Compile an identifier expression (variable lookup)
    pub fn compile_identifier(&mut self, ident: &Identifier) -> Result<BasicValueEnum<'ctx>, String> {
        let var_name = &ident.value;
        
        // Check if the variable exists in our symbol table
        if let Some((ptr, ty)) = self.variables.get(var_name) {
            // Load the value from the allocation
            let value = self.builder.build_load(*ty, *ptr, var_name).unwrap();
            Ok(value)
        } else {
            // Variable not found, check if it's a function
            if let Some(func) = self.module.get_function(var_name) {
                // Return the function pointer
                Ok(func.as_global_value().as_pointer_value().into())
            } else {
                // Not found as variable or function
                Err(format!("Unknown variable or function: {}", var_name))
            }
        }
    }
    
    /// Compile a prefix expression (unary operation)
    pub fn compile_prefix_expression(&mut self, prefix: &PrefixExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Compile the right-hand side expression
        let right = self.compile_expression(prefix.right.as_ref())?;
        
        // Apply the appropriate operation based on the operator
        match prefix.operator.as_str() {
            // Negation (-x)
            "-" => self.compile_negation(right),
            // Logical NOT (!x)
            "!" => self.compile_logical_not(right),
            // Bitwise NOT (~x)
            "~" => self.compile_bitwise_not(right),
            // Address-of operator (@x)
            "@" => Err("Address-of operator should be handled by PointerDereference node".to_string()),
            _ => Err(format!("Unknown prefix operator: {}", prefix.operator))
        }
    }
    
    /// Compile negation operation (-x)
    fn compile_negation(&mut self, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        if right.is_int_value() {
            // Integer negation
            let result = self.builder.build_int_neg(
                right.into_int_value(),
                "neg"
            ).unwrap();
            Ok(result.into())
        } else if right.is_float_value() {
            // Float negation
            let result = self.builder.build_float_neg(
                right.into_float_value(),
                "fneg"
            ).unwrap();
            Ok(result.into())
        } else {
            Err(format!("Cannot negate non-numeric value of type: {:?}", right.get_type()))
        }
    }
    
    /// Compile logical NOT operation (!x)
    fn compile_logical_not(&mut self, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        if right.is_int_value() {
            // For booleans (stored as i1 or i8) and integers
            let zero = right.get_type().into_int_type().const_zero();
            let is_zero = self.builder.build_int_compare(
                IntPredicate::EQ,
                right.into_int_value(),
                zero,
                "is_zero"
            ).unwrap();
            Ok(is_zero.into())
        } else {
            Err(format!("Cannot apply logical NOT to non-boolean value of type: {:?}", right.get_type()))
        }
    }
    
    /// Compile bitwise NOT operation (~x)
    fn compile_bitwise_not(&mut self, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        if right.is_int_value() {
            let result = self.builder.build_not(
                right.into_int_value(),
                "not"
            ).unwrap();
            Ok(result.into())
        } else {
            Err(format!("Cannot apply bitwise NOT to non-integer value of type: {:?}", right.get_type()))
        }
    }
    
    /// Compile an infix expression (binary operation)
    pub fn compile_infix_expression(&mut self, infix: &InfixExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Special case for && and || which need short-circuit evaluation
        if infix.operator == "&&" || infix.operator == "||" {
            return self.compile_logical_operation(infix);
        }
        
        // Regular case: evaluate both operands
        let left = self.compile_expression(infix.left.as_ref())?;
        let right = self.compile_expression(infix.right.as_ref())?;
        
        // Handle string concatenation special case
        if self.is_string_value(left) && self.is_string_value(right) && infix.operator == "+" {
            return self.compile_string_concat(left, right);
        }
        
        // Apply the appropriate operation based on operand types and operator
        match (left, right) {
            // Integer operations
            (BasicValueEnum::IntValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                self.compile_int_infix(left_val, right_val, &infix.operator)
            },
            
            // Float operations
            (BasicValueEnum::FloatValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                self.compile_float_infix(left_val, right_val, &infix.operator)
            },
            
            // Mixed int and float - convert int to float
            (BasicValueEnum::IntValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                let left_float = self.builder.build_signed_int_to_float(
                    left_val, 
                    right_val.get_type(), 
                    "int_to_float"
                ).unwrap();
                self.compile_float_infix(left_float, right_val, &infix.operator)
            },
            
            (BasicValueEnum::FloatValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                let right_float = self.builder.build_signed_int_to_float(
                    right_val, 
                    left_val.get_type(), 
                    "int_to_float"
                ).unwrap();
                self.compile_float_infix(left_val, right_float, &infix.operator)
            },
            
            // Pointer operations (only equality and inequality)
            (BasicValueEnum::PointerValue(left_ptr), BasicValueEnum::PointerValue(right_ptr)) => {
                match infix.operator.as_str() {
                    "==" => {
                        // Use int comparison since build_ptr_eq isn't available
                        let left_int = self.builder.build_ptr_to_int(
                            left_ptr,
                            self.context.i64_type(),
                            "ptr_to_int_left"
                        ).unwrap();
                        
                        let right_int = self.builder.build_ptr_to_int(
                            right_ptr,
                            self.context.i64_type(),
                            "ptr_to_int_right"
                        ).unwrap();
                        
                        let result = self.builder.build_int_compare(
                            IntPredicate::EQ,
                            left_int,
                            right_int,
                            "ptr_eq"
                        ).unwrap();
                        
                        Ok(result.into())
                    },
                    "!=" => {
                        // Use int comparison since build_ptr_ne isn't available
                        let left_int = self.builder.build_ptr_to_int(
                            left_ptr,
                            self.context.i64_type(),
                            "ptr_to_int_left"
                        ).unwrap();
                        
                        let right_int = self.builder.build_ptr_to_int(
                            right_ptr,
                            self.context.i64_type(),
                            "ptr_to_int_right"
                        ).unwrap();
                        
                        let result = self.builder.build_int_compare(
                            IntPredicate::NE,
                            left_int,
                            right_int,
                            "ptr_ne"
                        ).unwrap();
                        
                        Ok(result.into())
                    },
                    _ => Err(format!("Unsupported operation '{}' on pointers", infix.operator))
                }
            },
            
            // If left and right are different types, try to handle based on operator
            _ => Err(format!("Unsupported binary operation: {} {} {}", 
                left.get_type(), infix.operator, right.get_type()))
        }
    }
    
    /// Compile a logical operation with short-circuit evaluation (&&, ||)
    fn compile_logical_operation(&mut self, infix: &InfixExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Get the current function
        let function = self.current_function.ok_or_else(|| 
            "Logical operation outside of function context".to_string()
        )?;
        
        // Create basic blocks for the right operand and the merge point
        let right_block = self.context.append_basic_block(function, "logical_right");
        let merge_block = self.context.append_basic_block(function, "logical_merge");
        
        // Evaluate the left operand
        let left = self.compile_expression(infix.left.as_ref())?;
        
        if !left.is_int_value() {
            return Err("Logical operations require boolean operands".to_string());
        }
        
        let left_val = left.into_int_value();
        let bool_type = self.context.bool_type();
        
        // Convert to a boolean if not already (compare with zero)
        let left_bool = if left_val.get_type() != bool_type {
            let zero = left_val.get_type().const_zero();
            self.builder.build_int_compare(
                IntPredicate::NE,
                left_val,
                zero,
                "to_bool"
            ).unwrap()
        } else {
            left_val
        };
        
        // Create a phi node for the result
        let phi_node = self.builder.build_phi(bool_type, "logical_result").unwrap();
        
        if infix.operator == "&&" {
            // For &&, if left is false, short-circuit to false
            // Otherwise, evaluate right and use its value
            
            // If left is false, short-circuit
            phi_node.add_incoming(&[(&bool_type.const_int(0, false), self.builder.get_insert_block().unwrap())]);
            
            // Branch to right_block only if left is true
            self.builder.build_conditional_branch(left_bool, right_block, merge_block).unwrap();
        } else {
            // For ||, if left is true, short-circuit to true
            // Otherwise, evaluate right and use its value
            
            // If left is true, short-circuit
            phi_node.add_incoming(&[(&bool_type.const_int(1, false), self.builder.get_insert_block().unwrap())]);
            
            // Branch to right_block only if left is false
            self.builder.build_conditional_branch(left_bool, merge_block, right_block).unwrap();
        }
        
        // Compile the right operand in the right_block
        self.builder.position_at_end(right_block);
        let right = self.compile_expression(infix.right.as_ref())?;
        
        if !right.is_int_value() {
            return Err("Logical operations require boolean operands".to_string());
        }
        
        let right_val = right.into_int_value();
        
        // Convert to a boolean if not already
        let right_bool = if right_val.get_type() != bool_type {
            let zero = right_val.get_type().const_zero();
            self.builder.build_int_compare(
                IntPredicate::NE,
                right_val,
                zero,
                "to_bool"
            ).unwrap()
        } else {
            right_val
        };
        
        // Add the right value to the phi node
        phi_node.add_incoming(&[(&right_bool, self.builder.get_insert_block().unwrap())]);
        
        // Branch to the merge block
        self.builder.build_unconditional_branch(merge_block).unwrap();
        
        // Set insertion point to the merge block
        self.builder.position_at_end(merge_block);
        
        // Return the phi node value
        Ok(phi_node.as_basic_value())
    }
    
    /// Compile integer binary operations
    fn compile_int_infix(
        &mut self,
        left: inkwell::values::IntValue<'ctx>,
        right: inkwell::values::IntValue<'ctx>,
        op: &str
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match op {
            // Arithmetic
            "+" => {
                let result = self.builder.build_int_add(left, right, "add").unwrap();
                Ok(result.into())
            },
            "-" => {
                let result = self.builder.build_int_sub(left, right, "sub").unwrap();
                Ok(result.into())
            },
            "*" => {
                let result = self.builder.build_int_mul(left, right, "mul").unwrap();
                Ok(result.into())
            },
            "/" => {
                // Check for division by zero - in a real compiler we'd add runtime checks
                let result = self.builder.build_int_signed_div(left, right, "div").unwrap();
                Ok(result.into())
            },
            "%" => {
                let result = self.builder.build_int_signed_rem(left, right, "rem").unwrap();
                Ok(result.into())
            },
            
            // Bitwise operations
            "&" => {
                let result = self.builder.build_and(left, right, "and").unwrap();
                Ok(result.into())
            },
            "|" => {
                let result = self.builder.build_or(left, right, "or").unwrap();
                Ok(result.into())
            },
            "^" => {
                let result = self.builder.build_xor(left, right, "xor").unwrap();
                Ok(result.into())
            },
            "<<" => {
                let result = self.builder.build_left_shift(left, right, "shl").unwrap();
                Ok(result.into())
            },
            ">>" => {
                let result = self.builder.build_right_shift(left, right, true, "shr").unwrap();
                Ok(result.into())
            },
            
            // Comparison
            "==" => {
                let result = self.builder.build_int_compare(IntPredicate::EQ, left, right, "eq").unwrap();
                Ok(result.into())
            },
            "!=" => {
                let result = self.builder.build_int_compare(IntPredicate::NE, left, right, "ne").unwrap();
                Ok(result.into())
            },
            "<" => {
                let result = self.builder.build_int_compare(IntPredicate::SLT, left, right, "lt").unwrap();
                Ok(result.into())
            },
            "<=" => {
                let result = self.builder.build_int_compare(IntPredicate::SLE, left, right, "le").unwrap();
                Ok(result.into())
            },
            ">" => {
                let result = self.builder.build_int_compare(IntPredicate::SGT, left, right, "gt").unwrap();
                Ok(result.into())
            },
            ">=" => {
                let result = self.builder.build_int_compare(IntPredicate::SGE, left, right, "ge").unwrap();
                Ok(result.into())
            },
            
            _ => Err(format!("Unsupported integer operation: {}", op))
        }
    }
    
    /// Compile floating point binary operations
    fn compile_float_infix(
        &mut self,
        left: inkwell::values::FloatValue<'ctx>,
        right: inkwell::values::FloatValue<'ctx>,
        op: &str
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match op {
            // Arithmetic
            "+" => {
                let result = self.builder.build_float_add(left, right, "fadd").unwrap();
                Ok(result.into())
            },
            "-" => {
                let result = self.builder.build_float_sub(left, right, "fsub").unwrap();
                Ok(result.into())
            },
            "*" => {
                let result = self.builder.build_float_mul(left, right, "fmul").unwrap();
                Ok(result.into())
            },
            "/" => {
                let result = self.builder.build_float_div(left, right, "fdiv").unwrap();
                Ok(result.into())
            },
            "%" => {
                let result = self.builder.build_float_rem(left, right, "frem").unwrap();
                Ok(result.into())
            },
            
            // Comparison
            "==" => {
                let result = self.builder.build_float_compare(FloatPredicate::OEQ, left, right, "feq").unwrap();
                Ok(result.into())
            },
            "!=" => {
                let result = self.builder.build_float_compare(FloatPredicate::ONE, left, right, "fne").unwrap();
                Ok(result.into())
            },
            "<" => {
                let result = self.builder.build_float_compare(FloatPredicate::OLT, left, right, "flt").unwrap();
                Ok(result.into())
            },
            "<=" => {
                let result = self.builder.build_float_compare(FloatPredicate::OLE, left, right, "fle").unwrap();
                Ok(result.into())
            },
            ">" => {
                let result = self.builder.build_float_compare(FloatPredicate::OGT, left, right, "fgt").unwrap();
                Ok(result.into())
            },
            ">=" => {
                let result = self.builder.build_float_compare(FloatPredicate::OGE, left, right, "fge").unwrap();
                Ok(result.into())
            },
            
            _ => Err(format!("Unsupported floating point operation: {}", op))
        }
    }
    
    /// Compile an if expression (if condition { consequence } else { alternative })
    pub fn compile_if_expression(&mut self, if_expr: &IfStatement) -> Result<BasicValueEnum<'ctx>, String> {
        // Get the current function
        let function = self.current_function.ok_or_else(|| 
            "If expression outside of function context".to_string()
        )?;
        
        // Compile the condition expression
        let condition_value = self.compile_expression(if_expr.condition.as_ref())?;
        
        // Convert to a boolean value if needed
        let condition_bool = if condition_value.is_int_value() {
            let int_val = condition_value.into_int_value();
            let zero = int_val.get_type().const_zero();
            self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "if_cond").unwrap()
        } else {
            return Err("If condition must be a boolean or integer value".to_string());
        };
        
        // Create the blocks for then, else, and merge
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        let merge_block = self.context.append_basic_block(function, "merge");
        
        // Branch based on the condition
        self.builder.build_conditional_branch(condition_bool, then_block, else_block).unwrap();
        
        // Create a phi node for the result - this assumes both branches return the same type
        // We'll determine the result type based on the consequence and alternative blocks
        
        // For now, create a placeholder phi node - we'll set its type later
        let phi_node: Option<inkwell::values::PhiValue<'ctx>> = None;
        
        // Generate code for the 'then' block
        self.builder.position_at_end(then_block);
        let then_value = if let Some(block_stmt) = if_expr.consequence.as_any().downcast_ref::<BlockStatement>() {
            // Execute the block and get its result
            // For now, we'll assume the last statement's value is the result
            let mut result = None;
            
            for stmt in &block_stmt.statements {
                // Handle return statements specially
                if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
                    // If it's a return, compile it and we're done with this block
                    self.compile_statement(return_stmt)?;
                    // No need to set result, as we've already returned
                    break;
                } else if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    // For expression statements, compile and save the result
                    if let Some(expr) = &expr_stmt.expression {
                        result = Some(self.compile_expression(expr.as_ref())?);
                    }
                } else {
                    // For other statements, just compile them
                    self.compile_statement(stmt.as_ref())?;
                }
            }
            
            // If we didn't return and have a result, use it
            result
        } else {
            // If not a block statement (unusual), create a dummy value
            // In a proper implementation, we'd handle this better
            Some(self.context.i32_type().const_int(0, false).into())
        };
        
        // Branch to the merge block if we haven't already branched elsewhere
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }
        
        // Remember the then block's last value and basic block
        let then_end_block = self.builder.get_insert_block().unwrap();
        
        // Generate code for the 'else' block
        self.builder.position_at_end(else_block);
        let else_value = if let Some(alt) = &if_expr.alternative {
            if let Some(block_stmt) = alt.as_any().downcast_ref::<BlockStatement>() {
                // Execute the block and get its result
                // For now, we'll assume the last statement's value is the result
                let mut result = None;
                
                for stmt in &block_stmt.statements {
                    // Handle return statements specially
                    if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
                        // If it's a return, compile it and we're done with this block
                        self.compile_statement(return_stmt)?;
                        // No need to set result, as we've already returned
                        break;
                    } else if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                        // For expression statements, compile and save the result
                        if let Some(expr) = &expr_stmt.expression {
                            result = Some(self.compile_expression(expr.as_ref())?);
                        }
                    } else {
                        // For other statements, just compile them
                        self.compile_statement(stmt.as_ref())?;
                    }
                }
                
                // If we didn't return and have a result, use it
                result
            } else {
                // If not a block statement (unusual), create a dummy value
                Some(self.context.i32_type().const_int(0, false).into())
            }
        } else {
            // No alternative, use void/unit value (represented as i32 0 for now)
            Some(self.context.i32_type().const_int(0, false).into())
        };
        
        // Branch to the merge block if we haven't already branched elsewhere
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }
        
        // Remember the else block's last value and basic block
        let else_end_block = self.builder.get_insert_block().unwrap();
        
        // Emit the merge block
        self.builder.position_at_end(merge_block);
        
        // Now we can create the phi node with the correct result type
        // But only if both branches produced a result
        if let (Some(then_val), Some(else_val)) = (then_value, else_value) {
            // Check if the types are compatible
            if then_val.get_type() == else_val.get_type() {
                // Same type, create a phi node
                let phi = self.builder.build_phi(then_val.get_type(), "if_result").unwrap();
                phi.add_incoming(&[(&then_val, then_end_block), (&else_val, else_end_block)]);
                return Ok(phi.as_basic_value());
            } else {
                // Different types, see if we can convert one to the other
                // This is a simplification; a real implementation would have more robust type conversion
                return Err(format!("If expression branches have incompatible types: {:?} and {:?}", 
                    then_val.get_type(), else_val.get_type()));
            }
        } else if then_value.is_some() {
            // Only the then branch produced a result
            return Ok(then_value.unwrap());
        } else if else_value.is_some() {
            // Only the else branch produced a result
            return Ok(else_value.unwrap());
        } else {
            // Neither branch produced a result, return void/unit (i32 0)
            return Ok(self.context.i32_type().const_int(0, false).into());
        }
    }
    
    // Using the implementation from function.rs instead
    // fn compile_function_literal(&mut self, fn_lit: &FunctionLiteral) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    // Using the implementation from function.rs instead
    // fn compile_call_expression(&mut self, call_expr: &CallExpression) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    // Implementation in array.rs
    // fn compile_array_literal(&mut self, array_lit: &ArrayLiteral) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    // Implementation in array.rs
    // fn compile_index_expression(&mut self, index_expr: &IndexExpression) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    // Implementation in hash.rs
    // fn compile_hash_literal(&mut self, hash_lit: &HashLiteral) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    /// Compile a property access expression (object.property)
    pub fn compile_property_access(&mut self, prop_access: &PropertyAccessExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // In the refactored version, property access requires more robust type information
        // and structure field mappings that we don't have available in this stub implementation.
        // A full implementation would require proper struct type tracking.
        
        // For now, we'll return an error indicating this is not fully implemented
        Err("Property access not fully implemented in the refactored version".to_string())
    }
    
    /// Compile an assignment expression (variable = value)
    pub fn compile_assignment(&mut self, assign_expr: &AssignmentExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // First evaluate the right-hand side
        let value = self.compile_expression(assign_expr.value.as_ref())?;
        
        // The left-hand side must be an l-value (something we can assign to)
        // In this simplified implementation, we'll only handle direct variable assignment
        
        // Handle assignment to variable (identifier)
        if let Some(ident) = assign_expr.name.as_any().downcast_ref::<Identifier>() {
            let var_name = &ident.value;
            
            // Check if the variable exists in our symbol table
            if let Some((ptr, ty)) = self.variables.get(var_name) {
                // Type check the assignment - simplified implementation
                // In a full implementation, we'd handle type conversions
                // This simplified version just uses the value directly
                let value = value; // No type conversion
                
                
                // Store the value in the variable
                self.builder.build_store(*ptr, value).unwrap();
                
                // Assignment expressions return the assigned value
                Ok(value)
            } else {
                // Variable not found
                Err(format!("Cannot assign to unknown variable: {}", var_name))
            }
        } else {
            // For the refactored implementation, we'll only support variable assignment
            // Other types of assignment (array indexing, struct fields) would be added later
            Err(format!("Complex assignment not implemented in the refactored version"))
        }
    }
    
    // Using the implementation from core.rs instead
    // fn compile_type_conversion(&mut self, type_conv: &TypeConversionExpression) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
    
    // Implementation in stan.rs
    // fn compile_stan_expression(&mut self, stan_expr: &StanExpression) -> Result<BasicValueEnum<'ctx>, String> {
    //     // Placeholder - will be implemented in a separate module
    //     Err("Not implemented yet".to_string())
    // }
}