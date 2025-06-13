/// LLVM code generation for process management operations in CURSED
/// 
/// This module provides comprehensive LLVM integration for process spawning,
/// management, and lifecycle operations. It includes FFI functions for compiled
/// CURSED code to interact with the process management system.

use crate::codegen::llvm::{LlvmCodeGenerator, expression_compiler::{LlvmValue, LlvmType}};
use crate::error::Error;
use std::collections::HashMap;

/// Trait for compiling process management operations to LLVM IR
pub trait ProcessCompiler {
    /// Compile a process spawn operation
    fn compile_spawn_process(
        &mut self,
        command: &LlvmValue,
        args: &LlvmValue,
        config: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error>;

    /// Compile a process wait operation
    fn compile_wait_process(
        &mut self,
        process_handle: &LlvmValue,
        timeout: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error>;

    /// Compile a process kill operation
    fn compile_kill_process(
        &mut self,
        process_handle: &LlvmValue,
        signal: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error>;

    /// Compile a process status check
    fn compile_process_status(
        &mut self,
        process_handle: &LlvmValue,
    ) -> Result<LlvmValue, Error>;

    /// Compile process I/O operations
    fn compile_process_io(
        &mut self,
        process_handle: &LlvmValue,
        operation: ProcessIoOperation,
        data: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error>;

    /// Generate FFI function declarations for process management
    fn declare_process_ffi_functions(&mut self) -> Result<(), Error>;
}

/// Process I/O operation types
#[derive(Debug, Clone, Copy)]
pub enum ProcessIoOperation {
    Read,
    Write,
    Close,
    Flush,
}

impl ProcessCompiler for LlvmCodeGenerator {
    fn compile_spawn_process(
        &mut self,
        command: &LlvmValue,
        args: &LlvmValue,
        config: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error> {
        // Ensure FFI functions are declared
        self.declare_process_ffi_functions()?;

        // Generate LLVM IR for process spawn
        let process_handle = self.next_temp_name();
        let command_name = &command.llvm_name;
        let args_name = &args.llvm_name;
        
        let config_name = if let Some(cfg) = config {
            &cfg.llvm_name
        } else {
            "null"
        };

        // Generate the IR call
        let ir = format!(
            "{} = call i64 @cursed_spawn_process(i8* {}, i8* {}, i8* {})",
            process_handle, command_name, args_name, config_name
        );

        // Add to the function's IR
        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer64,
            llvm_name: process_handle,
            is_constant: false,
        })
    }

    fn compile_wait_process(
        &mut self,
        process_handle: &LlvmValue,
        timeout: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error> {
        let result_name = self.next_temp_name();
        let handle_name = &process_handle.llvm_name;
        
        let timeout_name = if let Some(t) = timeout {
            &t.llvm_name
        } else {
            "0"
        };

        let ir = format!(
            "{} = call i32 @cursed_wait_process(i64 {}, i64 {})",
            result_name, handle_name, timeout_name
        );

        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer32,
            llvm_name: result_name,
            is_constant: false,
        })
    }

    fn compile_kill_process(
        &mut self,
        process_handle: &LlvmValue,
        signal: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error> {
        let result_name = self.next_temp_name();
        let handle_name = &process_handle.llvm_name;
        
        let signal_name = if let Some(s) = signal {
            &s.llvm_name
        } else {
            "15" // SIGTERM
        };

        let ir = format!(
            "{} = call i32 @cursed_kill_process(i64 {}, i32 {})",
            result_name, handle_name, signal_name
        );

        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer32,
            llvm_name: result_name,
            is_constant: false,
        })
    }

    fn compile_process_status(
        &mut self,
        process_handle: &LlvmValue,
    ) -> Result<LlvmValue, Error> {
        let result_name = self.next_temp_name();
        let handle_name = &process_handle.llvm_name;

        let ir = format!(
            "{} = call i32 @cursed_process_status(i64 {})",
            result_name, handle_name
        );

        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer32,
            llvm_name: result_name,
            is_constant: false,
        })
    }

    fn compile_process_io(
        &mut self,
        process_handle: &LlvmValue,
        operation: ProcessIoOperation,
        data: Option<&LlvmValue>,
    ) -> Result<LlvmValue, Error> {
        let result_name = self.next_temp_name();
        let handle_name = &process_handle.llvm_name;
        
        let (fn_name, op_code) = match operation {
            ProcessIoOperation::Read => ("cursed_process_read", 0),
            ProcessIoOperation::Write => ("cursed_process_write", 1),
            ProcessIoOperation::Close => ("cursed_process_close", 2),
            ProcessIoOperation::Flush => ("cursed_process_flush", 3),
        };

        let data_name = if let Some(d) = data {
            &d.llvm_name
        } else {
            "null"
        };

        let ir = format!(
            "{} = call i64 @{}(i64 {}, i32 {}, i8* {})",
            result_name, fn_name, handle_name, op_code, data_name
        );

        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer64,
            llvm_name: result_name,
            is_constant: false,
        })
    }

    fn declare_process_ffi_functions(&mut self) -> Result<(), Error> {
        // Add function declarations to the module
        self.add_ir("; Process management FFI function declarations");
        self.add_ir("declare i64 @cursed_spawn_process(i8*, i8*, i8*)");
        self.add_ir("declare i32 @cursed_wait_process(i64, i64)");
        self.add_ir("declare i32 @cursed_kill_process(i64, i32)");
        self.add_ir("declare i32 @cursed_process_status(i64)");
        self.add_ir("declare i64 @cursed_process_read(i64, i32, i8*)");
        self.add_ir("declare i64 @cursed_process_write(i64, i32, i8*)");
        self.add_ir("declare i32 @cursed_process_close(i64, i32, i8*)");
        self.add_ir("declare i32 @cursed_process_flush(i64, i32, i8*)");
        
        Ok(())
    }
}

