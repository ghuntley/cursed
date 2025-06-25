/// LLVM Channel Integration for CURSED Programming Language
/// 
/// This module provides comprehensive LLVM code generation for CURSED's channel
/// system, enabling compilation of channel operations like `ch <- value` and
/// `value := <-ch` with proper integration to the runtime channel system.
/// 
/// Implements:
/// - Channel type compilation for `dm<T>` generic channels
/// - Send operation compilation for `ch <- value` syntax
/// - Receive operation compilation for `<-ch` syntax
/// - Channel creation and management lifecycle
/// - Integration with CURSED's error propagation and type system

use crate::ast::traits::{Expression, Node};
use crate::ast::expressions::{Literal, LiteralValue};
use crate::ast::operators::{BinaryExpression, UnaryExpression, AssignmentExpression};
use crate::ast::identifiers::Identifier;
use crate::ast::types::{TypeExpression, Type};
use crate::codegen::llvm::expression_compiler::{LlvmValue, LlvmType, ExpressionContext};
use crate::codegen::llvm::type_system::LlvmTypeRegistry;
use crate::error::{Error, CursedError};
use crate::debug::SourceLocation;
use crate::runtime::channels::{ChannelError, SendResult, ReceiveResult};

use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

/// LLVM channel operations compiler
#[derive(Debug)]
pub struct LlvmChannelCompiler<'ctx> {
    /// Type registry for channel type management
    type_registry: LlvmTypeRegistry,
    /// Expression context for variable and type management
    context: ExpressionContext<'ctx>,
    /// Generated LLVM IR output
    ir_output: Vec<String>,
    /// Channel type instances cache
    channel_types: HashMap<String, CompiledChannelType>,
    /// Runtime function declarations
    runtime_functions: HashMap<String, RuntimeFunction>,
}

/// Compiled channel type information
#[derive(Debug, Clone)]
pub struct CompiledChannelType {
    /// Element type of the channel (T in dm<T>)
    element_type: LlvmType,
    /// LLVM struct type for channel handle
    handle_type: String,
    /// Runtime type identifier
    type_id: u64,
    /// Size of channel buffer (if buffered)
    buffer_size: Option<usize>,
}

/// Runtime function metadata for LLVM integration
#[derive(Debug, Clone)]
pub struct RuntimeFunction {
    /// Function name in LLVM IR
    llvm_name: String,
    /// Return type
    return_type: LlvmType,
    /// Parameter types
    param_types: Vec<LlvmType>,
    /// Whether function can throw errors
    can_error: bool,
}

/// Channel operation compilation results
#[derive(Debug, Clone)]
pub struct ChannelOperation {
    /// Generated LLVM value
    result_value: LlvmValue,
    /// Generated IR instructions
    instructions: Vec<String>,
    /// Error handling code (if applicable)
    error_handling: Option<Vec<String>>,
}

impl<'ctx> LlvmChannelCompiler<'ctx> {
    /// Create new channel compiler with type registry
    #[instrument]
    pub fn new(type_registry: LlvmTypeRegistry, context: ExpressionContext<'ctx>) -> Self {
        let mut compiler = Self {
            type_registry,
            context,
            ir_output: Vec::new(),
            channel_types: HashMap::new(),
            runtime_functions: HashMap::new(),
        };
        
        compiler.initialize_runtime_functions();
        compiler
    }

    /// Initialize runtime function declarations for channel operations
    #[instrument(skip(self))]
    fn initialize_runtime_functions(&mut self) {
        debug!("Initializing runtime function declarations");
        
        // Channel creation function
        self.runtime_functions.insert("create_channel".to_string(), RuntimeFunction {
            llvm_name: "@cursed_channel_create".to_string(),
            return_type: LlvmType::Pointer(Box::new(LlvmType::Int32)), // Channel handle
            param_types: vec![
                LlvmType::Int32, // element_size
                LlvmType::Int32, // buffer_size
                LlvmType::Int64, // type_id
            ],
            can_error: true,
        });

        // Channel send function
        self.runtime_functions.insert("send".to_string(), RuntimeFunction {
            llvm_name: "@cursed_channel_send".to_string(),
            return_type: LlvmType::Int32, // SendResult enum
            param_types: vec![
                LlvmType::Pointer(Box::new(LlvmType::Int32)), // channel_handle
                LlvmType::Pointer(Box::new(LlvmType::Int32)), // value_ptr
                LlvmType::Boolean, // blocking
            ],
            can_error: true,
        });

        // Channel receive function
        self.runtime_functions.insert("receive".to_string(), RuntimeFunction {
            llvm_name: "@cursed_channel_receive".to_string(),
            return_type: LlvmType::Int32, // ReceiveResult enum
            param_types: vec![
                LlvmType::Pointer(Box::new(LlvmType::Int32)), // channel_handle
                LlvmType::Pointer(Box::new(LlvmType::Int32)), // output_ptr
                LlvmType::Boolean, // blocking
            ],
            can_error: true,
        });

        // Channel close function
        self.runtime_functions.insert("close".to_string(), RuntimeFunction {
            llvm_name: "@cursed_channel_close".to_string(),
            return_type: LlvmType::Int32, // Result code
            param_types: vec![
                LlvmType::Pointer(Box::new(LlvmType::Int32)), // channel_handle
            ],
            can_error: true,
        });

        info!("Initialized {} runtime functions", self.runtime_functions.len());
    }

