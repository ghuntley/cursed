//! LLVM Integration for Process Management and IPC Systems
//! 
//! This module provides LLVM code generation support for:
//! - Process spawning and management from compiled CURSED code
//! - IPC operations with proper memory management
//! - Integration with the existing LLVM infrastructure
//! - FFI functions for runtime process/IPC operations

use std::collections::HashMap;
use std::ffi::CString;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, PointerValue, IntValue, BasicValueEnum, BasicValue};
use inkwell::crate::types::{BasicTypeEnum, IntType, PointerType, StructType, FunctionType};
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use crate::error::CursedError;

/// LLVM integration for process management and IPC
pub struct ProcessIpcLlvmIntegration<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    
    // Type definitions
    string_type: PointerType<'ctx>,
    i32_type: IntType<'ctx>,
    i64_type: IntType<'ctx>,
    void_type: PointerType<'ctx>,
    
    // Process management types
    process_handle_type: StructType<'ctx>,
    process_options_type: StructType<'ctx>,
    process_stats_type: StructType<'ctx>,
    
    // IPC types
    ipc_message_type: StructType<'ctx>,
    shared_memory_type: StructType<'ctx>,
    message_queue_type: StructType<'ctx>,
    
    // Function cache
    function_cache: HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> ProcessIpcLlvmIntegration<'ctx> {
    /// Create a new LLVM integration instance
    pub fn new(
        context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx Builder<'ctx>,
    ) -> Result<(), Error> {
        let string_type = context.i8_type().ptr_type(AddressSpace::Generic);
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let void_type = context.i8_type().ptr_type(AddressSpace::Generic);
        
        // Define process management types
        let process_handle_type = context.struct_type(&[
            i32_type.into(),           // pid
            i64_type.into(),           // start_time
            i32_type.into(),           // status
            void_type.into(),          // internal_handle
        ], false);
        
        let process_options_type = context.struct_type(&[
            string_type.into(),        // working_directory
            string_type.into(),        // environment_vars
            i32_type.into(),           // timeout_seconds
            i32_type.into(),           // priority
            i32_type.into(),           // memory_limit_mb
            i32_type.into(),           // cpu_limit_percent
        ], false);
        
        let process_stats_type = context.struct_type(&[
            i32_type.into(),           // pid
            i64_type.into(),           // memory_usage
            i32_type.into(),           // cpu_usage_percent
            i64_type.into(),           // uptime_seconds
            i32_type.into(),           // thread_count
        ], false);
        
        // Define IPC types
        let ipc_message_type = context.struct_type(&[
            string_type.into(),        // message_id
            void_type.into(),          // data
            i32_type.into(),           // data_size
            i32_type.into(),           // priority
            i64_type.into(),           // timestamp
        ], false);
        
        let shared_memory_type = context.struct_type(&[
            string_type.into(),        // segment_id
            void_type.into(),          // memory_ptr
            i32_type.into(),           // size
            i32_type.into(),           // permissions
        ], false);
        
        let message_queue_type = context.struct_type(&[
            string_type.into(),        // queue_id
            void_type.into(),          // queue_handle
            i32_type.into(),           // max_capacity
            i32_type.into(),           // current_size
        ], false);
        
        let mut integration = Self {
            context,
            module,
            builder,
            string_type,
            i32_type,
            i64_type,
            void_type,
            process_handle_type,
            process_options_type,
            process_stats_type,
            ipc_message_type,
            shared_memory_type,
            message_queue_type,
            function_cache: HashMap::new(),
        };
        
        // Declare external functions
        integration.declare_external_functions()?;
        
        Ok(integration)
    }
    
    /// Declare external FFI functions for process and IPC operations
    fn declare_external_functions(&mut self) -> Result<(), Error> {
        // Process management functions
        self.declare_function(
            "cursed_spawn_process",
            self.context.i32_type().fn_type(&[
                self.string_type.into(),           // command
                self.string_type.into(),           // args
                self.process_options_type.ptr_type(AddressSpace::Generic).into(), // options
                self.process_handle_type.ptr_type(AddressSpace::Generic).into(),  // result
            ], false),
        )?;
        
        self.declare_function(
            "cursed_wait_process",
            self.context.i32_type().fn_type(&[
                self.process_handle_type.ptr_type(AddressSpace::Generic).into(),
                self.i32_type.into(),              // timeout_seconds
            ], false),
        )?;
        
        self.declare_function(
            "cursed_kill_process",
            self.context.i32_type().fn_type(&[
                self.process_handle_type.ptr_type(AddressSpace::Generic).into(),
                self.i32_type.into(),              // signal
            ], false),
        )?;
        
        self.declare_function(
            "cursed_get_process_stats",
            self.context.i32_type().fn_type(&[
                self.process_handle_type.ptr_type(AddressSpace::Generic).into(),
                self.process_stats_type.ptr_type(AddressSpace::Generic).into(),
            ], false),
        )?;
        
        // IPC functions
        self.declare_function(
            "cursed_create_shared_memory",
            self.context.i32_type().fn_type(&[
                self.string_type.into(),           // segment_id
                self.i32_type.into(),              // size
                self.shared_memory_type.ptr_type(AddressSpace::Generic).into(),
            ], false),
        )?;
        
        self.declare_function(
            "cursed_write_shared_memory",
            self.context.i32_type().fn_type(&[
                self.shared_memory_type.ptr_type(AddressSpace::Generic).into(),
                self.i32_type.into(),              // offset
                self.void_type.into(),             // data
                self.i32_type.into(),              // size
            ], false),
        )?;
        
        self.declare_function(
            "cursed_read_shared_memory",
            self.context.i32_type().fn_type(&[
                self.shared_memory_type.ptr_type(AddressSpace::Generic).into(),
                self.i32_type.into(),              // offset
                self.void_type.into(),             // buffer
                self.i32_type.into(),              // size
            ], false),
        )?;
        
        self.declare_function(
            "cursed_create_message_queue",
            self.context.i32_type().fn_type(&[
                self.string_type.into(),           // queue_id
                self.i32_type.into(),              // capacity
                self.message_queue_type.ptr_type(AddressSpace::Generic).into(),
            ], false),
        )?;
        
        self.declare_function(
            "cursed_send_message",
            self.context.i32_type().fn_type(&[
                self.message_queue_type.ptr_type(AddressSpace::Generic).into(),
                self.ipc_message_type.ptr_type(AddressSpace::Generic).into(),
            ], false),
        )?;
        
        self.declare_function(
            "cursed_receive_message",
            self.context.i32_type().fn_type(&[
                self.message_queue_type.ptr_type(AddressSpace::Generic).into(),
                self.ipc_message_type.ptr_type(AddressSpace::Generic).into(),
                self.i32_type.into(),              // timeout_ms
            ], false),
        )?;
        
        Ok(())
    }
    
    /// Declare a function in the module
    fn declare_function(&mut self, name: &str, fn_type: FunctionType<'ctx>) -> Result<(), Error> {
        let function = self.module.add_function(name, fn_type, None);
        self.function_cache.insert(name.to_string(), function);
        Ok(())
    }
    
    /// Get a declared function
    fn get_function(&self, name: &str) -> Result<(), Error> {
        self.function_cache.get(name)
            .copied()
            .ok_or_else(|| CursedError::CodegenError(format!("Function {} not found", name)))
    }
    
    /// Compile process spawning
    pub fn compile_spawn_process(
        &self,
        command: PointerValue<'ctx>,
        args: PointerValue<'ctx>,
        options: Option<PointerValue<'ctx>>,
    ) -> Result<(), Error> {
        let spawn_fn = self.get_function("cursed_spawn_process")?;
        
        // Allocate process handle
        let process_handle = self.builder.build_alloca(self.process_handle_type, "process_handle")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate process handle: {}", e)))?;
        
        // Prepare options (use default if None)
        let options_ptr = if let Some(opts) = options {
            opts
        } else {
            let default_options = self.builder.build_alloca(self.process_options_type, "default_options")
                .map_err(|e| CursedError::CodegenError(format!("Failed to allocate default options: {}", e)))?;
            
            // Initialize default options
            self.initialize_default_process_options(default_options)?;
            default_options
        };
        
        // Call spawn function
        let result = self.builder.build_call(
            spawn_fn,
            &[
                command.into(),
                args.into(),
                options_ptr.into(),
                process_handle.into(),
            ],
            "spawn_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call spawn function: {}", e)))?;
        
        // Check result
        let success = result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::CodegenError("Spawn function returned void".to_string()))?;
        
        // TODO: Add error handling based on result value
        
        Ok(process_handle)
    }
    
    /// Compile process waiting
    pub fn compile_wait_process(
        &self,
        process_handle: PointerValue<'ctx>,
        timeout: Option<IntValue<'ctx>>,
    ) -> Result<(), Error> {
        let wait_fn = self.get_function("cursed_wait_process")?;
        
        let timeout_val = timeout.unwrap_or_else(|| self.i32_type.const_int(0, false));
        
        let result = self.builder.build_call(
            wait_fn,
            &[process_handle.into(), timeout_val.into()],
            "wait_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call wait function: {}", e)))?;
        
        result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value().ok())
            .ok_or_else(|| CursedError::CodegenError("Wait function returned unexpected type".to_string()))
    }
    
    /// Compile process killing
    pub fn compile_kill_process(
        &self,
        process_handle: PointerValue<'ctx>,
        signal: Option<IntValue<'ctx>>,
    ) -> Result<(), Error> {
        let kill_fn = self.get_function("cursed_kill_process")?;
        
        let signal_val = signal.unwrap_or_else(|| self.i32_type.const_int(15, false)); // SIGTERM
        
        let result = self.builder.build_call(
            kill_fn,
            &[process_handle.into(), signal_val.into()],
            "kill_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call kill function: {}", e)))?;
        
        result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value().ok())
            .ok_or_else(|| CursedError::CodegenError("Kill function returned unexpected type".to_string()))
    }
    
    /// Compile getting process statistics
    pub fn compile_get_process_stats(
        &self,
        process_handle: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        let stats_fn = self.get_function("cursed_get_process_stats")?;
        
        // Allocate stats structure
        let stats = self.builder.build_alloca(self.process_stats_type, "process_stats")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate process stats: {}", e)))?;
        
        let result = self.builder.build_call(
            stats_fn,
            &[process_handle.into(), stats.into()],
            "stats_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call stats function: {}", e)))?;
        
        // TODO: Add error handling based on result value
        
        Ok(stats)
    }
    
    /// Compile shared memory creation
    pub fn compile_create_shared_memory(
        &self,
        segment_id: PointerValue<'ctx>,
        size: IntValue<'ctx>,
    ) -> Result<(), Error> {
        let create_fn = self.get_function("cursed_create_shared_memory")?;
        
        // Allocate shared memory structure
        let shm = self.builder.build_alloca(self.shared_memory_type, "shared_memory")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate shared memory: {}", e)))?;
        
        let result = self.builder.build_call(
            create_fn,
            &[segment_id.into(), size.into(), shm.into()],
            "create_shm_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call create shared memory: {}", e)))?;
        
        // TODO: Add error handling based on result value
        
        Ok(shm)
    }
    
    /// Compile shared memory write
    pub fn compile_write_shared_memory(
        &self,
        shm: PointerValue<'ctx>,
        offset: IntValue<'ctx>,
        data: PointerValue<'ctx>,
        size: IntValue<'ctx>,
    ) -> Result<(), Error> {
        let write_fn = self.get_function("cursed_write_shared_memory")?;
        
        let result = self.builder.build_call(
            write_fn,
            &[shm.into(), offset.into(), data.into(), size.into()],
            "write_shm_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call write shared memory: {}", e)))?;
        
        result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value().ok())
            .ok_or_else(|| CursedError::CodegenError("Write shared memory returned unexpected type".to_string()))
    }
    
    /// Compile shared memory read
    pub fn compile_read_shared_memory(
        &self,
        shm: PointerValue<'ctx>,
        offset: IntValue<'ctx>,
        buffer: PointerValue<'ctx>,
        size: IntValue<'ctx>,
    ) -> Result<(), Error> {
        let read_fn = self.get_function("cursed_read_shared_memory")?;
        
        let result = self.builder.build_call(
            read_fn,
            &[shm.into(), offset.into(), buffer.into(), size.into()],
            "read_shm_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call read shared memory: {}", e)))?;
        
        result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value().ok())
            .ok_or_else(|| CursedError::CodegenError("Read shared memory returned unexpected type".to_string()))
    }
    
    /// Compile message queue creation
    pub fn compile_create_message_queue(
        &self,
        queue_id: PointerValue<'ctx>,
        capacity: IntValue<'ctx>,
    ) -> Result<(), Error> {
        let create_fn = self.get_function("cursed_create_message_queue")?;
        
        // Allocate message queue structure
        let queue = self.builder.build_alloca(self.message_queue_type, "message_queue")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate message queue: {}", e)))?;
        
        let result = self.builder.build_call(
            create_fn,
            &[queue_id.into(), capacity.into(), queue.into()],
            "create_queue_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call create message queue: {}", e)))?;
        
        // TODO: Add error handling based on result value
        
        Ok(queue)
    }
    
    /// Compile message sending
    pub fn compile_send_message(
        &self,
        queue: PointerValue<'ctx>,
        message: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        let send_fn = self.get_function("cursed_send_message")?;
        
        let result = self.builder.build_call(
            send_fn,
            &[queue.into(), message.into()],
            "send_message_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call send message: {}", e)))?;
        
        result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value().ok())
            .ok_or_else(|| CursedError::CodegenError("Send message returned unexpected type".to_string()))
    }
    
    /// Compile message receiving
    pub fn compile_receive_message(
        &self,
        queue: PointerValue<'ctx>,
        timeout: Option<IntValue<'ctx>>,
    ) -> Result<(), Error> {
        let receive_fn = self.get_function("cursed_receive_message")?;
        
        // Allocate message structure
        let message = self.builder.build_alloca(self.ipc_message_type, "received_message")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate message: {}", e)))?;
        
        let timeout_val = timeout.unwrap_or_else(|| self.i32_type.const_int(0, false));
        
        let result = self.builder.build_call(
            receive_fn,
            &[queue.into(), message.into(), timeout_val.into()],
            "receive_message_result",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to call receive message: {}", e)))?;
        
        // TODO: Add error handling based on result value
        
        Ok(message)
    }
    
    /// Initialize default process options
    fn initialize_default_process_options(
        &self,
        options: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        // Set default values for process options
        let null_ptr = self.string_type.const_null();
        let zero_i32 = self.i32_type.const_int(0, false);
        let default_timeout = self.i32_type.const_int(30, false); // 30 seconds
        let normal_priority = self.i32_type.const_int(0, false);
        
        // working_directory = null
        let wd_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            0,
            "wd_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for working directory: {}", e)))?;
        self.builder.build_store(wd_gep, null_ptr)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store working directory: {}", e)))?;
        
        // environment_vars = null
        let env_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            1,
            "env_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for environment: {}", e)))?;
        self.builder.build_store(env_gep, null_ptr)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store environment: {}", e)))?;
        
        // timeout_seconds = 30
        let timeout_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            2,
            "timeout_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for timeout: {}", e)))?;
        self.builder.build_store(timeout_gep, default_timeout)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store timeout: {}", e)))?;
        
        // priority = 0 (normal)
        let priority_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            3,
            "priority_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for priority: {}", e)))?;
        self.builder.build_store(priority_gep, normal_priority)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store priority: {}", e)))?;
        
        // memory_limit_mb = 0 (no limit)
        let mem_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            4,
            "mem_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for memory limit: {}", e)))?;
        self.builder.build_store(mem_gep, zero_i32)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store memory limit: {}", e)))?;
        
        // cpu_limit_percent = 0 (no limit)
        let cpu_gep = self.builder.build_struct_gep(
            self.process_options_type,
            options,
            5,
            "cpu_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for CPU limit: {}", e)))?;
        self.builder.build_store(cpu_gep, zero_i32)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store CPU limit: {}", e)))?;
        
        Ok(())
    }
    
    /// Create an IPC message structure
    pub fn create_ipc_message(
        &self,
        message_id: PointerValue<'ctx>,
        data: PointerValue<'ctx>,
        data_size: IntValue<'ctx>,
        priority: IntValue<'ctx>,
    ) -> Result<(), Error> {
        let message = self.builder.build_alloca(self.ipc_message_type, "ipc_message")
            .map_err(|e| CursedError::CodegenError(format!("Failed to allocate IPC message: {}", e)))?;
        
        // Set message_id
        let id_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            0,
            "id_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for message ID: {}", e)))?;
        self.builder.build_store(id_gep, message_id)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store message ID: {}", e)))?;
        
        // Set data
        let data_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            1,
            "data_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for data: {}", e)))?;
        self.builder.build_store(data_gep, data)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store data: {}", e)))?;
        
        // Set data_size
        let size_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            2,
            "size_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for data size: {}", e)))?;
        self.builder.build_store(size_gep, data_size)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store data size: {}", e)))?;
        
        // Set priority
        let priority_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            3,
            "priority_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for priority: {}", e)))?;
        self.builder.build_store(priority_gep, priority)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store priority: {}", e)))?;
        
        // Set timestamp (current time - simplified to 0 for now)
        let timestamp_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            4,
            "timestamp_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for timestamp: {}", e)))?;
        let current_time = self.i64_type.const_int(0, false); // TODO: Get actual timestamp
        self.builder.build_store(timestamp_gep, current_time)
            .map_err(|e| CursedError::CodegenError(format!("Failed to store timestamp: {}", e)))?;
        
        Ok(message)
    }
    
    /// Extract data from an IPC message
    pub fn extract_message_data(
        &self,
        message: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        // Get data pointer
        let data_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            1,
            "data_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for data: {}", e)))?;
        let data = self.builder.build_load(self.void_type, data_gep, "data")
            .map_err(|e| CursedError::CodegenError(format!("Failed to load data: {}", e)))?
            .into_pointer_value();
        
        // Get data size
        let size_gep = self.builder.build_struct_gep(
            self.ipc_message_type,
            message,
            2,
            "size_gep",
        ).map_err(|e| CursedError::CodegenError(format!("Failed to build GEP for data size: {}", e)))?;
        let size = self.builder.build_load(self.i32_type, size_gep, "size")
            .map_err(|e| CursedError::CodegenError(format!("Failed to load size: {}", e)))?
            .into_int_value();
        
        Ok((data, size))
    }
    
    /// Get type definitions for external use
    pub fn get_process_handle_type(&self) -> StructType<'ctx> {
        self.process_handle_type
    }
    
    pub fn get_process_options_type(&self) -> StructType<'ctx> {
        self.process_options_type
    }
    
    pub fn get_process_stats_type(&self) -> StructType<'ctx> {
        self.process_stats_type
    }
    
    pub fn get_ipc_message_type(&self) -> StructType<'ctx> {
        self.ipc_message_type
    }
    
    pub fn get_shared_memory_type(&self) -> StructType<'ctx> {
        self.shared_memory_type
    }
    
    pub fn get_message_queue_type(&self) -> StructType<'ctx> {
        self.message_queue_type
    }
}

