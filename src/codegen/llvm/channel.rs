//! LLVM code generation for channels

use inkwell::values::BasicValueEnum;
use crate::ast::{Expression, ChannelExpression, SendExpression, ReceiveExpression};
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a channel creation expression
    #[tracing::instrument(skip(self, channel_expr), fields(element_type = ?channel_expr.element_type.string(), has_capacity = channel_expr.capacity.is_some()), level = "debug")]
    pub fn compile_channel_creation(&mut self, channel_expr: &ChannelExpression) -> Result<BasicValueEnum<'ctx>, String> {
        tracing::info!("Compiling channel creation");
        // Import the create_channel function from core::channel
        self.init_channel_helpers();
        
        // Get the element type as a string
        let element_type_str = channel_expr.element_type.string();
        
        // Create a string constant for the element type
        let element_type_const = self.builder.build_global_string_ptr(&element_type_str, "element_type").unwrap();
        
        // Determine which function to call based on whether capacity is provided
        if let Some(capacity_expr) = &channel_expr.capacity {
            // This is a buffered channel with capacity
            let create_buffered_channel_fn = self.module.get_function("create_buffered_channel").ok_or_else(|| 
                "create_buffered_channel function not found".to_string()
            )?;
            
            // Compile the capacity expression
            let capacity_value = self.compile_expression(capacity_expr.as_ref())?;
            
            // Call function with element type and capacity
            let result = self.builder.build_call(
                create_buffered_channel_fn,
                &[element_type_const.as_pointer_value().into(), capacity_value.into()],
                "buffered_channel"
            ).unwrap();
            
            // Return the channel object
            Ok(result.try_as_basic_value().left().unwrap())
        } else {
            // This is an unbuffered channel
            let create_channel_fn = self.module.get_function("create_channel").ok_or_else(|| 
                "create_channel function not found".to_string()
            )?;
            
            // Call the function with just the element type
            let result = self.builder.build_call(
                create_channel_fn,
                &[element_type_const.as_pointer_value().into()],
                "channel"
            ).unwrap();
            
            // Return the channel object
            Ok(result.try_as_basic_value().left().unwrap())
        }
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
        
        // Get the send_to_channel function
        let send_fn = self.module.get_function("send_to_channel").ok_or_else(|| 
            "send_to_channel function not found".to_string()
        )?;
        
        // Call the function with channel and value
        let result = self.builder.build_call(
            send_fn,
            &[channel_val.into(), value_void_ptr.into()],
            "send_result"
        ).unwrap();
        
        // Return result code (0 for success, non-zero for error)
        let result_val = result.try_as_basic_value().left().unwrap();
        Ok(result_val)
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
        
        // Get the try_send_to_channel function
        let try_send_fn = self.module.get_function("try_send_to_channel").ok_or_else(|| 
            "try_send_to_channel function not found".to_string()
        )?;
        
        // Call the function with channel and value
        let result = self.builder.build_call(
            try_send_fn,
            &[channel_val.into(), value_void_ptr.into()],
            "try_send_result"
        ).unwrap();
        
        // Return result code (0 for success, 1 for would block, -1 for error)
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
        
        // Get the receive_from_channel function
        let receive_fn = self.module.get_function("receive_from_channel").ok_or_else(|| 
            "receive_from_channel function not found".to_string()
        )?;
        
        // Call the function with channel
        let result = self.builder.build_call(
            receive_fn,
            &[channel_val.into()],
            "receive_result"
        ).unwrap();
        
        // Get the return value which is a void pointer
        let void_ptr = result.try_as_basic_value().left().unwrap().into_pointer_value();
        
        // For a complete implementation, we should know the channel's element type
        // and cast the void pointer to the correct type
        // For now, we'll assume it's an integer value
        
        // Load the value from the void pointer by casting it to the appropriate type
        let i64_type = self.context.i64_type();
        let i64_ptr_type = i64_type.ptr_type(inkwell::AddressSpace::default());
        let value_ptr = self.builder.build_bitcast(void_ptr, i64_ptr_type, "value_ptr").unwrap();
        let value = self.builder.build_load(i64_type, value_ptr.into_pointer_value(), "received_value").unwrap();
        
        // Return the received value
        Ok(value)
    }
    
    // Compile a channel close expression
    pub fn compile_channel_close(&mut self, channel_expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, String> {
        // Import the channel helper functions
        self.init_channel_helpers();
        
        // Compile the channel expression
        let channel_val = self.compile_expression(channel_expr)?;
        
        // Get the close_channel function
        let close_fn = self.module.get_function("close_channel").ok_or_else(|| 
            "close_channel function not found".to_string()
        )?;
        
        // Call the function with channel
        let result = self.builder.build_call(
            close_fn,
            &[channel_val.into()],
            "close_result"
        ).unwrap();
        
        // Return result code (0 for success, non-zero for error)
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
        self.module.add_function("close_channel", close_type, Some(inkwell::module::Linkage::External));
    }
}