    /// Compile channel type for generic `dm<T>` declaration
    #[instrument(skip(self), fields(element_type = ?element_type))]
    pub fn compile_channel_type(&mut self, element_type: &LlvmType, buffer_size: Option<usize>) -> Result<(), Error> {
        debug!("Compiling channel type for element type: {:?}", element_type);
        
        let type_key = format!("dm<{}>", element_type.to_llvm_string());
        
        // Check if already compiled
        if let Some(compiled) = self.channel_types.get(&type_key) {
            debug!("Using cached channel type: {}", type_key);
            return Ok(compiled.clone());
        }

        // Generate type ID for runtime identification
        let type_id = self.generate_type_id(&type_key);
        
        // Create LLVM struct type for channel handle
        let handle_type = self.get_channel_struct_type(element_type)?;
        
        let compiled_type = CompiledChannelType {
            element_type: element_type.clone(),
            handle_type,
            type_id,
            buffer_size,
        };

        self.channel_types.insert(type_key.clone(), compiled_type.clone());
        
        info!("Compiled channel type: {} with type_id: {}", type_key, type_id);
        Ok(compiled_type)
    }

    /// Generate LLVM struct layout for channel handles
    #[instrument(skip(self), fields(element_type = ?element_type))]
    fn get_channel_struct_type(&mut self, element_type: &LlvmType) -> Result<(), Error> {
        let element_llvm = element_type.to_llvm_string();
        let struct_name = format!("%channel_{}", element_llvm.replace("*", "_ptr"));
        
        // Generate channel handle struct
        let struct_def = format!(
            "%{} = type {{ i8*, i64, i32, i32 }}",
            struct_name.trim_start_matches('%')
        );
        
        // Add type definition to IR output
        self.ir_output.push(struct_def);
        
        // Add runtime metadata for channel operations
        let metadata = format!(
            "!{}_metadata = !{{ !\"element_type\", !\"{}\", !\"struct_name\", !\"{}\" }}",
            struct_name.trim_start_matches('%').replace(".", "_"),
            element_llvm,
            struct_name
        );
        self.ir_output.push(metadata);
        
        debug!("Generated channel struct type: {}", struct_name);
        Ok(struct_name)
    }

    /// Compile channel creation expression (e.g., `make(dm<int>, 10)`)
    #[instrument(skip(self), fields(element_type = ?element_type, buffer_size = ?buffer_size))]
    pub fn compile_channel_creation(&mut self, element_type: &LlvmType, buffer_size: Option<usize>) -> Result<(), Error> {
        debug!("Compiling channel creation for type: {:?}", element_type);
        
        let channel_type = self.compile_channel_type(element_type, buffer_size)?;
        let create_fn = self.runtime_functions.get("create_channel").unwrap();
        
        let result_temp = self.context.next_temp();
        let element_size = self.get_type_size(element_type);
        let buffer_size_val = buffer_size.unwrap_or(0);
        
        let call_instruction = format!(
            "{} = call {} {}(i32 {}, i32 {}, i64 {})",
            result_temp,
            create_fn.return_type.to_llvm_string(),
            create_fn.llvm_name,
            element_size,
            buffer_size_val,
            channel_type.type_id
        );

        // Error handling for channel creation
        let error_check = vec![
            format!("{}_is_null = icmp eq {}* {}, null", result_temp, 
                   create_fn.return_type.to_llvm_string().trim_end_matches('*'), result_temp),
            format!("br i1 {}_is_null, label %channel_create_error, label %channel_create_success", result_temp),
        ];

        let operation = ChannelOperation {
            result_value: LlvmValue {
                value_type: create_fn.return_type.clone(),
                llvm_name: result_temp.clone(),
                is_constant: false,
            },
            instructions: vec![call_instruction],
            error_handling: Some(error_check),
        };

        info!("Compiled channel creation operation");
        Ok(operation)
    }

