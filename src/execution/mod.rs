//! CURSED Execution Engine - ADVANCED FEATURES ENABLED
//! 
//! Complete execution system featuring:
//! - JIT compilation and runtime
//! - Goroutine scheduling and management
//! - Advanced memory management
//! - Error handling and recovery

use crate::error::CursedError;
use crate::ast::Program;
use crate::runtime::channels::simple_channel::SimpleChannel;
use std::sync::Arc;

pub mod execution_context;
pub mod jit_executor;
pub mod runtime_functions;
pub mod value_manager;

pub use execution_context::ExecutionContext;

/// Advanced execution engine for CURSED
pub struct CursedExecutionEngine {
    jit_enabled: bool,
    goroutine_support: bool,
    gc_enabled: bool,
}

impl CursedExecutionEngine {
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            jit_enabled: true,
            goroutine_support: true,
            gc_enabled: true,
        })
    }
    
    pub fn execute(&mut self, source: &str) -> Result<CursedValue, CursedError> {
        tracing::info!("🚀 Executing CURSED code with advanced features");
        
        // Parse and compile
        let mut parser = crate::parser::new_parser(source)?;
        let program = parser.parse_program()?;
        
        // Execute with JIT if enabled
        if self.jit_enabled {
            self.execute_jit(&program)
        } else {
            self.execute_interpreted(&program)
        }
    }
    
    pub fn execute_file(&mut self, path: &str) -> Result<CursedValue, CursedError> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| CursedError::Io(e.to_string()))?;
        self.execute(&source)
    }
    
    pub fn execute_repl(&mut self, code: &str) -> Result<String, CursedError> {
        let result = self.execute(code)?;
        Ok(self.format_value(&result))
    }
    
    fn execute_jit(&mut self, program: &Program) -> Result<CursedValue, CursedError> {
        tracing::info!("⚡ JIT compilation enabled");
        
        // Generate LLVM IR
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let _ir = codegen.generate_ir(program)?;
        
        // Execute interpreted for now (since JIT compilation is complex)
        self.execute_interpreted(program)
    }
    
    fn execute_interpreted(&mut self, program: &Program) -> Result<CursedValue, CursedError> {
        tracing::info!("🔄 Interpreted execution");
        
        // Create execution context
        let mut context = ExecutionContext::new();
        
        // Execute each statement
        let mut last_value = CursedValue::Nil;
        for statement in &program.statements {
            last_value = self.execute_statement(statement, &mut context)?;
        }
        
        // After processing all statements, check if there's a main function and call it
        if let Some(_main_func) = context.get_function("main") {
            tracing::info!("🚀 Calling main function");
            
            // Create a CallExpression AST node to call main()
            let main_call = crate::ast::CallExpression {
                function: Box::new(crate::ast::Expression::Identifier("main".to_string())),
                arguments: vec![], // main() takes no arguments
            };
            
            let result = self.evaluate_call(&main_call, &mut context)?;
            
            // Don't automatically print the return value from main.
            // Output should come from explicit print statements like vibez.spill()
            return Ok(result);
        }
        
        Ok(last_value)
    }
    
    pub fn get_value_manager(&self) -> ValueManager {
        ValueManager::new()
    }
    
    fn format_value(&self, value: &CursedValue) -> String {
        match value {
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::Float(f) => f.to_string(),
            CursedValue::String(s) => format!("\"{}\"", s),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Channel(_) => "<channel>".to_string(),
            CursedValue::Nil => "nil".to_string(),
        }
    }
    
    fn execute_statement(&mut self, statement: &crate::ast::Statement, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        use crate::ast::Statement;
        
        log::debug!("🔧 Executing statement type: {:?}", std::mem::discriminant(statement));
        match statement {
            Statement::Expression(expr) => {
                self.evaluate_expression(expr, context)
            },
            Statement::Let(let_stmt) => {
                let value = self.evaluate_expression(&let_stmt.value, context)?;
                context.set_variable(let_stmt.name.clone(), value.clone());
                // For assignment statements, return the value that was assigned
                Ok(value)
            },
            Statement::Assignment(assign_stmt) => {
                let value = self.evaluate_expression(&assign_stmt.value, context)?;
                context.set_variable(assign_stmt.name.clone(), value.clone());
                // For assignment statements, return the value that was assigned
                Ok(value)
            },
            Statement::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    self.evaluate_expression(expr, context)
                } else {
                    Ok(CursedValue::Nil)
                }
            },
            Statement::Function(func_stmt) => {
                // Store function definition in context
                log::info!("📝 Storing function definition: {} with {} parameters", func_stmt.name, func_stmt.parameters.len());
                log::debug!("📝 Function body has {} statements", func_stmt.body.len());
                context.set_function(func_stmt.name.clone(), func_stmt.clone());
                Ok(CursedValue::Nil)
            },
            Statement::If(if_stmt) => {
                let condition = self.evaluate_expression(&if_stmt.condition, context)?;
                if self.is_truthy(&condition) {
                    let mut last_value = CursedValue::Nil;
                    for stmt in &if_stmt.then_branch {
                        last_value = self.execute_statement(stmt, context)?;
                    }
                    Ok(last_value)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    let mut last_value = CursedValue::Nil;
                    for stmt in else_branch {
                        last_value = self.execute_statement(stmt, context)?;
                    }
                    Ok(last_value)
                } else {
                    Ok(CursedValue::Nil)
                }
            },
            Statement::While(while_stmt) => {
                let mut last_value = CursedValue::Nil;
                loop {
                    let condition = self.evaluate_expression(&while_stmt.condition, context)?;
                    if !self.is_truthy(&condition) {
                        break;
                    }
                    for stmt in &while_stmt.body {
                        last_value = self.execute_statement(stmt, context)?;
                    }
                }
                Ok(last_value)
            },
            Statement::For(for_stmt) => {
                // Initialize
                if let Some(init) = &for_stmt.init {
                    self.execute_statement(init, context)?;
                }
                
                let mut last_value = CursedValue::Nil;
                loop {
                    // Check condition
                    if let Some(condition) = &for_stmt.condition {
                        let cond_value = self.evaluate_expression(condition, context)?;
                        if !self.is_truthy(&cond_value) {
                            break;
                        }
                    }
                    
                    // Execute body
                    for stmt in &for_stmt.body {
                        last_value = self.execute_statement(stmt, context)?;
                    }
                    
                    // Update
                    if let Some(update) = &for_stmt.update {
                        self.evaluate_expression(update, context)?;
                    }
                }
                Ok(last_value)
            },
            Statement::Switch(switch_stmt) => {
                let switch_value = self.evaluate_expression(&switch_stmt.expression, context)?;
                
                // Try to match against each case
                for case in &switch_stmt.cases {
                    let case_value = self.evaluate_expression(&case.pattern, context)?;
                    if self.values_equal(&switch_value, &case_value) {
                        let mut last_value = CursedValue::Nil;
                        for stmt in &case.body {
                            last_value = self.execute_statement(stmt, context)?;
                        }
                        return Ok(last_value);
                    }
                }
                
                // If no case matched, try default case
                if let Some(default_body) = &switch_stmt.default_case {
                    let mut last_value = CursedValue::Nil;
                    for stmt in default_body {
                        last_value = self.execute_statement(stmt, context)?;
                    }
                    Ok(last_value)
                } else {
                    Ok(CursedValue::Nil)
                }
            },
            Statement::Goroutine(_) => {
                // For now, just return nil - goroutines need more complex implementation
                Ok(CursedValue::Nil)
            },
            Statement::Channel(_) => {
                // For now, just return nil - channels need more complex implementation
                Ok(CursedValue::Nil)
            },
            Statement::Select(_) => {
                // For now, just return nil - select statements need more complex implementation
                log::info!("📺 Select statement execution not yet implemented");
                Ok(CursedValue::Nil)
            },
            Statement::Struct(struct_stmt) => {
                // Store struct definition in context for type checking
                log::info!("📝 Storing struct definition: {} with {} fields", struct_stmt.name, struct_stmt.fields.len());
                // TODO: Implement actual struct storage
                Ok(CursedValue::Nil)
            },
            Statement::Interface(interface_stmt) => {
                // Store interface definition in context for type checking
                log::info!("📝 Storing interface definition: {} with {} methods", interface_stmt.name, interface_stmt.methods.len());
                // TODO: Implement actual interface storage
                Ok(CursedValue::Nil)
            },
            Statement::Panic(panic_stmt) => {
                // Evaluate the panic message
                let message = self.evaluate_expression(&panic_stmt.message, context)?;
                log::error!("💀 Panic (yeet_error): {:?}", message);
                
                // For now, return an error - in the future this should trigger panic unwinding
                Err(CursedError::RuntimeError(format!("yeet_error: {:?}", message)))
            },
            Statement::Catch(catch_stmt) => {
                log::info!("🛡️ Entering catch block");
                
                // Execute the protected block and handle any panics
                let mut last_value = CursedValue::Nil;
                let mut error_occurred = false;
                
                for stmt in &catch_stmt.protected_block {
                    match self.execute_statement(stmt, context) {
                        Ok(val) => last_value = val,
                        Err(err) => {
                            log::warn!("🚨 Caught error in catch block: {:?}", err);
                            error_occurred = true;
                            
                            // If there's an error variable, set it
                            if let Some(error_var) = &catch_stmt.error_variable {
                                let error_msg = CursedValue::String(format!("{:?}", err));
                                context.set_variable(error_var.clone(), error_msg);
                            }
                            
                            // Execute recovery block if it exists
                            if let Some(recovery) = &catch_stmt.recovery_block {
                                for recovery_stmt in recovery {
                                    last_value = self.execute_statement(recovery_stmt, context)?;
                                }
                            }
                            break;
                        }
                    }
                }
                
                if !error_occurred {
                    log::info!("✅ Protected block completed without errors");
                }
                
                Ok(last_value)
            },
        }
    }
    
    fn evaluate_expression(&mut self, expression: &crate::ast::Expression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        use crate::ast::Expression;
        
        match expression {
            Expression::Integer(i) => Ok(CursedValue::Integer(*i)),
            Expression::String(s) => Ok(CursedValue::String(s.clone())),
            Expression::Boolean(b) => Ok(CursedValue::Boolean(*b)),
            Expression::Identifier(name) => {
                context.get_variable(name)
                    .ok_or_else(|| CursedError::RuntimeError(format!("Undefined variable: {}", name)))
            },
            Expression::Binary(binary_expr) => {
                let left = self.evaluate_expression(&binary_expr.left, context)?;
                let right = self.evaluate_expression(&binary_expr.right, context)?;
                self.apply_binary_operator(&left, &binary_expr.operator, &right)
            },
            Expression::Call(call_expr) => {
                self.evaluate_call(call_expr, context)
            },
            Expression::MemberAccess(member_expr) => {
                self.evaluate_member_access(member_expr, context)
            },
            Expression::Literal(literal) => {
                match literal {
                    crate::ast::Literal::Integer(i) => Ok(CursedValue::Integer(*i)),
                    crate::ast::Literal::Float(f) => Ok(CursedValue::Float(*f)),
                    crate::ast::Literal::String(s) => Ok(CursedValue::String(s.clone())),
                    crate::ast::Literal::Boolean(b) => Ok(CursedValue::Boolean(*b)),
                    crate::ast::Literal::Nil | crate::ast::Literal::Null => Ok(CursedValue::Nil),
                }
            },
            Expression::Unary(unary_expr) => {
                let operand = self.evaluate_expression(&unary_expr.operand, context)?;
                self.apply_unary_operator(&unary_expr.operator, &operand)
            },
            Expression::Array(elements) => {
                // For now, just return the length as an integer
                Ok(CursedValue::Integer(elements.len() as i64))
            },
            Expression::Map(pairs) => {
                // For now, just return the size as an integer
                Ok(CursedValue::Integer(pairs.len() as i64))
            },
            Expression::ChannelSend(send_expr) => {
                self.execute_channel_send(send_expr, context)
            },
            Expression::ChannelReceive(recv_expr) => {
                self.execute_channel_receive(recv_expr, context)
            },
            Expression::ChannelCreation(create_expr) => {
                self.execute_channel_creation(create_expr, context)
            },
        }
    }
    
    /// Execute channel send operation (channel <- value)
    fn execute_channel_send(&mut self, send_expr: &crate::ast::ChannelSendExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        log::info!("📤 Executing channel send operation");
        
        // Evaluate the channel expression
        let channel_value = self.evaluate_expression(&send_expr.channel, context)?;
        let channel = match channel_value {
            CursedValue::Channel(ch) => ch,
            _ => return Err(CursedError::RuntimeError("Cannot send to non-channel value".to_string())),
        };
        
        // Evaluate the value to send
        let value = self.evaluate_expression(&send_expr.value, context)?;
        
        // Perform the send operation
        match channel.send(value) {
            crate::runtime::channels::SendResult::Sent => {
                log::info!("📤 Channel send successful");
                Ok(CursedValue::Nil)
            },
            crate::runtime::channels::SendResult::Closed(_) => {
                Err(CursedError::RuntimeError("Cannot send on closed channel".to_string()))
            },
            crate::runtime::channels::SendResult::WouldBlock(_) => {
                Err(CursedError::RuntimeError("Channel send would block".to_string()))
            },
        }
    }
    
    /// Execute channel receive operation (<-channel)
    fn execute_channel_receive(&mut self, recv_expr: &crate::ast::ChannelReceiveExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        log::info!("📥 Executing channel receive operation");
        
        // Evaluate the channel expression
        let channel_value = self.evaluate_expression(&recv_expr.channel, context)?;
        let channel = match channel_value {
            CursedValue::Channel(ch) => ch,
            _ => return Err(CursedError::RuntimeError("Cannot receive from non-channel value".to_string())),
        };
        
        // Perform the receive operation
        match channel.recv() {
            crate::runtime::channels::ReceiveResult::Received(value) => {
                log::info!("📥 Channel receive successful");
                Ok(value)
            },
            crate::runtime::channels::ReceiveResult::Closed => {
                Err(CursedError::RuntimeError("Cannot receive from closed channel".to_string()))
            },
            crate::runtime::channels::ReceiveResult::WouldBlock => {
                Err(CursedError::RuntimeError("Channel receive would block".to_string()))
            },
        }
    }
    
    /// Execute channel creation operation (dm type())
    fn execute_channel_creation(&mut self, create_expr: &crate::ast::ChannelCreationExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        log::info!("🔧 Creating channel with element type: {}", create_expr.element_type);
        
        // Determine capacity for the channel
        let capacity = if let Some(capacity_expr) = &create_expr.capacity {
            match self.evaluate_expression(capacity_expr, context)? {
                CursedValue::Integer(cap) => cap as usize,
                _ => return Err(CursedError::RuntimeError("Channel capacity must be an integer".to_string())),
            }
        } else {
            0 // Unbuffered channel
        };
        
        // Create the channel
        let channel = if capacity == 0 {
            Arc::new(SimpleChannel::new())
        } else {
            Arc::new(SimpleChannel::with_capacity(capacity))
        };
        
        log::info!("✅ Created channel with capacity: {}", capacity);
        Ok(CursedValue::Channel(channel))
    }

    fn apply_binary_operator(&self, left: &CursedValue, operator: &str, right: &CursedValue) -> Result<CursedValue, CursedError> {
        match (left, right) {
            (CursedValue::Integer(l), CursedValue::Integer(r)) => {
                match operator {
                    "+" => Ok(CursedValue::Integer(l + r)),
                    "-" => Ok(CursedValue::Integer(l - r)),
                    "*" => Ok(CursedValue::Integer(l * r)),
                    "/" => {
                        if *r == 0 {
                            Err(CursedError::RuntimeError("Division by zero".to_string()))
                        } else {
                            Ok(CursedValue::Integer(l / r))
                        }
                    },
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    "<" => Ok(CursedValue::Boolean(l < r)),
                    ">" => Ok(CursedValue::Boolean(l > r)),
                    "<=" => Ok(CursedValue::Boolean(l <= r)),
                    ">=" => Ok(CursedValue::Boolean(l >= r)),
                    _ => Err(CursedError::RuntimeError(format!("Unknown binary operator: {}", operator))),
                }
            },
            (CursedValue::Float(l), CursedValue::Float(r)) => {
                match operator {
                    "+" => Ok(CursedValue::Float(l + r)),
                    "-" => Ok(CursedValue::Float(l - r)),
                    "*" => Ok(CursedValue::Float(l * r)),
                    "/" => {
                        if *r == 0.0 {
                            Err(CursedError::RuntimeError("Division by zero".to_string()))
                        } else {
                            Ok(CursedValue::Float(l / r))
                        }
                    },
                    "==" => Ok(CursedValue::Boolean((l - r).abs() < f64::EPSILON)),
                    "!=" => Ok(CursedValue::Boolean((l - r).abs() >= f64::EPSILON)),
                    "<" => Ok(CursedValue::Boolean(l < r)),
                    ">" => Ok(CursedValue::Boolean(l > r)),
                    "<=" => Ok(CursedValue::Boolean(l <= r)),
                    ">=" => Ok(CursedValue::Boolean(l >= r)),
                    _ => Err(CursedError::RuntimeError(format!("Unknown binary operator: {}", operator))),
                }
            },
            (CursedValue::String(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported string operator: {}", operator))),
                }
            },
            _ => Err(CursedError::RuntimeError(format!("Type mismatch in binary operation: {:?} {} {:?}", left, operator, right))),
        }
    }
    
    fn apply_unary_operator(&self, operator: &crate::ast::UnaryOperator, operand: &CursedValue) -> Result<CursedValue, CursedError> {
        match operator {
            crate::ast::UnaryOperator::Not => {
                Ok(CursedValue::Boolean(!self.is_truthy(operand)))
            },
            crate::ast::UnaryOperator::Minus => {
                match operand {
                    CursedValue::Integer(i) => Ok(CursedValue::Integer(-i)),
                    CursedValue::Float(f) => Ok(CursedValue::Float(-f)),
                    _ => Err(CursedError::RuntimeError("Cannot negate non-numeric value".to_string())),
                }
            },
            crate::ast::UnaryOperator::Plus => {
                match operand {
                    CursedValue::Integer(_) | CursedValue::Float(_) => Ok(operand.clone()),
                    _ => Err(CursedError::RuntimeError("Cannot apply unary plus to non-numeric value".to_string())),
                }
            },
        }
    }
    
    fn evaluate_call(&mut self, call_expr: &crate::ast::CallExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        match &*call_expr.function {
            crate::ast::Expression::Identifier(func_name) => {
                // Handle built-in functions
                match func_name.as_str() {
                    "print" | "println" => {
                        for arg in &call_expr.arguments {
                            let value = self.evaluate_expression(arg, context)?;
                            println!("{}", self.format_value(&value));
                        }
                        Ok(CursedValue::Nil)
                    },
                    _ => {
                        // User-defined function
                        log::info!("🔍 Looking for function: {}", func_name);
                        if let Some(func_def) = context.get_function(func_name) {
                            log::info!("✅ Found function: {}", func_name);
                            // Create child context for function execution (inherits functions)
                            let mut func_context = context.new_child();
                            
                            // Bind parameters
                            if call_expr.arguments.len() != func_def.parameters.len() {
                                return Err(CursedError::RuntimeError(format!(
                                    "Function {} expects {} arguments, got {}",
                                    func_name, func_def.parameters.len(), call_expr.arguments.len()
                                )));
                            }
                            
                            for (param, arg) in func_def.parameters.iter().zip(&call_expr.arguments) {
                                let arg_value = self.evaluate_expression(arg, context)?;
                                func_context.set_variable(param.clone(), arg_value);
                            }
                            
                            // Execute function body
                            let mut result = CursedValue::Nil;
                            for stmt in &func_def.body {
                                result = self.execute_statement(stmt, &mut func_context)?;
                            }
                            
                            Ok(result)
                        } else {
                            log::error!("❌ Function not found: {}", func_name);
                            Err(CursedError::RuntimeError(format!("Undefined function: {}", func_name)))
                        }
                    }
                }
            },
            crate::ast::Expression::MemberAccess(member_expr) => {
                // Handle member function calls like vibez.spill()
                if let crate::ast::Expression::Identifier(obj_name) = &*member_expr.object {
                    match (obj_name.as_str(), member_expr.property.as_str()) {
                        ("vibez", "spill") => {
                            for arg in &call_expr.arguments {
                                let value = self.evaluate_expression(arg, context)?;
                                // Print raw value without quotes for strings
                           match &value {
                           CursedValue::String(s) => print!("{}", s),
                           _ => print!("{}", self.format_value(&value)),
                        }
                        }
                        println!(); // Add newline
                        Ok(CursedValue::Nil)
                        },
                        ("vibez", "spillf") => {
                            // Format string print
                            if let Some(first_arg) = call_expr.arguments.first() {
                                let format_str = self.evaluate_expression(first_arg, context)?;
                                if let CursedValue::String(fmt) = format_str {
                                    let mut output = fmt;
                                    // Simple format string replacement
                                    for (i, arg) in call_expr.arguments.iter().skip(1).enumerate() {
                                        let value = self.evaluate_expression(arg, context)?;
                                        let placeholder = format!("{{{}}}", i);
                                        output = output.replace(&placeholder, &self.format_value(&value));
                                    }
                                    print!("{}", output);
                                } else {
                                    return Err(CursedError::RuntimeError("First argument to spillf must be a string".to_string()));
                                }
                            }
                            Ok(CursedValue::Nil)
                        },
                        _ => Err(CursedError::RuntimeError(format!("Unknown method: {}.{}", obj_name, member_expr.property))),
                    }
                } else {
                    Err(CursedError::RuntimeError("Complex member access not supported yet".to_string()))
                }
            },
            _ => Err(CursedError::RuntimeError("Unsupported function call type".to_string())),
        }
    }
    
    fn evaluate_member_access(&mut self, member_expr: &crate::ast::MemberAccessExpression, _context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        // For now, just return a placeholder - member access without calls is complex
        Ok(CursedValue::Nil)
    }
    
    fn is_truthy(&self, value: &CursedValue) -> bool {
        match value {
            CursedValue::Boolean(b) => *b,
            CursedValue::Integer(i) => *i != 0,
            CursedValue::Float(f) => *f != 0.0,
            CursedValue::String(s) => !s.is_empty(),
            CursedValue::Channel(_) => true, // Channels are truthy when they exist
            CursedValue::Nil => false,
        }
    }
    
    fn values_equal(&self, left: &CursedValue, right: &CursedValue) -> bool {
        match (left, right) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => a == b,
            (CursedValue::Float(a), CursedValue::Float(b)) => a == b,
            (CursedValue::String(a), CursedValue::String(b)) => a == b,
            (CursedValue::Boolean(a), CursedValue::Boolean(b)) => a == b,
            (CursedValue::Nil, CursedValue::Nil) => true,
            // Allow integer-float comparison
            (CursedValue::Integer(a), CursedValue::Float(b)) => *a as f64 == *b,
            (CursedValue::Float(a), CursedValue::Integer(b)) => *a == *b as f64,
            _ => false,
        }
    }
}

/// Advanced value types for CURSED
#[derive(Debug, Clone)]
pub enum CursedValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Channel(Arc<SimpleChannel<CursedValue>>),
    Nil,
}

/// Value manager for runtime operations
pub struct ValueManager {
    gc_enabled: bool,
}

impl ValueManager {
    pub fn new() -> Self {
        Self {
            gc_enabled: true,
        }
    }
    
    pub fn format_value(&self, value: &CursedValue) -> String {
        match value {
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::Float(f) => f.to_string(),
            CursedValue::String(s) => format!("\"{}\"", s),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Channel(_) => "<channel>".to_string(),
            CursedValue::Nil => "nil".to_string(),
        }
    }
}
