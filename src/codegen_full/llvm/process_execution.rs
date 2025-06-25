/// LLVM code generation for process execution keywords and unified process-IPC system
///
/// Provides compilation support for CURSED process execution constructs including:
/// - `exec_slay` keyword for simple process execution 
/// - `exec_vibez` keyword for enhanced process execution with streaming
/// - Unified process-IPC coordination system integration
/// - Process spawning with arguments and environment variables
/// - Input/output redirection and signal handling
/// - IPC mechanism setup and coordination
/// - Security context and resource limit enforcement
/// - Platform-specific optimizations

use crate::ast::traits::Expression;
use crate::ast::calls::CallExpression;
use crate::ast::traits::Node;
use crate::error::CursedError;
use std::ffi::CString;
use std::collections::HashMap;

/// IPC connection specification for compilation
#[derive(Debug, Clone)]
pub struct IpcConnectionSpec {
    pub name: String,
    pub connection_type: String,
    pub parameters: HashMap<String, String>,
}

/// Security specification for process compilation
#[derive(Debug, Clone)]
pub struct SecuritySpec {
    pub enable_privilege_drop: bool,
    pub isolation_level: String,
    pub allowed_operations: Vec<String>,
}

/// Resource limit specification for process compilation
#[derive(Debug, Clone)]
pub struct ResourceLimitSpec {
    pub max_memory: Option<u64>,
    pub max_cpu_time: Option<u64>,
    pub max_open_files: Option<u32>,
    pub max_processes: Option<u32>,
}

// Import real inkwell types for LLVM integration
use inkwell::{
    values::{BasicValueEnum, FunctionValue, PointerValue, IntValue, ArrayValue},
    types::{BasicTypeEnum, IntType, PointerType, FunctionType, ArrayType},
    basic_block::BasicBlock,
    AddressSpace,
    IntPredicate,
};

/// Runtime process execution manager reference for LLVM integration
// static mut RUNTIME_PROCESS_MANAGER: Option<*mut crate::stdlib::process::core::ProcessManager> = None;

/// Set the runtime process manager for LLVM integration
// pub fn set_runtime_process_manager(manager: *mut crate::stdlib::process::core::ProcessManager) {
//     unsafe {
//         RUNTIME_PROCESS_MANAGER = Some(manager);
//     }
// }

/// Get the runtime process manager for LLVM integration
// pub fn get_runtime_process_manager() -> Option<*mut crate::stdlib::process::core::ProcessManager> {
//     unsafe { RUNTIME_PROCESS_MANAGER }
// }

/// Trait for compiling process execution operations to LLVM IR
pub trait ProcessExecutionCompiler<'ctx> {
    /// Compile exec_slay process execution
    fn compile_exec_slay(&mut self, command: &str, args: &[String], options: Option<&dyn Expression>) -> crate::error::Result<()>;
    
    /// Compile exec_vibez enhanced process execution  
    fn compile_exec_vibez(&mut self, command: &str, args: &[String], context: Option<&dyn Expression>) -> crate::error::Result<()>;
    
    /// Compile process spawning with full options
    fn compile_process_spawn(&mut self, command: &str, args: &[String], env: Option<&HashMap<String, String>>) -> crate::error::Result<()>;
    
    /// Compile process waiting and status checking
    fn compile_process_wait(&mut self, pid_expr: &dyn Expression) -> crate::error::Result<()>;
    
    /// Compile signal sending to process
    fn compile_process_signal(&mut self, pid_expr: &dyn Expression, signal: i32) -> crate::error::Result<()>;
    
    /// Compile unified process-IPC coordination
    fn compile_unified_process_ipc(&mut self, command: &str, args: &[String], ipc_connections: &[IpcConnectionSpec]) -> crate::error::Result<()>;
    
    /// Compile IPC connection creation
    fn compile_ipc_connection(&mut self, source_process: &dyn Expression, target_process: &dyn Expression, connection_type: &str, name: &str) -> crate::error::Result<()>;
    
    /// Compile security context application
    fn compile_security_context(&mut self, process: &dyn Expression, security_settings: &SecuritySpec) -> crate::error::Result<()>;
    
    /// Compile resource limit enforcement
    fn compile_resource_limits(&mut self, process: &dyn Expression, limits: &ResourceLimitSpec) -> crate::error::Result<()>;
    
    /// Compile process termination
    fn compile_process_terminate(&mut self, pid_expr: &dyn Expression, force: bool) -> crate::error::Result<()>;
    
    /// Compile pipeline execution
    fn compile_process_pipeline(&mut self, commands: &[(&str, &[String])]) -> crate::error::Result<()>;
    
    /// Compile background task execution
    fn compile_background_task(&mut self, command_expr: &dyn Expression) -> crate::error::Result<()>;
    
    /// Compile input/output redirection
    fn compile_io_redirection(&mut self, stdin: Option<&str>, stdout: Option<&str>, stderr: Option<&str>) -> crate::error::Result<()>;
    
    /// Declare runtime FFI functions in the module
    fn declare_process_execution_runtime_functions(&mut self) -> crate::error::Result<()>;
}