    /// Compile send operation for `ch <- value` syntax
    #[instrument(skip(self), fields(channel_expr = ?channel_expr, value_expr = ?value_expr))]
    pub fn compile_send_operation(&mut self, channel_expr: &dyn Expression, value_expr: &dyn Expression, blocking: bool) -> Result<(), Error> {
        debug!("Compiling send operation, blocking: {}", blocking);
        
        // Compile channel expression to get channel handle
        let channel_value = self.compile_channel_expression(channel_expr)?;
        
        // Compile value expression
        let value_result = self.compile_value_expression(value_expr)?;
        
        // Get send function metadata
        let send_fn = self.runtime_functions.get("send").unwrap();
        
        let result_temp = self.context.next_temp();
        let value_ptr_temp = self.context.next_temp();
        
        // Allocate stack space for value
        let mut instructions = vec![
            format!("{} = alloca {}", value_ptr_temp, value_result.value_type.to_llvm_string()),
            format!("store {} {}, {}* {}", 
                   value_result.value_type.to_llvm_string(), 
                   value_result.llvm_name,
                   value_result.value_type.to_llvm_string(),
                   value_ptr_temp),
        ];

        // Call runtime send function
        let send_call = format!(
            "{} = call {} {}({}* {}, i8* {}, i1 {})",
            result_temp,
            send_fn.return_type.to_llvm_string(),
            send_fn.llvm_name,
            channel_value.value_type.to_llvm_string().trim_end_matches('*'),
            channel_value.llvm_name,
            value_ptr_temp,
            if blocking { "true" } else { "false" }
        );
        instructions.push(send_call);

        // Error handling for send operation
        let error_handling = self.generate_send_error_handling(&result_temp)?;

        let operation = ChannelOperation {
            result_value: LlvmValue {
                value_type: send_fn.return_type.clone(),
                llvm_name: result_temp,
                is_constant: false,
            },
            instructions,
            error_handling: Some(error_handling),
        };

        info!("Compiled send operation");
        Ok(operation)
    }

    /// Compile receive operation for `<-ch` syntax
    #[instrument(skip(self), fields(channel_expr = ?channel_expr))]
    pub fn compile_receive_operation(&mut self, channel_expr: &dyn Expression, blocking: bool) -> Result<(), Error> {
        debug!("Compiling receive operation, blocking: {}", blocking);
        
        // Compile channel expression to get channel handle
        let channel_value = self.compile_channel_expression(channel_expr)?;
        
        // Determine element type from channel
        let element_type = self.extract_channel_element_type(&channel_value)?;
        
        // Get receive function metadata
        let receive_fn = self.runtime_functions.get("receive").unwrap();
        
        let result_temp = self.context.next_temp();
        let output_ptr_temp = self.context.next_temp();
        let receive_result_temp = self.context.next_temp();
        
        // Allocate stack space for received value
        let mut instructions = vec![
            format!("{} = alloca {}", output_ptr_temp, element_type.to_llvm_string()),
        ];

        // Call runtime receive function
        let receive_call = format!(
            "{} = call {} {}({}* {}, i8* {}, i1 {})",
            receive_result_temp,
            receive_fn.return_type.to_llvm_string(),
            receive_fn.llvm_name,
            channel_value.value_type.to_llvm_string().trim_end_matches('*'),
            channel_value.llvm_name,
            output_ptr_temp,
            if blocking { "true" } else { "false" }
        );
        instructions.push(receive_call);

        // Load received value
        let load_value = format!(
            "{} = load {}, {}* {}",
            result_temp,
            element_type.to_llvm_string(),
            element_type.to_llvm_string(),
            output_ptr_temp
        );
        instructions.push(load_value);

        // Error handling for receive operation
        let error_handling = self.generate_receive_error_handling(&receive_result_temp)?;

        let operation = ChannelOperation {
            result_value: LlvmValue {
                value_type: element_type,
                llvm_name: result_temp,
                is_constant: false,
            },
            instructions,
            error_handling: Some(error_handling),
        };

        info!("Compiled receive operation");
        Ok(operation)
    }

