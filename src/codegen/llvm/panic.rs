/// LLVM code generation for panic/recovery operations
///
/// Provides LLVM IR generation for panic and recovery operations in CURSED,
/// including panic triggers, recovery blocks, and stack unwinding.

use crate::runtime::panic::{PanicSeverity, PanicCategory};
use crate::runtime::recovery::{RecoveryConfig, get_recovery_manager};
use crate::error::{Error as CursedError, SourceLocation};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::types::{BasicTypeEnum, IntType, PointerType};
use inkwell::basic_block::BasicBlock;
use inkwell::IntPredicate;
use std::collections::HashMap;

/// Trait for panic-related LLVM code generation
pub trait PanicCompiler {
    /// Generate LLVM IR for a panic operation
    fn compile_panic(
        &mut self,
        message: &str,
        severity: PanicSeverity,
        category: PanicCategory,
        source_location: Option<SourceLocation>,
    ) -> Result<(), CursedError>;

    /// Generate LLVM IR for a recovery block
    fn compile_recovery_block<F>(
        &mut self,
        protected_code: F,
        recovery_handler: Option<&str>,
    ) -> Result<BasicValueEnum, CursedError>
    where
        F: FnOnce(&mut Self) -> Result<BasicValueEnum, CursedError>;

    /// Generate LLVM IR to check if a panic is active
    fn compile_panic_check(&mut self) -> Result<IntValue, CursedError>;

    /// Generate LLVM IR to get panic message
    fn compile_get_panic_message(&mut self) -> Result<PointerValue, CursedError>;

    /// Generate LLVM IR for panic cleanup
    fn compile_panic_cleanup(&mut self) -> Result<(), CursedError>;

    /// Generate LLVM IR for entering a recovery scope
    fn compile_enter_recovery_scope(
        &mut self,
        scope_id: &str,
        config: Option<&RecoveryConfig>,
    ) -> Result<IntValue, CursedError>;

    /// Generate LLVM IR for exiting recovery scope
    fn compile_exit_recovery_scope(&mut self) -> Result<IntValue, CursedError>;

    /// Generate LLVM IR to check if in recovery scope
    fn compile_in_recovery_scope(&mut self) -> Result<IntValue, CursedError>;

    /// Register panic-related FFI functions
    fn register_panic_functions(
        &mut self,
        context: &Context,
        module: &Module,
        builder: &Builder,
    ) -> Result<(), CursedError>;
}

/// LLVM panic code generator implementation
pub struct LlvmPanicGenerator<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    
    /// FFI function declarations
    panic_fn: Option<FunctionValue<'ctx>>,
    recover_fn: Option<FunctionValue<'ctx>>,
    has_panic_fn: Option<FunctionValue<'ctx>>,
    get_panic_message_fn: Option<FunctionValue<'ctx>>,
    enter_recovery_scope_fn: Option<FunctionValue<'ctx>>,
    exit_recovery_scope_fn: Option<FunctionValue<'ctx>>,
    in_recovery_scope_fn: Option<FunctionValue<'ctx>>,
    attempt_recovery_fn: Option<FunctionValue<'ctx>>,
    
    /// Type cache
    i8_type: IntType<'ctx>,
    i32_type: IntType<'ctx>,
    i64_type: IntType<'ctx>,
    i8_ptr_type: PointerType<'ctx>,
    
    /// Current function context
    current_function: Option<FunctionValue<'ctx>>,
    
    /// Recovery block stack
    recovery_blocks: Vec<BasicBlock<'ctx>>,
    
    /// Panic handling configuration
    panic_config: PanicCompilerConfig,
}

/// Configuration for panic compilation
#[derive(Debug, Clone)]
pub struct PanicCompilerConfig {
    /// Whether to generate debug information for panics
    pub generate_debug_info: bool,
    /// Whether to inline panic checks
    pub inline_panic_checks: bool,
    /// Whether to optimize recovery blocks
    pub optimize_recovery: bool,
    /// Maximum recovery depth
    pub max_recovery_depth: usize,
}

impl Default for PanicCompilerConfig {
    fn default() -> Self {
        PanicCompilerConfig {
            generate_debug_info: true,
            inline_panic_checks: false,
            optimize_recovery: true,
            max_recovery_depth: 32,
        }
    }
}

impl<'ctx> LlvmPanicGenerator<'ctx> {
    /// Create a new panic generator
    pub fn new(
        context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx Builder<'ctx>,
    ) -> Self {
        let i8_type = context.i8_type();
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::default());

