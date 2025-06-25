/// LLVM code generation for Inter-Process Communication (IPC) operations in CURSED
/// 
/// This module provides comprehensive LLVM integration for IPC mechanisms including
/// shared memory, named pipes, message queues, semaphores, and signals. It includes
/// FFI functions for compiled CURSED code to interact with the IPC system.

use crate::codegen::llvm::{LlvmCodeGenerator, expression_compiler::{LlvmValue, LlvmType}};
use crate::error::{CursedError, CursedError};

use std::collections::HashMap;
use inkwell::{
    values::{BasicValueEnum, PointerValue},
    AddressSpace,
    types::{BasicTypeEnum, IntType, PointerType},
};

// Type aliases for consistency
type Cursedcrate::error::Result<()>;

/// Trait for compiling IPC operations to LLVM IR
pub trait IpcCompiler {
    /// Compile shared memory operations
    fn compile_shared_memory_op(
        &mut self,
        operation: SharedMemoryOperation,
        name: &LlvmValue,
        size: Option<&LlvmValue>,
        data: Option<&LlvmValue>,
    ) -> crate::error::Result<()>;

    /// Compile named pipe operations
    fn compile_pipe_op(
        &mut self,
        operation: PipeOperation,
        name: BasicValueEnum,
        data: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum>;

    /// Compile message queue operations
    fn compile_message_queue_op(
        &mut self,
        operation: MessageQueueOperation,
        name: BasicValueEnum,
        message: Option<BasicValueEnum>,
        priority: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum>;

    /// Compile semaphore operations
    fn compile_semaphore_op(
        &mut self,
        operation: SemaphoreOperation,
        name: BasicValueEnum,
        count: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum>;

    /// Compile signal operations
    fn compile_signal_op(
        &mut self,
        operation: SignalOperation,
        signal: BasicValueEnum,
        target: Option<BasicValueEnum>,
        handler: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum>;

    /// Generate FFI function declarations for IPC operations
    fn declare_ipc_ffi_functions(&mut self) -> CursedResult<()>;
}

/// Shared memory operation types
#[derive(Debug, Clone, Copy)]
pub enum SharedMemoryOperation {
    Create,
    Open,
    Read,
    Write,
    Close,
    Remove,
}

/// Named pipe operation types
#[derive(Debug, Clone, Copy)]
pub enum PipeOperation {
    Create,
    Open,
    Read,
    Write,
    Close,
}

/// Message queue operation types
#[derive(Debug, Clone, Copy)]
pub enum MessageQueueOperation {
    Create,
    Open,
    Send,
    Receive,
    Peek,
    Close,
    Remove,
}

/// Semaphore operation types
#[derive(Debug, Clone, Copy)]
pub enum SemaphoreOperation {
    Create,
    Open,
    Acquire,
    Release,
    TryAcquire,
    Close,
    Remove,
}

/// Signal operation types
#[derive(Debug, Clone, Copy)]
pub enum SignalOperation {
    Send,
    Register,
    Block,
    Unblock,
    Wait,
}

impl IpcCompiler for LlvmCodeGenerator {
    fn compile_shared_memory_op(
        &mut self,
        operation: SharedMemoryOperation,
        name: &LlvmValue,
        size: Option<&LlvmValue>,
        data: Option<&LlvmValue>,
    ) -> crate::error::Result<()> {
        // Ensure FFI functions are declared
        self.declare_ipc_ffi_functions()?;

        let fn_name = match operation {
            SharedMemoryOperation::Create => "cursed_shm_create",
            SharedMemoryOperation::Open => "cursed_shm_open",
            SharedMemoryOperation::Read => "cursed_shm_read",
            SharedMemoryOperation::Write => "cursed_shm_write",
            SharedMemoryOperation::Close => "cursed_shm_close",
            SharedMemoryOperation::Remove => "cursed_shm_remove",
        };

        let result_name = self.next_temp_name();
        let name_ptr = &name.llvm_name;
        
        let size_val = if let Some(s) = size {
            &s.llvm_name
        } else {
            "0"
        };

        let data_ptr = if let Some(d) = data {
            &d.llvm_name
        } else {
            "null"
        };

        // Generate the IR call
        let ir = format!(
            "{} = call i64 @{}(i8* {}, i64 {}, i8* {})",
            result_name, fn_name, name_ptr, size_val, data_ptr
        );

        self.add_ir(&ir);

        Ok(LlvmValue {
            value_type: LlvmType::Integer64,
            llvm_name: result_name,
            is_constant: false,
        })
    }

    fn compile_pipe_op(
        &mut self,
        operation: PipeOperation,
        name: BasicValueEnum,
        data: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum> {
        let context = &self.context;
        let builder = &self.builder;
        let module = &self.module;

        let fn_name = match operation {
            PipeOperation::Create => "cursed_pipe_create",
            PipeOperation::Open => "cursed_pipe_open",
            PipeOperation::Read => "cursed_pipe_read",
            PipeOperation::Write => "cursed_pipe_write",
            PipeOperation::Close => "cursed_pipe_close",
        };

        let pipe_fn = module.get_function(fn_name)
            .ok_or_else(|| CursedError::codegen_error(
                "pipe_op",
                &format!("FFI function {} not found", fn_name)
            ))?;

        // Convert name to string pointer
        let name_ptr = self.ensure_string_pointer(name)?;

        // Convert data to pointer (or null if not provided)
        let data_ptr = if let Some(d) = data {
            self.ensure_data_pointer(d)?
        } else {
            context.i8_type().ptr_type(AddressSpace::default()).const_null()
        };

        // Call the FFI function
        let result = builder.build_call(
            pipe_fn,
            &[name_ptr.into(), data_ptr.into()],
            "pipe_result"
        ).map_err(|e| CursedError::codegen_error("pipe_op", &e.to_string()))?;

        Ok(result.try_as_basic_value().left().unwrap_or_else(|| {
            context.i64_type().const_int(0, false).into()
        }))
    }

    fn compile_message_queue_op(
        &mut self,
        operation: MessageQueueOperation,
        name: BasicValueEnum,
        message: Option<BasicValueEnum>,
        priority: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum> {
        let context = &self.context;
        let builder = &self.builder;
        let module = &self.module;

        let fn_name = match operation {
            MessageQueueOperation::Create => "cursed_mq_create",
            MessageQueueOperation::Open => "cursed_mq_open",
            MessageQueueOperation::Send => "cursed_mq_send",
            MessageQueueOperation::Receive => "cursed_mq_receive",
            MessageQueueOperation::Peek => "cursed_mq_peek",
            MessageQueueOperation::Close => "cursed_mq_close",
            MessageQueueOperation::Remove => "cursed_mq_remove",
        };

        let mq_fn = module.get_function(fn_name)
            .ok_or_else(|| CursedError::codegen_error(
                "message_queue_op",
                &format!("FFI function {} not found", fn_name)
            ))?;

        // Convert name to string pointer
        let name_ptr = self.ensure_string_pointer(name)?;

        // Convert message to pointer (or null if not provided)
        let message_ptr = if let Some(m) = message {
            self.ensure_data_pointer(m)?
        } else {
            context.i8_type().ptr_type(AddressSpace::default()).const_null()
        };

        // Convert priority to i32 (or 0 if not provided)
        let priority_i32 = if let Some(p) = priority {
            self.convert_to_int32(p)?
        } else {
            context.i32_type().const_int(0, false)
        };

        // Call the FFI function
        let result = builder.build_call(
            mq_fn,
            &[name_ptr.into(), message_ptr.into(), priority_i32.into()],
            "mq_result"
        ).map_err(|e| CursedError::codegen_error("message_queue_op", &e.to_string()))?;

        Ok(result.try_as_basic_value().left().unwrap_or_else(|| {
            context.i64_type().const_int(0, false).into()
        }))
    }

    fn compile_semaphore_op(
        &mut self,
        operation: SemaphoreOperation,
        name: BasicValueEnum,
        count: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum> {
        let context = &self.context;
        let builder = &self.builder;
        let module = &self.module;

        let fn_name = match operation {
            SemaphoreOperation::Create => "cursed_sem_create",
            SemaphoreOperation::Open => "cursed_sem_open",
            SemaphoreOperation::Acquire => "cursed_sem_acquire",
            SemaphoreOperation::Release => "cursed_sem_release",
            SemaphoreOperation::TryAcquire => "cursed_sem_try_acquire",
            SemaphoreOperation::Close => "cursed_sem_close",
            SemaphoreOperation::Remove => "cursed_sem_remove",
        };

        let sem_fn = module.get_function(fn_name)
            .ok_or_else(|| CursedError::codegen_error(
                "semaphore_op",
                &format!("FFI function {} not found", fn_name)
            ))?;

        // Convert name to string pointer
        let name_ptr = self.ensure_string_pointer(name)?;

        // Convert count to i32 (or 1 if not provided)
        let count_i32 = if let Some(c) = count {
            self.convert_to_int32(c)?
        } else {
            context.i32_type().const_int(1, false)
        };

        // Call the FFI function
        let result = builder.build_call(
            sem_fn,
            &[name_ptr.into(), count_i32.into()],
            "sem_result"
        ).map_err(|e| CursedError::codegen_error("semaphore_op", &e.to_string()))?;

        Ok(result.try_as_basic_value().left().unwrap_or_else(|| {
            context.i32_type().const_int(0, false).into()
        }))
    }

    fn compile_signal_op(
        &mut self,
        operation: SignalOperation,
        signal: BasicValueEnum,
        target: Option<BasicValueEnum>,
        handler: Option<BasicValueEnum>,
    ) -> CursedResult<BasicValueEnum> {
        let context = &self.context;
        let builder = &self.builder;
        let module = &self.module;

        let fn_name = match operation {
            SignalOperation::Send => "cursed_signal_send",
            SignalOperation::Register => "cursed_signal_register",
            SignalOperation::Block => "cursed_signal_block",
            SignalOperation::Unblock => "cursed_signal_unblock",
            SignalOperation::Wait => "cursed_signal_wait",
        };

        let signal_fn = module.get_function(fn_name)
            .ok_or_else(|| CursedError::codegen_error(
                "signal_op",
                &format!("FFI function {} not found", fn_name)
            ))?;

        // Convert signal to i32
        let signal_i32 = self.convert_to_int32(signal)?;

        // Convert target to i64 (PID, or 0 for self)
        let target_i64 = if let Some(t) = target {
            self.convert_to_int64(t)?
        } else {
            context.i64_type().const_int(0, false)
        };

        // Convert handler to pointer (or null if not provided)
        let handler_ptr = if let Some(h) = handler {
            self.ensure_function_pointer(h)?
        } else {
            context.i8_type().ptr_type(AddressSpace::default()).const_null()
        };

        // Call the FFI function
        let result = builder.build_call(
            signal_fn,
            &[signal_i32.into(), target_i64.into(), handler_ptr.into()],
            "signal_result"
        ).map_err(|e| CursedError::codegen_error("signal_op", &e.to_string()))?;

        Ok(result.try_as_basic_value().left().unwrap_or_else(|| {
            context.i32_type().const_int(0, false).into()
        }))
    }

    fn declare_ipc_ffi_functions(&mut self) -> CursedResult<()> {
        let context = &self.context;
        let module = &self.module;

        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let void_type = context.void_type();

        // Shared Memory Functions
        // cursed_shm_create(name: *const i8, size: i64, data: *const i8) -> i64
        let shm_create_fn_type = i64_type.fn_type(
            &[i8_ptr_type.into(), i64_type.into(), i8_ptr_type.into()],
            false
        );
        module.add_function("cursed_shm_create", shm_create_fn_type, None);
        module.add_function("cursed_shm_open", shm_create_fn_type, None);
        module.add_function("cursed_shm_read", shm_create_fn_type, None);
        module.add_function("cursed_shm_write", shm_create_fn_type, None);
        module.add_function("cursed_shm_close", shm_create_fn_type, None);
        module.add_function("cursed_shm_remove", shm_create_fn_type, None);

        // Named Pipe Functions
        // cursed_pipe_create(name: *const i8, data: *const i8) -> i64
        let pipe_fn_type = i64_type.fn_type(
            &[i8_ptr_type.into(), i8_ptr_type.into()],
            false
        );
        module.add_function("cursed_pipe_create", pipe_fn_type, None);
        module.add_function("cursed_pipe_open", pipe_fn_type, None);
        module.add_function("cursed_pipe_read", pipe_fn_type, None);
        module.add_function("cursed_pipe_write", pipe_fn_type, None);
        module.add_function("cursed_pipe_close", pipe_fn_type, None);

        // Message Queue Functions
        // cursed_mq_create(name: *const i8, message: *const i8, priority: i32) -> i64
        let mq_fn_type = i64_type.fn_type(
            &[i8_ptr_type.into(), i8_ptr_type.into(), i32_type.into()],
            false
        );
        module.add_function("cursed_mq_create", mq_fn_type, None);
        module.add_function("cursed_mq_open", mq_fn_type, None);
        module.add_function("cursed_mq_send", mq_fn_type, None);
        module.add_function("cursed_mq_receive", mq_fn_type, None);
        module.add_function("cursed_mq_peek", mq_fn_type, None);
        module.add_function("cursed_mq_close", mq_fn_type, None);
        module.add_function("cursed_mq_remove", mq_fn_type, None);

        // Semaphore Functions
        // cursed_sem_create(name: *const i8, count: i32) -> i32
        let sem_fn_type = i32_type.fn_type(
            &[i8_ptr_type.into(), i32_type.into()],
            false
        );
        module.add_function("cursed_sem_create", sem_fn_type, None);
        module.add_function("cursed_sem_open", sem_fn_type, None);
        module.add_function("cursed_sem_acquire", sem_fn_type, None);
        module.add_function("cursed_sem_release", sem_fn_type, None);
        module.add_function("cursed_sem_try_acquire", sem_fn_type, None);
        module.add_function("cursed_sem_close", sem_fn_type, None);
        module.add_function("cursed_sem_remove", sem_fn_type, None);

        // Signal Functions
        // cursed_signal_send(signal: i32, target: i64, handler: *const i8) -> i32
        let signal_fn_type = i32_type.fn_type(
            &[i32_type.into(), i64_type.into(), i8_ptr_type.into()],
            false
        );
        module.add_function("cursed_signal_send", signal_fn_type, None);
        module.add_function("cursed_signal_register", signal_fn_type, None);
        module.add_function("cursed_signal_block", signal_fn_type, None);
        module.add_function("cursed_signal_unblock", signal_fn_type, None);
        module.add_function("cursed_signal_wait", signal_fn_type, None);

        Ok(())
    }
}

impl LlvmCodeGenerator {
    /// Helper to ensure a value is a string pointer
    fn ensure_string_pointer(&self, value: BasicValueEnum) -> CursedResult<PointerValue> {
        match value {
            BasicValueEnum::PointerValue(ptr) => Ok(ptr),
            BasicValueEnum::ArrayValue(arr) => {
                let ptr = self.builder.build_bitcast(
                    arr,
                    self.context.i8_type().ptr_type(AddressSpace::default()),
                    "string_ptr"
                ).map_err(|e| CursedError::codegen_error("ensure_string_pointer", &e.to_string()))?;
                Ok(ptr.into_pointer_value())
            }
            _ => {
                // For other types, create a string representation and return pointer
                let str_val = format!("{:?}", value);
                let global_str = self.builder.build_global_string(&str_val, "temp_str")
                    .map_err(|e| CursedError::codegen_error("ensure_string_pointer", &e.to_string()))?;
                Ok(global_str.as_pointer_value())
            }
        }
    }

    /// Helper to convert a value to i32
    fn convert_to_int32(&self, value: BasicValueEnum) -> CursedResult<inkwell::values::IntValue> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                match int_val.get_type().get_bit_width() {
                    32 => Ok(int_val),
                    64 => {
                        // Truncate i64 to i32
                        let i32_type = self.context.i32_type();
                        self.builder.build_int_truncate(int_val, i32_type, "trunc_i32")
                            .map_err(|e| CursedError::codegen_error("convert_to_int32", &e.to_string()))
                    }
                    8 | 16 => {
                        // Zero extend to i32
                        let i32_type = self.context.i32_type();
                        self.builder.build_int_z_extend(int_val, i32_type, "zext_i32")
                            .map_err(|e| CursedError::codegen_error("convert_to_int32", &e.to_string()))
                    }
                    _ => Err(CursedError::codegen_error("convert_to_int32", "Unsupported integer width"))
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                // Convert float to i32
                let i32_type = self.context.i32_type();
                self.builder.build_float_to_signed_int(float_val, i32_type, "float_to_i32")
                    .map_err(|e| CursedError::codegen_error("convert_to_int32", &e.to_string()))
            }
            _ => Err(CursedError::codegen_error("convert_to_int32", "Cannot convert value to i32"))
        }
    }

    /// Helper to convert a value to i64
    fn convert_to_int64(&self, value: BasicValueEnum) -> CursedResult<inkwell::values::IntValue> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                match int_val.get_type().get_bit_width() {
                    64 => Ok(int_val),
                    32 | 16 | 8 => {
                        // Zero extend to i64
                        let i64_type = self.context.i64_type();
                        self.builder.build_int_z_extend(int_val, i64_type, "zext_i64")
                            .map_err(|e| CursedError::codegen_error("convert_to_int64", &e.to_string()))
                    }
                    _ => Err(CursedError::codegen_error("convert_to_int64", "Unsupported integer width"))
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                // Convert float to i64
                let i64_type = self.context.i64_type();
                self.builder.build_float_to_signed_int(float_val, i64_type, "float_to_i64")
                    .map_err(|e| CursedError::codegen_error("convert_to_int64", &e.to_string()))
            }
            _ => Err(CursedError::codegen_error("convert_to_int64", "Cannot convert value to i64"))
        }
    }

    /// Helper to ensure a value is a data pointer
    fn ensure_data_pointer(&self, value: BasicValueEnum) -> CursedResult<PointerValue> {
        match value {
            BasicValueEnum::PointerValue(ptr) => Ok(ptr),
            BasicValueEnum::ArrayValue(arr) => {
                let ptr = self.builder.build_bitcast(
                    arr,
                    self.context.i8_type().ptr_type(AddressSpace::default()),
                    "data_ptr"
                ).map_err(|e| CursedError::codegen_error("ensure_data_pointer", &e.to_string()))?;
                Ok(ptr.into_pointer_value())
            }
            _ => {
                // Store value on stack and return pointer
                let alloca = self.builder.build_alloca(value.get_type(), "data_alloca")
                    .map_err(|e| CursedError::codegen_error("ensure_data_pointer", &e.to_string()))?;
                self.builder.build_store(alloca, value)
                    .map_err(|e| CursedError::codegen_error("ensure_data_pointer", &e.to_string()))?;
                Ok(alloca)
            }
        }
    }

    /// Helper to ensure a value is a function pointer
    fn ensure_function_pointer(&self, value: BasicValueEnum) -> CursedResult<PointerValue> {
        match value {
            BasicValueEnum::PointerValue(ptr) => Ok(ptr),
            _ => Err(CursedError::codegen_error(
                "ensure_function_pointer",
                "Value cannot be converted to function pointer"
            ))
        }
    }
}

/// FFI functions for IPC operations (to be implemented in the runtime)

// Shared Memory FFI Functions
#[no_mangle]
pub extern "C" fn cursed_shm_create(name: *const i8, size: i64, _data: *const i8) -> i64 {
//     use crate::stdlib::ipc::{create_shared_memory, SharedMemoryConfig};
    use std::ffi::CStr;

    if name.is_null() || size <= 0 {
        return -1; // CursedError: invalid parameters
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    let config = match SharedMemoryConfig::new(name_str, size as usize) {
        Ok(cfg) => cfg,
        Err(_) => return -3, // CursedError: invalid config
    };

    match create_shared_memory(config) {
        Ok(shm) => store_shm_handle(shm) as i64,
        Err(_) => -4, // CursedError: creation failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_shm_open(name: *const i8, _size: i64, _data: *const i8) -> i64 {
//     use crate::stdlib::ipc::open_shared_memory;
    use std::ffi::CStr;

    if name.is_null() {
        return -1; // CursedError: invalid parameters
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match open_shared_memory(name_str) {
        Ok(shm) => store_shm_handle(shm) as i64,
        Err(_) => -3, // CursedError: open failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_shm_read(name: *const i8, size: i64, data: *const i8) -> i64 {
    if name.is_null() || data.is_null() || size <= 0 {
        return -1;
    }

    let handle = name as usize; // Simplified handle system
    match get_shm_handle_mut(handle) {
        Some(shm) => {
            match shm.read_bytes() {
                Ok(bytes) => {
                    let len = bytes.len().min(size as usize);
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            bytes.as_ptr(),
                            data as *mut u8,
                            len
                        );
                    }
                    len as i64
                }
                Err(_) => -2, // CursedError: read failed
            }
        }
        None => -3, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_shm_write(name: *const i8, size: i64, data: *const i8) -> i64 {
    if name.is_null() || data.is_null() || size <= 0 {
        return -1;
    }

    let handle = name as usize; // Simplified handle system
    let data_slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, size as usize)
    };

    match get_shm_handle_mut(handle) {
        Some(shm) => {
            match shm.write_bytes(data_slice) {
                Ok(()) => size,
                Err(_) => -2, // CursedError: write failed
            }
        }
        None => -3, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_shm_close(name: *const i8, _size: i64, _data: *const i8) -> i64 {
    let handle = name as usize;
    match remove_shm_handle(handle) {
        Some(_) => 0, // Success
        None => -1,   // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_shm_remove(name: *const i8, _size: i64, _data: *const i8) -> i64 {
//     use crate::stdlib::ipc::remove_shared_memory;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match remove_shared_memory(name_str) {
        Ok(()) => 0, // Success
        Err(_) => -3, // CursedError: remove failed
    }
}

// Named Pipe FFI Functions
#[no_mangle]
pub extern "C" fn cursed_pipe_create(name: *const i8, _data: *const i8) -> i64 {
//     use crate::stdlib::ipc::{create_named_pipe, PipeMode};
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match create_named_pipe(name_str, PipeMode::ReadWrite) {
        Ok(pipe) => store_pipe_handle(pipe) as i64,
        Err(_) => -3, // CursedError: creation failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_pipe_open(name: *const i8, _data: *const i8) -> i64 {
//     use crate::stdlib::ipc::{open_pipe, PipeMode};
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match open_pipe(name_str, PipeMode::ReadWrite) {
        Ok(pipe) => store_pipe_handle(pipe) as i64,
        Err(_) => -3, // CursedError: open failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_pipe_read(name: *const i8, data: *const i8) -> i64 {
    if name.is_null() || data.is_null() {
        return -1;
    }

    let handle = name as usize;
    match get_pipe_handle_mut(handle) {
        Some(pipe) => {
            match pipe.read_string() {
                Ok(string) => {
                    let bytes = string.as_bytes();
                    let len = bytes.len().min(1024); // Max 1KB
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            bytes.as_ptr(),
                            data as *mut u8,
                            len
                        );
                    }
                    len as i64
                }
                Err(_) => -2, // CursedError: read failed
            }
        }
        None => -3, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_pipe_write(name: *const i8, data: *const i8) -> i64 {
    if name.is_null() || data.is_null() {
        return -1;
    }

    let data_str = unsafe {
        match std::ffi::CStr::from_ptr(data).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    let handle = name as usize;
    match get_pipe_handle_mut(handle) {
        Some(pipe) => {
            match pipe.write(data_str) {
                Ok(()) => data_str.len() as i64,
                Err(_) => -3, // CursedError: write failed
            }
        }
        None => -4, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_pipe_close(name: *const i8, _data: *const i8) -> i64 {
    let handle = name as usize;
    match remove_pipe_handle(handle) {
        Some(_) => 0, // Success
        None => -1,   // CursedError: invalid handle
    }
}

// Message Queue FFI Functions
#[no_mangle]
pub extern "C" fn cursed_mq_create(name: *const i8, _message: *const i8, _priority: i32) -> i64 {
//     use crate::stdlib::ipc::create_message_queue;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match create_message_queue(name_str, 10) {
        Ok(mq) => store_mq_handle(mq) as i64,
        Err(_) => -3, // CursedError: creation failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_send(name: *const i8, message: *const i8, priority: i32) -> i64 {
//     use crate::stdlib::ipc::{Message, MessagePriority};
    use std::ffi::CStr;

    if name.is_null() || message.is_null() {
        return -1;
    }

    let message_str = unsafe {
        match CStr::from_ptr(message).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    let msg_priority = match priority {
        0 => MessagePriority::Low,
        1 => MessagePriority::Medium,
        2 => MessagePriority::High,
        _ => MessagePriority::Medium,
    };

    let handle = name as usize;
    match get_mq_handle_mut(handle) {
        Some(mq) => {
            let msg = match Message::new(message_str, msg_priority) {
                Ok(m) => m,
                Err(_) => return -3, // CursedError: message creation failed
            };

            match mq.send(msg) {
                Ok(()) => 0, // Success
                Err(_) => -4, // CursedError: send failed
            }
        }
        None => -5, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_open(name: *const i8, _message: *const i8, _priority: i32) -> i64 {
//     use crate::stdlib::ipc::open_message_queue;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match open_message_queue(name_str) {
        Ok(mq) => store_mq_handle(mq) as i64,
        Err(_) => -3, // CursedError: open failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_receive(name: *const i8, message: *const i8, _priority: i32) -> i64 {
    if name.is_null() || message.is_null() {
        return -1;
    }

    let handle = name as usize;
    match get_mq_handle_mut(handle) {
        Some(mq) => {
            match mq.receive() {
                Ok(msg) => {
                    let content = msg.content();
                    let len = content.len().min(1024); // Max 1KB
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            content.as_ptr(),
                            message as *mut u8,
                            len
                        );
                    }
                    len as i64
                }
                Err(_) => -2, // CursedError: receive failed
            }
        }
        None => -3, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_peek(name: *const i8, message: *const i8, _priority: i32) -> i64 {
    if name.is_null() || message.is_null() {
        return -1;
    }

    let handle = name as usize;
    match get_mq_handle_mut(handle) {
        Some(mq) => {
            match mq.peek() {
                Ok(Some(msg)) => {
                    let content = msg.content();
                    let len = content.len().min(1024); // Max 1KB
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            content.as_ptr(),
                            message as *mut u8,
                            len
                        );
                    }
                    len as i64
                }
                Ok(None) => 0, // No message available
                Err(_) => -2, // CursedError: peek failed
            }
        }
        None => -3, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_close(name: *const i8, _message: *const i8, _priority: i32) -> i64 {
    let handle = name as usize;
    match remove_mq_handle(handle) {
        Some(_) => 0, // Success
        None => -1,   // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_mq_remove(name: *const i8, _message: *const i8, _priority: i32) -> i64 {
//     use crate::stdlib::ipc::remove_message_queue;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match remove_message_queue(name_str) {
        Ok(()) => 0, // Success
        Err(_) => -3, // CursedError: remove failed
    }
}

// Semaphore FFI Functions
#[no_mangle]
pub extern "C" fn cursed_sem_create(name: *const i8, count: i32) -> i32 {
//     use crate::stdlib::ipc::create_semaphore;
    use std::ffi::CStr;

    if name.is_null() || count < 0 {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match create_semaphore(name_str, count as u32) {
        Ok(sem) => store_sem_handle(sem) as i32,
        Err(_) => -3, // CursedError: creation failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_open(name: *const i8, _count: i32) -> i32 {
//     use crate::stdlib::ipc::open_semaphore;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match open_semaphore(name_str) {
        Ok(sem) => store_sem_handle(sem) as i32,
        Err(_) => -3, // CursedError: open failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_acquire(name: *const i8, _count: i32) -> i32 {
    let handle = name as usize;
    match get_sem_handle_mut(handle) {
        Some(sem) => {
            match sem.acquire() {
                Ok(()) => 0, // Success
                Err(_) => -2, // CursedError: acquire failed
            }
        }
        None => -1, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_release(name: *const i8, _count: i32) -> i32 {
    let handle = name as usize;
    match get_sem_handle_mut(handle) {
        Some(sem) => {
            match sem.release() {
                Ok(()) => 0, // Success
                Err(_) => -2, // CursedError: release failed
            }
        }
        None => -1, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_try_acquire(name: *const i8, _count: i32) -> i32 {
    let handle = name as usize;
    match get_sem_handle_mut(handle) {
        Some(sem) => {
            match sem.try_acquire() {
                Ok(true) => 1,  // Success: acquired
                Ok(false) => 0, // Would block: semaphore unavailable
                Err(_) => -2,   // CursedError: try_acquire failed
            }
        }
        None => -1, // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_close(name: *const i8, _count: i32) -> i32 {
    let handle = name as usize;
    match remove_sem_handle(handle) {
        Some(_) => 0, // Success
        None => -1,   // CursedError: invalid handle
    }
}

#[no_mangle]
pub extern "C" fn cursed_sem_remove(name: *const i8, _count: i32) -> i32 {
//     use crate::stdlib::ipc::remove_semaphore;
    use std::ffi::CStr;

    if name.is_null() {
        return -1;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // CursedError: invalid UTF-8
        }
    };

    match remove_semaphore(name_str) {
        Ok(()) => 0, // Success
        Err(_) => -3, // CursedError: remove failed
    }
}

// Signal handling FFI Functions
#[no_mangle]
pub extern "C" fn cursed_signal_send(signal: i32, target: i64, _handler: *const i8) -> i32 {
//     use crate::stdlib::ipc::send_signal;

    let target_pid = if target == 0 {
        std::process::id() as u64
    } else {
        target as u64
    };

    // Convert i32 signal to Signal enum (simplified mapping)
    let signal_enum = match signal {
//         1 => crate::stdlib::ipc::Signal::SIGHUP,
//         2 => crate::stdlib::ipc::Signal::SIGINT,
//         9 => crate::stdlib::ipc::Signal::SIGKILL,
//         15 => crate::stdlib::ipc::Signal::SIGTERM,
//         10 => crate::stdlib::ipc::Signal::SIGUSR1,
//         12 => crate::stdlib::ipc::Signal::SIGUSR2,
        _ => return -1, // Unsupported signal
    };

    match send_signal(target_pid, signal_enum) {
        Ok(()) => 0, // Success
        Err(_) => -2, // CursedError: send failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_signal_register(signal: i32, _target: i64, handler: *const i8) -> i32 {
//     use crate::stdlib::ipc::{register_signal_handler, Signal, SignalAction};

    if handler.is_null() {
        return -1;
    }

    // Convert i32 signal to Signal enum (simplified mapping)
    let signal_enum = match signal {
        1 => Signal::SIGHUP,
        2 => Signal::SIGINT,
        9 => Signal::SIGKILL,
        15 => Signal::SIGTERM,
        10 => Signal::SIGUSR1,
        12 => Signal::SIGUSR2,
        _ => return -1, // Unsupported signal
    };

    // For simplicity, use a default handler (in production this would be more complex)
    let action = SignalAction::Default;

    match register_signal_handler(signal_enum, action) {
        Ok(()) => 0, // Success
        Err(_) => -2, // CursedError: registration failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_signal_block(signal: i32, _target: i64, _handler: *const i8) -> i32 {
//     use crate::stdlib::ipc::{block_signal, Signal};

    // Convert i32 signal to Signal enum (simplified mapping)
    let signal_enum = match signal {
        1 => Signal::SIGHUP,
        2 => Signal::SIGINT,
        9 => Signal::SIGKILL,
        15 => Signal::SIGTERM,
        10 => Signal::SIGUSR1,
        12 => Signal::SIGUSR2,
        _ => return -1, // Unsupported signal
    };

    match block_signal(signal_enum) {
        Ok(()) => 0, // Success
        Err(_) => -2, // CursedError: block failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_signal_unblock(signal: i32, _target: i64, _handler: *const i8) -> i32 {
//     use crate::stdlib::ipc::{unblock_signal, Signal};

    // Convert i32 signal to Signal enum (simplified mapping)
    let signal_enum = match signal {
        1 => Signal::SIGHUP,
        2 => Signal::SIGINT,
        9 => Signal::SIGKILL,
        15 => Signal::SIGTERM,
        10 => Signal::SIGUSR1,
        12 => Signal::SIGUSR2,
        _ => return -1, // Unsupported signal
    };

    match unblock_signal(signal_enum) {
        Ok(()) => 0, // Success
        Err(_) => -2, // CursedError: unblock failed
    }
}

#[no_mangle]
pub extern "C" fn cursed_signal_wait(signal: i32, _target: i64, _handler: *const i8) -> i32 {
//     use crate::stdlib::ipc::{wait_for_signal, Signal};

    // Convert i32 signal to Signal enum (simplified mapping)
    let signal_enum = match signal {
        1 => Signal::SIGHUP,
        2 => Signal::SIGINT,
        9 => Signal::SIGKILL,
        15 => Signal::SIGTERM,
        10 => Signal::SIGUSR1,
        12 => Signal::SIGUSR2,
        _ => return -1, // Unsupported signal
    };

    match wait_for_signal(signal_enum) {
        Ok(received_signal) => {
            // Return the signal number that was received
            match received_signal {
                Signal::SIGHUP => 1,
                Signal::SIGINT => 2,
                Signal::SIGKILL => 9,
                Signal::SIGTERM => 15,
                Signal::SIGUSR1 => 10,
                Signal::SIGUSR2 => 12,
                _ => signal, // Return original signal if mapping not found
            }
        }
        Err(_) => -2, // CursedError: wait failed
    }
}

// Global IPC handle registries
use std::sync::Mutex;
// use crate::stdlib::ipc::{SharedMemory, NamedPipe, MessageQueue, Semaphore};

lazy_static::lazy_static! {
    static ref SHM_REGISTRY: Mutex<HashMap<usize, SharedMemory>> = Mutex::new(HashMap::new());
    static ref PIPE_REGISTRY: Mutex<HashMap<usize, NamedPipe>> = Mutex::new(HashMap::new());
    static ref MQ_REGISTRY: Mutex<HashMap<usize, MessageQueue>> = Mutex::new(HashMap::new());
    static ref SEM_REGISTRY: Mutex<HashMap<usize, Semaphore>> = Mutex::new(HashMap::new());
    static ref NEXT_IPC_HANDLE: Mutex<usize> = Mutex::new(1);
}

fn store_shm_handle(shm: SharedMemory) -> usize {
    let mut registry = SHM_REGISTRY.lock().unwrap();
    let mut next_handle = NEXT_IPC_HANDLE.lock().unwrap();
    
    let handle = *next_handle;
    *next_handle += 1;
    
    registry.insert(handle, shm);
    handle
}

fn get_shm_handle_mut(handle: usize) -> Option<SharedMemory> {
    let registry = SHM_REGISTRY.lock().unwrap();
    registry.get(&handle).cloned()
}

fn remove_shm_handle(handle: usize) -> Option<SharedMemory> {
    let mut registry = SHM_REGISTRY.lock().unwrap();
    registry.remove(&handle)
}

fn store_pipe_handle(pipe: NamedPipe) -> usize {
    let mut registry = PIPE_REGISTRY.lock().unwrap();
    let mut next_handle = NEXT_IPC_HANDLE.lock().unwrap();
    
    let handle = *next_handle;
    *next_handle += 1;
    
    registry.insert(handle, pipe);
    handle
}

fn get_pipe_handle_mut(handle: usize) -> Option<NamedPipe> {
    let registry = PIPE_REGISTRY.lock().unwrap();
    registry.get(&handle).cloned()
}

fn remove_pipe_handle(handle: usize) -> Option<NamedPipe> {
    let mut registry = PIPE_REGISTRY.lock().unwrap();
    registry.remove(&handle)
}

fn store_mq_handle(mq: MessageQueue) -> usize {
    let mut registry = MQ_REGISTRY.lock().unwrap();
    let mut next_handle = NEXT_IPC_HANDLE.lock().unwrap();
    
    let handle = *next_handle;
    *next_handle += 1;
    
    registry.insert(handle, mq);
    handle
}

fn get_mq_handle_mut(handle: usize) -> Option<MessageQueue> {
    let registry = MQ_REGISTRY.lock().unwrap();
    registry.get(&handle).cloned()
}

fn store_sem_handle(sem: Semaphore) -> usize {
    let mut registry = SEM_REGISTRY.lock().unwrap();
    let mut next_handle = NEXT_IPC_HANDLE.lock().unwrap();
    
    let handle = *next_handle;
    *next_handle += 1;
    
    registry.insert(handle, sem);
    handle
}

fn get_sem_handle_mut(handle: usize) -> Option<Semaphore> {
    let registry = SEM_REGISTRY.lock().unwrap();
    registry.get(&handle).cloned()
}

fn remove_sem_handle(handle: usize) -> Option<Semaphore> {
    let mut registry = SEM_REGISTRY.lock().unwrap();
    registry.remove(&handle)
}

fn remove_mq_handle(handle: usize) -> Option<MessageQueue> {
    let mut registry = MQ_REGISTRY.lock().unwrap();
    registry.remove(&handle)
}

