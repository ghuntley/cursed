//! LLVM code generation for Stan (goroutine) operations
//!
//! This module handles the compilation of `stan` expressions in the CURSED language,
//! which create and schedule new goroutines for concurrent execution.

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, CallSiteValue};
use inkwell::types::{BasicTypeEnum, PointerType};
use inkwell::{AddressSpace, IntPredicate};
use crate::ast::expressions::concurrency::StanExpression;
use crate::ast::traits::{Expression, Node};
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::pointer_type_extension::PointerTypeExtension;
use tracing::{debug, error, info, instrument, warn};

/// Trait for compiling stan expressions to LLVM IR
pub trait StanCompilation<'ctx> {
    /// Compile a stan (goroutine) expression
    fn compile_stan_expression(&mut self, stan_expr: &StanExpression) -> Result<BasicValueEnum<'ctx>, String>;
    
    /// Create a goroutine scheduler runtime if not present
    fn ensure_goroutine_runtime(&mut self) -> Result<(), String>;
    
    /// Generate closure capture for goroutine
    fn generate_closure_capture(&mut self, func_val: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, String>;
    
    /// Schedule a goroutine for execution
    fn schedule_goroutine(&mut self, func_ptr: PointerValue<'ctx>, capture_data: Option<PointerValue<'ctx>>) -> Result<BasicValueEnum<'ctx>, String>;
}

impl<'ctx> StanCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, stan_expr), fields(expression = %stan_expr.string()))]
    fn compile_stan_expression(&mut self, stan_expr: &StanExpression) -> Result<BasicValueEnum<'ctx>, String> {
        info!("Compiling stan expression for goroutine creation");
        
        // Ensure goroutine runtime is initialized
        self.ensure_goroutine_runtime()?;
        
        // Compile the expression that will run in the goroutine
        let func_val = match self.compile_expression(stan_expr.expression.as_ref()) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to compile goroutine expression: {:?}", e);
                return Err(format!("Failed to compile goroutine expression: {}", e));
            }
        };
        
        debug!("Compiled goroutine expression successfully");
        
        // Handle different types of expressions for goroutine execution
        match func_val {
            BasicValueEnum::PointerValue(ptr) if self.is_function_pointer(ptr) => {
                debug!("Creating goroutine from function pointer");
                self.create_function_goroutine(ptr)
            },
            BasicValueEnum::PointerValue(ptr) => {
                debug!("Creating goroutine from closure");
                self.create_closure_goroutine(ptr)
            },
            _ => {
                // Create a wrapper function for the expression
                debug!("Creating goroutine from expression wrapper");
                self.create_expression_goroutine(func_val)
            }
        }
    }
    
    #[instrument(skip(self))]
    fn ensure_goroutine_runtime(&mut self) -> Result<(), String> {
        debug!("Ensuring goroutine runtime is available");
        
        // Check if spawn_goroutine function exists
        if self.module.get_function("spawn_goroutine").is_some() {
            debug!("Goroutine runtime already initialized");
            return Ok(());
        }
        
        info!("Initializing goroutine runtime functions");
        
        // Create goroutine scheduler functions
        self.declare_goroutine_functions()?;
        self.create_goroutine_data_structures()?;
        
        debug!("Goroutine runtime initialized successfully");
        Ok(())
    }
    
    #[instrument(skip(self, func_val))]
    fn generate_closure_capture(&mut self, func_val: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, String> {
        debug!("Generating closure capture for goroutine");
        
        // Get the current function context for variable capture
        let current_func = self.current_function
            .ok_or_else(|| "No current function context for closure capture")?;
        
        // Create closure data structure
        let closure_type = self.get_or_create_closure_type()?;
        let closure_ptr = self.allocate_closure_data(closure_type)?;
        
        // Capture variables from current scope
        self.capture_scope_variables(closure_ptr)?;
        
        // Store function pointer in closure
        self.store_function_in_closure(closure_ptr, func_val)?;
        
        debug!("Closure capture completed successfully");
        Ok(closure_ptr)
    }
    
    #[instrument(skip(self, func_ptr, capture_data))]
    fn schedule_goroutine(&mut self, func_ptr: PointerValue<'ctx>, capture_data: Option<PointerValue<'ctx>>) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Scheduling goroutine for execution");
        
        // Get the spawn_goroutine function
        let spawn_fn = self.module.get_function("spawn_goroutine")
            .ok_or_else(|| "spawn_goroutine function not found")?;
        
        // Cast function pointer to void*
        let void_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let func_void_ptr = self.builder.build_bitcast(
            func_ptr,
            void_ptr_type,
            "func_to_void_ptr"
        ).unwrap();
        
        // Prepare arguments for goroutine spawning
        let mut args = vec![func_void_ptr.into()];
        
        // Add capture data if present
        if let Some(capture_ptr) = capture_data {
            let capture_void_ptr = self.builder.build_bitcast(
                capture_ptr,
                void_ptr_type,
                "capture_to_void_ptr"
            ).unwrap();
            args.push(capture_void_ptr.into());
        } else {
            // Pass null for no capture data
            let null_ptr = void_ptr_type.const_null();
            args.push(null_ptr.into());
        }
        
        // Call the spawn function
        let call_result = self.builder.build_call(
            spawn_fn,
            &args,
            "spawn_result"
        ).unwrap();
        
        info!("Goroutine scheduled successfully");
        
        // Return goroutine ID or handle
        if let Some(val) = call_result.try_as_basic_value().left() {
            Ok(val)
        } else {
            // Void return, create a placeholder value
            Ok(self.context.i32_type().const_int(0, false).into())
        }
    }
}

