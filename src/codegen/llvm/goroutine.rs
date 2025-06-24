/// LLVM code generation for goroutine operations
///
/// Provides compilation support for CURSED goroutine constructs including:
/// - `stan` keyword for goroutine spawning 
/// - `yolo` yield points in loops
/// - Safe point instrumentation for GC coordination

use crate::ast::expressions::GoroutineSpawn;
use crate::ast::traits::Node;
use crate::error::Error;
use std::ffi::CString;
use std::collections::HashMap;

// Import real inkwell types for LLVM integration
use inkwell::{
    values::{BasicValueEnum, FunctionValue, PointerValue, IntValue},
    types::{BasicTypeEnum, IntType, PointerType, FunctionType},
    basic_block::BasicBlock,
    AddressSpace,
    IntPredicate,
};

/// Runtime goroutine scheduler reference for LLVM integration
static mut RUNTIME_SCHEDULER: Option<*mut crate::runtime::goroutine::GoroutineScheduler> = None;

/// Set the runtime scheduler for LLVM integration
pub fn set_runtime_scheduler(scheduler: *mut crate::runtime::goroutine::GoroutineScheduler) {
    unsafe {
        RUNTIME_SCHEDULER = Some(scheduler);
    }
}

/// Get the runtime scheduler for LLVM integration
pub fn get_runtime_scheduler() -> Option<*mut crate::runtime::goroutine::GoroutineScheduler> {
    unsafe { RUNTIME_SCHEDULER }
}

/// Trait for compiling goroutine operations to LLVM IR
pub trait GoroutineCompiler<'ctx> {
    /// Compile goroutine spawn expression (stan)
    fn compile_goroutine_spawn(&mut self, spawn: &GoroutineSpawn) -> Result<(), Error>;
    
    /// Generate yield point for cooperative scheduling
    fn generate_yield_point(&mut self, location: &str) -> Result<(), Error>;
    
    /// Generate safe point for GC coordination
    fn generate_safe_point(&mut self, location: &str) -> Result<(), Error>;
    
    /// Generate goroutine scheduler setup code
    fn setup_goroutine_runtime(&mut self) -> Result<(), Error>;
    
    /// Declare runtime FFI functions in the module
    fn declare_goroutine_runtime_functions(&mut self) -> Result<(), Error>;
}

