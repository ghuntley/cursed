/// LLVM Type Switch Compilation for CURSED
/// 
/// This module implements LLVM code generation for type switches (vibe_check with type cases).
/// Type switches allow runtime type checking of interface values and binding variables
/// with specific types in each case.
/// 
/// Syntax: vibe_check interface_var.(Type) { mood ConcreteType: ... }

use crate::ast::statements::control_flow::SwitchStatement;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::Error;
use crate::codegen::llvm::type_system::LlvmTypeRegistry;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, IntValue};
use inkwell::basic_block::BasicBlock;
use inkwell::IntPredicate;
use std::collections::HashMap;
use tracing::{debug, warn, info};

/// Type switch compilation context
#[derive(Debug)]
pub struct TypeSwitchContext<'ctx> {
    /// Current function being compiled
    pub current_function: FunctionValue<'ctx>,
    /// Variable bindings in each case
    pub case_variables: HashMap<String, PointerValue<'ctx>>,
    /// Type registry for runtime type checking
    pub type_registry: &'ctx LlvmTypeRegistry,
    /// Variable scope stack
    pub variable_scopes: Vec<HashMap<String, PointerValue<'ctx>>>,
}

/// Type case information for compilation
#[derive(Debug, Clone)]
pub struct TypeCase {
    /// Type name to match against
    pub type_name: String,
    /// Variable name to bind (if any)
    pub bound_variable: Option<String>,
    /// Statements to execute if type matches
    pub statements: Vec<Box<dyn Statement>>,
}