    /// Compile channel close operation
    #[instrument(skip(self), fields(channel_expr = ?channel_expr))]
    pub fn compile_channel_close(&mut self, channel_expr: &dyn Expression) -> Result<(), Error> {
        debug!("Compiling channel close operation");
        
        // Compile channel expression to get channel handle
        let channel_value = self.compile_channel_expression(channel_expr)?;
        
        // Get close function metadata
        let close_fn = self.runtime_functions.get("close").unwrap();
        
        let result_temp = self.context.next_temp();
        
        // Call runtime close function
        let close_call = format!(
            "{} = call {} {}({}* {})",
            result_temp,
            close_fn.return_type.to_llvm_string(),
            close_fn.llvm_name,
            channel_value.value_type.to_llvm_string().trim_end_matches('*'),
            channel_value.llvm_name
        );

        // Error handling for close operation
        let error_handling = vec![
            format!("{}_success = icmp eq i32 {}, 0", result_temp, result_temp),
            format!("br i1 {}_success, label %close_success, label %close_error", result_temp),
        ];

        let operation = ChannelOperation {
            result_value: LlvmValue {
                value_type: close_fn.return_type.clone(),
                llvm_name: result_temp,
                is_constant: false,
            },
            instructions: vec![close_call],
            error_handling: Some(error_handling),
        };

        info!("Compiled channel close operation");
        Ok(operation)
    }

    /// Helper: Compile channel expression to get channel handle
    #[instrument(skip(self))]
    fn compile_channel_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // This would integrate with the main expression compiler
        // For now, simplified implementation
        if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            if let Some(value) = self.context.get_variable(&identifier.value) {
                Ok(value.clone())
            } else {
                Err(Error::Runtime(format!("Unknown channel variable: {}", identifier.value)))
            }
        } else {
            Err(Error::Runtime("Invalid channel expression".to_string()))
        }
    }

    /// Helper: Compile value expression for sending
    #[instrument(skip(self))]
    fn compile_value_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // This would integrate with the main expression compiler
        // For now, simplified implementation for literals
        if let Some(literal) = expr.as_any().downcast_ref::<Literal>() {
            match &literal.value {
                LiteralValue::Integer(i) => Ok(LlvmValue {
                    value_type: LlvmType::Int32,
                    llvm_name: i.to_string(),
                    is_constant: true,
                }),
                LiteralValue::String(s) => Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: format!("\"{}\"", s),
                    is_constant: true,
                }),
                _ => Err(Error::Runtime("Unsupported literal type".to_string())),
            }
        } else {
            Err(Error::Runtime("Complex expressions not yet supported".to_string()))
        }
    }

    /// Helper: Extract element type from channel value
    #[instrument(skip(self))]
    fn extract_channel_element_type(&self, channel_value: &LlvmValue) -> Result<(), Error> {
        // Parse channel type to extract element type
        // This is simplified - real implementation would use type registry
        match &channel_value.value_type {
            LlvmType::Pointer(_) => Ok(LlvmType::Int32), // Default for now
            _ => Err(Error::Runtime("Invalid channel type".to_string())),
        }
    }

    /// Helper: Generate error handling for send operations
    #[instrument(skip(self))]
    fn generate_send_error_handling(&self, result_temp: &str) -> Result<(), Error> {
        Ok(vec![
            format!("{}_is_error = icmp ne i32 {}, 0", result_temp, result_temp),
            format!("br i1 {}_is_error, label %send_error, label %send_success", result_temp),
            // Error block would handle SendResult enum cases
            "send_error:".to_string(),
            format!("  ; Handle send error code in {}", result_temp),
            "  br label %send_success".to_string(),
            "send_success:".to_string(),
        ])
    }

    /// Helper: Generate error handling for receive operations
    #[instrument(skip(self))]
    fn generate_receive_error_handling(&self, result_temp: &str) -> Result<(), Error> {
        Ok(vec![
            format!("{}_is_error = icmp ne i32 {}, 0", result_temp, result_temp),
            format!("br i1 {}_is_error, label %receive_error, label %receive_success", result_temp),
            // Error block would handle ReceiveResult enum cases
            "receive_error:".to_string(),
            format!("  ; Handle receive error code in {}", result_temp),
            "  br label %receive_success".to_string(),
            "receive_success:".to_string(),
        ])
    }

    /// Helper: Generate type ID for runtime type identification
    #[instrument(skip(self))]
    fn generate_type_id(&self, type_name: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        type_name.hash(&mut hasher);
        hasher.finish()
    }

    /// Helper: Get size of LLVM type in bytes
    #[instrument(skip(self))]
    fn get_type_size(&self, llvm_type: &LlvmType) -> usize {
        match llvm_type {
            LlvmType::Int32 => 4,
            LlvmType::Int64 => 8,
            LlvmType::Float64 => 8,
            LlvmType::Boolean => 1,
            LlvmType::String => 8, // Pointer size
            LlvmType::Pointer(_) => 8,
            LlvmType::Void => 0,
            LlvmType::Function { .. } => 8,
        }
    }

    /// Generate final LLVM IR output
    #[instrument(skip(self))]
    pub fn generate_ir(&self) -> String {
        let mut output = Vec::new();
        
        // Add runtime function declarations
        output.push("; Runtime function declarations for channel operations".to_string());
        for func in self.runtime_functions.values() {
            let params: Vec<String> = func.param_types.iter()
                .map(|t| t.to_llvm_string())
                .collect();
            output.push(format!(
                "declare {} {}({})",
                func.return_type.to_llvm_string(),
                func.llvm_name,
                params.join(", ")
            ));
        }
        
        output.push("".to_string());
        
        // Add generated IR
        output.extend(self.ir_output.iter().cloned());
        
        output.join("\n")
    }
}