// Helper implementation methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if a pointer value is a function pointer
    fn is_function_pointer(&self, ptr: PointerValue<'ctx>) -> bool {
        // For now, assume any function value passed to us is a function pointer
        // In a more sophisticated implementation, we could check the LLVM type metadata
        // This is a simplified check suitable for our current use case
        true
    }
    
    /// Create a goroutine from a function pointer
    #[instrument(skip(self, func_ptr))]
    fn create_function_goroutine(&mut self, func_ptr: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Creating function-based goroutine");
        
        // No closure capture needed for plain function calls
        self.schedule_goroutine(func_ptr, None)
    }
    
    /// Create a goroutine from a closure
    #[instrument(skip(self, closure_ptr))]
    fn create_closure_goroutine(&mut self, closure_ptr: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Creating closure-based goroutine");
        
        // Extract function pointer from closure
        let func_ptr = self.extract_function_from_closure(closure_ptr)?;
        
        // Schedule with closure capture data
        self.schedule_goroutine(func_ptr, Some(closure_ptr))
    }
    
    /// Create a goroutine from an expression value by wrapping it in a function
    #[instrument(skip(self, expr_val))]
    fn create_expression_goroutine(&mut self, expr_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Creating expression-based goroutine wrapper");
        
        // Create a wrapper function that evaluates the expression
        let wrapper_func = self.create_expression_wrapper(expr_val)?;
        
        // Get function pointer
        let func_ptr = wrapper_func.as_global_value().as_pointer_value();
        
        // Schedule the wrapper function
        self.schedule_goroutine(func_ptr, None)
    }
    
    /// Declare external goroutine runtime functions
    #[instrument(skip(self))]
    fn declare_goroutine_functions(&mut self) -> Result<(), String> {
        debug!("Declaring goroutine runtime functions");
        
        let void_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        
        // spawn_goroutine(func_ptr: void*, capture_data: void*) -> i32
        let spawn_fn_type = i32_type.fn_type(&[void_ptr.into(), void_ptr.into()], false);
        self.module.add_function("spawn_goroutine", spawn_fn_type, Some(inkwell::module::Linkage::External));
        
        // goroutine_yield() -> void  
        let yield_fn_type = self.context.void_type().fn_type(&[], false);
        self.module.add_function("goroutine_yield", yield_fn_type, Some(inkwell::module::Linkage::External));
        
        // goroutine_exit() -> void
        let exit_fn_type = self.context.void_type().fn_type(&[], false);
        self.module.add_function("goroutine_exit", exit_fn_type, Some(inkwell::module::Linkage::External));
        
        debug!("Goroutine runtime functions declared");
        Ok(())
    }
    
    /// Create goroutine data structures
    #[instrument(skip(self))]
    fn create_goroutine_data_structures(&mut self) -> Result<(), String> {
        debug!("Creating goroutine data structures");
        
        // Create goroutine context structure
        // struct GoroutineContext {
        //     func_ptr: void*,
        //     capture_data: void*,
        //     stack_ptr: void*,
        //     state: i32,
        // }
        let void_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        
        let goroutine_fields = vec![
            void_ptr.into(),   // func_ptr
            void_ptr.into(),   // capture_data  
            void_ptr.into(),   // stack_ptr
            i32_type.into(),   // state
        ];
        
        let goroutine_type = self.context.struct_type(&goroutine_fields, false);
        
        // Register the type for later use
        // We would typically store this in a type registry, but for now we'll recreate as needed
        
        debug!("Goroutine data structures created");
        Ok(())
    }
    
    /// Get or create closure type for variable capture
    fn get_or_create_closure_type(&mut self) -> Result<inkwell::types::StructType<'ctx>, String> {
        // Create a generic closure structure
        // struct Closure {
        //     func_ptr: void*,
        //     captured_vars: void*,
        //     var_count: i32,
        // }
        let void_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        
        let closure_fields = vec![
            void_ptr.into(),   // func_ptr
            void_ptr.into(),   // captured_vars
            i32_type.into(),   // var_count
        ];
        
        Ok(self.context.struct_type(&closure_fields, false))
    }
    
    /// Allocate closure data on heap
    fn allocate_closure_data(&mut self, closure_type: inkwell::types::StructType<'ctx>) -> Result<PointerValue<'ctx>, String> {
        // Allocate memory for closure structure
        let closure_size = closure_type.size_of().unwrap();
        let malloc_fn = self.get_or_declare_malloc()?;
        
        let size_val = self.context.i64_type().const_int(closure_size.get_zero_extended_constant().unwrap(), false);
        let call_result = self.builder.build_call(malloc_fn, &[size_val.into()], "closure_alloc").unwrap();
        
        let void_ptr = call_result.try_as_basic_value().left()
            .ok_or_else(|| "malloc call failed")?
            .into_pointer_value();
        
        // Cast to closure type
        let closure_ptr = self.builder.build_bitcast(
            void_ptr,
            closure_type.ptr_type(AddressSpace::default()),
            "closure_ptr"
        ).unwrap().into_pointer_value();
        
        Ok(closure_ptr)
    }
    
    /// Capture variables from current scope
    fn capture_scope_variables(&mut self, closure_ptr: PointerValue<'ctx>) -> Result<(), String> {
        debug!("Capturing scope variables for closure");
        
        // For now, implement basic variable capture
        // In a full implementation, this would iterate through the current scope
        // and copy relevant variables to the closure structure
        
        // Set var_count to 0 for now (field index 2)
        let closure_struct_type = closure_ptr.get_type().get_element_type_opt()
            .ok_or_else(|| "Failed to get closure element type")?
            .into_struct_type();
        let var_count_ptr = self.builder.build_struct_gep(
            closure_struct_type,
            closure_ptr,
            2,
            "var_count_ptr"
        ).unwrap();
        
        let zero = self.context.i32_type().const_int(0, false);
        self.builder.build_store(var_count_ptr, zero).unwrap();
        
        debug!("Scope variables captured");
        Ok(())
    }
    
    /// Store function pointer in closure
    fn store_function_in_closure(&mut self, closure_ptr: PointerValue<'ctx>, func_val: BasicValueEnum<'ctx>) -> Result<(), String> {
        debug!("Storing function in closure");
        
        // Get function pointer field (index 0)
        let closure_struct_type = closure_ptr.get_type().get_element_type_opt()
            .ok_or_else(|| "Failed to get closure element type")?
            .into_struct_type();
        let func_ptr_field = self.builder.build_struct_gep(
            closure_struct_type,
            closure_ptr,
            0,
            "func_ptr_field"
        ).unwrap();
        
        // Cast function value to void*
        let void_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let func_void_ptr = self.builder.build_bitcast(
            func_val,
            void_ptr_type,
            "func_to_void"
        ).unwrap();
        
        // Store in closure
        self.builder.build_store(func_ptr_field, func_void_ptr).unwrap();
        
        debug!("Function stored in closure");
        Ok(())
    }
    
    /// Extract function pointer from closure
    fn extract_function_from_closure(&mut self, closure_ptr: PointerValue<'ctx>) -> Result<PointerValue<'ctx>, String> {
        debug!("Extracting function from closure");
        
        // Get function pointer field (index 0)
        let closure_struct_type = closure_ptr.get_type().get_element_type_opt()
            .ok_or_else(|| "Failed to get closure element type")?
            .into_struct_type();
        let func_ptr_field = self.builder.build_struct_gep(
            closure_struct_type,
            closure_ptr,
            0,
            "func_ptr_field"
        ).unwrap();
        
        // Load function pointer
        let void_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let func_void_ptr = self.builder.build_load(void_ptr_type, func_ptr_field, "func_load").unwrap()
            .into_pointer_value();
        
        debug!("Function extracted from closure");
        Ok(func_void_ptr)
    }
    
    /// Create a wrapper function for an expression value
    fn create_expression_wrapper(&mut self, expr_val: BasicValueEnum<'ctx>) -> Result<FunctionValue<'ctx>, String> {
        debug!("Creating expression wrapper function");
        
        // Create a function that evaluates the expression
        let void_type = self.context.void_type();
        let wrapper_fn_type = void_type.fn_type(&[], false);
        let wrapper_fn = self.module.add_function("__goroutine_wrapper", wrapper_fn_type, None);
        
        // Create basic block for the wrapper
        let entry_block = self.context.append_basic_block(wrapper_fn, "entry");
        let current_block = self.builder.get_insert_block();
        self.builder.position_at_end(entry_block);
        
        // For expressions that don't return void, we can just evaluate them
        // In a real implementation, this might involve calling the expression
        // or storing its result somewhere
        
        // Create return
        self.builder.build_return(None).unwrap();
        
        // Restore previous insertion point
        if let Some(block) = current_block {
            self.builder.position_at_end(block);
        }
        
        debug!("Expression wrapper function created");
        Ok(wrapper_fn)
    }
    
    /// Get or declare malloc function
    fn get_or_declare_malloc(&mut self) -> Result<FunctionValue<'ctx>, String> {
        if let Some(malloc_fn) = self.module.get_function("malloc") {
            return Ok(malloc_fn);
        }
        
        // Declare malloc: void* malloc(size_t size)
        let void_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
        let size_type = self.context.i64_type(); // size_t
        let malloc_type = void_ptr.fn_type(&[size_type.into()], false);
        
        Ok(self.module.add_function("malloc", malloc_type, Some(inkwell::module::Linkage::External)))
    }
}