/// Type switch compilation trait
pub trait TypeSwitchCompilation<'ctx> {
    /// Compile a type switch statement to LLVM IR
    fn compile_type_switch(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        switch_expr: &dyn Expression,
        type_cases: &[TypeCase],
        default_case: Option<&[Box<dyn Statement>]>,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<(), Error>;

    /// Generate runtime type checking code
    fn generate_type_check(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<IntValue<'ctx>, Error>;

    /// Bind a type variable with proper type safety
    fn bind_type_variable(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        variable_name: &str,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error>;

    /// Extract concrete value from interface
    fn extract_interface_value(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

/// Implementation of type switch compilation
pub struct LlvmTypeSwitchCompiler;

impl<'ctx> TypeSwitchCompilation<'ctx> for LlvmTypeSwitchCompiler {
    fn compile_type_switch(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        switch_expr: &dyn Expression,
        type_cases: &[TypeCase],
        default_case: Option<&[Box<dyn Statement>]>,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<(), Error> {
        debug!("Compiling type switch with {} cases", type_cases.len());

        // Create basic blocks for each case and control flow
        let mut case_blocks = Vec::new();
        let end_block = context.append_basic_block(ctx.current_function, "type_switch_end");
        let default_block = context.append_basic_block(ctx.current_function, "type_switch_default");

        // Create blocks for each type case
        for (i, case) in type_cases.iter().enumerate() {
            let case_block = context.append_basic_block(
                ctx.current_function, 
                &format!("type_case_{}", i)
            );
            case_blocks.push(case_block);
        }

        // Compile the switch expression (should be an interface value)
        let switch_value = self.compile_expression(context, module, builder, switch_expr, ctx)?;
        
        // Generate type checks and branching
        self.generate_type_switch_branches(
            context,
            module,
            builder,
            switch_value,
            type_cases,
            &case_blocks,
            default_block,
            ctx,
        )?;

        // Compile each type case
        for (i, case) in type_cases.iter().enumerate() {
            builder.position_at_end(case_blocks[i]);
            
            // Bind the type variable if specified
            if let Some(ref var_name) = case.bound_variable {
                let bound_var = self.bind_type_variable(
                    context,
                    module,
                    builder,
                    switch_value,
                    &case.type_name,
                    var_name,
                    ctx,
                )?;
                
                ctx.case_variables.insert(var_name.clone(), bound_var);
                debug!("Bound variable '{}' to type '{}'", var_name, case.type_name);
            }

            // Compile case statements
            for stmt in &case.statements {
                self.compile_statement(context, module, builder, stmt.as_ref(), ctx)?;
            }

            // Jump to end block if no explicit break
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(end_block);
            }
        }

        // Compile default case
        builder.position_at_end(default_block);
        if let Some(default_stmts) = default_case {
            for stmt in default_stmts {
                self.compile_statement(context, module, builder, stmt.as_ref(), ctx)?;
            }
        }
        if !builder.get_insert_block().unwrap().get_terminator().is_some() {
            builder.build_unconditional_branch(end_block);
        }

        // Position at end block for subsequent code
        builder.position_at_end(end_block);
        info!("Type switch compilation completed successfully");

        Ok(())
    }

    fn generate_type_check(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<IntValue<'ctx>, Error> {
        debug!("Generating type check for target type: {}", target_type);

        // Extract type ID from interface value
        let type_id = self.extract_interface_type_id(context, builder, interface_value)?;
        
        // Get expected type ID from registry
        let expected_type_id = self.get_expected_type_id(target_type, type_registry)?;
        let expected_id_value = context.i64_type().const_int(expected_type_id, false);

        // Compare type IDs
        let type_match = builder.build_int_compare(
            IntPredicate::EQ,
            type_id,
            expected_id_value,
            "type_check_result",
        );

        debug!("Generated type check comparison for type ID {}", expected_type_id);
        Ok(type_match)
    }

    fn bind_type_variable(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        variable_name: &str,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Binding variable '{}' to type '{}'", variable_name, target_type);

        // Extract the concrete value from the interface
        let concrete_value = self.extract_interface_value(
            context,
            builder,
            interface_value,
            target_type,
            ctx.type_registry,
        )?;

        // Get the LLVM type for the target type
        let (llvm_type_str, _) = self.map_type_to_llvm(target_type, ctx.type_registry)?;
        let llvm_type = self.parse_llvm_type(context, &llvm_type_str)?;

        // Allocate storage for the bound variable
        let var_ptr = builder.build_alloca(llvm_type, variable_name);
        
        // Store the extracted value
        builder.build_store(var_ptr, concrete_value);

        // Add to current scope
        if let Some(current_scope) = ctx.variable_scopes.last_mut() {
            current_scope.insert(variable_name.to_string(), var_ptr);
        }

        info!("Successfully bound variable '{}' with type '{}'", variable_name, target_type);
        Ok(var_ptr)
    }

    fn extract_interface_value(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting interface value for type: {}", target_type);

        // Interface values are stored as {data_ptr, vtable_ptr}
        // Extract the data pointer
        let data_ptr = self.extract_interface_data_ptr(context, builder, interface_value)?;
        
        // Get target type information
        let (llvm_type_str, _) = self.map_type_to_llvm(target_type, type_registry)?;
        
        // Cast data pointer to target type
        let target_llvm_type = self.parse_llvm_type(context, &llvm_type_str)?;
        let target_ptr_type = target_llvm_type.ptr_type(inkwell::AddressSpace::Generic);
        
        let typed_ptr = builder.build_bitcast(
            data_ptr,
            target_ptr_type,
            "typed_data_ptr",
        );

        // Load the value if it's not already a pointer type
        let result = if llvm_type_str.ends_with('*') {
            typed_ptr // Already a pointer
        } else {
            builder.build_load(typed_ptr, "extracted_value")
        };

        debug!("Successfully extracted interface value for type: {}", target_type);
        Ok(result)
    }
}

impl LlvmTypeSwitchCompiler {
    /// Generate branching logic for type switch cases
    fn generate_type_switch_branches<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        switch_value: BasicValueEnum<'ctx>,
        type_cases: &[TypeCase],
        case_blocks: &[BasicBlock<'ctx>],
        default_block: BasicBlock<'ctx>,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<(), Error> {
        debug!("Generating type switch branches for {} cases", type_cases.len());

        let mut current_block = builder.get_insert_block().unwrap();

        for (i, case) in type_cases.iter().enumerate() {
            // Generate type check
            let type_match = self.generate_type_check(
                context,
                module,
                builder,
                switch_value,
                &case.type_name,
                ctx.type_registry,
            )?;

            // Create next check block (or use default for last case)
            let next_block = if i + 1 < type_cases.len() {
                context.append_basic_block(ctx.current_function, &format!("type_check_{}", i + 1))
            } else {
                default_block
            };

            // Branch based on type check result
            builder.build_conditional_branch(type_match, case_blocks[i], next_block);

            // Move to next check block
            if i + 1 < type_cases.len() {
                builder.position_at_end(next_block);
            }
        }

        debug!("Type switch branches generated successfully");
        Ok(())
    }

    /// Extract type ID from interface value
    fn extract_interface_type_id<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<IntValue<'ctx>, Error> {
        // Interface value structure: {data_ptr: i8*, vtable_ptr: i8*}
        // Type ID is embedded in the vtable pointer (or derived from it)
        
        let vtable_ptr = self.extract_interface_vtable_ptr(context, builder, interface_value)?;
        
        // Convert vtable pointer to integer (simple type ID scheme)
        let type_id = builder.build_ptr_to_int(
            vtable_ptr.into_pointer_value(),
            context.i64_type(),
            "type_id_from_vtable",
        );

        Ok(type_id)
    }

    /// Extract data pointer from interface value
    fn extract_interface_data_ptr<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Interface structure: {i8* data_ptr, i8* vtable_ptr}
        let interface_ptr = if interface_value.is_pointer_value() {
            interface_value.into_pointer_value()
        } else {
            // Allocate temporary storage if needed
            let temp_ptr = builder.build_alloca(interface_value.get_type(), "temp_interface");
            builder.build_store(temp_ptr, interface_value);
            temp_ptr
        };

        // Get pointer to data field (index 0)
        let data_ptr_ptr = builder.build_struct_gep(
            interface_ptr,
            0,
            "data_ptr_ptr",
        )?;
        
        // Load the data pointer
        let data_ptr = builder.build_load(data_ptr_ptr, "data_ptr");
        
        Ok(data_ptr)
    }

    /// Extract vtable pointer from interface value  
    fn extract_interface_vtable_ptr<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Interface structure: {i8* data_ptr, i8* vtable_ptr}
        let interface_ptr = if interface_value.is_pointer_value() {
            interface_value.into_pointer_value()
        } else {
            let temp_ptr = builder.build_alloca(interface_value.get_type(), "temp_interface");
            builder.build_store(temp_ptr, interface_value);
            temp_ptr
        };

        // Get pointer to vtable field (index 1)
        let vtable_ptr_ptr = builder.build_struct_gep(
            interface_ptr,
            1,
            "vtable_ptr_ptr",
        )?;
        
        // Load the vtable pointer
        let vtable_ptr = builder.build_load(vtable_ptr_ptr, "vtable_ptr");
        
        Ok(vtable_ptr)
    }

    /// Get expected type ID for a given type name
    fn get_expected_type_id(
        &self,
        type_name: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<u64, Error> {
        // Check if it's an interface type
        if let Some(interface_type) = type_registry.get_interface(type_name) {
            return Ok(interface_type.type_id);
        }
        
        // Check if it's a struct type
        if let Some(_struct_type) = type_registry.get_struct(type_name) {
            // Calculate type ID for struct (using same method as type system)
            return Ok(self.calculate_type_id(type_name));
        }
        
        // Primitive types have predefined IDs
        match type_name {
            "normie" => Ok(1),
            "facts" => Ok(2),
            "tea" => Ok(3),
            "sus" => Ok(4),
            _ => Err(Error::TypeCompilation(format!("Unknown type for type switch: {}", type_name)))
        }
    }

    /// Calculate type ID using hash function
    fn calculate_type_id(&self, type_name: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        type_name.hash(&mut hasher);
        hasher.finish()
    }

    /// Map CURSED type to LLVM type representation
    fn map_type_to_llvm(
        &self,
        cursed_type: &str,
        type_registry: &LlvmTypeRegistry,
    ) -> Result<(String, usize), Error> {
        match cursed_type {
            "normie" => Ok(("i64".to_string(), 8)),
            "facts" => Ok(("i1".to_string(), 1)),
            "tea" => Ok(("i8*".to_string(), 8)),
            "sus" => Ok(("i8*".to_string(), 8)),
            _ => {
                // Check for registered types
                if let Some(struct_type) = type_registry.get_struct(cursed_type) {
                    Ok((format!("%struct.{}*", cursed_type), 8))
                } else if let Some(_interface_type) = type_registry.get_interface(cursed_type) {
                    Ok(("{i8*, i8*}".to_string(), 16))
                } else {
                    Err(Error::TypeCompilation(format!("Unsupported type in type switch: {}", cursed_type)))
                }
            }
        }
    }

    /// Parse LLVM type string to actual LLVM type
    fn parse_llvm_type<'ctx>(
        &self,
        context: &'ctx Context,
        type_str: &str,
    ) -> Result<inkwell::types::BasicTypeEnum<'ctx>, Error> {
        match type_str {
            "i1" => Ok(context.bool_type().into()),
            "i8" => Ok(context.i8_type().into()),
            "i32" => Ok(context.i32_type().into()),
            "i64" => Ok(context.i64_type().into()),
            "i8*" => Ok(context.i8_type().ptr_type(inkwell::AddressSpace::Generic).into()),
            "{i8*, i8*}" => {
                let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
                let struct_type = context.struct_type(&[ptr_type.into(), ptr_type.into()], false);
                Ok(struct_type.into())
            },
            _ if type_str.starts_with("%struct.") && type_str.ends_with('*') => {
                // Struct pointer type - simplified parsing
                let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
                Ok(ptr_type.into())
            },
            _ => Err(Error::TypeCompilation(format!("Cannot parse LLVM type: {}", type_str)))
        }
    }

    /// Compile an expression (placeholder - would integrate with existing expression compiler)
    fn compile_expression<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        expr: &dyn Expression,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // This would integrate with the existing expression compilation system
        // For now, return a placeholder interface value
        let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let interface_type = context.struct_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        
        // Create null interface value as placeholder
        let null_ptr = i8_ptr_type.const_null();
        let interface_value = interface_type.const_named_struct(&[null_ptr.into(), null_ptr.into()]);
        
        warn!("Using placeholder interface value - integrate with expression compiler");
        Ok(interface_value.into())
    }

    /// Compile a statement (placeholder - would integrate with existing statement compiler)
    fn compile_statement<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        stmt: &dyn Statement,
        ctx: &mut TypeSwitchContext<'ctx>,
    ) -> Result<(), Error> {
        // This would integrate with the existing statement compilation system
        debug!("Compiling statement in type switch case: {}", stmt.string());
        Ok(())
    }
}