/// Implementation of GoroutineCompiler for the real LLVM code generator
impl<'ctx> GoroutineCompiler<'ctx> for crate::codegen::llvm::LlvmCodeGeneratorReal<'ctx> {
    fn compile_goroutine_spawn(&mut self, spawn: &GoroutineSpawn) -> Result<(), Error> {
        tracing::info!("Compiling goroutine spawn expression (stan keyword)");
        
        // Ensure runtime functions are declared
        self.declare_goroutine_runtime_functions()?;
        
        // Extract function name from spawn expression
        let function_name = self.extract_function_name(spawn)?;
        
        // Get or create the function to be spawned
        let target_function = match self.module().get_function(&function_name) {
            Some(func) => func,
            None => {
                // If function doesn't exist, create a wrapper that calls it
                let void_type = self.context().void_type();
                let fn_type = void_type.fn_type(&[], false);
                self.module().add_function(&function_name, fn_type, None)
            }
        };
        
        // Get the spawn function from the runtime
        let spawn_fn = self.module().get_function("cursed_spawn_goroutine")
            .ok_or_else(|| Error::Compile("cursed_spawn_goroutine function not found".to_string()))?;
        
        // Get runtime scheduler pointer
        let scheduler_ptr = self.get_runtime_scheduler_ptr()?;
        
        // Create function pointer for the target function
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::Generic);
        let target_fn_ptr = self.builder().build_bitcast(
            target_function.as_global_value().as_pointer_value(),
            i8_ptr_type,
            "target_fn_ptr"
        );
        
        // Call cursed_spawn_goroutine(scheduler_ptr, function_ptr)
        let spawn_call_result = self.builder().build_call(
            spawn_fn,
            &[
                scheduler_ptr.into(),
                target_fn_ptr.into(),
            ],
            "goroutine_id"
        );
        
        let goroutine_id = spawn_call_result.try_as_basic_value().left()
            .ok_or_else(|| Error::Compile("Failed to get goroutine spawn result".to_string()))?;
        
        tracing::info!(
            function_name = %function_name,
            "Successfully compiled goroutine spawn"
        );
        
        Ok(goroutine_id)
    }
    
    fn generate_yield_point(&mut self, location: &str) -> Result<(), Error> {
        tracing::info!(location = %location, "Generating yield point (yolo keyword)");
        
        // Ensure runtime functions are declared
        self.declare_goroutine_runtime_functions()?;
        
        // Get current basic block and function
        let current_block = self.builder().get_insert_block()
            .ok_or_else(|| Error::Compile("No current basic block for yield point".to_string()))?;
        let function = current_block.get_parent()
            .ok_or_else(|| Error::Compile("No parent function for yield point".to_string()))?;
        
        // Check if GC coordination is requested
        let gc_requested_fn = self.module().get_function("cursed_gc_requested")
            .ok_or_else(|| Error::Compile("cursed_gc_requested function not found".to_string()))?;
        
        let scheduler_ptr = self.get_runtime_scheduler_ptr()?;
        let gc_check_result = self.builder().build_call(
            gc_requested_fn,
            &[scheduler_ptr.into()],
            "gc_requested"
        );
        
        let gc_requested = gc_check_result.try_as_basic_value().left()
            .and_then(|v| v.into_int_value())
            .ok_or_else(|| Error::Compile("Failed to get GC request check result".to_string()))?;
        
        // Create basic blocks for control flow
        let yield_block = self.context().append_basic_block(function, "yield_block");
        let continue_block = self.context().append_basic_block(function, "continue_block");
        
        // Branch based on GC request
        self.builder().build_conditional_branch(gc_requested, yield_block, continue_block);
        
        // Yield block - call yield function
        self.builder().position_at_end(yield_block);
        let yield_fn = self.module().get_function("cursed_yield_goroutine")
            .ok_or_else(|| Error::Compile("cursed_yield_goroutine function not found".to_string()))?;
        
        self.builder().build_call(
            yield_fn,
            &[scheduler_ptr.into()],
            ""
        );
        self.builder().build_unconditional_branch(continue_block);
        
        // Continue block
        self.builder().position_at_end(continue_block);
        
        tracing::info!(location = %location, "Successfully generated yield point");
        Ok(())
    }
    
    fn generate_safe_point(&mut self, location: &str) -> Result<(), Error> {
        tracing::info!(location = %location, "Generating GC safe point");
        
        // Ensure runtime functions are declared
        self.declare_goroutine_runtime_functions()?;
        
        // Create a global string constant for the location
        let location_str = self.create_global_string(location);
        
        // Get the safe point function
        let safe_point_fn = self.module().get_function("cursed_safe_point")
            .ok_or_else(|| Error::Compile("cursed_safe_point function not found".to_string()))?;
        
        let scheduler_ptr = self.get_runtime_scheduler_ptr()?;
        
        // Call cursed_safe_point(scheduler_ptr, location_str)
        self.builder().build_call(
            safe_point_fn,
            &[
                scheduler_ptr.into(),
                location_str.into(),
            ],
            ""
        );
        
        tracing::info!(location = %location, "Successfully generated safe point");
        Ok(())
    }
    
    fn setup_goroutine_runtime(&mut self) -> Result<(), Error> {
        tracing::info!("Setting up goroutine runtime");
        
        // Ensure runtime functions are declared
        self.declare_goroutine_runtime_functions()?;
        
        // For now, we'll return the static scheduler pointer
        // In a full implementation, this would initialize a new scheduler
        let scheduler_ptr = self.get_runtime_scheduler_ptr()?;
        
        tracing::info!("Successfully set up goroutine runtime");
        Ok(scheduler_ptr)
    }
    
    fn declare_goroutine_runtime_functions(&mut self) -> Result<(), Error> {
        let i8_type = self.context().i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);
        let i64_type = self.context().i64_type();
        let bool_type = self.context().bool_type();
        let void_type = self.context().void_type();
        
        // Declare cursed_spawn_goroutine(scheduler_ptr, function_ptr) -> goroutine_id
        if self.module().get_function("cursed_spawn_goroutine").is_none() {
            let spawn_fn_type = i64_type.fn_type(&[
                i8_ptr_type.into(), // scheduler_ptr
                i8_ptr_type.into(), // function_ptr (changed from extern "C" fn())
            ], false);
            self.module().add_function("cursed_spawn_goroutine", spawn_fn_type, None);
        }
        
        // Declare cursed_yield_goroutine(scheduler_ptr)
        if self.module().get_function("cursed_yield_goroutine").is_none() {
            let yield_fn_type = void_type.fn_type(&[
                i8_ptr_type.into(), // scheduler_ptr
            ], false);
            self.module().add_function("cursed_yield_goroutine", yield_fn_type, None);
        }
        
        // Declare cursed_safe_point(scheduler_ptr, location)
        if self.module().get_function("cursed_safe_point").is_none() {
            let safe_point_fn_type = void_type.fn_type(&[
                i8_ptr_type.into(), // scheduler_ptr
                i8_ptr_type.into(), // location
            ], false);
            self.module().add_function("cursed_safe_point", safe_point_fn_type, None);
        }
        
        // Declare cursed_gc_requested(scheduler_ptr) -> bool
        if self.module().get_function("cursed_gc_requested").is_none() {
            let gc_requested_fn_type = bool_type.fn_type(&[
                i8_ptr_type.into(), // scheduler_ptr
            ], false);
            self.module().add_function("cursed_gc_requested", gc_requested_fn_type, None);
        }
        
        tracing::info!("Successfully declared goroutine runtime functions");
        Ok(())
    }
}