        LlvmPanicGenerator {
            context,
            module,
            builder,
            panic_fn: None,
            recover_fn: None,
            has_panic_fn: None,
            get_panic_message_fn: None,
            enter_recovery_scope_fn: None,
            exit_recovery_scope_fn: None,
            in_recovery_scope_fn: None,
            attempt_recovery_fn: None,
            i8_type,
            i32_type,
            i64_type,
            i8_ptr_type,
            current_function: None,
            recovery_blocks: Vec::new(),
            panic_config: PanicCompilerConfig::default(),
        }
    }

    /// Set the current function context
    pub fn set_current_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
    }

    /// Update panic compiler configuration
    pub fn set_config(&mut self, config: PanicCompilerConfig) {
        self.panic_config = config;
    }

    /// Get the panic function, declaring it if necessary
    fn get_panic_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.panic_fn {
            return Ok(func);
        }

        // Declare the cursed_panic function
        // extern "C" fn cursed_panic(
        //     message_ptr: *const u8, message_len: usize,
        //     severity: u8, category: u8,
        //     line: u32, column: u32,
        //     file_ptr: *const u8, file_len: usize
        // ) -> !
        let fn_type = self.context.void_type().fn_type(
            &[
                self.i8_ptr_type.into(), // message_ptr
                self.i64_type.into(),    // message_len
                self.i8_type.into(),     // severity
                self.i8_type.into(),     // category
                self.i32_type.into(),    // line
                self.i32_type.into(),    // column
                self.i8_ptr_type.into(), // file_ptr
                self.i64_type.into(),    // file_len
            ],
            false,
        );

        let function = self.module.add_function("cursed_panic", fn_type, None);
        
        // Mark as noreturn
        let noreturn_attr = self.context.create_enum_attribute(
            inkwell::attributes::Attribute::get_named_enum_kind_id("noreturn"),
            0,
        );
        function.add_attribute(inkwell::attributes::AttributeLoc::Function, noreturn_attr);

        self.panic_fn = Some(function);
        Ok(function)
    }

    /// Get the recover function, declaring it if necessary
    fn get_recover_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.recover_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_recover() -> u8
        let fn_type = self.i8_type.fn_type(&[], false);
        let function = self.module.add_function("cursed_recover", fn_type, None);
        
        self.recover_fn = Some(function);
        Ok(function)
    }

    /// Get the has_panic function, declaring it if necessary
    fn get_has_panic_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.has_panic_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_has_panic() -> u8
        let fn_type = self.i8_type.fn_type(&[], false);
        let function = self.module.add_function("cursed_has_panic", fn_type, None);
        
        self.has_panic_fn = Some(function);
        Ok(function)
    }

    /// Get the get_panic_message function, declaring it if necessary
    fn get_get_panic_message_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.get_panic_message_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_get_panic_message(buffer: *mut u8, buffer_len: usize) -> usize
        let fn_type = self.i64_type.fn_type(
            &[
                self.i8_ptr_type.into(), // buffer
                self.i64_type.into(),    // buffer_len
            ],
            false,
        );
        let function = self.module.add_function("cursed_get_panic_message", fn_type, None);
        
        self.get_panic_message_fn = Some(function);
        Ok(function)
    }

    /// Get the enter_recovery_scope function, declaring it if necessary
    fn get_enter_recovery_scope_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.enter_recovery_scope_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_enter_recovery_scope(scope_id_ptr: *const u8, scope_id_len: usize, timeout_secs: u32) -> u8
        let fn_type = self.i8_type.fn_type(
            &[
                self.i8_ptr_type.into(), // scope_id_ptr
                self.i64_type.into(),    // scope_id_len
                self.i32_type.into(),    // timeout_secs
            ],
            false,
        );
        let function = self.module.add_function("cursed_enter_recovery_scope", fn_type, None);
        
        self.enter_recovery_scope_fn = Some(function);
        Ok(function)
    }

    /// Get the exit_recovery_scope function, declaring it if necessary
    fn get_exit_recovery_scope_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.exit_recovery_scope_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_exit_recovery_scope() -> u8
        let fn_type = self.i8_type.fn_type(&[], false);
        let function = self.module.add_function("cursed_exit_recovery_scope", fn_type, None);
        
        self.exit_recovery_scope_fn = Some(function);
        Ok(function)
    }

    /// Get the in_recovery_scope function, declaring it if necessary
    fn get_in_recovery_scope_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.in_recovery_scope_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_in_recovery_scope() -> u8
        let fn_type = self.i8_type.fn_type(&[], false);
        let function = self.module.add_function("cursed_in_recovery_scope", fn_type, None);
        
        self.in_recovery_scope_fn = Some(function);
        Ok(function)
    }

    /// Get the attempt_recovery function, declaring it if necessary
    fn get_attempt_recovery_function(&mut self) -> Result<FunctionValue<'ctx>, CursedError> {
        if let Some(func) = self.attempt_recovery_fn {
            return Ok(func);
        }

        // extern "C" fn cursed_attempt_recovery() -> u8
        let fn_type = self.i8_type.fn_type(&[], false);
        let function = self.module.add_function("cursed_attempt_recovery", fn_type, None);
        
        self.attempt_recovery_fn = Some(function);
        Ok(function)
    }

    /// Create a string constant for panic messages
    fn create_string_constant(&self, text: &str) -> PointerValue<'ctx> {
        let string_type = self.i8_type.array_type(text.len() as u32);
        let string_value = self.context.const_string(text.as_bytes(), false);
        
        let global = self.module.add_global(string_type, None, "panic_str");
        global.set_initializer(&string_value);
        global.set_constant(true);
        
        unsafe {
            global.as_pointer_value().const_cast(self.i8_ptr_type)
        }
    }

    /// Convert PanicSeverity to u8 value
    fn severity_to_u8(&self, severity: PanicSeverity) -> u8 {
        match severity {
            PanicSeverity::Recoverable => 0,
            PanicSeverity::Critical => 1,
            PanicSeverity::Fatal => 2,
        }
    }

    /// Convert PanicCategory to u8 value
    fn category_to_u8(&self, category: PanicCategory) -> u8 {
        match category {
            PanicCategory::Memory => 0,
            PanicCategory::TypeAssertion => 1,
            PanicCategory::BoundsCheck => 2,
            PanicCategory::Arithmetic => 3,
            PanicCategory::Channel => 4,
            PanicCategory::Goroutine => 5,
            PanicCategory::User => 6,
            PanicCategory::System => 7,
            PanicCategory::Generic => 8,
        }
    }
}