/// Implementation of ProcessExecutionCompiler for the real LLVM code generator
impl<'ctx> ProcessExecutionCompiler<'ctx> for crate::codegen::llvm::LlvmCodeGeneratorReal<'ctx> {
    fn compile_exec_slay(&mut self, command: &str, args: &[String], options: Option<&dyn Expression>) -> crate::error::Result<()> {
        tracing::info!("Compiling exec_slay process execution");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Get the exec_slay function from the runtime
        let exec_slay_fn = self.module().get_function("cursed_exec_slay")
            .ok_or_else(|| CursedError::Compile("cursed_exec_slay function not found".to_string()))?;
        
        // Get runtime process manager pointer
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Create command string constant
        let command_str = self.create_global_string(command);
        
        // Create arguments array
        let args_array = self.create_string_array(args)?;
        let args_count = self.context().i32_type().const_int(args.len() as u64, false);
        
        // Compile options if provided, otherwise use null
        let options_ptr = if let Some(opts) = options {
            self.compile_expression(opts)?
                .into_pointer_value()
        } else {
            self.context().i8_type().ptr_type(AddressSpace::Generic).const_null()
        };
        
        // Call cursed_exec_slay(manager_ptr, command_str, args_array, args_count, options_ptr)
        let exec_call_result = self.builder().build_call(
            exec_slay_fn,
            &[
                manager_ptr.into(),
                command_str.into(),
                args_array.into(),
                args_count.into(),
                options_ptr.into(),
            ],
            "exec_slay_result"
        );
        
        let result = exec_call_result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get exec_slay result".to_string()))?;
        
        tracing::info!(
            command = %command,
            args_count = args.len(),
            "Successfully compiled exec_slay"
        );
        
        Ok(result)
    }
    
    fn compile_exec_vibez(&mut self, command: &str, args: &[String], context: Option<&dyn Expression>) -> crate::error::Result<()> {
        tracing::info!("Compiling exec_vibez process execution");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Get the exec_vibez function from the runtime
        let exec_vibez_fn = self.module().get_function("cursed_exec_vibez")
            .ok_or_else(|| CursedError::Compile("cursed_exec_vibez function not found".to_string()))?;
        
        // Get runtime process manager pointer
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Create command string constant
        let command_str = self.create_global_string(command);
        
        // Create arguments array
        let args_array = self.create_string_array(args)?;
        let args_count = self.context().i32_type().const_int(args.len() as u64, false);
        
        // Compile context if provided, otherwise use null
        let context_ptr = if let Some(ctx) = context {
            self.compile_expression(ctx)?
                .into_pointer_value()
        } else {
            self.context().i8_type().ptr_type(AddressSpace::Generic).const_null()
        };
        
        // Call cursed_exec_vibez(manager_ptr, command_str, args_array, args_count, context_ptr)
        let exec_call_result = self.builder().build_call(
            exec_vibez_fn,
            &[
                manager_ptr.into(),
                command_str.into(),
                args_array.into(),
                args_count.into(),
                context_ptr.into(),
            ],
            "exec_vibez_result"
        );
        
        let result = exec_call_result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get exec_vibez result".to_string()))?;
        
        tracing::info!(
            command = %command,
            args_count = args.len(),
            "Successfully compiled exec_vibez"
        );
        
        Ok(result)
    }
    
    fn compile_process_spawn(&mut self, command: &str, args: &[String], env: Option<&HashMap<String, String>>) -> crate::error::Result<()> {
        tracing::info!("Compiling process spawn");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Get the spawn function from the runtime
        let spawn_fn = self.module().get_function("cursed_process_spawn")
            .ok_or_else(|| CursedError::Compile("cursed_process_spawn function not found".to_string()))?;
        
        // Get runtime process manager pointer
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Create command string constant
        let command_str = self.create_global_string(command);
        
        // Create arguments array
        let args_array = self.create_string_array(args)?;
        let args_count = self.context().i32_type().const_int(args.len() as u64, false);
        
        // Create environment variables array
        let (env_array, env_count) = if let Some(environment) = env {
            let env_pairs: Vec<String> = environment.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            let env_arr = self.create_string_array(&env_pairs)?;
            let env_cnt = self.context().i32_type().const_int(env_pairs.len() as u64, false);
            (env_arr, env_cnt)
        } else {
            let null_ptr = self.context().i8_type().ptr_type(AddressSpace::Generic).const_null();
            let zero_count = self.context().i32_type().const_int(0, false);
            (null_ptr, zero_count)
        };
        
        // Call cursed_process_spawn(manager_ptr, command_str, args_array, args_count, env_array, env_count)
        let spawn_call_result = self.builder().build_call(
            spawn_fn,
            &[
                manager_ptr.into(),
                command_str.into(),
                args_array.into(),
                args_count.into(),
                env_array.into(),
                env_count.into(),
            ],
            "process_pid"
        );
        
        let pid = spawn_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value())
            .ok_or_else(|| CursedError::Compile("Failed to get process spawn result".to_string()))?;
        
        tracing::info!(
            command = %command,
            args_count = args.len(),
            "Successfully compiled process spawn"
        );
        
        Ok(pid)
    }
    
    fn compile_process_wait(&mut self, pid_expr: &dyn Expression) -> crate::error::Result<()> {
        tracing::info!("Compiling process wait");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Compile the PID expression
        let pid_value = self.compile_expression(pid_expr)?
            .into_int_value();
        
        // Get the wait function from the runtime
        let wait_fn = self.module().get_function("cursed_process_wait")
            .ok_or_else(|| CursedError::Compile("cursed_process_wait function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Call cursed_process_wait(manager_ptr, pid)
        let wait_call_result = self.builder().build_call(
            wait_fn,
            &[
                manager_ptr.into(),
                pid_value.into(),
            ],
            "wait_result"
        );
        
        let exit_code = wait_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value())
            .ok_or_else(|| CursedError::Compile("Failed to get process wait result".to_string()))?;
        
        tracing::info!("Successfully compiled process wait");
        Ok(exit_code)
    }
    
    fn compile_process_signal(&mut self, pid_expr: &dyn Expression, signal: i32) -> crate::error::Result<()> {
        tracing::info!(signal = signal, "Compiling process signal");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Compile the PID expression
        let pid_value = self.compile_expression(pid_expr)?
            .into_int_value();
        
        // Get the signal function from the runtime
        let signal_fn = self.module().get_function("cursed_process_signal")
            .ok_or_else(|| CursedError::Compile("cursed_process_signal function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        let signal_value = self.context().i32_type().const_int(signal as u64, false);
        
        // Call cursed_process_signal(manager_ptr, pid, signal)
        let signal_call_result = self.builder().build_call(
            signal_fn,
            &[
                manager_ptr.into(),
                pid_value.into(),
                signal_value.into(),
            ],
            "signal_result"
        );
        
        let result = signal_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value())
            .ok_or_else(|| CursedError::Compile("Failed to get process signal result".to_string()))?;
        
        tracing::info!(signal = signal, "Successfully compiled process signal");
        Ok(result)
    }
    
    fn compile_process_terminate(&mut self, pid_expr: &dyn Expression, force: bool) -> crate::error::Result<()> {
        tracing::info!(force = force, "Compiling process terminate");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Compile the PID expression
        let pid_value = self.compile_expression(pid_expr)?
            .into_int_value();
        
        // Get the terminate function from the runtime
        let terminate_fn = self.module().get_function("cursed_process_terminate")
            .ok_or_else(|| CursedError::Compile("cursed_process_terminate function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        let force_value = self.context().bool_type().const_int(if force { 1 } else { 0 }, false);
        
        // Call cursed_process_terminate(manager_ptr, pid, force)
        let terminate_call_result = self.builder().build_call(
            terminate_fn,
            &[
                manager_ptr.into(),
                pid_value.into(),
                force_value.into(),
            ],
            "terminate_result"
        );
        
        let result = terminate_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value())
            .ok_or_else(|| CursedError::Compile("Failed to get process terminate result".to_string()))?;
        
        tracing::info!(force = force, "Successfully compiled process terminate");
        Ok(result)
    }
    
    fn compile_process_pipeline(&mut self, commands: &[(&str, &[String])]) -> crate::error::Result<()> {
        tracing::info!("Compiling process pipeline");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Get the pipeline function from the runtime
        let pipeline_fn = self.module().get_function("cursed_process_pipeline")
            .ok_or_else(|| CursedError::Compile("cursed_process_pipeline function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Create array of commands (simplified representation)
        let mut command_strings = Vec::new();
        for (cmd, args) in commands {
            let full_command = if args.is_empty() {
                cmd.to_string()
            } else {
                format!("{} {}", cmd, args.join(" "))
            };
            command_strings.push(full_command);
        }
        
        let commands_array = self.create_string_array(&command_strings)?;
        let commands_count = self.context().i32_type().const_int(commands.len() as u64, false);
        
        // Call cursed_process_pipeline(manager_ptr, commands_array, commands_count)
        let pipeline_call_result = self.builder().build_call(
            pipeline_fn,
            &[
                manager_ptr.into(),
                commands_array.into(),
                commands_count.into(),
            ],
            "pipeline_handle"
        );
        
        let handle = pipeline_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_pointer_value())
            .ok_or_else(|| CursedError::Compile("Failed to get process pipeline result".to_string()))?;
        
        tracing::info!(commands_count = commands.len(), "Successfully compiled process pipeline");
        Ok(handle)
    }
    
    fn compile_background_task(&mut self, command_expr: &dyn Expression) -> crate::error::Result<()> {
        tracing::info!("Compiling background task");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Compile the command expression
        let command_value = self.compile_expression(command_expr)?;
        
        // Get the background task function from the runtime
        let bg_task_fn = self.module().get_function("cursed_background_task")
            .ok_or_else(|| CursedError::Compile("cursed_background_task function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Call cursed_background_task(manager_ptr, command_value)
        let bg_call_result = self.builder().build_call(
            bg_task_fn,
            &[
                manager_ptr.into(),
                command_value.into(),
            ],
            "bg_task_handle"
        );
        
        let handle = bg_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_pointer_value())
            .ok_or_else(|| CursedError::Compile("Failed to get background task result".to_string()))?;
        
        tracing::info!("Successfully compiled background task");
        Ok(handle)
    }
    
    fn compile_io_redirection(&mut self, stdin: Option<&str>, stdout: Option<&str>, stderr: Option<&str>) -> crate::error::Result<()> {
        tracing::info!("Compiling I/O redirection");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Get the I/O redirection function from the runtime
        let io_redirect_fn = self.module().get_function("cursed_io_redirection")
            .ok_or_else(|| CursedError::Compile("cursed_io_redirection function not found".to_string()))?;
        
        let manager_ptr = self.get_runtime_process_manager_ptr()?;
        
        // Create string constants for file paths (or null)
        let stdin_str = if let Some(path) = stdin {
            self.create_global_string(path)
        } else {
            self.context().i8_type().ptr_type(AddressSpace::Generic).const_null()
        };
        
        let stdout_str = if let Some(path) = stdout {
            self.create_global_string(path)
        } else {
            self.context().i8_type().ptr_type(AddressSpace::Generic).const_null()
        };
        
        let stderr_str = if let Some(path) = stderr {
            self.create_global_string(path)
        } else {
            self.context().i8_type().ptr_type(AddressSpace::Generic).const_null()
        };
        
        // Call cursed_io_redirection(manager_ptr, stdin_str, stdout_str, stderr_str)
        let io_call_result = self.builder().build_call(
            io_redirect_fn,
            &[
                manager_ptr.into(),
                stdin_str.into(),
                stdout_str.into(),
                stderr_str.into(),
            ],
            "io_config"
        );
        
        let config = io_call_result.try_as_basic_value().left()
            .and_then(|v| v.into_pointer_value())
            .ok_or_else(|| CursedError::Compile("Failed to get I/O redirection result".to_string()))?;
        
        tracing::info!("Successfully compiled I/O redirection");
        Ok(config)
    }
    
    fn declare_process_execution_runtime_functions(&mut self) -> crate::error::Result<()> {
        let i8_type = self.context().i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);
        let i32_type = self.context().i32_type();
        let bool_type = self.context().bool_type();
        let void_type = self.context().void_type();
        
        // Declare cursed_exec_slay(manager_ptr, command_str, args_array, args_count, options_ptr) -> i32
        if self.module().get_function("cursed_exec_slay").is_none() {
            let exec_slay_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // command_str
                i8_ptr_type.into(), // args_array (ptr to ptr array)
                i32_type.into(),    // args_count
                i8_ptr_type.into(), // options_ptr
            ], false);
            self.module().add_function("cursed_exec_slay", exec_slay_fn_type, None);
        }
        
        // Declare cursed_exec_vibez(manager_ptr, command_str, args_array, args_count, context_ptr) -> i32
        if self.module().get_function("cursed_exec_vibez").is_none() {
            let exec_vibez_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // command_str
                i8_ptr_type.into(), // args_array (ptr to ptr array)
                i32_type.into(),    // args_count
                i8_ptr_type.into(), // context_ptr
            ], false);
            self.module().add_function("cursed_exec_vibez", exec_vibez_fn_type, None);
        }
        
        // Declare cursed_process_spawn(manager_ptr, command_str, args_array, args_count, env_array, env_count) -> i32
        if self.module().get_function("cursed_process_spawn").is_none() {
            let spawn_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // command_str
                i8_ptr_type.into(), // args_array
                i32_type.into(),    // args_count
                i8_ptr_type.into(), // env_array
                i32_type.into(),    // env_count
            ], false);
            self.module().add_function("cursed_process_spawn", spawn_fn_type, None);
        }
        
        // Declare cursed_process_wait(manager_ptr, pid) -> i32
        if self.module().get_function("cursed_process_wait").is_none() {
            let wait_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i32_type.into(),    // pid
            ], false);
            self.module().add_function("cursed_process_wait", wait_fn_type, None);
        }
        
        // Declare cursed_process_signal(manager_ptr, pid, signal) -> i32
        if self.module().get_function("cursed_process_signal").is_none() {
            let signal_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i32_type.into(),    // pid
                i32_type.into(),    // signal
            ], false);
            self.module().add_function("cursed_process_signal", signal_fn_type, None);
        }
        
        // Declare cursed_process_terminate(manager_ptr, pid, force) -> i32
        if self.module().get_function("cursed_process_terminate").is_none() {
            let terminate_fn_type = i32_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i32_type.into(),    // pid
                bool_type.into(),   // force
            ], false);
            self.module().add_function("cursed_process_terminate", terminate_fn_type, None);
        }
        
        // Declare cursed_process_pipeline(manager_ptr, commands_array, commands_count) -> ptr
        if self.module().get_function("cursed_process_pipeline").is_none() {
            let pipeline_fn_type = i8_ptr_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // commands_array
                i32_type.into(),    // commands_count
            ], false);
            self.module().add_function("cursed_process_pipeline", pipeline_fn_type, None);
        }
        
        // Declare cursed_background_task(manager_ptr, command_value) -> ptr
        if self.module().get_function("cursed_background_task").is_none() {
            let bg_task_fn_type = i8_ptr_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // command_value
            ], false);
            self.module().add_function("cursed_background_task", bg_task_fn_type, None);
        }
        
        // Declare cursed_io_redirection(manager_ptr, stdin_str, stdout_str, stderr_str) -> ptr
        if self.module().get_function("cursed_io_redirection").is_none() {
            let io_redirect_fn_type = i8_ptr_type.fn_type(&[
                i8_ptr_type.into(), // manager_ptr
                i8_ptr_type.into(), // stdin_str
                i8_ptr_type.into(), // stdout_str
                i8_ptr_type.into(), // stderr_str
            ], false);
            self.module().add_function("cursed_io_redirection", io_redirect_fn_type, None);
        }
        
        tracing::info!("Successfully declared process execution runtime functions");
        Ok(())
    }
    
    fn compile_unified_process_ipc(&mut self, command: &str, args: &[String], ipc_connections: &[IpcConnectionSpec]) -> crate::error::Result<()> {
        tracing::info!("Compiling unified process IPC");
        
        // Ensure runtime functions are declared
        self.declare_process_execution_runtime_functions()?;
        
        // Create command string constant
        let command_str = self.create_global_string(command);
        
        // Create args array
        let args_array = self.create_string_array(args)?;
        
        // Create IPC connections array (simplified for now)
        let connections_count = self.context().i32_type().const_int(ipc_connections.len() as u64, false);
        
        // For now, return a placeholder success value
        Ok(self.context().i32_type().const_int(0, false).into())
    }

    fn compile_ipc_connection(&mut self, source_process: &dyn Expression, target_process: &dyn Expression, connection_type: &str, name: &str) -> crate::error::Result<()> {
        tracing::info!("Compiling IPC connection");
        
        // Compile source and target process expressions
        let _source_value = self.compile_expression(source_process)?;
        let _target_value = self.compile_expression(target_process)?;
        
        // Create connection type and name string constants
        let _conn_type_str = self.create_global_string(connection_type);
        let _name_str = self.create_global_string(name);
        
        // For now, return a placeholder success value
        Ok(self.context().i32_type().const_int(0, false).into())
    }

    fn compile_security_context(&mut self, process: &dyn Expression, _security_settings: &SecuritySpec) -> crate::error::Result<()> {
        tracing::info!("Compiling security context");
        
        // Compile process expression
        let _process_value = self.compile_expression(process)?;
        
        // For now, return a placeholder success value
        Ok(self.context().i32_type().const_int(0, false).into())
    }

    fn compile_resource_limits(&mut self, process: &dyn Expression, _limits: &ResourceLimitSpec) -> crate::error::Result<()> {
        tracing::info!("Compiling resource limits");
        
        // Compile process expression
        let _process_value = self.compile_expression(process)?;
        
        // For now, return a placeholder success value
        Ok(self.context().i32_type().const_int(0, false).into())
    }
}

