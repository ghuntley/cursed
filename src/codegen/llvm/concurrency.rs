//! Code generation for concurrency primitives

use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::types::BasicTypeEnum;
use crate::ast::expressions::concurrency::StanExpression;
use crate::ast::expressions::channel::{ChannelExpression, SendExpression, ReceiveExpression};
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use crate::ast::statements::GoStatement;
use crate::ast::expressions::CallExpression;
use tracing::{instrument};

// Import required traits
use super::expression::ExpressionCompilation;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper method to calculate the size of a type in bytes
    #[instrument(level = "debug", skip(self))]
    fn get_type_size_in_bytes(&self, ty: BasicTypeEnum<'ctx>) -> u64 {
        if ty.is_int_type() {
            let bit_width = ty.into_int_type().get_bit_width();
            (bit_width + 7) / 8 // Round up to nearest byte
        } else if ty.is_float_type() {
            // Handle float types based on their size
            let float_ty = ty.into_float_type();
            // Handle float types based on their size by comparing with well-known types
            if float_ty == self.context().f16_type() {
                2 // 16-bit float
            } else if float_ty == self.context().f32_type() {
                4 // 32-bit float
            } else if float_ty == self.context().f64_type() {
                8 // 64-bit float
            } else {
                16 // Largest float types (FP128, etc.)
            }
        } else if ty.is_pointer_type() {
            8 // Assuming 64-bit pointers
        } else if ty.is_struct_type() {
            let struct_type = ty.into_struct_type();
            let mut size = 0;
            for i in 0..struct_type.count_fields() {
                if let Some(field_type) = struct_type.get_field_type_at_index(i) {
                    size += self.get_type_size_in_bytes(field_type);
                }
            }
            if size == 0 { 1 } else { size } // Empty structs take at least 1 byte
        } else if ty.is_array_type() {
            let array_type = ty.into_array_type();
            let elem_size = self.get_type_size_in_bytes(array_type.get_element_type());
            let len = array_type.len();
            elem_size * len
        } else if ty.is_vector_type() {
            let vector_type = ty.into_vector_type();
            let elem_size = self.get_type_size_in_bytes(vector_type.get_element_type());
            let len = vector_type.get_size();
            elem_size * len
        } else {
            // Default fallback for types we don't handle explicitly
            8 // Reasonable default for unknown types
        }
    }
    
    /// Compile a goroutine (stan) expression
    pub fn compile_goroutine_expression(
        &mut self,
        goroutine: &StanExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling goroutine expression");
        
        // First, compile the function call expression that will be run in the goroutine
        // This would typically be a CallExpression that will execute in the goroutine
        let func_expr = self.compile_expression(&*goroutine.expression)?;
        
        // In a full implementation, we would:
        // 1. Extract the function to call from the expression
        // 2. Create a new goroutine context/function that calls this function
        // 3. Call the runtime's goroutine creation function (e.g., cursed_create_goroutine)
        // 4. Pass the function and any captured context to the runtime
        
        // For now, return a placeholder value
        tracing::info!("Goroutine implementation is incomplete - returning placeholder");
        Ok(self.context().i32_type().const_int(0, false).into())
    }
    
    /// Compile a channel (dm) creation expression
    pub fn compile_channel_expression(
        &mut self,
        channel: &ChannelExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the element type for the channel
        // Use a monomorphization method instead
        use crate::codegen::llvm::function_monomorphization::FunctionMonomorphization;
        let elem_type = self.monomorphization_type_to_llvm_type(&channel.element_type)?;
        
        // Create a channel pointer type
        let channel_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        
        // Get the make_channel runtime function
        let make_channel_fn = self.get_or_create_make_channel_fn()?;
        
        // Call the make_channel function with the size of the element type and capacity
        let elem_size = self.get_type_size_in_bytes(elem_type);
        let size_of_elem = self.context().i64_type().const_int(elem_size, false);
        
        let capacity_value = if let Some(cap_expr) = &channel.capacity {
            let compiled_cap = self.compile_expression(cap_expr.as_ref())?;
            if compiled_cap.is_int_value() {
                compiled_cap.into_int_value()
            } else {
                return Err(Error::codegen("Channel capacity must be an integer".to_string()));
            }
        } else {
            self.context().i64_type().const_int(0, false) // Default to unbuffered channel
        };
        
        // Call the make_channel function from our runtime implementation
        tracing::debug!("Calling cursed_make_channel with element size and capacity");
        let args = &[size_of_elem.into(), capacity_value.into()];
        let channel_ptr = self.builder().build_call(make_channel_fn, args, "channel")
            .map_err(|e| Error::codegen(format!("Failed to call cursed_make_channel: {}", e)))?;
        
        Ok(channel_ptr.try_as_basic_value().left().unwrap())
    }
    
    /// Compile a send expression (ch <- value)
    pub fn compile_send_expression(
        &mut self,
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
            return Err(Error::codegen("Channel expression must evaluate to a pointer".to_string()));
        };
        
        // We need to create a temporary alloca for the value to get its address
        let value_type = value.get_type();
        let value_ptr = self.builder().build_alloca(value_type, "send_value")
            .map_err(|e| Error::codegen(format!("Failed to allocate send value: {}", e)))?;
        
        // Store the value in the temporary
        self.builder().build_store(value_ptr, value)
            .map_err(|e| Error::codegen(format!("Failed to store send value: {}", e)))?;
        
        // Convert the value ptr to i8*
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let value_i8_ptr = self.builder().build_bitcast(value_ptr, i8_ptr_type, "value_i8_ptr")
            .map_err(|e| Error::codegen(format!("Failed to bitcast value pointer: {}", e)))?;
        
        // Call the send function from our runtime implementation
        tracing::debug!("Calling cursed_send_to_channel");
        let args = &[channel_ptr.into(), value_i8_ptr.into()];
        let send_result = self.builder().build_call(send_fn, args, "send_result")
            .map_err(|e| Error::codegen(format!("Failed to call cursed_send_to_channel: {}", e)))?;
        
        // The send function returns void, so we return a dummy value
        let i32_type = self.context().i32_type();
        Ok(i32_type.const_int(0, false).into())
    }
    
    /// Compile a receive expression (<-ch)
    pub fn compile_receive_expression(
        &mut self,
        receive: &ReceiveExpression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Compile the channel expression
        let channel = self.compile_expression(&*receive.channel)?;
        
        // Get the receive_from_channel runtime function
        let receive_fn = self.get_or_create_receive_fn()?;
        
        // Create a result pointer for the received value
        // Use a monomorphization method instead
        use crate::codegen::llvm::function_monomorphization::FunctionMonomorphization;
        let value_type = self.monomorphization_type_to_llvm_type(&receive.element_type)?;
        let result_ptr = self.builder().build_alloca(value_type, "receive_result")
            .map_err(|e| Error::codegen(format!("Failed to allocate receive result: {}", e)))?;
        
        // Convert the result ptr to i8*
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let result_i8_ptr = self.builder().build_bitcast(result_ptr, i8_ptr_type, "result_i8_ptr")
            .map_err(|e| Error::codegen(format!("Failed to bitcast result pointer: {}", e)))?;
        
        // Call the receive function from our runtime implementation
        let channel_ptr = if channel.is_pointer_value() {
            channel.into_pointer_value()
        } else {
            return Err(Error::codegen("Channel expression must evaluate to a pointer".to_string()));
        };
        
        tracing::debug!("Calling cursed_receive_from_channel");
        let args = &[channel_ptr.into(), result_i8_ptr.into()];
        let _ = self.builder().build_call(receive_fn, args, "receive_call")
            .map_err(|e| Error::codegen(format!("Failed to call cursed_receive_from_channel: {}", e)))?;
        
        // Load and return the result
        Ok(self.builder().build_load(value_type, result_ptr, "receive_load")
            .map_err(|e| Error::codegen(format!("Failed to load receive result: {}", e)))?
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
        let i64_type = self.context().i64_type();
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
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
        let void_type = self.context().void_type();
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
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
        let void_type = self.context().void_type();
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let fn_type = void_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        
        // Add the function to the module
        Ok(self.module().add_function(fn_name, fn_type, None))
    }
}