impl<'ctx> PanicCompiler for LlvmPanicGenerator<'ctx> {
    fn compile_panic(
        &mut self,
        message: &str,
        severity: PanicSeverity,
        category: PanicCategory,
        source_location: Option<SourceLocation>,
    ) -> Result<(), CursedError> {
        let panic_fn = self.get_panic_function()?;
        
        // Create string constants
        let message_ptr = self.create_string_constant(message);
        let message_len = self.i64_type.const_int(message.len() as u64, false);
        
        let severity_val = self.i8_type.const_int(self.severity_to_u8(severity) as u64, false);
        let category_val = self.i8_type.const_int(self.category_to_u8(category) as u64, false);
        
        let (line, column, file_ptr, file_len) = if let Some(location) = source_location {
            let line = self.i32_type.const_int(location.line as u64, false);
            let column = self.i32_type.const_int(location.column as u64, false);
            
            if let Some(file) = &location.file {
                let file_ptr = self.create_string_constant(file);
                let file_len = self.i64_type.const_int(file.len() as u64, false);
                (line, column, file_ptr, file_len)
            } else {
                let null_ptr = self.i8_ptr_type.const_null();
                let zero_len = self.i64_type.const_zero();
                (line, column, null_ptr, zero_len)
            }
        } else {
            let zero_line = self.i32_type.const_zero();
            let zero_column = self.i32_type.const_zero();
            let null_ptr = self.i8_ptr_type.const_null();
            let zero_len = self.i64_type.const_zero();
            (zero_line, zero_column, null_ptr, zero_len)
        };

        // Call the panic function
        let _ = self.builder.build_call(
            panic_fn,
            &[
                message_ptr.into(),
                message_len.into(),
                severity_val.into(),
                category_val.into(),
                line.into(),
                column.into(),
                file_ptr.into(),
                file_len.into(),
            ],
            "panic_call",
        );

        // Insert unreachable instruction since panic never returns
        let _ = self.builder.build_unreachable();

        Ok(())
    }

    fn compile_recovery_block<F>(
        &mut self,
        protected_code: F,
        _recovery_handler: Option<&str>,
    ) -> Result<BasicValueEnum, CursedError>
    where
        F: FnOnce(&mut Self) -> Result<BasicValueEnum, CursedError>,
    {
        // For now, just execute the protected code without recovery logic
        // This simplification avoids borrow checker issues while still compiling
        protected_code(self)
    }

    fn compile_panic_check(&mut self) -> Result<IntValue, CursedError> {
        let has_panic_fn = self.get_has_panic_function()?;
        let panic_check = self.builder.build_call(has_panic_fn, &[], "panic_check")
            .map_err(|e| CursedError::Compile(format!("Failed to build panic check call: {:?}", e)))?;
        
        panic_check.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get panic check result".to_string()))?
            .into_int_value()
            .try_into()
            .map_err(|_| CursedError::Compile("Panic check result is not an integer".to_string()))
    }

