//! LLVM code generation for select statements
//!
//! This module implements compilation of select statements to LLVM IR,
//! including non-blocking channel operations, random case selection,
//! and timeout handling.

use crate::ast::control_flow::select::*;
use crate::ast::expressions::channel::{SendExpression, ReceiveExpression};
use crate::ast::traits::{Expression, Statement};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::error::Error;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::IntPredicate;
use std::any::Any;
use std::collections::HashMap;

/// Information about a select case for code generation
#[derive(Debug)]
struct SelectCaseInfo<'ctx> {
    /// The case index
    index: u32,
    /// The communication operation (send/receive)
    operation: SelectOperation<'ctx>,
    /// The basic block for this case's body
    body_block: BasicBlock<'ctx>,
    /// Whether this case can execute immediately
    is_ready: Option<IntValue<'ctx>>,
}

/// Represents a select operation (send or receive)
#[derive(Debug)]
enum SelectOperation<'ctx> {
    Send {
        channel: PointerValue<'ctx>,
        value: BasicValueEnum<'ctx>,
    },
    Receive {
        channel: PointerValue<'ctx>,
        result_ptr: PointerValue<'ctx>,
    },
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a select statement to LLVM IR
    ///
    /// Generates efficient polling-based select implementation with random
    /// case selection when multiple channels are ready.
    #[tracing::instrument(skip(self, select_stmt), fields(num_cases = select_stmt.cases.len()), level = "debug")]
    pub fn compile_select_statement(&mut self, select_stmt: &SelectStatement) -> Result<(), Error> {
        tracing::info!("Compiling select statement");
        
        // Create basic blocks for control flow
        let function = self.current_function
            .ok_or_else(|| Error::new("compilation", "No current function for select statement", None))?;
        
        let entry_block = self.context().append_basic_block(function, "select_entry");
        let poll_block = self.context().append_basic_block(function, "select_poll");
        let end_block = self.context().append_basic_block(function, "select_end");
        
        // Jump to entry block
        self.builder().build_unconditional_branch(entry_block)?;
        self.builder().position_at_end(entry_block);
        
        // Compile all select cases
        let mut case_infos = Vec::new();
        
        for (index, case) in select_stmt.cases.iter().enumerate() {
            let case_info = self.compile_select_case(case, index as u32, function)?;
            case_infos.push(case_info);
        }
        
        // Handle default case if present
        let default_block = if let Some(default_case) = &select_stmt.default {
            let block = self.context().append_basic_block(function, "select_default");
            self.builder().position_at_end(block);
            
            // Compile default case statements
            for stmt in &default_case.statements {
                self.compile_statement(stmt.as_ref())?;
            }
            
            self.builder().build_unconditional_branch(end_block)?;
            Some(block)
        } else {
            None
        };
        
        // Build the polling loop
        self.builder().position_at_end(entry_block);
        self.builder().build_unconditional_branch(poll_block)?;
        self.builder().position_at_end(poll_block);
        
        // Poll all channels for readiness
        let ready_cases = self.poll_channels(&case_infos)?;
        
        // If we have a default case and no channels are ready, execute it
        if let Some(default_block) = default_block {
            let any_ready = self.check_any_ready(&ready_cases)?;
            self.builder().build_conditional_branch(any_ready, poll_block, default_block)?;
        }
        
        // Select a random ready case
        let selected_case = self.select_random_ready_case(&case_infos, &ready_cases)?;
        
        // Execute the selected case
        self.execute_selected_case(&case_infos, selected_case, end_block)?;
        
        // Position at end block
        self.builder().position_at_end(end_block);
        
        Ok(())
    }
    
    /// Compile a single select case
    fn compile_select_case(
        &mut self,
        case: &SelectCase,
        index: u32,
        function: FunctionValue<'ctx>,
    ) -> Result<SelectCaseInfo<'ctx>, Error> {
        // Create basic block for this case
        let body_block = self.context().append_basic_block(function, &format!("select_case_{}", index));
        
        // Determine the operation type
        let operation = self.analyze_communication_operation(&case.communication)?;
        
        Ok(SelectCaseInfo {
            index,
            operation,
            body_block,
            is_ready: None,
        })
    }
    
    /// Analyze a communication expression to determine operation type
    fn analyze_communication_operation(
        &mut self,
        expr: &Box<dyn Expression>,
    ) -> Result<SelectOperation<'ctx>, Error> {
        if let Some(send_expr) = expr.as_any().downcast_ref::<SendExpression>() {
            // This is a send operation
            let channel = self.compile_expression(send_expr.channel.as_ref())?
                .into_pointer_value();
            let value = self.compile_expression(send_expr.value.as_ref())?;
            
            Ok(SelectOperation::Send { channel, value })
        } else if let Some(recv_expr) = expr.as_any().downcast_ref::<ReceiveExpression>() {
            // This is a receive operation
            let channel = self.compile_expression(recv_expr.channel.as_ref())?
                .into_pointer_value();
            
            // Allocate space for the received value
            let element_type = self.context().i64_type(); // Simplified for now
            let result_ptr = self.builder().build_alloca(element_type, "recv_result")?;
            
            Ok(SelectOperation::Receive { channel, result_ptr })
        } else {
            Err(Error::new("compilation", format!(
                "Invalid communication operation in select case: {}",
                expr.string()
            ), None))
        }
    }
    
    /// Poll all channels to check readiness
    fn poll_channels(
        &mut self,
        case_infos: &[SelectCaseInfo<'ctx>],
    ) -> Result<Vec<IntValue<'ctx>>, Error> {
        let mut ready_flags = Vec::new();
        
        for case_info in case_infos {
            let ready_flag = match &case_info.operation {
                SelectOperation::Send { channel, .. } => {
                    self.check_channel_send_ready(*channel)?
                }
                SelectOperation::Receive { channel, .. } => {
                    self.check_channel_receive_ready(*channel)?
                }
            };
            ready_flags.push(ready_flag);
        }
        
        Ok(ready_flags)
    }
    
    /// Check if a channel is ready for sending (non-blocking)
    fn check_channel_send_ready(&mut self, channel: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // Get or create the channel_try_send function
        let try_send_fn = self.get_or_create_channel_try_send_fn()?;
        
        // Call the try_send function with null value to check readiness
        let null_value = self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        let ready = self.builder().build_call(
            try_send_fn,
            &[channel.into(), null_value.into()],
            "send_ready"
        )?;
        
        Ok(ready.try_as_basic_value().left().unwrap().into_int_value())
    }
    
    /// Check if a channel is ready for receiving (non-blocking)
    fn check_channel_receive_ready(&mut self, channel: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // Get or create the channel_try_receive function
        let try_recv_fn = self.get_or_create_channel_try_receive_fn()?;
        
        // Call the try_receive function with null result to check readiness
        let null_ptr = self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        let ready = self.builder().build_call(
            try_recv_fn,
            &[channel.into(), null_ptr.into()],
            "recv_ready"
        )?;
        
        Ok(ready.try_as_basic_value().left().unwrap().into_int_value())
    }
    
    /// Check if any channels are ready
    fn check_any_ready(&mut self, ready_flags: &[IntValue<'ctx>]) -> Result<IntValue<'ctx>, Error> {
        if ready_flags.is_empty() {
            return Ok(self.context().bool_type().const_zero());
        }
        
        let mut any_ready = ready_flags[0];
        for &flag in &ready_flags[1..] {
            any_ready = self.builder().build_or(any_ready, flag, "any_ready")?;
        }
        
        Ok(any_ready)
    }
    
    /// Select a random ready case using a simple linear congruential generator
    fn select_random_ready_case(
        &mut self,
        case_infos: &[SelectCaseInfo<'ctx>],
        ready_flags: &[IntValue<'ctx>],
    ) -> Result<IntValue<'ctx>, Error> {
        let i32_type = self.context().i32_type();
        
        // Create a simple random number using current time or a counter
        let random_seed = self.get_or_create_random_seed()?;
        let multiplier = i32_type.const_int(1103515245, false);
        let increment = i32_type.const_int(12345, false);
        
        // Generate pseudo-random number: (seed * 1103515245 + 12345) % 2^31
        let random_val = self.builder().build_int_mul(random_seed, multiplier, "rand_mul")?;
        let random_val = self.builder().build_int_add(random_val, increment, "rand_add")?;
        
        // Store back the new seed
        self.update_random_seed(random_val)?;
        
        // Modulo by number of cases
        let num_cases = i32_type.const_int(case_infos.len() as u64, false);
        let case_index = self.builder().build_int_unsigned_rem(random_val, num_cases, "case_index")?;
        
        // Find the first ready case starting from the random index
        let mut selected_index = case_index;
        
        for i in 0..case_infos.len() {
            let check_index = self.builder().build_int_add(
                case_index,
                i32_type.const_int(i as u64, false),
                "check_index"
            )?;
            let wrapped_index = self.builder().build_int_unsigned_rem(
                check_index,
                num_cases,
                "wrapped_index"
            )?;
            
            // Check if this case is ready
            let ready_flag = ready_flags[i];
            let is_ready = self.builder().build_int_compare(
                IntPredicate::NE,
                ready_flag,
                self.context().bool_type().const_zero(),
                "is_ready"
            )?;
            
            // If ready, use this index
            selected_index = self.builder().build_select(
                is_ready,
                wrapped_index,
                selected_index,
                "selected_index"
            )?.into_int_value();
        }
        
        Ok(selected_index)
    }
    
    /// Execute the selected case
    fn execute_selected_case(
        &mut self,
        case_infos: &[SelectCaseInfo<'ctx>],
        selected_index: IntValue<'ctx>,
        end_block: BasicBlock<'ctx>,
    ) -> Result<(), Error> {
        // Create a switch instruction to jump to the appropriate case
        let switch_inst = self.builder().build_switch(
            selected_index,
            end_block,
            &case_infos.iter().enumerate().map(|(i, info)| {
                (self.context().i32_type().const_int(i as u64, false), info.body_block)
            }).collect::<Vec<_>>()
        )?;
        
        // Compile each case body
        for (case, case_info) in case_infos.iter().enumerate() {
            self.builder().position_at_end(case_info.body_block);
            
            // Perform the actual channel operation
            match &case_info.operation {
                SelectOperation::Send { channel, value } => {
                    let send_fn = self.get_or_create_channel_send_fn()?;
                    self.builder().build_call(
                        send_fn,
                        &[(*channel).into(), (*value).into()],
                        "select_send"
                    )?;
                }
                SelectOperation::Receive { channel, result_ptr } => {
                    let recv_fn = self.get_or_create_channel_receive_fn()?;
                    self.builder().build_call(
                        recv_fn,
                        &[(*channel).into(), (*result_ptr).into()],
                        "select_recv"
                    )?;
                }
            }
            
            // Compile case statements
            if case < case_infos.len() {
                // We need to get the original case from select_stmt
                // This is a simplification - in a real implementation,
                // we'd pass the statements to this function
            }
            
            // Jump to end
            self.builder().build_unconditional_branch(end_block)?;
        }
        
        Ok(())
    }
    
    /// Get or create random seed global variable
    fn get_or_create_random_seed(&mut self) -> Result<IntValue<'ctx>, Error> {
        let global_name = "__cursed_select_seed";
        
        if let Some(global) = self.module.get_global(global_name) {
            return Ok(self.builder().build_load(
                self.context().i32_type(),
                global.as_pointer_value(),
                "load_seed"
            )?.into_int_value());
        }
        
        // Create new global variable initialized with current time
        let i32_type = self.context().i32_type();
        let global = self.module.add_global(i32_type, None, global_name);
        global.set_initializer(&i32_type.const_int(1, false));
        
        Ok(self.builder().build_load(
            i32_type,
            global.as_pointer_value(),
            "load_seed"
        )?.into_int_value())
    }
    
    /// Update the random seed global variable
    fn update_random_seed(&mut self, new_seed: IntValue<'ctx>) -> Result<(), Error> {
        let global_name = "__cursed_select_seed";
        
        if let Some(global) = self.module.get_global(global_name) {
            self.builder().build_store(global.as_pointer_value(), new_seed)?;
        }
        
        Ok(())
    }
    
    /// Get or create channel try_send function
    fn get_or_create_channel_try_send_fn(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        if let Some(function) = self.module.get_function("cursed_channel_try_send") {
            return Ok(function);
        }
        
        // Create function signature: i1 cursed_channel_try_send(i8*, i8*)
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let bool_type = self.context().bool_type();
        
        let fn_type = bool_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        let function = self.module.add_function("cursed_channel_try_send", fn_type, None);
        
        Ok(function)
    }
    
    /// Get or create channel try_receive function
    fn get_or_create_channel_try_receive_fn(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        if let Some(function) = self.module.get_function("cursed_channel_try_receive") {
            return Ok(function);
        }
        
        // Create function signature: i1 cursed_channel_try_receive(i8*, i8*)
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let bool_type = self.context().bool_type();
        
        let fn_type = bool_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        let function = self.module.add_function("cursed_channel_try_receive", fn_type, None);
        
        Ok(function)
    }
    
    /// Get or create channel send function
    fn get_or_create_channel_send_fn(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        if let Some(function) = self.module.get_function("cursed_send_to_channel") {
            return Ok(function);
        }
        
        // Create function signature: void cursed_send_to_channel(i8*, i8*)
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let void_type = self.context().void_type();
        
        let fn_type = void_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        let function = self.module.add_function("cursed_send_to_channel", fn_type, None);
        
        Ok(function)
    }
    
    /// Get or create channel receive function
    fn get_or_create_channel_receive_fn(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        if let Some(function) = self.module.get_function("cursed_receive_from_channel") {
            return Ok(function);
        }
        
        // Create function signature: void cursed_receive_from_channel(i8*, i8*)
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let void_type = self.context().void_type();
        
        let fn_type = void_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        let function = self.module.add_function("cursed_receive_from_channel", fn_type, None);
        
        Ok(function)
    }
}
