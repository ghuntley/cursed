use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;
use crate::codegen::llvm::pointer_type_extension::PointerTypeExtension;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use tracing::{debug, info, trace, warn};

/// Enhanced trait for implementing interface type assertions in LLVM
/// with improved runtime type information and error handling
pub trait EnhancedInterfaceTypeAssertion<'ctx> {
    /// Compile a type assertion expression, returning both the converted value and a success flag
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type with detailed runtime checking
    fn check_instance_of_enhanced(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get runtime type information for detailed type checking
    fn get_runtime_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Register type in the runtime type registry
    fn register_runtime_type(&mut self, type_name: &str) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedInterfaceTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        trace!("Compiling enhanced type assertion for: {}", type_assertion.string());
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        debug!("Compiled expression value of type: {:?}", expr_value.get_type());
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Register the type for runtime type checking
        self.register_runtime_type(&type_assertion.type_name)?;
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of_enhanced(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        )?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Get concrete type information for proper casting
        let concrete_type = self.get_llvm_type_for(&type_assertion.type_name)?
            .ptr_type(AddressSpace::default());
        
        // Cast the data pointer to the concrete type pointer
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            concrete_type,
            "casted_ptr"
        )?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Add detailed logging for failed assertions in debug builds
        if self.debug_mode() {
            // Call to runtime logging function for failed assertions
            self.emit_assertion_failure_log(&type_assertion.type_name)?;
        }
        
        let null_ptr = concrete_type.const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let result_type = self.tuple_type(vec![concrete_type.into(), self.context().bool_type().into()]);
        let phi = self.builder().build_phi(result_type, "assertion_result")?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Completed type assertion compilation with result type: {:?}", result_type);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn check_instance_of_enhanced(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        trace!("Enhanced instance check for type: {}", target_type_name);
        
        // Get runtime type information from the interface value
        let runtime_type_info = self.get_runtime_type_info(interface_value)?;
        
        // Get the expected type ID for the target type
        let expected_type_id = self.get_type_id(target_type_name)?;
        
        // Get the actual type ID and name from runtime type info
        let actual_type_id = self.extract_type_id_from_rtinfo(runtime_type_info)?;
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "is_instance_of"
        )?;
        
        debug!("Type assertion comparison result returned");
        
        Ok(result.into())
    }
    
    fn get_runtime_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Enhanced version that gets complete runtime type info instead of just ID
        // This would include type name, hierarchy, etc. for more sophisticated assertions
        
        // Extract the vtable pointer
        let vtable_ptr = self.extract_vtable_ptr(interface_value)?;
        
        // Type info is the first field in the vtable
        // Get the struct type from the pointer type
        let vtable_struct_type = match vtable_ptr.get_type().get_element_type() {
            inkwell::types::BasicTypeEnum::StructType(struct_type) => struct_type,
            _ => return Err(Error::from_str("Expected vtable pointer to point to struct type")),
        };
        
        let type_info_ptr = self.builder().build_struct_gep(
            vtable_struct_type,
            vtable_ptr,
            0, // Index of type info pointer
            "type_info_ptr"
        )?;
        
        // Load the type info pointer (pointer to runtime type information struct)
        let type_info_ptr_loaded = self.builder().build_load(
            self.pointer_type(),
            type_info_ptr,
            "type_info_ptr_loaded"
        )?;
        
        // Load the actual type info structure
        let type_info = self.builder().build_load(
            self.get_runtime_type_info_type(),
            type_info_ptr_loaded.into_pointer_value(),
            "type_info"
        )?;
        
        Ok(type_info)
    }
    
    fn register_runtime_type(&mut self, type_name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // This would register the type in a global registry if it's not already registered
        // For now, we just ensure we have a unique ID for it
        trace!("Registering runtime type: {}", type_name);
        let type_id = self.get_type_id(type_name)?;
        
        // In a more complete implementation, we would:  
        // 1. Check if type is already registered
        // 2. If not, create runtime type info
        // 3. Store in global registry
        // 4. Return the type ID
        
        Ok(type_id)
    }
}

// Helper methods extension
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Extract the vtable pointer from an interface value
    fn extract_vtable_ptr(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        // Extract the vtable pointer (second field of interface value)
        let vtable_ptr = if interface_value.is_struct_value() {
            // Direct interface value
            self.builder().build_extract_value(
                interface_value.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            )?
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            )?;
            self.builder().build_extract_value(
                loaded.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            )?
        } else {
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_value
            )));
        };
        
        Ok(vtable_ptr.into_pointer_value())
    }
    
    // Extract type ID from runtime type info structure
    fn extract_type_id_from_rtinfo(
        &mut self,
        runtime_type_info: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Type ID is the first field in the runtime type info structure
        let type_id = self.builder().build_extract_value(
            runtime_type_info.into_struct_value(),
            0, // Index of type ID
            "extracted_type_id"
        )?;
        
        Ok(type_id)
    }
    
    // Get the LLVM type for a runtime type info structure
    fn get_runtime_type_info_type(&self) -> StructType<'ctx> {
        // Define the structure of runtime type information:
        // - type_id: i64
        // - type_name_ptr: i8*
        // - super_type_ptr: RTInfo* (for inheritance)
        // - method_count: i32
        // - methods_ptr: i8* (pointer to method info array)
        
        let ctx = self.context();
        let i64_type = ctx.i64_type();
        let ptr_type = self.pointer_type();
        let i32_type = ctx.i32_type();
        
        ctx.struct_type(&[
            i64_type.into(),           // type_id
            ptr_type.into(),           // type_name_ptr
            ptr_type.into(),           // super_type_ptr
            i32_type.into(),           // method_count
            ptr_type.into(),           // methods_ptr
        ], false)
    }
    
    // Get the LLVM type for a given type name
    fn get_llvm_type_for(&self, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error> {
        // In a real implementation, this would look up the type in a registry
        // For now, we just return a generic pointer type
        
        Ok(self.pointer_type().into())
    }
    
    // Emit logging for failed type assertions in debug mode
    fn emit_assertion_failure_log(&mut self, target_type: &str) -> Result<(), Error> {
        // This would call a runtime function to log the failure
        // For now, we just do nothing
        
        trace!("Type assertion to {} failed", target_type);
        
        Ok(())
    }
    
    // Check if we're in debug mode
    fn debug_mode(&self) -> bool {
        // This would check the compilation options
        // For now, we assume debug mode is always on
        true
    }
}