//! Code generation for concurrency primitives

use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::ast::expressions::concurrency::GoroutineExpression;
use crate::ast::expressions::channel::{ChannelExpression, SendExpression, ReceiveExpression};
use crate::error::Error;
use super::generator::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a goroutine (stan) expression
    pub fn compile_goroutine_expression(
        &self,
        goroutine: &GoroutineExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // First, compile the function call expression that will be run in the goroutine
        // We don't actually use this result, but it helps to validate that the expression is valid
        let _ = self.compile_expression(&*goroutine.expression)?;
        
        // Get the function to call
        let function = match &*goroutine.expression {
            // TODO: Extract the function from the call expression
            // This is a simplified version that assumes direct function calls
            _ => return Err(Error::CodegenError("Only function calls are supported in goroutines".to_string()))
        };
        
        // For now, we'll return a dummy value since goroutines don't return a value
        let i32_type = self.context.i32_type();
        Ok(i32_type.const_int(0, false).into())
    }
    
    /// Compile a channel (dm) creation expression
    pub fn compile_channel_expression(
        &self,
        channel: &ChannelExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the element type for the channel
        let elem_type = self.type_to_llvm_type(&channel.element_type_name)?;
        
        // Create a channel pointer type
        let channel_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        
        // Get the make_channel runtime function
        let make_channel_fn = self.get_or_create_make_channel_fn()?;
        
        // Call the make_channel function with the size of the element type and capacity
        let size_of_elem = if elem_type.is_int_type() {
            self.context.i64_type().const_int(elem_type.into_int_type().get_bit_width() as u64 / 8, false)
        } else if elem_type.is_float_type() {
            self.context.i64_type().const_int(elem_type.into_float_type().get_bit_width() as u64 / 8, false)
        } else if elem_type.is_pointer_type() {
            self.context.i64_type().const_int(8, false) // Assuming 64-bit pointers
        } else if elem_type.is_struct_type() {
            // TODO: Get the size of the struct properly
            self.context.i64_type().const_int(16, false) // Placeholder
        } else {
            return Err(Error::CodegenError(format!("Unsupported channel element type: {}", channel.element_type_name)));
        };
        
        let capacity = self.context.i64_type().const_int(channel.capacity as u64, false);
        
        // Call the make_channel function
        let args = &[size_of_elem.into(), capacity.into()];
        let channel_ptr = self.builder.build_call(make_channel_fn, args, "channel")
            .map_err(|e| Error::CodegenError(format!("Failed to call make_channel: {}", e)))?;
        
        Ok(channel_ptr.try_as_basic_value().left().unwrap())
    }
    
    /// Compile a send expression (ch <- value)
    pub fn compile_send_expression(
        &self,
        send: &SendExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Compile the channel and value expressions
        let channel = self.compile_expression(&*send.channel)?;
        let value = self.compile_expression(&*send.value)?;
        
        // Get the send_to_channel runtime function
        let send_fn = self.get_or_create_send_fn()?;
        
        // Call the send function
        let channel_ptr = if channel.is_pointer_value() {
            channel.into_pointer_value()
        } else {
            return Err(Error::CodegenError("Channel expression must evaluate to a pointer".to_string()));
        };
        
        // We need to create a temporary alloca for the value to get its address
        let value_type = value.get_type();
        let value_ptr = self.builder.build_alloca(value_type, "send_value")
            .map_err(|e| Error::CodegenError(format!("Failed to allocate send value: {}", e)))?;
        
        // Store the value in the temporary
        self.builder.build_store(value_ptr, value)
            .map_err(|e| Error::CodegenError(format!("Failed to store send value: {}", e)))?;
        
        // Convert the value ptr to i8*
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let value_i8_ptr = self.builder.build_bit_cast(value_ptr, i8_ptr_type, "value_i8_ptr")
            .map_err(|e| Error::CodegenError(format!("Failed to bitcast value pointer: {}", e)))?;
        
        // Call the send function
        let args = &[channel_ptr.into(), value_i8_ptr.into()];
        let send_result = self.builder.build_call(send_fn, args, "send_result")
            .map_err(|e| Error::CodegenError(format!("Failed to call send: {}", e)))?;
        
        // The send function returns void, so we return a dummy value
        let i32_type = self.context.i32_type();
        Ok(i32_type.const_int(0, false).into())
    }
    
    /// Compile a receive expression (<-ch)
    pub fn compile_receive_expression(
        &self,
        receive: &ReceiveExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Compile the channel expression
        let channel = self.compile_expression(&*receive.channel)?;
        
        // Get the receive_from_channel runtime function
        let receive_fn = self.get_or_create_receive_fn()?;
        
        // Create a result pointer for the received value
        let value_type = self.type_to_llvm_type(&receive.element_type_name)?;
        let result_ptr = self.builder.build_alloca(value_type, "receive_result")
            .map_err(|e| Error::CodegenError(format!("Failed to allocate receive result: {}", e)))?;
        
        // Convert the result ptr to i8*
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let result_i8_ptr = self.builder.build_bit_cast(result_ptr, i8_ptr_type, "result_i8_ptr")
            .map_err(|e| Error::CodegenError(format!("Failed to bitcast result pointer: {}", e)))?;
        
        // Call the receive function
        let channel_ptr = if channel.is_pointer_value() {
            channel.into_pointer_value()
        } else {
            return Err(Error::CodegenError("Channel expression must evaluate to a pointer".to_string()));
        };
        
        let args = &[channel_ptr.into(), result_i8_ptr.into()];
        let _ = self.builder.build_call(receive_fn, args, "receive_call")
            .map_err(|e| Error::CodegenError(format!("Failed to call receive: {}", e)))?;
        
        // Load and return the result
        Ok(self.builder.build_load(value_type, result_ptr, "receive_load")
            .map_err(|e| Error::CodegenError(format!("Failed to load receive result: {}", e)))?
            .into())
    }
    
    // Helper methods to get or create the runtime functions
    
    fn get_or_create_make_channel_fn(&self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_make_channel";
        
        // Check if the function already exists
        if let Some(func) = self.module().get_function(fn_name) {
            return Ok(func);
        }
        
        // Create the function declaration
        let i64_type = self.context.i64_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let fn_type = i8_ptr_type.fn_type(&[i64_type.into(), i64_type.into()], false);
        
        // Add the function to the module
        Ok(self.module().add_function(fn_name, fn_type, None))
    }
    
    fn get_or_create_send_fn(&self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_send_to_channel";
        
        // Check if the function already exists
        if let Some(func) = self.module().get_function(fn_name) {
            return Ok(func);
        }
        
        // Create the function declaration
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let fn_type = void_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        
        // Add the function to the module
        Ok(self.module().add_function(fn_name, fn_type, None))
    }
    
    fn get_or_create_receive_fn(&self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_receive_from_channel";
        
        // Check if the function already exists
        if let Some(func) = self.module().get_function(fn_name) {
            return Ok(func);
        }
        
        // Create the function declaration
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let fn_type = void_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        
        // Add the function to the module
        Ok(self.module().add_function(fn_name, fn_type, None))
    }
}