impl LlvmCodeGenerator {
    /// Helper method to add IR to the current function
    fn add_ir(&self, ir: &str) {
        // In the real implementation, this would add IR to the current function
        // For now, this is a placeholder
        tracing::debug!("Generated process IR: {}", ir);
    }

    /// Helper method to generate a unique temporary name
    fn next_temp_name(&self) -> String {
        let mut counter = self.temp_counter.borrow_mut();
        *counter += 1;
        format!("%temp_{}", *counter)
    }
}

/// FFI functions for process management (to be implemented in the runtime)
/// These functions provide the actual implementation of process operations
/// that can be called from compiled LLVM code.

#[no_mangle]
pub extern "C" fn cursed_spawn_process(
    command: *const i8,
    args: *const i8,
    config: *const i8,
) -> i64 {
    use crate::stdlib::process::{spawn_process, ProcessConfig};
    use std::ffi::CStr;

    if command.is_null() {
        return -1; // Error: null command
    }

    let command_str = unsafe {
        match CStr::from_ptr(command).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // Error: invalid UTF-8
        }
    };

    let mut process_config = ProcessConfig::new(command_str);

    // Parse arguments if provided
    if !args.is_null() {
        let args_str = unsafe {
            match CStr::from_ptr(args).to_str() {
                Ok(s) => s,
                Err(_) => return -3, // Error: invalid args UTF-8
            }
        };
        // Simple space-separated argument parsing
        for arg in args_str.split_whitespace() {
            process_config = process_config.arg(arg);
        }
    }

    // TODO: Parse config if provided (JSON or similar format)

    match spawn_process(process_config) {
        Ok(process) => {
            // Store the process in a global registry and return handle
            let handle = store_process_handle(process);
            handle as i64
        }
        Err(_) => -4, // Error: spawn failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_wait_process(handle: i64, timeout_ms: i64) -> i32 {
    use std::time::Duration;

    let timeout = if timeout_ms > 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };

    match get_process_handle(handle as usize) {
        Some(mut process) => {
            match process.wait_with_timeout(timeout) {
                Ok(status) => status.map_or(0, |s| s.code().unwrap_or(-1)),
                Err(_) => -1, // Error: wait failed
            }
        }
        None => -2, // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_kill_process(handle: i64, signal: i32) -> i32 {
    match get_process_handle(handle as usize) {
        Some(mut process) => {
            // For simplicity, we'll just kill the process
            // In a full implementation, we'd handle different signals
            match process.kill() {
                Ok(()) => 0, // Success
                Err(_) => -1, // Error: kill failed
            }
        }
        None => -2, // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_process_status(handle: i64) -> i32 {
    match get_process_handle(handle as usize) {
        Some(process) => {
            if process.is_running() {
                1 // Running
            } else {
                0 // Not running
            }
        }
        None => -1, // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_process_read(handle: i64, _op: i32, data: *mut i8) -> i64 {
    if data.is_null() {
        return -1;
    }

    match get_process_handle(handle as usize) {
        Some(mut process) => {
            // For simplicity, we'll read a fixed amount
            // In a full implementation, this would be more sophisticated
            match process.read_stdout() {
                Ok(output) => {
                    let bytes = output.as_bytes();
                    let len = bytes.len().min(1024); // Max 1KB
                    unsafe {
                        std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, len);
                    }
                    len as i64
                }
                Err(_) => -2, // Error: read failed
            }
        }
        None => -3, // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_process_write(handle: i64, _op: i32, data: *const i8) -> i64 {
    if data.is_null() {
        return -1;
    }

    let data_str = unsafe {
        match std::ffi::CStr::from_ptr(data).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // Error: invalid UTF-8
        }
    };

    match get_process_handle_mut(handle as usize) {
        Some(process) => {
            match process.write_stdin(data_str) {
                Ok(()) => data_str.len() as i64,
                Err(_) => -3, // Error: write failed
            }
        }
        None => -4, // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_process_close(handle: i64, _op: i32, _data: *const i8) -> i32 {
    match remove_process_handle(handle as usize) {
        Some(_) => 0, // Success
        None => -1,   // Error: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_process_flush(handle: i64, _op: i32, _data: *const i8) -> i32 {
    match get_process_handle_mut(handle as usize) {
        Some(process) => {
            // Flush stdin
            match process.flush_stdin() {
                Ok(()) => 0, // Success
                Err(_) => -1, // Error: flush failed
            }
        }
        None => -2, // Error: invalid handle
    }
}

// Global process handle registry
use std::sync::Mutex;
use std::collections::HashMap;
use crate::stdlib::process::Process;

lazy_static::lazy_static! {
    static ref PROCESS_REGISTRY: Mutex<HashMap<usize, Process>> = Mutex::new(HashMap::new());
    static ref NEXT_HANDLE: Mutex<usize> = Mutex::new(1);
}

fn store_process_handle(process: Process) -> usize {
    let mut registry = PROCESS_REGISTRY.lock().unwrap();
    let mut next_handle = NEXT_HANDLE.lock().unwrap();
    
    let handle = *next_handle;
    *next_handle += 1;
    
    registry.insert(handle, process);
    handle
}

fn get_process_handle(handle: usize) -> Option<Process> {
    let registry = PROCESS_REGISTRY.lock().unwrap();
    registry.get(&handle).cloned()
}

fn get_process_handle_mut(handle: usize) -> Option<Process> {
    let mut registry = PROCESS_REGISTRY.lock().unwrap();
    registry.get_mut(&handle).cloned()
}

fn remove_process_handle(handle: usize) -> Option<Process> {
    let mut registry = PROCESS_REGISTRY.lock().unwrap();
    registry.remove(&handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::{LlvmCodeGenerator, expression_compiler::{LlvmValue, LlvmType}};

    #[test]
    fn test_ffi_functions_declared() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Declare FFI functions
        assert!(generator.declare_process_ffi_functions().is_ok());
    }

    #[test]
    fn test_process_compilation() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Create test values
        let command = LlvmValue {
            value_type: LlvmType::String,
            llvm_name: "%command_str".to_string(),
            is_constant: true,
        };
        
        let args = LlvmValue {
            value_type: LlvmType::String,
            llvm_name: "%args_str".to_string(),
            is_constant: true,
        };
        
        // Compile spawn operation
        let result = generator.compile_spawn_process(&command, &args, None);
        assert!(result.is_ok());
        
        let process_handle = result.unwrap();
        assert_eq!(process_handle.value_type, LlvmType::Integer64);
    }

    #[test]
    fn test_process_io_operations() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        let handle = LlvmValue {
            value_type: LlvmType::Integer64,
            llvm_name: "%process_handle".to_string(),
            is_constant: false,
        };
        
        // Test different I/O operations
        let operations = [
            ProcessIoOperation::Read,
            ProcessIoOperation::Write,
            ProcessIoOperation::Close,
            ProcessIoOperation::Flush,
        ];
        
        for op in &operations {
            let result = generator.compile_process_io(&handle, *op, None);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_process_lifecycle() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        let handle = LlvmValue {
            value_type: LlvmType::Integer64,
            llvm_name: "%process_handle".to_string(),
            is_constant: false,
        };
        
        // Test wait operation
        let wait_result = generator.compile_wait_process(&handle, None);
        assert!(wait_result.is_ok());
        
        // Test status check
        let status_result = generator.compile_process_status(&handle);
        assert!(status_result.is_ok());
        
        // Test kill operation  
        let kill_result = generator.compile_kill_process(&handle, None);
        assert!(kill_result.is_ok());
    }
}
