//! LLVM code generation for channels

use inkwell::values::BasicValueEnum;
use crate::ast::{Expression, ChannelExpression, SendExpression, ReceiveExpression};
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a channel creation expression
    #[tracing::instrument(skip(self, channel_expr), fields(element_type = ?channel_expr.element_type.string(), has_capacity = channel_expr.capacity.is_some()), level = "debug")]
    pub fn compile_channel_creation(&mut self, channel_expr: &ChannelExpression) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!("Compiling channel creation");
        // Import the enhanced channel creation function
        self.init_channel_helpers();
        
        // Get the element type as a string and determine size
        let element_type_str = channel_expr.element_type.string();
        let element_size = match element_type_str.as_str() {
            "byte" => 1u64,
            "normie" | "int" | "i32" => 4u64,
            "thicc" | "i64" | "float" | "f64" => 8u64,
            _ => 8u64, // Default to 8 bytes for unknown types
        };
        
        // Get the capacity (0 for unbuffered)
        let capacity = if let Some(capacity_expr) = &channel_expr.capacity {
            // Compile the capacity expression
            let capacity_value = self.compile_expression(capacity_expr.as_ref())?;
            match capacity_value {
                BasicValueEnum::IntValue(int_val) => {
                    int_val.get_zero_extended_constant().unwrap_or(0)
                },
                _ => return Err("Channel capacity must be an integer".to_string()),
            }
        } else {
            0u64 // Unbuffered channel
        };
        
        // Get the enhanced channel creation function
        let create_channel_fn = self.module.get_function("cursed_make_channel").ok_or_else(|| 
            "cursed_make_channel function not found".to_string()
        )?;
        
        // Create constants for element size and capacity
        let element_size_const = self.context.i64_type().const_int(element_size, false);
        let capacity_const = self.context.i64_type().const_int(capacity, false);
        
        // Call the enhanced function with element size and capacity
        let result = self.builder.build_call(
            create_channel_fn,
            &[element_size_const.into(), capacity_const.into()],
            "channel"
        ).unwrap();
        
        // Return the channel pointer
        Ok(result.try_as_basic_value().left().unwrap())
    }
    
    // Compile a send expression (either blocking or non-blocking)
    #[tracing::instrument(skip(self, send_expr), fields(non_blocking = send_expr.non_blocking), level = "debug")]
    pub fn compile_send_expression(&mut self, send_expr: &SendExpression) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!(non_blocking = send_expr.non_blocking, "Compiling channel send operation");
        // Import the send_to_channel function
        self.init_channel_helpers();
        
        // Compile the channel and value expressions
        let channel_val = self.compile_expression(send_expr.channel.as_ref())?;
        let value_val = self.compile_expression(send_expr.value.as_ref())?;
        
        // For improved safety, we should type check the value against the channel's element type
        // This would require storing channel type information when channels are created
        // For now, we'll just cast the value to a void pointer if needed
        let value_ptr = if !value_val.is_pointer_value() {
            // Store the value in a temporary allocation and pass its address
            let value_type = value_val.get_type();
            let temp_alloca = self.builder.build_alloca(value_type, "send_value_temp").unwrap();
            self.builder.build_store(temp_alloca, value_val).unwrap();
            temp_alloca // PointerValue is already a pointer, no need to call as_pointer_value()
        } else {
            value_val.into_pointer_value()
        };
        
        // Convert to i8* type (void pointer) as required by send_to_channel
        let void_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let value_void_ptr = self.builder.build_bitcast(value_ptr, void_ptr_type, "value_void_ptr").unwrap();
        
        // Get the enhanced send function
        let send_fn = self.module.get_function("cursed_send_to_channel").ok_or_else(|| 
            "cursed_send_to_channel function not found".to_string()
        )?;
        
        // Call the function with channel and value
        self.builder.build_call(
            send_fn,
            &[channel_val.into(), value_void_ptr.into()],
            "send_result"
        ).unwrap();
        
        // Return success (void return from FFI function)
        let success_const = self.context.i32_type().const_int(0, false);
        Ok(success_const.into())
    }
    
    // Compile a non-blocking send expression
    pub fn compile_nonblocking_send_expression(&mut self, send_expr: &SendExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Import the try_send_to_channel function
        self.init_channel_helpers();
        
        // Compile the channel and value expressions
        let channel_val = self.compile_expression(send_expr.channel.as_ref())?;
        let value_val = self.compile_expression(send_expr.value.as_ref())?;
        
        // For improved safety, we should type check the value against the channel's element type
        // This would require storing channel type information when channels are created
        // For now, we'll just cast the value to a void pointer if needed
        let value_ptr = if !value_val.is_pointer_value() {
            // Store the value in a temporary allocation and pass its address
            let value_type = value_val.get_type();
            let temp_alloca = self.builder.build_alloca(value_type, "send_value_temp").unwrap();
            self.builder.build_store(temp_alloca, value_val).unwrap();
            temp_alloca // PointerValue is already a pointer, no need to call as_pointer_value()
        } else {
            value_val.into_pointer_value()
        };
        
        // Convert to i8* type (void pointer) as required by try_send_to_channel
        let void_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let value_void_ptr = self.builder.build_bitcast(value_ptr, void_ptr_type, "value_void_ptr").unwrap();
        
        // Get the enhanced try_send function
        let try_send_fn = self.module.get_function("cursed_try_send_to_channel").ok_or_else(|| 
            "cursed_try_send_to_channel function not found".to_string()
        )?;
        
        // Call the function with channel and value
        let result = self.builder.build_call(
            try_send_fn,
            &[channel_val.into(), value_void_ptr.into()],
            "try_send_result"
        ).unwrap();
        
        // Return result code (1 for success, 0 for would block, -1 for error)
        let result_val = result.try_as_basic_value().left().unwrap();
        Ok(result_val)
    }
    
    // Compile a receive expression (either blocking or non-blocking)
    #[tracing::instrument(skip(self, recv_expr), fields(non_blocking = recv_expr.non_blocking), level = "debug")]
    pub fn compile_receive_expression(&mut self, recv_expr: &ReceiveExpression) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!(non_blocking = recv_expr.non_blocking, "Compiling channel receive operation");
        // Import the receive_from_channel function
        self.init_channel_helpers();
        
        // Compile the channel expression
        let channel_val = self.compile_expression(recv_expr.channel.as_ref())?;
        
        // Allocate space for the received value
        let i64_type = self.context.i64_type();
        let result_alloca = self.builder.build_alloca(i64_type, "receive_result").unwrap();
        
        // Cast to void pointer for FFI call
        let void_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let result_void_ptr = self.builder.build_bitcast(result_alloca, void_ptr_type, "result_void_ptr").unwrap();
        
        // Get the enhanced receive function
        let receive_fn = self.module.get_function("cursed_receive_from_channel").ok_or_else(|| 
            "cursed_receive_from_channel function not found".to_string()
        )?;
        
        // Call the function with channel and result pointer
        self.builder.build_call(
            receive_fn,
            &[channel_val.into(), result_void_ptr.into()],
            "receive_call"
        ).unwrap();
        
        // Load the received value from the allocated space
        let value = self.builder.build_load(i64_type, result_alloca, "received_value").unwrap();
        
        // Return the received value
        Ok(value)
    }
    
    /// Compile a channel close expression with comprehensive error handling
    #[tracing::instrument(skip(self, channel_expr), level = "debug")]
    pub fn compile_channel_close(&mut self, channel_expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!("Compiling channel close operation");
        
        // Import the channel helper functions
        self.init_channel_helpers();
        
        // Compile the channel expression
        let channel_val = self.compile_expression(channel_expr)?;
        
        // Get the close_channel function
        let close_fn = self.module.get_function("cursed_close_channel").ok_or_else(|| 
            "cursed_close_channel function not found".to_string()
        )?;
        
        // Call the function with channel
        let result = self.builder.build_call(
            close_fn,
            &[channel_val.into()],
            "close_result"
        ).unwrap();
        
        // Return result code (0 for success, non-zero for error)
        let result_val = result.try_as_basic_value().left().unwrap();
        
        // Create error checking blocks
        let current_fn = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let success_block = self.context.append_basic_block(current_fn, "close_success");
        let error_block = self.context.append_basic_block(current_fn, "close_error");
        let cont_block = self.context.append_basic_block(current_fn, "close_continue");
        
        // Check if the result is 0 (success)
        let zero = self.context.i32_type().const_int(0, false);
        let is_success = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            result_val.into_int_value(),
            zero,
            "is_close_success"
        ).unwrap();
        
        // Branch based on success
        self.builder.build_conditional_branch(is_success, success_block, error_block).unwrap();
        
        // Success block
        self.builder.position_at_end(success_block);
        self.builder.build_unconditional_branch(cont_block).unwrap();
        
        // Error block - could add logging or error handling here
        self.builder.position_at_end(error_block);
        self.builder.build_unconditional_branch(cont_block).unwrap();
        
        // Continue block
        self.builder.position_at_end(cont_block);
        
        Ok(result_val)
    }
    
    /// Compile a graceful channel close with timeout
    pub fn compile_channel_close_gracefully(&mut self, channel_expr: &dyn Expression, timeout_ms: u64) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!(timeout_ms = timeout_ms, "Compiling graceful channel close operation");
        
        // Import the channel helper functions
        self.init_channel_helpers();
        
        // Compile the channel expression
        let channel_val = self.compile_expression(channel_expr)?;
        
        // Get the graceful close function
        let close_fn = self.module.get_function("cursed_close_channel_gracefully").ok_or_else(|| 
            "cursed_close_channel_gracefully function not found".to_string()
        )?;
        
        // Create timeout constant
        let timeout_val = self.context.i64_type().const_int(timeout_ms, false);
        
        // Call the function with channel and timeout
        let result = self.builder.build_call(
            close_fn,
            &[channel_val.into(), timeout_val.into()],
            "graceful_close_result"
        ).unwrap();
        
        // Return result code
        let result_val = result.try_as_basic_value().left().unwrap();
        Ok(result_val)
    }
    
    // Initialize the channel helper functions
    pub fn init_channel_helpers(&mut self) {
        // Skip initialization if we've already done it
        if self.module.get_function("create_channel").is_some() {
            return;
        }
        
        // Set up common types
        let void_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let int32_type = self.context.i32_type();
        
        // Declare the unbuffered channel creation function
        let create_channel_type = void_ptr_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("create_channel", create_channel_type, Some(inkwell::module::Linkage::External));
        
        // Declare the buffered channel creation function
        let create_buffered_type = void_ptr_type.fn_type(&[void_ptr_type.into(), int32_type.into()], false);
        self.module.add_function("create_buffered_channel", create_buffered_type, Some(inkwell::module::Linkage::External));
        
        // Declare the send function
        let send_type = int32_type.fn_type(&[void_ptr_type.into(), void_ptr_type.into()], false);
        self.module.add_function("send_to_channel", send_type, Some(inkwell::module::Linkage::External));
        
        // Declare the non-blocking send function
        let try_send_type = int32_type.fn_type(&[void_ptr_type.into(), void_ptr_type.into()], false);
        self.module.add_function("try_send_to_channel", try_send_type, Some(inkwell::module::Linkage::External));
        
        // Declare the receive function
        let receive_type = void_ptr_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("receive_from_channel", receive_type, Some(inkwell::module::Linkage::External));
        
        // Declare the non-blocking receive function
        let try_receive_type = void_ptr_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("try_receive_from_channel", try_receive_type, Some(inkwell::module::Linkage::External));
        
        // Declare the close channel function
        let close_type = int32_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("cursed_close_channel", close_type, Some(inkwell::module::Linkage::External));
        
        // Declare the graceful close channel function  
        let graceful_close_type = int32_type.fn_type(&[void_ptr_type.into(), self.context.i64_type().into()], false);
        self.module.add_function("cursed_close_channel_gracefully", graceful_close_type, Some(inkwell::module::Linkage::External));
    }
}