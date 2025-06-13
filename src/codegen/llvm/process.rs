/// LLVM code generation for process management operations
/// 
/// This module provides LLVM IR generation for CURSED process management
/// and IPC operations, enabling efficient compilation of process spawning,
/// control, and inter-process communication.

use std::collections::HashMap;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::ast::expressions::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{CursedError, Result as CursedResult};

/// Process management compilation trait
pub trait ProcessCompilation {
    /// Compile process spawn operation
    fn compile_process_spawn(&mut self, command: &str, args: &[String]) -> CursedResult<LLVMValueRef>;
    
    /// Compile process control operation
    fn compile_process_control(&mut self, pid_expr: &Expression, operation: ProcessControlOp) -> CursedResult<LLVMValueRef>;
    
    /// Compile IPC channel creation
    fn compile_ipc_channel_create(&mut self, channel_type: IpcChannelType, config: &Expression) -> CursedResult<LLVMValueRef>;
    
    /// Compile IPC send operation
    fn compile_ipc_send(&mut self, channel_expr: &Expression, data_expr: &Expression) -> CursedResult<LLVMValueRef>;
    
    /// Compile IPC receive operation
    fn compile_ipc_receive(&mut self, channel_expr: &Expression, timeout_expr: Option<&Expression>) -> CursedResult<LLVMValueRef>;
    
    /// Compile shared memory operations
    fn compile_shared_memory(&mut self, operation: SharedMemoryOp, args: &[&Expression]) -> CursedResult<LLVMValueRef>;
    
    /// Compile signal operations
    fn compile_signal_operation(&mut self, operation: SignalOp, args: &[&Expression]) -> CursedResult<LLVMValueRef>;
}

/// Process control operations
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessControlOp {
    Kill,
    Terminate,
    Pause,
    Resume,
    Wait,
    GetStatus,
    SetPriority,
    GetInfo,
}

/// IPC channel types
#[derive(Debug, Clone, PartialEq)]
pub enum IpcChannelType {
    Pipe,
    NamedPipe,
    MessageQueue,
    SharedMemory,
    Socket,
    Semaphore,
}

/// Shared memory operations
#[derive(Debug, Clone, PartialEq)]
pub enum SharedMemoryOp {
    Create,
    Open,
    Map,
    Unmap,
    Read,
    Write,
    Sync,
    Lock,
    Unlock,
}

/// Signal operations
#[derive(Debug, Clone, PartialEq)]
pub enum SignalOp {
    Send,
    Register,
    Unregister,
    Block,
    Unblock,
    Wait,
}