/// FFI functions for runtime integration
extern "C" {
    /// Spawn a new process
    pub fn cursed_spawn_process(
        command: *const std::os::raw::c_char,
        args: *const std::os::raw::c_char,
        options: *const std::ffi::c_void,
        result: *mut std::ffi::c_void,
    ) -> i32;
    
    /// Wait for process completion
    pub fn cursed_wait_process(
        process_handle: *const std::ffi::c_void,
        timeout_seconds: i32,
    ) -> i32;
    
    /// Kill a process
    pub fn cursed_kill_process(
        process_handle: *const std::ffi::c_void,
        signal: i32,
    ) -> i32;
    
    /// Get process statistics
    pub fn cursed_get_process_stats(
        process_handle: *const std::ffi::c_void,
        stats: *mut std::ffi::c_void,
    ) -> i32;
    
    /// Create shared memory segment
    pub fn cursed_create_shared_memory(
        segment_id: *const std::os::raw::c_char,
        size: i32,
        result: *mut std::ffi::c_void,
    ) -> i32;
    
    /// Write to shared memory
    pub fn cursed_write_shared_memory(
        shm: *const std::ffi::c_void,
        offset: i32,
        data: *const std::ffi::c_void,
        size: i32,
    ) -> i32;
    
    /// Read from shared memory
    pub fn cursed_read_shared_memory(
        shm: *const std::ffi::c_void,
        offset: i32,
        buffer: *mut std::ffi::c_void,
        size: i32,
    ) -> i32;
    
    /// Create message queue
    pub fn cursed_create_message_queue(
        queue_id: *const std::os::raw::c_char,
        capacity: i32,
        result: *mut std::ffi::c_void,
    ) -> i32;
    
    /// Send message to queue
    pub fn cursed_send_message(
        queue: *const std::ffi::c_void,
        message: *const std::ffi::c_void,
    ) -> i32;
    
    /// Receive message from queue
    pub fn cursed_receive_message(
        queue: *const std::ffi::c_void,
        message: *mut std::ffi::c_void,
        timeout_ms: i32,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_process_ipc_integration_creation() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        let integration = ProcessIpcLlvmIntegration::new(&context, &module, &builder);
        assert!(integration.is_ok());
        
        let integration = integration.unwrap();
        
        // Test type definitions
        assert_eq!(integration.process_handle_type.count_fields(), 4);
        assert_eq!(integration.process_options_type.count_fields(), 6);
        assert_eq!(integration.process_stats_type.count_fields(), 5);
        assert_eq!(integration.ipc_message_type.count_fields(), 5);
        assert_eq!(integration.shared_memory_type.count_fields(), 4);
        assert_eq!(integration.message_queue_type.count_fields(), 4);
    }
    
    #[test]
    fn test_function_declarations() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        let integration = ProcessIpcLlvmIntegration::new(&context, &module, &builder).unwrap();
        
        // Test that functions are declared
        assert!(integration.get_function("cursed_spawn_process").is_ok());
        assert!(integration.get_function("cursed_wait_process").is_ok());
        assert!(integration.get_function("cursed_kill_process").is_ok());
        assert!(integration.get_function("cursed_get_process_stats").is_ok());
        assert!(integration.get_function("cursed_create_shared_memory").is_ok());
        assert!(integration.get_function("cursed_write_shared_memory").is_ok());
        assert!(integration.get_function("cursed_read_shared_memory").is_ok());
        assert!(integration.get_function("cursed_create_message_queue").is_ok());
        assert!(integration.get_function("cursed_send_message").is_ok());
        assert!(integration.get_function("cursed_receive_message").is_ok());
    }
    
    #[test]
    fn test_type_getters() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        let integration = ProcessIpcLlvmIntegration::new(&context, &module, &builder).unwrap();
        
        // Test type getters
        assert_eq!(integration.get_process_handle_type().count_fields(), 4);
        assert_eq!(integration.get_process_options_type().count_fields(), 6);
        assert_eq!(integration.get_process_stats_type().count_fields(), 5);
        assert_eq!(integration.get_ipc_message_type().count_fields(), 5);
        assert_eq!(integration.get_shared_memory_type().count_fields(), 4);
        assert_eq!(integration.get_message_queue_type().count_fields(), 4);
    }
}