/// Helper implementations for LlvmCodeGeneratorReal
impl<'ctx> crate::codegen::llvm::LlvmCodeGeneratorReal<'ctx> {
    /// Extract function name from goroutine spawn expression
    fn extract_function_name(&self, spawn: &GoroutineSpawn) -> Result<(), Error> {
        // Extract function name from the AST node
        let spawn_str = spawn.string();
        
        // Parse "stan function_name" or "stan function_name(args)"
        let parts: Vec<&str> = spawn_str.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "stan" {
            let function_name = parts[1];
            // Remove parentheses if present
            let clean_name = function_name.split('(').next().unwrap_or(function_name);
            Ok(clean_name.to_string())
        } else {
            Err(Error::Compile(format!("Invalid goroutine spawn expression: {}", spawn_str)))
        }
    }
    
    /// Get runtime scheduler pointer
    fn get_runtime_scheduler_ptr(&self) -> Result<(), Error> {
        // Get the static scheduler pointer from the runtime
        let scheduler_opt = get_runtime_scheduler();
        let scheduler_ptr_raw = scheduler_opt
            .ok_or_else(|| Error::Runtime("No runtime scheduler available".to_string()))?;
        
        // Convert raw pointer to LLVM pointer value
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::Generic);
        let scheduler_ptr_int = self.context().i64_type().const_int(scheduler_ptr_raw as u64, false);
        let scheduler_ptr = self.builder().build_int_to_ptr(
            scheduler_ptr_int,
            i8_ptr_type,
            "scheduler_ptr"
        );
        
        Ok(scheduler_ptr)
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
}

/// Generate yield points in loops (for `yolo` keyword) - compatible version for legacy usage
pub fn generate_loop_yield_point<'ctx>(
    generator: &mut impl GoroutineCompiler<'ctx>,
    loop_location: &str,
) -> Result<(), Error> {
    // Generate conditional yield points based on GC coordination requests
    generator.generate_yield_point(loop_location)
}

/// Convenience function for creating goroutine runtime integration
pub fn initialize_goroutine_runtime<'ctx>(
    generator: &mut impl GoroutineCompiler<'ctx>,
) -> Result<(), Error> {
    // Set up the goroutine runtime and declare all necessary functions
    generator.declare_goroutine_runtime_functions()?;
    generator.setup_goroutine_runtime()?;
    
    tracing::info!("Goroutine runtime successfully initialized for LLVM compilation");
    Ok(())
}

/// Update the runtime FFI functions to match our LLVM declarations
pub mod runtime_integration {
    use super::*;
    
    /// Initialize a runtime scheduler and set it as the global scheduler
    pub fn initialize_scheduler() -> Result<(), Error> {
        let mut scheduler = Box::new(crate::runtime::goroutine::GoroutineScheduler::new());
        scheduler.start()
            .map_err(|e| Error::Runtime(format!("Failed to start scheduler: {}", e)))?;
        
        let scheduler_ptr = Box::into_raw(scheduler);
        set_runtime_scheduler(scheduler_ptr);
        
        tracing::info!("Runtime scheduler initialized and started");
        Ok(scheduler_ptr)
    }
    
    /// Clean up the runtime scheduler
    pub fn cleanup_scheduler() -> Result<(), Error> {
        if let Some(scheduler_ptr) = get_runtime_scheduler() {
            unsafe {
                let mut scheduler = Box::from_raw(scheduler_ptr);
                scheduler.stop()
                    .map_err(|e| Error::Runtime(format!("Failed to stop scheduler: {}", e)))?;
            }
            
            // Clear the global reference
            set_runtime_scheduler(std::ptr::null_mut());
            tracing::info!("Runtime scheduler cleaned up");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::goroutine::GoroutineScheduler;
    
    #[test]
    fn test_runtime_scheduler_integration() {
        // Test setting and getting the runtime scheduler
        let mut scheduler = GoroutineScheduler::new();
        let scheduler_ptr = &mut scheduler as *mut GoroutineScheduler;
        
        set_runtime_scheduler(scheduler_ptr);
        let retrieved = get_runtime_scheduler();
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), scheduler_ptr);
        
        // Clean up
        set_runtime_scheduler(std::ptr::null_mut());
    }
}
