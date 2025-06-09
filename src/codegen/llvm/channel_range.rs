//! LLVM code generation for channel range operations
//!
//! This module implements code generation for channel range constructs,
//! enabling efficient iteration over channel values with proper closure detection.

use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::basic_block::BasicBlock;
use inkwell::{IntPredicate, FloatPredicate};
use crate::ast::control_flow::channel_range::{ChannelRangeClause, ChannelRangeForStatement, ChannelClosureDetection};
use crate::ast::Expression;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::variables::{VariableHandling, VariableScope};
use super::expression::ExpressionCompilation;
use super::statement::StatementCompilation;

/// Channel range compilation trait
pub trait ChannelRangeCompilation<'ctx> {
    /// Compile a channel range clause
    fn compile_channel_range_clause(&mut self, clause: &ChannelRangeClause) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a channel range for statement
    fn compile_channel_range_for_statement(&mut self, stmt: &ChannelRangeForStatement) -> Result<(), Error>;
    
    /// Compile channel closure detection
    fn compile_channel_closure_detection(&mut self, closure: &ChannelClosureDetection) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate channel receive loop with closure detection
    fn generate_channel_receive_loop(
        &mut self,
        channel: BasicValueEnum<'ctx>,
        value_var: &str,
        ok_var: Option<&str>,
        body_block: BasicBlock<'ctx>,
        end_block: BasicBlock<'ctx>
    ) -> Result<(), Error>;
    
    /// Check if channel is closed
    fn check_channel_closed(&mut self, channel: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Receive value from channel with closure detection
    fn receive_with_closure_detection(&mut self, channel: BasicValueEnum<'ctx>) -> Result<(BasicValueEnum<'ctx>, IntValue<'ctx>), Error>;
}

impl<'ctx> ChannelRangeCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    /// Compile a channel range clause
    #[tracing::instrument(skip(self, clause), level = "debug")]
    fn compile_channel_range_clause(&mut self, clause: &ChannelRangeClause) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::info!("Compiling channel range clause");
        
        // Compile the channel expression
        let channel_value = self.compile_expression(clause.channel.as_ref())?;
        
        // For a channel range clause, we return the channel value
        // The actual iteration logic is handled in the for statement
        Ok(channel_value)
    }
    
    /// Compile a channel range for statement
    #[tracing::instrument(skip(self, stmt), fields(with_ok = stmt.ok_var.is_some()), level = "debug")]
    fn compile_channel_range_for_statement(&mut self, stmt: &ChannelRangeForStatement) -> Result<(), Error> {
        tracing::info!(with_ok = stmt.ok_var.is_some(), "Compiling channel range for statement");
        
        // Initialize channel helpers for FFI functions
        self.init_channel_range_helpers();
        
        // Compile the channel expression
        let channel_value = self.compile_channel_range_clause(&stmt.channel_range)?;
        
        // Create basic blocks for the loop
        let function = self.current_function().ok_or_else(|| Error::from_str("Channel range for statement outside function"))?;
        let loop_head = self.context().append_basic_block(function, "channel_range_head");
        let loop_body = self.context().append_basic_block(function, "channel_range_body");
        let loop_end = self.context().append_basic_block(function, "channel_range_end");
        
        // Create a loop context for break/continue
        let context = super::LoopContext {
            name: "channel_range".to_string(),
            break_block: loop_end,
            continue_block: loop_head, // Continue goes back to the receive operation
        };
        self.push_loop_context(context);
        
        // Create a new variable scope for the loop
        let scope = VariableScope::new();
        self.push_scope(scope);
        
        // Branch to the loop head
        self.builder().build_unconditional_branch(loop_head)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the loop head (receive operation)
        self.builder().position_at_end(loop_head);
        
        // Receive from channel with closure detection
        let (received_value, is_open) = self.receive_with_closure_detection(channel_value)?;
        
        // If we have an ok variable, store the channel status
        if let Some(ok_var) = &stmt.ok_var {
            // Convert i32 to bool for the ok variable
            let bool_value = self.builder().build_int_compare(
                IntPredicate::NE,
                is_open,
                self.context().i32_type().const_int(0, false),
                "channel_open_bool"
            ).map_err(|e| Error::from_str(&format!("Failed to build comparison: {}", e)))?;
            
            // Store the ok variable
            self.store_variable(ok_var, bool_value.into())?;
        }
        
        // Check if channel is still open
        let is_closed = self.builder().build_int_compare(
            IntPredicate::EQ,
            is_open,
            self.context().i32_type().const_int(0, false),
            "channel_closed"
        ).map_err(|e| Error::from_str(&format!("Failed to build comparison: {}", e)))?;
        
        // Branch based on channel status
        self.builder().build_conditional_branch(is_closed, loop_end, loop_body)
            .map_err(|e| Error::from_str(&format!("Failed to build conditional branch: {}", e)))?;
        
        // Build the loop body
        self.builder().position_at_end(loop_body);
        
        // Store the received value in the value variable
        self.store_variable(&stmt.value_var, received_value)?;
        
        // Compile the loop body statements
        for body_stmt in &stmt.body.statements {
            self.compile_statement(&**body_stmt)?;
        }
        
        // Branch back to loop head if not terminated
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(loop_head)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the loop end
        self.builder().position_at_end(loop_end);
        
        // Pop scope and loop context
        self.pop_scope();
        self.pop_loop_context();
        
        Ok(())
    }
    
    /// Compile channel closure detection
    #[tracing::instrument(skip(self, closure), level = "debug")]
    fn compile_channel_closure_detection(&mut self, closure: &ChannelClosureDetection) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::info!("Compiling channel closure detection");
        
        // Initialize channel helpers
        self.init_channel_range_helpers();
        
        // Compile the channel expression
        let channel_value = self.compile_expression(closure.channel.as_ref())?;
        
        // Check if channel is closed
        let is_closed = self.check_channel_closed(channel_value)?;
        
        Ok(is_closed.into())
    }
    
    /// Generate channel receive loop with closure detection
    #[tracing::instrument(skip(self, body_block, end_block), level = "debug")]
    fn generate_channel_receive_loop(
        &mut self,
        channel: BasicValueEnum<'ctx>,
        value_var: &str,
        ok_var: Option<&str>,
        body_block: BasicBlock<'ctx>,
        end_block: BasicBlock<'ctx>
    ) -> Result<(), Error> {
        tracing::info!("Generating channel receive loop");
        
        // Receive from channel with closure detection
        let (received_value, is_open) = self.receive_with_closure_detection(channel)?;
        
        // Store the received value
        self.store_variable(value_var, received_value)?;
        
        // If we have an ok variable, store the channel status
        if let Some(ok_var) = ok_var {
            // Convert i32 to bool for the ok variable
            let bool_value = self.builder().build_int_compare(
                IntPredicate::NE,
                is_open,
                self.context().i32_type().const_int(0, false),
                "channel_open_bool"
            ).map_err(|e| Error::from_str(&format!("Failed to build comparison: {}", e)))?;
            
            self.store_variable(ok_var, bool_value.into())?;
        }
        
        // Check if channel is still open
        let is_closed = self.builder().build_int_compare(
            IntPredicate::EQ,
            is_open,
            self.context().i32_type().const_int(0, false),
            "channel_closed"
        ).map_err(|e| Error::from_str(&format!("Failed to build comparison: {}", e)))?;
        
        // Branch based on channel status
        self.builder().build_conditional_branch(is_closed, end_block, body_block)
            .map_err(|e| Error::from_str(&format!("Failed to build conditional branch: {}", e)))?;
        
        Ok(())
    }
    
    /// Check if channel is closed
    #[tracing::instrument(skip(self), level = "debug")]
    fn check_channel_closed(&mut self, channel: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        tracing::debug!("Checking if channel is closed");
        
        // Get or declare the channel_is_closed function
        let is_closed_fn = match self.module.get_function("cursed_channel_is_closed") {
            Some(func) => func,
            None => {
                // Declare the function
                let void_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
                let i32_type = self.context().i32_type();
                let fn_type = i32_type.fn_type(&[void_ptr_type.into()], false);
                
                self.module.add_function("cursed_channel_is_closed", fn_type, Some(inkwell::module::Linkage::External))
            }
        };
        
        // Call the function
        let result = self.builder().build_call(
            is_closed_fn,
            &[channel.into()],
            "channel_closed_result"
        ).map_err(|e| Error::from_str(&format!("Failed to call channel_is_closed: {}", e)))?;
        
        Ok(result.try_as_basic_value().left().unwrap().into_int_value())
    }
    
    /// Receive value from channel with closure detection
    #[tracing::instrument(skip(self), level = "debug")]
    fn receive_with_closure_detection(&mut self, channel: BasicValueEnum<'ctx>) -> Result<(BasicValueEnum<'ctx>, IntValue<'ctx>), Error> {
        tracing::debug!("Receiving from channel with closure detection");
        
        // Get or declare the channel_receive_with_status function
        let receive_fn = match self.module.get_function("cursed_receive_with_status") {
            Some(func) => func,
            None => {
                // Declare the function
                let void_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
                let i32_type = self.context().i32_type();
                
                // Function returns a struct with {value_ptr, status}
                let struct_type = self.context().struct_type(&[void_ptr_type.into(), i32_type.into()], false);
                let fn_type = struct_type.fn_type(&[void_ptr_type.into()], false);
                
                self.module.add_function("cursed_receive_with_status", fn_type, Some(inkwell::module::Linkage::External))
            }
        };
        
        // Call the function
        let result = self.builder().build_call(
            receive_fn,
            &[channel.into()],
            "receive_result"
        ).map_err(|e| Error::from_str(&format!("Failed to call receive_with_status: {}", e)))?;
        
        let result_struct = result.try_as_basic_value().left().unwrap().into_struct_value();
        
        // Extract the value pointer
        let value_ptr = self.builder().build_extract_value(result_struct, 0, "value_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to extract value: {}", e)))?;
        
        // Extract the status
        let status = self.builder().build_extract_value(result_struct, 1, "status")
            .map_err(|e| Error::from_str(&format!("Failed to extract status: {}", e)))?
            .into_int_value();
        
        // For now, assume the value is an i64 and load it
        // In a complete implementation, we'd need type information
        let i64_type = self.context().i64_type();
        let i64_ptr_type = i64_type.ptr_type(inkwell::AddressSpace::default());
        let typed_ptr = self.builder().build_bitcast(value_ptr.into_pointer_value(), i64_ptr_type, "typed_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to cast pointer: {}", e)))?;
        
        let value = self.builder().build_load(i64_type, typed_ptr.into_pointer_value(), "received_value")
            .map_err(|e| Error::from_str(&format!("Failed to load value: {}", e)))?;
        
        Ok((value, status))
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize channel range helper functions
    pub fn init_channel_range_helpers(&mut self) {
        // Initialize basic channel helpers first
        // TODO: Implement init_channel_helpers or remove dependency
        
        // Skip initialization if we've already done it
        if self.module.get_function("cursed_channel_is_closed").is_some() {
            return;
        }
        
        let void_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context().i32_type();
        
        // Declare the channel_is_closed function
        let is_closed_type = i32_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("cursed_channel_is_closed", is_closed_type, Some(inkwell::module::Linkage::External));
        
        // Declare the receive_with_status function
        let struct_type = self.context().struct_type(&[void_ptr_type.into(), i32_type.into()], false);
        let receive_status_type = struct_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("cursed_receive_with_status", receive_status_type, Some(inkwell::module::Linkage::External));
        
        // Declare the try_receive function for non-blocking operations
        let try_receive_type = struct_type.fn_type(&[void_ptr_type.into()], false);
        self.module.add_function("cursed_try_receive", try_receive_type, Some(inkwell::module::Linkage::External));
    }
}