impl<'ctx> TypeSwitchContext<'ctx> {
    /// Create new type switch context
    pub fn new(
        current_function: FunctionValue<'ctx>,
        type_registry: &'ctx LlvmTypeRegistry,
    ) -> Self {
        Self {
            current_function,
            case_variables: HashMap::new(),
            type_registry,
            variable_scopes: vec![HashMap::new()],
        }
    }

    /// Push new variable scope
    pub fn push_scope(&mut self) {
        self.variable_scopes.push(HashMap::new());
    }

    /// Pop variable scope
    pub fn pop_scope(&mut self) {
        self.variable_scopes.pop();
    }

    /// Lookup variable in current scopes
    pub fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        // Check case variables first
        if let Some(ptr) = self.case_variables.get(name) {
            return Some(*ptr);
        }
        
        // Check variable scopes
        for scope in self.variable_scopes.iter().rev() {
            if let Some(ptr) = scope.get(name) {
                return Some(*ptr);
            }
        }
        None
    }
}

/// FFI functions for runtime type switch support
extern "C" {
    /// Check if interface value is of specified type
    fn cursed_type_switch_check(interface_ptr: *const u8, type_id: u64) -> bool;
    
    /// Extract data pointer from interface value
    fn cursed_type_switch_extract(interface_ptr: *const u8) -> *mut u8;
    
    /// Get type ID from interface value
    fn cursed_type_switch_get_type_id(interface_ptr: *const u8) -> u64;
}