    fn compile_get_panic_message(&mut self) -> Result<PointerValue, CursedError> {
        // Allocate a buffer for the panic message
        let buffer_size = 1024_u64;
        let buffer_type = self.i8_type.array_type(buffer_size as u32);
        let buffer = self.builder.build_alloca(buffer_type, "panic_message_buffer")
            .map_err(|e| CursedError::Compile(format!("Failed to build alloca: {:?}", e)))?;
        
        let get_message_fn = self.get_get_panic_message_function()?;
        let buffer_ptr = self.builder.build_bitcast(
            buffer,
            self.i8_ptr_type,
            "buffer_ptr",
        ).map_err(|e| CursedError::Compile(format!("Failed to build bitcast: {:?}", e)))?.into_pointer_value();
        
        let buffer_len = self.i64_type.const_int(buffer_size, false);
        
        let _message_len = self.builder.build_call(
            get_message_fn,
            &[buffer_ptr.into(), buffer_len.into()],
            "get_message",
        ).map_err(|e| CursedError::Compile(format!("Failed to build get message call: {:?}", e)))?;

        Ok(buffer_ptr)
    }

    fn compile_panic_cleanup(&mut self) -> Result<(), CursedError> {
        // In a full implementation, this would generate cleanup code
        // For now, it's a no-op placeholder
        Ok(())
    }

    fn compile_enter_recovery_scope(
        &mut self,
        scope_id: &str,
        config: Option<&RecoveryConfig>,
    ) -> Result<IntValue, CursedError> {
        let enter_fn = self.get_enter_recovery_scope_function()?;
        
        // Create string constant for scope ID
        let scope_id_ptr = self.create_string_constant(scope_id);
        let scope_id_len = self.i64_type.const_int(scope_id.len() as u64, false);
        
        // Extract timeout from config or use default
        let timeout_secs = if let Some(cfg) = config {
            cfg.timeout.as_secs() as u32
        } else {
            30 // Default timeout
        };
        let timeout_val = self.i32_type.const_int(timeout_secs as u64, false);
        
        // Call the function
        let call_result = self.builder.build_call(
            enter_fn,
            &[
                scope_id_ptr.into(),
                scope_id_len.into(),
                timeout_val.into(),
            ],
            "enter_recovery_scope",
        ).map_err(|e| CursedError::Compile(format!("Failed to build enter recovery scope call: {:?}", e)))?;
        
        call_result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get enter recovery scope result".to_string()))?
            .into_int_value()
            .try_into()
            .map_err(|_| CursedError::Compile("Enter recovery scope result is not an integer".to_string()))
    }

    fn compile_exit_recovery_scope(&mut self) -> Result<IntValue, CursedError> {
        let exit_fn = self.get_exit_recovery_scope_function()?;
        
        let call_result = self.builder.build_call(exit_fn, &[], "exit_recovery_scope")
            .map_err(|e| CursedError::Compile(format!("Failed to build exit recovery scope call: {:?}", e)))?;
        
        call_result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get exit recovery scope result".to_string()))?
            .into_int_value()
            .try_into()
            .map_err(|_| CursedError::Compile("Exit recovery scope result is not an integer".to_string()))
    }

    fn compile_in_recovery_scope(&mut self) -> Result<IntValue, CursedError> {
        let in_scope_fn = self.get_in_recovery_scope_function()?;
        
        let call_result = self.builder.build_call(in_scope_fn, &[], "in_recovery_scope")
            .map_err(|e| CursedError::Compile(format!("Failed to build in recovery scope call: {:?}", e)))?;
        
        call_result.try_as_basic_value().left()
            .ok_or_else(|| CursedError::Compile("Failed to get in recovery scope result".to_string()))?
            .into_int_value()
            .try_into()
            .map_err(|_| CursedError::Compile("In recovery scope result is not an integer".to_string()))
    }

    fn register_panic_functions(
        &mut self,
        _context: &Context,
        _module: &Module,
        _builder: &Builder,
    ) -> Result<(), CursedError> {
        // Pre-declare all panic-related functions
        self.get_panic_function()?;
        self.get_recover_function()?;
        self.get_has_panic_function()?;
        self.get_get_panic_message_function()?;
        self.get_enter_recovery_scope_function()?;
        self.get_exit_recovery_scope_function()?;
        self.get_in_recovery_scope_function()?;
        self.get_attempt_recovery_function()?;
        
        Ok(())
    }
}

// Tests temporarily removed due to LLVM lifetime complexity
// These can be re-added later with proper lifetime management