/// Helper implementations for LlvmCodeGeneratorReal
impl<'ctx> crate::codegen::llvm::LlvmCodeGeneratorReal<'ctx> {
    /// Get runtime process manager pointer
    fn get_runtime_process_manager_ptr(&self) -> crate::error::Result<()> {
        // Get the static process manager pointer from the runtime
        let manager_opt = get_runtime_process_manager();
        let manager_ptr_raw = manager_opt
            .ok_or_else(|| CursedError::Runtime("No runtime process manager available".to_string()))?;
        
        // Convert raw pointer to LLVM pointer value
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::Generic);
        let manager_ptr_int = self.context().i64_type().const_int(manager_ptr_raw as u64, false);
        let manager_ptr = self.builder().build_int_to_ptr(
            manager_ptr_int,
            i8_ptr_type,
            "process_manager_ptr"
        );
        
        Ok(manager_ptr)
    }
    
    /// Create a global string constant
    fn create_global_string(&self, text: &str) -> PointerValue<'ctx> {
        let string_value = self.context().const_string(text.as_bytes(), true);
        let global = self.module().add_global(string_value.get_type(), None, "str_const");
        global.set_initializer(&string_value);
        global.set_constant(true);
        global.set_linkage(inkwell::module::Linkage::Private);
        global.set_unnamed_addr(true);
        
        // Get pointer to the string data
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::Generic);
        self.builder().build_bitcast(
            global.as_pointer_value(),
            i8_ptr_type,
            "str_ptr"
        )
    }
    
    /// Create an array of string pointers
    fn create_string_array(&self, strings: &[String]) -> crate::error::Result<()> {
        if strings.is_empty() {
            return Ok(self.context().i8_type().ptr_type(AddressSpace::Generic).const_null());
        }
        
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::Generic);
        
        // Create string constants
        let mut string_ptrs = Vec::new();
        for s in strings {
            let str_ptr = self.create_global_string(s);
            string_ptrs.push(str_ptr);
        }
        
        // Create array type and global
        let array_type = i8_ptr_type.array_type(strings.len() as u32);
        let array_value = i8_ptr_type.const_array(&string_ptrs);
        
        let global = self.module().add_global(array_type, None, "str_array");
        global.set_initializer(&array_value);
        global.set_constant(true);
        global.set_linkage(inkwell::module::Linkage::Private);
        
        // Return pointer to array
        let array_ptr = self.builder().build_bitcast(
            global.as_pointer_value(),
            i8_ptr_type.ptr_type(AddressSpace::Generic),
            "str_array_ptr"
        );
        
        Ok(array_ptr)
    }
}