/// Utility functions for type switch compilation
pub struct TypeSwitchUtils;

impl TypeSwitchUtils {
    /// Parse type switch expression to extract interface expression and type
    pub fn parse_type_switch_expr(expr_str: &str) -> Result<(String, String), Error> {
        // Parse "variable.(Type)" syntax
        if let Some(dot_pos) = expr_str.find(".(") {
            if let Some(end_pos) = expr_str.rfind(')') {
                let variable = expr_str[..dot_pos].trim().to_string();
                let type_name = expr_str[dot_pos + 2..end_pos].trim().to_string();
                return Ok((variable, type_name));
            }
        }
        
        Err(Error::TypeCompilation(format!("Invalid type switch expression: {}", expr_str)))
    }

    /// Generate LLVM declarations for runtime type switch functions
    pub fn generate_runtime_declarations<'ctx>(
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> Result<(), Error> {
        let i8_type = context.i8_type();
        let i64_type = context.i64_type();
        let bool_type = context.bool_type();
        let ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);

        // cursed_type_switch_check
        let check_fn_type = bool_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        module.add_function("cursed_type_switch_check", check_fn_type, None);

        // cursed_type_switch_extract  
        let extract_fn_type = ptr_type.fn_type(&[ptr_type.into()], false);
        module.add_function("cursed_type_switch_extract", extract_fn_type, None);

