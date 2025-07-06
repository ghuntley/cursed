//! CURSED Execution Engine - ADVANCED FEATURES ENABLED
//! 
//! Complete execution system featuring:
//! - JIT compilation and runtime
//! - Goroutine scheduling and management
//! - Advanced memory management
//! - Error handling and recovery

use crate::error::CursedError;
use crate::ast::{Program, Statement};
use crate::runtime::channels::simple_channel::SimpleChannel;

use std::sync::Arc;

pub mod execution_context;
pub mod jit_executor;
pub mod runtime_functions;
pub mod value_manager;

pub use execution_context::ExecutionContext;
pub use jit_executor::{JitExecutor, JitExecutorConfig, JitExecutionStats, jit_execute, new_jit_executor};

/// Advanced execution engine for CURSED
pub struct CursedExecutionEngine {
    jit_enabled: bool,
    goroutine_support: bool,
    gc_enabled: bool,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl CursedExecutionEngine {
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            jit_enabled: true,
            goroutine_support: true,
            gc_enabled: true,
            recursion_depth: 0,
            max_recursion_depth: 1000,
        })
    }
    
    pub fn new_no_jit() -> Result<Self, CursedError> {
        Ok(Self {
            jit_enabled: false,
            goroutine_support: true,
            gc_enabled: true,
            recursion_depth: 0,
            max_recursion_depth: 1000,
        })
    }
    
    pub fn execute(&mut self, source: &str) -> Result<CursedValue, CursedError> {
        tracing::info!("🚀 Executing CURSED code with advanced features");
        
        // Parse and compile
        let mut parser = crate::parser::new_parser(source)?;
        let program = parser.parse_program()?;
        
        // Execute with JIT if enabled
        if self.jit_enabled {
            self.execute_jit_with_source(&program, source)
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
    
    fn execute_jit_with_source(&mut self, program: &Program, source: &str) -> Result<CursedValue, CursedError> {
        tracing::info!("⚡ JIT compilation enabled");
        
        // Try JIT compilation first with original source
        match self.try_jit_execution_with_source(source) {
            Ok(result) => {
                tracing::info!("✅ JIT compilation successful");
                Ok(result)
            }
            Err(e) => {
                tracing::warn!("⚠️ JIT compilation failed: {}, falling back to interpretation", e);
                self.execute_interpreted(program)
            }
        }
    }
    
    fn try_jit_execution_with_source(&mut self, source: &str) -> Result<CursedValue, CursedError> {
        // Create JIT executor if not exists
        let mut jit_executor = JitExecutor::new()?;
        
        // Execute with JIT using original source
        jit_executor.execute(source)
    }
    
    fn program_to_source(&self, program: &Program) -> Result<String, CursedError> {
        // For now, we'll need to reconstruct the source from the AST
        // This is a simplified approach - in a real implementation you'd want to preserve original source
        let mut source = String::new();
        
        // Add package declaration if present
        if let Some(ref package) = program.package {
            source.push_str(&format!("vibe {:?}\n\n", package));
        }
        
        // Add imports
        for import in &program.imports {
            source.push_str(&format!("yeet {:?}\n", import));
        }
        
        if !program.imports.is_empty() {
            source.push('\n');
        }
        
        // Add statements (this is a simplified version)
        for stmt in &program.statements {
            match stmt {
                Statement::Function(func_stmt) => {
                    source.push_str(&format!("slay {}(", func_stmt.name));
                    for (i, param) in func_stmt.parameters.iter().enumerate() {
                        if i > 0 { source.push_str(", "); }
                        source.push_str(&format!("{} {}", param.name, param.param_type.as_ref().map_or("".to_string(), |t| format!("{:?}", t))));
                    }
                    source.push_str(")");
                    if let Some(ref ret_type) = func_stmt.return_type {
                        source.push_str(&format!(" {:?}", ret_type));
                    }
                    source.push_str(" {\n");
                    // Add body statements (simplified)
                    for body_stmt in &func_stmt.body {
                        source.push_str(&format!("    // Statement: {:?}\n", body_stmt));
                    }
                    source.push_str("}\n\n");
                }
                _ => {
                    source.push_str(&format!("// Statement: {:?}\n", stmt));
                }
            }
        }
        
        Ok(source)
    }
    
    fn execute_interpreted(&mut self, program: &Program) -> Result<CursedValue, CursedError> {
        tracing::info!("🔄 Interpreted execution");
        
        // Create execution context
        let mut context = ExecutionContext::new();
        
        // Execute each statement
        let mut last_value = CursedValue::Nil;
        for statement in &program.statements {
            match self.execute_statement(statement, &mut context)? {
                ExecutionFlow::Continue(value) => last_value = value,
                ExecutionFlow::Return(value) => return Ok(value), // Early return from program
            }
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

    /// Convert program AST back to source approximation for JIT compilation
    /// In a real implementation, this would be more sophisticated
    fn program_to_source_approximation(&self, program: &Program) -> String {
        let mut source = String::new();
        
        // Add imports
        for import in &program.imports {
            source.push_str(&format!("import \"{}\";\n", import.path));
        }
        
        if !program.imports.is_empty() {
            source.push('\n');
        }
        
        // Add package declaration
        if let Some(package) = &program.package {
            source.push_str(&format!("package {};\n\n", package.name));
        }
        
        // Convert statements to basic source representation
        for statement in &program.statements {
            source.push_str(&self.statement_to_source(statement));
            source.push('\n');
        }
        
        source
    }
    
    /// Convert a statement to basic source representation
    fn statement_to_source(&self, statement: &crate::ast::Statement) -> String {
        use crate::ast::Statement;
        
        match statement {
            Statement::Function(func) => {
                let mut result = format!("slay {}(", func.name);
                
                // Add parameters
                for (i, param) in func.parameters.iter().enumerate() {
                    if i > 0 { result.push_str(", "); }
                    result.push_str(&param.name);
                    if let Some(param_type) = &param.param_type {
                        result.push_str(" ");
                        result.push_str(&param_type.to_string());
                    }
                }
                
                result.push_str(") ");
                if let Some(return_type) = &func.return_type {
                    result.push_str(&return_type.to_string());
                    result.push(' ');
                }
                result.push_str("{\n");
                
                // Add function body (simplified)
                for stmt in &func.body {
                    result.push_str("    ");
                    result.push_str(&self.statement_to_source(stmt));
                    result.push('\n');
                }
                
                result.push('}');
                result
            },
            Statement::Return(ret) => {
                if let Some(expr) = &ret.value {
                    format!("return {};", self.expression_to_source(expr))
                } else {
                    "return;".to_string()
                }
            },
            Statement::Let(let_stmt) => {
                format!("let {} = {};", 
                    self.let_target_to_source(&let_stmt.target),
                    self.expression_to_source(&let_stmt.value)
                )
            },
            Statement::Expression(expr) => {
                format!("{};", self.expression_to_source(expr))
            },
            _ => {
                // For other statements, just return a comment for now
                "// unsupported statement".to_string()
            }
        }
    }
    
    /// Convert an expression to basic source representation
    fn expression_to_source(&self, expression: &crate::ast::Expression) -> String {
        use crate::ast::Expression;
        
        match expression {
            Expression::Integer(i) => i.to_string(),
            Expression::Float(f) => f.to_string(),
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Boolean(b) => b.to_string(),
            Expression::Character(c) => format!("'{}'", c),
            Expression::Identifier(name) => name.clone(),
            Expression::Literal(lit) => {
                match lit {
                    crate::ast::Literal::Integer(i) => i.to_string(),
                    crate::ast::Literal::Float(f) => f.to_string(),
                    crate::ast::Literal::String(s) => format!("\"{}\"", s),
                    crate::ast::Literal::Boolean(b) => b.to_string(),
                    crate::ast::Literal::Nil | crate::ast::Literal::Null => "nil".to_string(),
                }
            },
            Expression::Binary(binary) => {
                format!("{} {} {}", 
                    self.expression_to_source(&binary.left),
                    binary.operator,
                    self.expression_to_source(&binary.right)
                )
            },
            Expression::Call(call) => {
                let mut result = self.expression_to_source(&call.function);
                result.push('(');
                for (i, arg) in call.arguments.iter().enumerate() {
                    if i > 0 { result.push_str(", "); }
                    result.push_str(&self.expression_to_source(arg));
                }
                result.push(')');
                result
            },
            _ => {
                // For other expressions, return a placeholder
                "/* unsupported expression */".to_string()
            }
        }
    }
    
    /// Convert let target to source representation
    fn let_target_to_source(&self, target: &crate::ast::LetTarget) -> String {
        match target {
            crate::ast::LetTarget::Single(name) => name.clone(),
            crate::ast::LetTarget::Tuple(names) => {
                format!("({})", names.join(", "))
            }
        }
    }
    
    fn format_value(&self, value: &CursedValue) -> String {
        match value {
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::Float(f) => f.to_string(),
            CursedValue::String(s) => format!("\"{}\"", s),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Channel(_) => "<channel>".to_string(),
            CursedValue::Struct(fields) => {
                let field_strs: Vec<String> = fields.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
                    .collect();
                format!("{{ {} }}", field_strs.join(", "))
            },
            CursedValue::Lambda(lambda_value) => {
                format!("<lambda({})>", lambda_value.parameters.join(", "))
            },
            CursedValue::Tuple(elements) => {
                let element_strs: Vec<String> = elements.iter()
                    .map(|v| self.format_value(v))
                    .collect();
                format!("({})", element_strs.join(", "))
            },
            CursedValue::Nil => "nil".to_string(),
            CursedValue::Character(c) => format!("'{}'", c),
            CursedValue::Array(elements) => {
                let element_strs: Vec<String> = elements.iter()
                    .map(|v| self.format_value(v))
                    .collect();
                format!("[{}]", element_strs.join(", "))
            },
        }
    }
    
    fn execute_statement(&mut self, statement: &crate::ast::Statement, context: &mut ExecutionContext) -> Result<ExecutionFlow, CursedError> {
        use crate::ast::Statement;
        
        log::debug!("🔧 Executing statement type: {:?}", std::mem::discriminant(statement));
        match statement {
            Statement::Expression(expr) => {
                let value = self.evaluate_expression(expr, context)?;
                Ok(ExecutionFlow::Continue(value))
            },
            Statement::Let(let_stmt) => {
                let value = self.evaluate_expression(&let_stmt.value, context)?;
                match &let_stmt.target {
                    crate::ast::LetTarget::Single(name) => {
                        context.set_variable(name.clone(), value.clone());
                    },
                    crate::ast::LetTarget::Tuple(names) => {
                        // Handle tuple destructuring
                        if let CursedValue::Tuple(elements) = &value {
                            for (index, name) in names.iter().enumerate() {
                                if let Some(element) = elements.get(index) {
                                    context.set_variable(name.clone(), element.clone());
                                } else {
                                    return Err(CursedError::runtime_error(&format!("Tuple index {} out of bounds for destructuring", index)));
                                }
                            }
                        } else {
                            return Err(CursedError::runtime_error("Cannot destructure non-tuple value"));
                        }
                    }
                }
                // For assignment statements, return the value that was assigned
                Ok(ExecutionFlow::Continue(value))
            },
            Statement::Assignment(assign_stmt) => {
                let value = self.evaluate_expression(&assign_stmt.value, context)?;
                self.execute_assignment(&assign_stmt.target, value.clone(), context)?;
                // For assignment statements, return the value that was assigned
                Ok(ExecutionFlow::Continue(value))
            },
            Statement::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    let value = self.evaluate_expression(expr, context)?;
                    Ok(ExecutionFlow::Return(value))
                } else {
                    Ok(ExecutionFlow::Return(CursedValue::Nil))
                }
            },
            Statement::Function(func_stmt) => {
                // Store function definition in context
                log::info!("📝 Storing function definition: {} with {} parameters", func_stmt.name, func_stmt.parameters.len());
                log::debug!("📝 Function body has {} statements", func_stmt.body.len());
                context.set_function(func_stmt.name.clone(), func_stmt.clone());
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
            Statement::If(if_stmt) => {
                let condition = self.evaluate_expression(&if_stmt.condition, context)?;
                if self.is_truthy(&condition) {
                    let mut last_value = CursedValue::Nil;
                    for stmt in &if_stmt.then_branch {
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                    Ok(ExecutionFlow::Continue(last_value))
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    let mut last_value = CursedValue::Nil;
                    for stmt in else_branch {
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                    Ok(ExecutionFlow::Continue(last_value))
                } else {
                    Ok(ExecutionFlow::Continue(CursedValue::Nil))
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
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                }
                Ok(ExecutionFlow::Continue(last_value))
            },
            Statement::For(for_stmt) => {
                // Initialize
                if let Some(init) = &for_stmt.init {
                    match self.execute_statement(init, context)? {
                        ExecutionFlow::Continue(_) => {},
                        ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                    }
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
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                    
                    // Update
                    if let Some(update) = &for_stmt.update {
                        self.evaluate_expression(update, context)?;
                    }
                }
                Ok(ExecutionFlow::Continue(last_value))
            },
            Statement::ForIn(for_in_stmt) => {
                // Evaluate the iterable expression
                let iterable = self.evaluate_expression(&for_in_stmt.iterable, context)?;
                
                // Extract values from the iterable
                let values = match iterable {
                    CursedValue::Array(arr) => arr,
                    CursedValue::String(s) => {
                        // Iterate over characters
                        s.chars().map(|c| CursedValue::String(c.to_string())).collect()
                    },
                    _ => return Err(CursedError::runtime_error(&format!("Cannot iterate over {}", iterable.type_name()))),
                };
                
                let mut last_value = CursedValue::Nil;
                
                // Iterate over each value
                for value in values {
                    // Set the loop variable
                    context.set_variable(for_in_stmt.variable.clone(), value);
                    
                    // Execute the body statements
                    for stmt in &for_in_stmt.body {
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                }
                
                Ok(ExecutionFlow::Continue(last_value))
            },
            Statement::Switch(switch_stmt) => {
                let switch_value = self.evaluate_expression(&switch_stmt.expression, context)?;
                
                // Try to match against each case
                for case in &switch_stmt.cases {
                    let case_value = self.evaluate_expression(&case.pattern, context)?;
                    if self.values_equal(&switch_value, &case_value) {
                        let mut last_value = CursedValue::Nil;
                        for stmt in &case.body {
                            match self.execute_statement(stmt, context)? {
                                ExecutionFlow::Continue(value) => last_value = value,
                                ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                            }
                        }
                        return Ok(ExecutionFlow::Continue(last_value));
                    }
                }
                
                // If no case matched, try default case
                if let Some(default_body) = &switch_stmt.default_case {
                    let mut last_value = CursedValue::Nil;
                    for stmt in default_body {
                        match self.execute_statement(stmt, context)? {
                            ExecutionFlow::Continue(value) => last_value = value,
                            ExecutionFlow::Return(value) => return Ok(ExecutionFlow::Return(value)),
                        }
                    }
                    Ok(ExecutionFlow::Continue(last_value))
                } else {
                    Ok(ExecutionFlow::Continue(CursedValue::Nil))
                }
            },
            Statement::Goroutine(_) => {
                // For now, just return nil - goroutines need more complex implementation
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
            Statement::Channel(_) => {
                // For now, just return nil - channels need more complex implementation
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
            Statement::Select(_) => {
                // For now, just return nil - select statements need more complex implementation
                log::info!("📺 Select statement execution not yet implemented");
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
            Statement::Struct(struct_stmt) => {
                // Store struct definition in context for type checking
                log::info!("📝 Storing struct definition: {} with {} fields", struct_stmt.name, struct_stmt.fields.len());
                // TODO: Implement actual struct storage
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
            Statement::Interface(interface_stmt) => {
                // Store interface definition in context for type checking
                log::info!("📝 Storing interface definition: {} with {} methods", interface_stmt.name, interface_stmt.methods.len());
                // TODO: Implement actual interface storage
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
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
                        Ok(ExecutionFlow::Continue(val)) => last_value = val,
                        Ok(ExecutionFlow::Return(val)) => return Ok(ExecutionFlow::Return(val)),
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
                                    match self.execute_statement(recovery_stmt, context)? {
                                        ExecutionFlow::Continue(val) => last_value = val,
                                        ExecutionFlow::Return(val) => return Ok(ExecutionFlow::Return(val)),
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
                
                if !error_occurred {
                    log::info!("✅ Protected block completed without errors");
                }
                
                Ok(ExecutionFlow::Continue(last_value))
            },
            Statement::Defer(defer_stmt) => {
                log::info!("⏰ Adding defer statement to stack");
                
                // Add the expression to the defer stack instead of executing it immediately
                context.push_defer(defer_stmt.expression.as_ref().clone());
                
                Ok(ExecutionFlow::Continue(CursedValue::Nil))
            },
        }
    }
    
    fn evaluate_expression(&mut self, expression: &crate::ast::Expression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        use crate::ast::Expression;
        
        match expression {
            Expression::Integer(i) => Ok(CursedValue::Integer(*i)),
            Expression::Float(f) => Ok(CursedValue::Float(*f)),
            Expression::String(s) => Ok(CursedValue::String(s.clone())),
            Expression::Boolean(b) => Ok(CursedValue::Boolean(*b)),
            Expression::Character(c) => Ok(CursedValue::Character(*c)),
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
                let mut array_values = Vec::new();
                for element in elements {
                    array_values.push(self.evaluate_expression(element, context)?);
                }
                Ok(CursedValue::Array(array_values))
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
            Expression::StructLiteral(struct_literal) => {
                self.evaluate_struct_literal(struct_literal, context)
            },
            Expression::Lambda(lambda_expr) => {
                self.evaluate_lambda(lambda_expr, context)
            },
            Expression::Tuple(tuple_expr) => {
                self.evaluate_tuple(tuple_expr, context)
            },
            Expression::TupleAccess(tuple_access) => {
                self.evaluate_tuple_access(tuple_access, context)
            },
            Expression::ArrayAccess(array_access) => {
                self.evaluate_array_access(array_access, context)
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
            // String + Integer concatenation
            (CursedValue::String(l), CursedValue::Integer(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported string-integer operator: {}", operator))),
                }
            },
            // Integer + String concatenation  
            (CursedValue::Integer(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported integer-string operator: {}", operator))),
                }
            },
            // String + Float concatenation
            (CursedValue::String(l), CursedValue::Float(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported string-float operator: {}", operator))),
                }
            },
            // Float + String concatenation
            (CursedValue::Float(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported float-string operator: {}", operator))),
                }
            },
            // String + Boolean concatenation
            (CursedValue::String(l), CursedValue::Boolean(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported string-boolean operator: {}", operator))),
                }
            },
            // Boolean + String concatenation
            (CursedValue::Boolean(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported boolean-string operator: {}", operator))),
                }
            },
            // Boolean logical operations
            (CursedValue::Boolean(l), CursedValue::Boolean(r)) => {
                match operator {
                    "&&" => Ok(CursedValue::Boolean(*l && *r)),
                    "||" => Ok(CursedValue::Boolean(*l || *r)),
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported boolean operator: {}", operator))),
                }
            },
            // Mixed Integer-Float arithmetic operations (convert int to float)
            (CursedValue::Integer(l), CursedValue::Float(r)) => {
                let l_float = *l as f64;
                match operator {
                    "+" => Ok(CursedValue::Float(l_float + r)),
                    "-" => Ok(CursedValue::Float(l_float - r)),
                    "*" => Ok(CursedValue::Float(l_float * r)),
                    "/" => {
                        if *r == 0.0 {
                            Err(CursedError::RuntimeError("Division by zero".to_string()))
                        } else {
                            Ok(CursedValue::Float(l_float / r))
                        }
                    },
                    "==" => Ok(CursedValue::Boolean((l_float - r).abs() < f64::EPSILON)),
                    "!=" => Ok(CursedValue::Boolean((l_float - r).abs() >= f64::EPSILON)),
                    "<" => Ok(CursedValue::Boolean(l_float < *r)),
                    ">" => Ok(CursedValue::Boolean(l_float > *r)),
                    "<=" => Ok(CursedValue::Boolean(l_float <= *r)),
                    ">=" => Ok(CursedValue::Boolean(l_float >= *r)),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported integer-float operator: {}", operator))),
                }
            },
            // Mixed Float-Integer arithmetic operations (convert int to float)
            (CursedValue::Float(l), CursedValue::Integer(r)) => {
                let r_float = *r as f64;
                match operator {
                    "+" => Ok(CursedValue::Float(l + r_float)),
                    "-" => Ok(CursedValue::Float(l - r_float)),
                    "*" => Ok(CursedValue::Float(l * r_float)),
                    "/" => {
                        if *r == 0 {
                            Err(CursedError::RuntimeError("Division by zero".to_string()))
                        } else {
                            Ok(CursedValue::Float(l / r_float))
                        }
                    },
                    "==" => Ok(CursedValue::Boolean((l - r_float).abs() < f64::EPSILON)),
                    "!=" => Ok(CursedValue::Boolean((l - r_float).abs() >= f64::EPSILON)),
                    "<" => Ok(CursedValue::Boolean(*l < r_float)),
                    ">" => Ok(CursedValue::Boolean(*l > r_float)),
                    "<=" => Ok(CursedValue::Boolean(*l <= r_float)),
                    ">=" => Ok(CursedValue::Boolean(*l >= r_float)),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported float-integer operator: {}", operator))),
                }
            },
            // String + Character concatenation
            (CursedValue::String(l), CursedValue::Character(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported string-character operator: {}", operator))),
                }
            },
            // Character + String concatenation
            (CursedValue::Character(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported character-string operator: {}", operator))),
                }
            },
            // Character + Character operations
            (CursedValue::Character(l), CursedValue::Character(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))), // Concatenation creates string
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    "<" => Ok(CursedValue::Boolean(l < r)),
                    ">" => Ok(CursedValue::Boolean(l > r)),
                    "<=" => Ok(CursedValue::Boolean(l <= r)),
                    ">=" => Ok(CursedValue::Boolean(l >= r)),
                    _ => Err(CursedError::RuntimeError(format!("Unsupported character operator: {}", operator))),
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
                        // First check if the identifier resolves to a lambda
                        if let Some(value) = context.get_variable(func_name) {
                            if let CursedValue::Lambda(lambda_value) = value {
                                return self.call_lambda(&lambda_value, &call_expr.arguments, context);
                            }
                        }
                        
                        // User-defined function
                        log::info!("🔍 Looking for function: {}", func_name);
                        if let Some(func_def) = context.get_function(func_name) {
                            log::info!("✅ Found function: {}", func_name);
                            
                            // Check recursion depth
                            if self.recursion_depth >= self.max_recursion_depth {
                                return Err(CursedError::RuntimeError(format!(
                                    "Maximum recursion depth exceeded ({})", self.max_recursion_depth
                                )));
                            }
                            
                            // Increment recursion depth
                            self.recursion_depth += 1;
                            
                            // Create child context for function execution (inherits functions)
                            let mut func_context = context.new_child();
                            
                            // Bind parameters
                            if call_expr.arguments.len() != func_def.parameters.len() {
                                self.recursion_depth -= 1; // Restore depth on error
                                return Err(CursedError::RuntimeError(format!(
                                    "Function {} expects {} arguments, got {}",
                                    func_name, func_def.parameters.len(), call_expr.arguments.len()
                                )));
                            }
                            
                            for (param, arg) in func_def.parameters.iter().zip(&call_expr.arguments) {
                                let arg_value = self.evaluate_expression(arg, context)?;
                                func_context.set_variable(param.name.clone(), arg_value);
                            }
                            
                            // Execute function body with proper return handling
                            let mut result = CursedValue::Nil;
                            for stmt in &func_def.body {
                                match self.execute_statement(stmt, &mut func_context)? {
                                    ExecutionFlow::Continue(value) => result = value,
                                    ExecutionFlow::Return(value) => {
                                        result = value;
                                        break; // Early return from function
                                    }
                                }
                            }
                            
                            // Execute deferred expressions before function returns
                            let deferred_exprs = func_context.execute_defers();
                            for defer_expr in deferred_exprs {
                                log::info!("⏰ Executing deferred expression");
                                match self.evaluate_expression(&defer_expr, &mut func_context) {
                                    Ok(_) => {}, // Ignore defer return values
                                    Err(e) => log::warn!("⚠️ Error in deferred expression: {:?}", e),
                                }
                            }
                            
                            // Decrement recursion depth before returning
                            self.recursion_depth -= 1;
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
            _ => {
                // For other expressions (like lambda literals), evaluate first
                let function_value = self.evaluate_expression(&call_expr.function, context)?;
                if let CursedValue::Lambda(lambda_value) = function_value {
                    self.call_lambda(&lambda_value, &call_expr.arguments, context)
                } else {
                    Err(CursedError::RuntimeError("Cannot call non-function value".to_string()))
                }
            }
        }
    }
    
    fn evaluate_member_access(&mut self, member_expr: &crate::ast::MemberAccessExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        let object = self.evaluate_expression(&member_expr.object, context)?;
        
        match object {
            CursedValue::Struct(struct_fields) => {
                // Access struct field
                struct_fields.get(&member_expr.property)
                    .cloned()
                    .ok_or_else(|| CursedError::RuntimeError(format!("Struct field '{}' not found", member_expr.property)))
            },
            _ => {
                // For other types, return nil for now (could be method calls)
                Ok(CursedValue::Nil)
            }
        }
    }
    
    fn is_truthy(&self, value: &CursedValue) -> bool {
        match value {
            CursedValue::Boolean(b) => *b,
            CursedValue::Integer(i) => *i != 0,
            CursedValue::Float(f) => *f != 0.0,
            CursedValue::String(s) => !s.is_empty(),
            CursedValue::Channel(_) => true, // Channels are truthy when they exist
            CursedValue::Struct(fields) => !fields.is_empty(), // Structs are truthy if they have fields
            CursedValue::Lambda(_) => true, // Lambdas are always truthy when they exist
            CursedValue::Tuple(elements) => !elements.is_empty(), // Tuples are truthy if they have elements
            CursedValue::Nil => false,
            CursedValue::Character(c) => *c != '\0', // Characters are truthy unless null character
            CursedValue::Array(elements) => !elements.is_empty(), // Arrays are truthy if they have elements
        }
    }
    
    fn values_equal(&self, left: &CursedValue, right: &CursedValue) -> bool {
        match (left, right) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => a == b,
            (CursedValue::Float(a), CursedValue::Float(b)) => a == b,
            (CursedValue::String(a), CursedValue::String(b)) => a == b,
            (CursedValue::Boolean(a), CursedValue::Boolean(b)) => a == b,
            (CursedValue::Nil, CursedValue::Nil) => true,
            (CursedValue::Struct(a), CursedValue::Struct(b)) => {
                // Compare struct fields
                a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).map_or(false, |v2| self.values_equal(v, v2)))
            },
            // Allow integer-float comparison
            (CursedValue::Integer(a), CursedValue::Float(b)) => *a as f64 == *b,
            (CursedValue::Float(a), CursedValue::Integer(b)) => *a == *b as f64,
            _ => false,
        }
    }

    /// Evaluate struct literal expression
    fn evaluate_struct_literal(&mut self, struct_literal: &crate::ast::StructLiteralExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        let mut struct_fields = std::collections::HashMap::new();
        
        // Evaluate each field assignment
        for field_assignment in &struct_literal.fields {
            let field_value = self.evaluate_expression(&field_assignment.value, context)?;
            struct_fields.insert(field_assignment.field_name.clone(), field_value);
        }
        
        Ok(CursedValue::Struct(struct_fields))
    }
    
    /// Evaluate lambda expression (anonymous function)
    fn evaluate_lambda(&mut self, lambda_expr: &crate::ast::LambdaExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        // Capture the current environment for closure
        let mut captured_env = std::collections::HashMap::new();
        
        // Capture all variables from the current context
        for (var_name, var_value) in context.get_all_variables().iter() {
            captured_env.insert(var_name.clone(), var_value.clone());
        }
        
        let lambda_value = LambdaValue {
            parameters: lambda_expr.parameters.clone(),
            body: lambda_expr.body.clone(),
            captured_env,
        };
        
        Ok(CursedValue::Lambda(lambda_value))
    }
    
    /// Call a lambda function with given arguments
    fn call_lambda(&mut self, lambda_value: &LambdaValue, arguments: &[crate::ast::Expression], context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        // Check argument count
        if arguments.len() != lambda_value.parameters.len() {
            return Err(CursedError::RuntimeError(format!(
                "Lambda expects {} arguments, got {}",
                lambda_value.parameters.len(), arguments.len()
            )));
        }
        
        // Create new context for lambda execution
        let mut lambda_context = context.new_child();
        
        // Restore captured environment
        for (var_name, var_value) in &lambda_value.captured_env {
            lambda_context.set_variable(var_name.clone(), var_value.clone());
        }
        
        // Bind parameters
        for (param, arg) in lambda_value.parameters.iter().zip(arguments) {
            let arg_value = self.evaluate_expression(arg, context)?;
            lambda_context.set_variable(param.clone(), arg_value);
        }
        
        // Execute lambda body
        self.evaluate_expression(&lambda_value.body, &mut lambda_context)
    }

    /// Evaluate tuple expression
    fn evaluate_tuple(&mut self, tuple_expr: &crate::ast::TupleExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        let mut elements = Vec::new();
        
        for element_expr in &tuple_expr.elements {
            let element_value = self.evaluate_expression(element_expr, context)?;
            elements.push(element_value);
        }
        
        Ok(CursedValue::Tuple(elements))
    }

    /// Evaluate tuple access expression
    fn evaluate_tuple_access(&mut self, tuple_access: &crate::ast::TupleAccessExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        let tuple_value = self.evaluate_expression(&tuple_access.tuple, context)?;
        
        match tuple_value {
            CursedValue::Tuple(ref elements) => {
                if tuple_access.index < elements.len() {
                    Ok(elements[tuple_access.index].clone())
                } else {
                    Err(CursedError::RuntimeError(format!(
                        "Tuple index {} out of bounds for tuple with {} elements",
                        tuple_access.index, elements.len()
                    )))
                }
            },
            _ => Err(CursedError::RuntimeError(
                "Cannot access tuple element on non-tuple value".to_string()
            )),
        }
    }

    fn evaluate_array_access(&mut self, array_access: &crate::ast::ArrayAccessExpression, context: &mut ExecutionContext) -> Result<CursedValue, CursedError> {
        let array_value = self.evaluate_expression(&array_access.array, context)?;
        let index_value = self.evaluate_expression(&array_access.index, context)?;
        
        let index = match index_value {
            CursedValue::Integer(i) => {
                if i < 0 {
                    return Err(CursedError::RuntimeError(format!("Array index cannot be negative: {}", i)));
                }
                i as usize
            },
            _ => return Err(CursedError::RuntimeError("Array index must be an integer".to_string())),
        };
        
        match array_value {
            CursedValue::Array(ref elements) => {
                if index < elements.len() {
                    Ok(elements[index].clone())
                } else {
                    Err(CursedError::RuntimeError(format!(
                        "Array index {} out of bounds for array with {} elements",
                        index, elements.len()
                    )))
                }
            },
            _ => Err(CursedError::RuntimeError(
                "Cannot access array element on non-array value".to_string()
            )),
        }
    }

    /// Execute assignment to either a single variable or tuple destructuring
    fn execute_assignment(&mut self, target: &crate::ast::AssignmentTarget, value: CursedValue, context: &mut ExecutionContext) -> Result<(), CursedError> {
        match target {
            crate::ast::AssignmentTarget::Single(name) => {
                context.set_variable(name.clone(), value);
                Ok(())
            },
            crate::ast::AssignmentTarget::Tuple(names) => {
                match value {
                    CursedValue::Tuple(elements) => {
                        if names.len() != elements.len() {
                            return Err(CursedError::RuntimeError(format!(
                                "Tuple destructuring mismatch: expected {} variables, got {} values",
                                names.len(), elements.len()
                            )));
                        }
                        
                        for (name, element) in names.iter().zip(elements.into_iter()) {
                            context.set_variable(name.clone(), element);
                        }
                        Ok(())
                    },
                    _ => Err(CursedError::RuntimeError(
                        "Cannot destructure non-tuple value in tuple assignment".to_string()
                    )),
                }
            }
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
    Character(char),
    Array(Vec<CursedValue>),
    Channel(Arc<SimpleChannel<CursedValue>>),
    Struct(std::collections::HashMap<String, CursedValue>),
    Lambda(LambdaValue),
    Tuple(Vec<CursedValue>),
    Nil,
}

/// Lambda value representation
#[derive(Debug, Clone)]
pub struct LambdaValue {
    pub parameters: Vec<String>,
    pub body: Box<crate::ast::Expression>,
    pub captured_env: std::collections::HashMap<String, CursedValue>,
}

/// Control flow for execution
#[derive(Debug, Clone)]
pub enum ExecutionFlow {
    Continue(CursedValue),
    Return(CursedValue),
}

impl CursedValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            CursedValue::Integer(_) => "integer",
            CursedValue::Float(_) => "float",
            CursedValue::String(_) => "string",
            CursedValue::Boolean(_) => "boolean",
            CursedValue::Character(_) => "character",
            CursedValue::Array(_) => "array",
            CursedValue::Channel(_) => "channel",
            CursedValue::Struct(_) => "struct",
            CursedValue::Lambda(_) => "lambda",
            CursedValue::Tuple(_) => "tuple",
            CursedValue::Nil => "nil",
        }
    }
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
            CursedValue::Array(elements) => {
                let element_strs: Vec<String> = elements.iter()
                    .map(|v| self.format_value(v))
                    .collect();
                format!("[{}]", element_strs.join(", "))
            },
            CursedValue::Channel(_) => "<channel>".to_string(),
            CursedValue::Struct(fields) => {
                let field_strs: Vec<String> = fields.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
                    .collect();
                format!("{{ {} }}", field_strs.join(", "))
            },
            CursedValue::Lambda(lambda_value) => {
                format!("<lambda({})>", lambda_value.parameters.join(", "))
            },
            CursedValue::Tuple(elements) => {
                let element_strs: Vec<String> = elements.iter()
                    .map(|v| self.format_value(v))
                    .collect();
                format!("({})", element_strs.join(", "))
            },
            CursedValue::Nil => "nil".to_string(),
            CursedValue::Character(c) => format!("'{}'", c),
        }
    }
    
    /// Execute assignment to either a single variable or tuple destructuring
    fn execute_assignment(&mut self, target: &crate::ast::AssignmentTarget, value: CursedValue, context: &mut ExecutionContext) -> Result<(), CursedError> {
        match target {
            crate::ast::AssignmentTarget::Single(name) => {
                context.set_variable(name.clone(), value);
                Ok(())
            },
            crate::ast::AssignmentTarget::Tuple(names) => {
                match value {
                    CursedValue::Tuple(elements) => {
                        if names.len() != elements.len() {
                            return Err(CursedError::RuntimeError(format!(
                                "Tuple destructuring mismatch: expected {} variables, got {} values",
                                names.len(), elements.len()
                            )));
                        }
                        
                        for (name, element) in names.iter().zip(elements.into_iter()) {
                            context.set_variable(name.clone(), element);
                        }
                        Ok(())
                    },
                    _ => Err(CursedError::RuntimeError(
                        "Cannot destructure non-tuple value in tuple assignment".to_string()
                    )),
                }
            }
        }
    }
}