/// Channel operation trait for AST integration
pub trait ChannelExpressionCompiler {
    /// Compile send expression (`ch <- value`)
    fn compile_send_expression(&mut self, channel: &dyn Expression, value: &dyn Expression) -> Result<(), Error>;
    
    /// Compile receive expression (`<-ch`)
    fn compile_receive_expression(&mut self, channel: &dyn Expression) -> Result<(), Error>;
    
    /// Compile channel creation expression
    fn compile_channel_creation_expression(&mut self, element_type: &LlvmType, buffer_size: Option<usize>) -> Result<(), Error>;
}

impl<'ctx> ChannelExpressionCompiler for LlvmChannelCompiler<'ctx> {
    #[instrument(skip(self))]
    fn compile_send_expression(&mut self, channel: &dyn Expression, value: &dyn Expression) -> Result<(), Error> {
        let operation = self.compile_send_operation(channel, value, true)?;
        
        // Add instructions to IR output
        self.ir_output.extend(operation.instructions);
        if let Some(error_handling) = operation.error_handling {
            self.ir_output.extend(error_handling);
        }
        
        Ok(operation.result_value)
    }

    #[instrument(skip(self))]
    fn compile_receive_expression(&mut self, channel: &dyn Expression) -> Result<(), Error> {
        let operation = self.compile_receive_operation(channel, true)?;
        
        // Add instructions to IR output
        self.ir_output.extend(operation.instructions);
        if let Some(error_handling) = operation.error_handling {
            self.ir_output.extend(error_handling);
        }
        
        Ok(operation.result_value)
    }

    #[instrument(skip(self))]
    fn compile_channel_creation_expression(&mut self, element_type: &LlvmType, buffer_size: Option<usize>) -> Result<(), Error> {
        let operation = self.compile_channel_creation(element_type, buffer_size)?;
        
        // Add instructions to IR output
        self.ir_output.extend(operation.instructions);
        if let Some(error_handling) = operation.error_handling {
            self.ir_output.extend(error_handling);
        }
        
        Ok(operation.result_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::type_system::LlvmTypeRegistry;

    #[test]
    fn test_channel_type_compilation() {
        let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new());
        let element_type = LlvmType::Int32;
        
        let result = compiler.compile_channel_type(&element_type, Some(10));
        assert!(result.is_ok());
        
        let channel_type = result.unwrap();
        assert_eq!(channel_type.element_type, element_type);
        assert_eq!(channel_type.buffer_size, Some(10));
    }

    #[test]
    fn test_runtime_function_initialization() {
        let compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new());
        
        assert!(compiler.runtime_functions.contains_key("create_channel"));
        assert!(compiler.runtime_functions.contains_key("send"));
        assert!(compiler.runtime_functions.contains_key("receive"));
        assert!(compiler.runtime_functions.contains_key("close"));
    }

    #[test]
    fn test_ir_generation() {
        let compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new());
        let ir = compiler.generate_ir();
        
        assert!(ir.contains("declare"));
        assert!(ir.contains("@cursed_channel_create"));
        assert!(ir.contains("@cursed_channel_send"));
        assert!(ir.contains("@cursed_channel_receive"));
    }
}