        // cursed_type_switch_get_type_id
        let get_id_fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        module.add_function("cursed_type_switch_get_type_id", get_id_fn_type, None);

        debug!("Generated runtime type switch function declarations");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::type_system::LlvmTypeRegistry;
    use inkwell::context::Context;

    #[test]
    fn test_type_switch_utils() {
        let result = TypeSwitchUtils::parse_type_switch_expr("value.(String)");
        assert!(result.is_ok());
        let (var, type_name) = result.unwrap();
        assert_eq!(var, "value");
        assert_eq!(type_name, "String");
    }

    #[test]
    fn test_type_switch_context_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let registry = LlvmTypeRegistry::new();
        
        let ctx = TypeSwitchContext::new(function, &registry);
        assert_eq!(ctx.case_variables.len(), 0);
        assert_eq!(ctx.variable_scopes.len(), 1);
    }

    #[test]
    fn test_type_id_calculation() {
        let compiler = LlvmTypeSwitchCompiler;
        let id1 = compiler.calculate_type_id("String");
        let id2 = compiler.calculate_type_id("String");
        let id3 = compiler.calculate_type_id("Number");
        
        assert_eq!(id1, id2); // Same type should have same ID
        assert_ne!(id1, id3); // Different types should have different IDs
    }

    #[test]
    fn test_type_mapping() {
        let compiler = LlvmTypeSwitchCompiler;
        let registry = LlvmTypeRegistry::new();
        
        let (llvm_type, size) = compiler.map_type_to_llvm("normie", &registry).unwrap();
        assert_eq!(llvm_type, "i64");
        assert_eq!(size, 8);
        
        let (llvm_type, size) = compiler.map_type_to_llvm("facts", &registry).unwrap();
        assert_eq!(llvm_type, "i1");
        assert_eq!(size, 1);
    }
}