impl ProcessCompilation for LlvmCodeGenerator {
    fn compile_process_spawn(&mut self, command: &str, args: &[String]) -> CursedResult<LLVMValueRef> {
        // Get or create the process spawn function
        let spawn_fn = self.get_or_create_process_spawn_function()?;
        
        // Create command string constant
        let command_str = self.create_string_constant(command)?;
        
        // Create args array
        let args_array = self.create_string_array_constant(args)?;
        let args_count = self.create_i32_constant(args.len() as i64);
        
        // Call the spawn function
        let mut call_args = vec![command_str, args_array, args_count];
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(spawn_fn)),
                spawn_fn,
                call_args.as_mut_ptr(),
                call_args.len() as u32,
                c_str!("spawn_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_process_control(&mut self, pid_expr: &Expression, operation: ProcessControlOp) -> CursedResult<LLVMValueRef> {
        // Compile the PID expression
        let pid_value = self.compile_expression(pid_expr)?;
        
        // Get the appropriate control function
        let control_fn = match operation {
            ProcessControlOp::Kill => self.get_or_create_process_kill_function()?,
            ProcessControlOp::Terminate => self.get_or_create_process_terminate_function()?,
            ProcessControlOp::Pause => self.get_or_create_process_pause_function()?,
            ProcessControlOp::Resume => self.get_or_create_process_resume_function()?,
            ProcessControlOp::Wait => self.get_or_create_process_wait_function()?,
            ProcessControlOp::GetStatus => self.get_or_create_process_get_status_function()?,
            ProcessControlOp::SetPriority => return Err(CursedError::CodegenError {
                message: "SetPriority requires additional priority argument".to_string(),
            }),
            ProcessControlOp::GetInfo => self.get_or_create_process_get_info_function()?,
        };
        
        // Call the control function
        let mut call_args = vec![pid_value];
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(control_fn)),
                control_fn,
                call_args.as_mut_ptr(),
                call_args.len() as u32,
                c_str!("control_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_ipc_channel_create(&mut self, channel_type: IpcChannelType, config: &Expression) -> CursedResult<LLVMValueRef> {
        // Compile configuration
        let config_value = self.compile_expression(config)?;
        
        // Get the appropriate creation function
        let create_fn = match channel_type {
            IpcChannelType::Pipe => self.get_or_create_pipe_create_function()?,
            IpcChannelType::NamedPipe => self.get_or_create_named_pipe_create_function()?,
            IpcChannelType::MessageQueue => self.get_or_create_message_queue_create_function()?,
            IpcChannelType::SharedMemory => self.get_or_create_shared_memory_create_function()?,
            IpcChannelType::Socket => self.get_or_create_socket_create_function()?,
            IpcChannelType::Semaphore => self.get_or_create_semaphore_create_function()?,
        };
        
        // Call the creation function
        let mut call_args = vec![config_value];
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(create_fn)),
                create_fn,
                call_args.as_mut_ptr(),
                call_args.len() as u32,
                c_str!("channel_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_ipc_send(&mut self, channel_expr: &Expression, data_expr: &Expression) -> CursedResult<LLVMValueRef> {
        // Compile channel and data expressions
        let channel_value = self.compile_expression(channel_expr)?;
        let data_value = self.compile_expression(data_expr)?;
        
        // Get IPC send function
        let send_fn = self.get_or_create_ipc_send_function()?;
        
        // Call the send function
        let mut call_args = vec![channel_value, data_value];
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(send_fn)),
                send_fn,
                call_args.as_mut_ptr(),
                call_args.len() as u32,
                c_str!("send_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_ipc_receive(&mut self, channel_expr: &Expression, timeout_expr: Option<&Expression>) -> CursedResult<LLVMValueRef> {
        // Compile channel expression
        let channel_value = self.compile_expression(channel_expr)?;
        
        // Compile timeout if provided
        let timeout_value = if let Some(timeout) = timeout_expr {
            self.compile_expression(timeout)?
        } else {
            // Use null/zero for no timeout
            self.create_i64_constant(0)
        };
        
        // Get IPC receive function
        let receive_fn = self.get_or_create_ipc_receive_function()?;
        
        // Call the receive function
        let mut call_args = vec![channel_value, timeout_value];
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(receive_fn)),
                receive_fn,
                call_args.as_mut_ptr(),
                call_args.len() as u32,
                c_str!("receive_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_shared_memory(&mut self, operation: SharedMemoryOp, args: &[&Expression]) -> CursedResult<LLVMValueRef> {
        // Compile all argument expressions
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.compile_expression(arg)?);
        }
        
        // Get the appropriate shared memory function
        let shm_fn = match operation {
            SharedMemoryOp::Create => self.get_or_create_shm_create_function()?,
            SharedMemoryOp::Open => self.get_or_create_shm_open_function()?,
            SharedMemoryOp::Map => self.get_or_create_shm_map_function()?,
            SharedMemoryOp::Unmap => self.get_or_create_shm_unmap_function()?,
            SharedMemoryOp::Read => self.get_or_create_shm_read_function()?,
            SharedMemoryOp::Write => self.get_or_create_shm_write_function()?,
            SharedMemoryOp::Sync => self.get_or_create_shm_sync_function()?,
            SharedMemoryOp::Lock => self.get_or_create_shm_lock_function()?,
            SharedMemoryOp::Unlock => self.get_or_create_shm_unlock_function()?,
        };
        
        // Call the shared memory function
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(shm_fn)),
                shm_fn,
                arg_values.as_mut_ptr(),
                arg_values.len() as u32,
                c_str!("shm_result").as_ptr(),
            )
        };
        
        Ok(result)
    }

    fn compile_signal_operation(&mut self, operation: SignalOp, args: &[&Expression]) -> CursedResult<LLVMValueRef> {
        // Compile all argument expressions
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.compile_expression(arg)?);
        }
        
        // Get the appropriate signal function
        let signal_fn = match operation {
            SignalOp::Send => self.get_or_create_signal_send_function()?,
            SignalOp::Register => self.get_or_create_signal_register_function()?,
            SignalOp::Unregister => self.get_or_create_signal_unregister_function()?,
            SignalOp::Block => self.get_or_create_signal_block_function()?,
            SignalOp::Unblock => self.get_or_create_signal_unblock_function()?,
            SignalOp::Wait => self.get_or_create_signal_wait_function()?,
        };
        
        // Call the signal function
        let result = unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(signal_fn)),
                signal_fn,
                arg_values.as_mut_ptr(),
                arg_values.len() as u32,
                c_str!("signal_result").as_ptr(),
            )
        };
        
        Ok(result)
    }
}