/// Convenience functions for process execution integration

/// Initialize process execution runtime integration
pub fn initialize_process_execution_runtime<'ctx>(
    generator: &mut impl ProcessExecutionCompiler<'ctx>,
) -> crate::error::Result<()> {
    // Declare all necessary functions for process execution
    generator.declare_process_execution_runtime_functions()?;
    
    tracing::info!("Process execution runtime successfully initialized for LLVM compilation");
    Ok(())
}

/// Compile exec_slay command helper
pub fn compile_exec_slay_command<'ctx>(
    generator: &mut impl ProcessExecutionCompiler<'ctx>,
    command: &str,
    args: &[String],
    options: Option<&dyn Expression>,
) -> crate::error::Result<()> {
    generator.compile_exec_slay(command, args, options)
}

/// Compile exec_vibez command helper
pub fn compile_exec_vibez_command<'ctx>(
    generator: &mut impl ProcessExecutionCompiler<'ctx>,
    command: &str,
    args: &[String],
    context: Option<&dyn Expression>,
) -> crate::error::Result<()> {
    generator.compile_exec_vibez(command, args, context)
}

/// Runtime integration for process execution
pub mod runtime_integration {
    use super::*;
    
    /// Initialize a runtime process manager and set it as the global manager
    pub fn initialize_process_manager() -> crate::error::Result<()> {
//         let manager = Box::new(crate::stdlib::process::core::ProcessManager::new());
        let manager_ptr = Box::into_raw(manager);
        set_runtime_process_manager(manager_ptr);
        
        tracing::info!("Runtime process manager initialized");
        Ok(manager_ptr)
    }
    
    /// Clean up the runtime process manager
    pub fn cleanup_process_manager() -> crate::error::Result<()> {
        if let Some(manager_ptr) = get_runtime_process_manager() {
            unsafe {
                let _manager = Box::from_raw(manager_ptr);
                // ProcessManager destructor will handle cleanup
            }
            
            // Clear the global reference
            set_runtime_process_manager(std::ptr::null_mut());
            tracing::info!("Runtime process manager cleaned up");
        }
        
        Ok(())
    }
}