impl LlvmCodeGenerator {
    // Process management function declarations

    fn get_or_create_process_spawn_function(&mut self) -> CursedResult<LLVMValueRef> {
        let function_name = "cursed_process_spawn";
        
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: i32 spawn(i8* command, i8** args, i32 args_count)
        let string_type = self.get_string_type();
        let string_array_type = unsafe { LLVMPointerType(string_type, 0) };
        let i32_type = self.get_i32_type();
        
        let param_types = vec![string_type, string_array_type, i32_type];
        let function_type = unsafe {
            LLVMFunctionType(
                i32_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0, // not variadic
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        // Add to function registry
        self.register_function(function_name, function);
        
        Ok(function)
    }

    fn get_or_create_process_kill_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_kill")
    }

    fn get_or_create_process_terminate_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_terminate")
    }

    fn get_or_create_process_pause_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_pause")
    }

    fn get_or_create_process_resume_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_resume")
    }

    fn get_or_create_process_wait_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_wait")
    }

    fn get_or_create_process_get_status_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_simple_process_function("cursed_process_get_status")
    }

    fn get_or_create_process_get_info_function(&mut self) -> CursedResult<LLVMValueRef> {
        let function_name = "cursed_process_get_info";
        
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: ProcessInfo* get_info(i32 pid)
        let i32_type = self.get_i32_type();
        let process_info_type = self.get_or_create_process_info_type()?;
        let process_info_ptr_type = unsafe { LLVMPointerType(process_info_type, 0) };
        
        let param_types = vec![i32_type];
        let function_type = unsafe {
            LLVMFunctionType(
                process_info_ptr_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    fn get_or_create_simple_process_function(&mut self, function_name: &str) -> CursedResult<LLVMValueRef> {
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: i32 function(i32 pid)
        let i32_type = self.get_i32_type();
        let param_types = vec![i32_type];
        let function_type = unsafe {
            LLVMFunctionType(
                i32_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    // IPC function declarations

    fn get_or_create_pipe_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_pipe_create")
    }

    fn get_or_create_named_pipe_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_named_pipe_create")
    }

    fn get_or_create_message_queue_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_message_queue_create")
    }

    fn get_or_create_shared_memory_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_shared_memory_create")
    }

    fn get_or_create_socket_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_socket_create")
    }

    fn get_or_create_semaphore_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_ipc_create_function("cursed_semaphore_create")
    }

    fn get_or_create_ipc_create_function(&mut self, function_name: &str) -> CursedResult<LLVMValueRef> {
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: IpcHandle* create(IpcConfig* config)
        let config_type = self.get_or_create_ipc_config_type()?;
        let config_ptr_type = unsafe { LLVMPointerType(config_type, 0) };
        let handle_type = self.get_or_create_ipc_handle_type()?;
        let handle_ptr_type = unsafe { LLVMPointerType(handle_type, 0) };
        
        let param_types = vec![config_ptr_type];
        let function_type = unsafe {
            LLVMFunctionType(
                handle_ptr_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    fn get_or_create_ipc_send_function(&mut self) -> CursedResult<LLVMValueRef> {
        let function_name = "cursed_ipc_send";
        
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: i32 send(IpcHandle* channel, void* data)
        let handle_type = self.get_or_create_ipc_handle_type()?;
        let handle_ptr_type = unsafe { LLVMPointerType(handle_type, 0) };
        let void_ptr_type = self.get_void_ptr_type();
        let i32_type = self.get_i32_type();
        
        let param_types = vec![handle_ptr_type, void_ptr_type];
        let function_type = unsafe {
            LLVMFunctionType(
                i32_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    fn get_or_create_ipc_receive_function(&mut self) -> CursedResult<LLVMValueRef> {
        let function_name = "cursed_ipc_receive";
        
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        // Function signature: void* receive(IpcHandle* channel, i64 timeout)
        let handle_type = self.get_or_create_ipc_handle_type()?;
        let handle_ptr_type = unsafe { LLVMPointerType(handle_type, 0) };
        let i64_type = self.get_i64_type();
        let void_ptr_type = self.get_void_ptr_type();
        
        let param_types = vec![handle_ptr_type, i64_type];
        let function_type = unsafe {
            LLVMFunctionType(
                void_ptr_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    // Shared memory function declarations

    fn get_or_create_shm_create_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_create", vec![], self.get_void_ptr_type())
    }

    fn get_or_create_shm_open_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_open", vec![self.get_string_type()], self.get_void_ptr_type())
    }

    fn get_or_create_shm_map_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_map", vec![self.get_void_ptr_type()], self.get_void_ptr_type())
    }

    fn get_or_create_shm_unmap_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_unmap", vec![self.get_void_ptr_type()], self.get_i32_type())
    }

    fn get_or_create_shm_read_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![
            self.get_void_ptr_type(), // shm handle
            self.get_i64_type(),      // offset
            self.get_i64_type(),      // length
        ];
        self.get_or_create_shm_function("cursed_shm_read", param_types, self.get_void_ptr_type())
    }

    fn get_or_create_shm_write_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![
            self.get_void_ptr_type(), // shm handle
            self.get_i64_type(),      // offset
            self.get_void_ptr_type(), // data
            self.get_i64_type(),      // length
        ];
        self.get_or_create_shm_function("cursed_shm_write", param_types, self.get_i32_type())
    }

    fn get_or_create_shm_sync_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_sync", vec![self.get_void_ptr_type()], self.get_i32_type())
    }

    fn get_or_create_shm_lock_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_lock", vec![self.get_void_ptr_type()], self.get_i32_type())
    }

    fn get_or_create_shm_unlock_function(&mut self) -> CursedResult<LLVMValueRef> {
        self.get_or_create_shm_function("cursed_shm_unlock", vec![self.get_void_ptr_type()], self.get_i32_type())
    }

    fn get_or_create_shm_function(&mut self, function_name: &str, param_types: Vec<LLVMTypeRef>, return_type: LLVMTypeRef) -> CursedResult<LLVMValueRef> {
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        let function_type = unsafe {
            LLVMFunctionType(
                return_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    // Signal function declarations

    fn get_or_create_signal_send_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![
            self.get_i32_type(), // pid
            self.get_i32_type(), // signal
        ];
        self.get_or_create_signal_function("cursed_signal_send", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_register_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![
            self.get_i32_type(),      // signal
            self.get_void_ptr_type(), // handler function pointer
        ];
        self.get_or_create_signal_function("cursed_signal_register", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_unregister_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![self.get_i32_type()]; // signal
        self.get_or_create_signal_function("cursed_signal_unregister", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_block_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![self.get_i32_type()]; // signal
        self.get_or_create_signal_function("cursed_signal_block", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_unblock_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![self.get_i32_type()]; // signal
        self.get_or_create_signal_function("cursed_signal_unblock", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_wait_function(&mut self) -> CursedResult<LLVMValueRef> {
        let param_types = vec![
            self.get_i32_type(), // signal
            self.get_i64_type(), // timeout
        ];
        self.get_or_create_signal_function("cursed_signal_wait", param_types, self.get_i32_type())
    }

    fn get_or_create_signal_function(&mut self, function_name: &str, param_types: Vec<LLVMTypeRef>, return_type: LLVMTypeRef) -> CursedResult<LLVMValueRef> {
        if let Some(existing) = self.get_function(function_name) {
            return Ok(existing);
        }
        
        let function_type = unsafe {
            LLVMFunctionType(
                return_type,
                param_types.as_ptr() as *mut _,
                param_types.len() as u32,
                0,
            )
        };
        
        let function = unsafe {
            LLVMAddFunction(self.module, c_str!(function_name).as_ptr(), function_type)
        };
        
        self.register_function(function_name, function);
        Ok(function)
    }

    // Type creation helpers

    fn get_or_create_process_info_type(&mut self) -> CursedResult<LLVMTypeRef> {
        let type_name = "ProcessInfo";
        
        if let Some(existing) = self.get_struct_type(type_name) {
            return Ok(existing);
        }
        
        // Create ProcessInfo struct type
        let i32_type = self.get_i32_type();
        let i64_type = self.get_i64_type();
        let string_type = self.get_string_type();
        
        let member_types = vec![
            i32_type,    // pid
            string_type, // command
            i32_type,    // status
            i64_type,    // start_time
            i32_type,    // parent_pid
            i64_type,    // memory_usage
            i64_type,    // cpu_time
        ];
        
        let struct_type = unsafe {
            LLVMStructCreateNamed(self.context, c_str!(type_name).as_ptr())
        };
        
        unsafe {
            LLVMStructSetBody(
                struct_type,
                member_types.as_ptr() as *mut _,
                member_types.len() as u32,
                0, // not packed
            );
        }
        
        self.register_struct_type(type_name, struct_type);
        Ok(struct_type)
    }

    fn get_or_create_ipc_handle_type(&mut self) -> CursedResult<LLVMTypeRef> {
        let type_name = "IpcHandle";
        
        if let Some(existing) = self.get_struct_type(type_name) {
            return Ok(existing);
        }
        
        // Create IpcHandle struct type
        let i32_type = self.get_i32_type();
        let i64_type = self.get_i64_type();
        let void_ptr_type = self.get_void_ptr_type();
        
        let member_types = vec![
            i32_type,      // handle_type
            i64_type,      // handle_id
            void_ptr_type, // handle_data
            i32_type,      // permissions
        ];
        
        let struct_type = unsafe {
            LLVMStructCreateNamed(self.context, c_str!(type_name).as_ptr())
        };
        
        unsafe {
            LLVMStructSetBody(
                struct_type,
                member_types.as_ptr() as *mut _,
                member_types.len() as u32,
                0,
            );
        }
        
        self.register_struct_type(type_name, struct_type);
        Ok(struct_type)
    }

    fn get_or_create_ipc_config_type(&mut self) -> CursedResult<LLVMTypeRef> {
        let type_name = "IpcConfig";
        
        if let Some(existing) = self.get_struct_type(type_name) {
            return Ok(existing);
        }
        
        // Create IpcConfig struct type
        let string_type = self.get_string_type();
        let i32_type = self.get_i32_type();
        let i64_type = self.get_i64_type();
        
        let member_types = vec![
            string_type, // name
            i32_type,    // config_type
            i64_type,    // size_or_capacity
            i32_type,    // permissions
            i32_type,    // flags
        ];
        
        let struct_type = unsafe {
            LLVMStructCreateNamed(self.context, c_str!(type_name).as_ptr())
        };
        
        unsafe {
            LLVMStructSetBody(
                struct_type,
                member_types.as_ptr() as *mut _,
                member_types.len() as u32,
                0,
            );
        }
        
        self.register_struct_type(type_name, struct_type);
        Ok(struct_type)
    }

    // Helper functions for constants and arrays

    fn create_string_constant(&mut self, s: &str) -> CursedResult<LLVMValueRef> {
        let c_string = std::ffi::CString::new(s)
            .map_err(|e| CursedError::CodegenError {
                message: format!("Invalid string for constant: {}", e),
            })?;
        
        let string_constant = unsafe {
            LLVMBuildGlobalStringPtr(
                self.builder,
                c_string.as_ptr(),
                c_str!("string_const").as_ptr(),
            )
        };
        
        Ok(string_constant)
    }

    fn create_string_array_constant(&mut self, strings: &[String]) -> CursedResult<LLVMValueRef> {
        let string_type = self.get_string_type();
        let array_type = unsafe { LLVMArrayType(string_type, strings.len() as u32) };
        
        let mut string_constants = Vec::new();
        for s in strings {
            string_constants.push(self.create_string_constant(s)?);
        }
        
        let array_constant = unsafe {
            LLVMConstArray(
                string_type,
                string_constants.as_mut_ptr(),
                string_constants.len() as u32,
            )
        };
        
        // Create global variable for the array
        let global_array = unsafe {
            LLVMAddGlobal(self.module, array_type, c_str!("string_array").as_ptr())
        };
        
        unsafe {
            LLVMSetInitializer(global_array, array_constant);
            LLVMSetGlobalConstant(global_array, 1);
        }
        
        // Return pointer to the array
        let array_ptr = unsafe {
            LLVMBuildBitCast(
                self.builder,
                global_array,
                unsafe { LLVMPointerType(string_type, 0) },
                c_str!("array_ptr").as_ptr(),
            )
        };
        
        Ok(array_ptr)
    }

    fn create_i32_constant(&mut self, value: i64) -> LLVMValueRef {
        unsafe { LLVMConstInt(self.get_i32_type(), value as u64, 0) }
    }

    fn create_i64_constant(&mut self, value: i64) -> LLVMValueRef {
        unsafe { LLVMConstInt(self.get_i64_type(), value as u64, 0) }
    }
}

// Macro for creating C string literals
macro_rules! c_str {
    ($s:expr) => {
        std::ffi::CString::new($s).unwrap()
    };
}

// Declare the macro for internal use
pub(crate) use c_str;
