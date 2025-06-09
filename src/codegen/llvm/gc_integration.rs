//! Garbage Collection Integration with LLVM
//!
//! This module provides integration between the CURSED garbage collector
//! and LLVM's garbage collection support features, enabling efficient
//! memory management for compiled CURSED programs.

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue, PointerValue};
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::builder::Builder;
use inkwell::AddressSpace;
use std::collections::HashMap;
use tracing::{debug, info, instrument};

/// GC metadata for a type, describing which fields contain GC-managed pointers
#[derive(Debug, Clone)]
pub struct GcTypeMetadata {
    /// Name of the type
    pub type_name: String,
    /// Size of the type in bytes
    pub type_size: usize,
    /// Indices of fields that contain GC-managed pointers
    pub pointer_fields: Vec<usize>,
    /// Field names for debugging
    pub field_names: Vec<String>,
    /// Whether this type requires finalization
    pub needs_finalization: bool,
}

impl GcTypeMetadata {
    /// Creates new GC metadata for a type
    pub fn new(type_name: &str, type_size: usize) -> Self {
        GcTypeMetadata {
            type_name: type_name.to_string(),
            type_size,
            pointer_fields: Vec::new(),
            field_names: Vec::new(),
            needs_finalization: false,
        }
    }

    /// Adds a pointer field to the metadata
    pub fn add_pointer_field(&mut self, field_index: usize, field_name: &str) {
        self.pointer_fields.push(field_index);
        if self.field_names.len() <= field_index {
            self.field_names.resize(field_index + 1, String::new());
        }
        self.field_names[field_index] = field_name.to_string();
    }

    /// Marks the type as needing finalization
    pub fn set_needs_finalization(&mut self, needs_finalization: bool) {
        self.needs_finalization = needs_finalization;
    }

    /// Gets the GC map as a byte array for LLVM
    pub fn get_gc_map(&self) -> Vec<u8> {
        let mut gc_map = vec![0u8; (self.type_size + 7) / 8]; // Bit map
        
        for &field_index in &self.pointer_fields {
            let byte_offset = field_index / 8;
            let bit_offset = field_index % 8;
            
            if byte_offset < gc_map.len() {
                gc_map[byte_offset] |= 1 << bit_offset;
            }
        }
        
        gc_map
    }
}

/// LLVM GC integration manager
pub struct LlvmGcIntegration<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// LLVM module
    module: &'ctx Module<'ctx>,
    /// GC metadata for registered types
    type_metadata: HashMap<String, GcTypeMetadata>,
    /// Global variable holding the GC descriptor table
    gc_descriptor_table: Option<GlobalValue<'ctx>>,
    /// Whether GC support is enabled
    gc_enabled: bool,
}

impl<'ctx> LlvmGcIntegration<'ctx> {
    /// Creates a new LLVM GC integration manager
    #[instrument(skip(context, module))]
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        LlvmGcIntegration {
            context,
            module,
            type_metadata: HashMap::new(),
            gc_descriptor_table: None,
            gc_enabled: true,
        }
    }

    /// Enables or disables GC support
    pub fn set_gc_enabled(&mut self, enabled: bool) {
        self.gc_enabled = enabled;
    }

    /// Registers GC metadata for a type
    #[instrument(skip(self, metadata))]
    pub fn register_type_metadata(&mut self, metadata: GcTypeMetadata) {
        debug!(type_name = %metadata.type_name, pointer_fields = metadata.pointer_fields.len(), "Registering GC metadata for type");
        self.type_metadata.insert(metadata.type_name.clone(), metadata);
    }

    /// Gets GC metadata for a type
    pub fn get_type_metadata(&self, type_name: &str) -> Option<&GcTypeMetadata> {
        self.type_metadata.get(type_name)
    }

    /// Generates LLVM GC descriptor table (simplified for now)
    #[instrument(skip(self))]
    pub fn generate_gc_descriptor_table(&mut self) -> Result<(), String> {
        if !self.gc_enabled || self.type_metadata.is_empty() {
            return Ok(());
        }

        debug!(type_count = self.type_metadata.len(), "GC descriptor table generation skipped (simplified implementation)");
        
        // For now, just create a simple global indicating GC is enabled
        let i32_type = self.context.i32_type();
        let gc_enabled_global = self.module.add_global(
            i32_type,
            Some(AddressSpace::default()),
            "cursed_gc_enabled"
        );
        gc_enabled_global.set_initializer(&i32_type.const_int(1, false));
        gc_enabled_global.set_constant(true);

        info!("GC integration initialized (simplified)");
        Ok(())
    }

    /// Generates GC root registration for a function
    #[instrument(skip(self, function, builder))]
    pub fn generate_gc_root_registration(
        &self,
        function: FunctionValue<'ctx>,
        builder: &Builder<'ctx>,
        roots: &[(PointerValue<'ctx>, &str)]
    ) -> Result<(), String> {
        if !self.gc_enabled || roots.is_empty() {
            return Ok(());
        }

        debug!(function_name = %function.get_name().to_string_lossy(), root_count = roots.len(), "Generating GC root registration");

        // Get the llvm.gcroot intrinsic
        let gcroot_function = self.module.get_function("llvm.gcroot")
            .ok_or("llvm.gcroot intrinsic not found")?;

        let null_ptr = self.context.i8_type().ptr_type(AddressSpace::default()).const_null();

        for (root_ptr, type_name) in roots {
            // Cast the root pointer to i8**
            let root_ptr_cast = builder.build_bitcast(
                *root_ptr,
                self.context.i8_type().ptr_type(AddressSpace::default()).ptr_type(AddressSpace::default()),
                "root_cast"
            ).map_err(|e| format!("Failed to cast root pointer: {:?}", e))?;

            // Call llvm.gcroot
            builder.build_call(
                gcroot_function,
                &[root_ptr_cast.into(), null_ptr.into()],
                "gcroot"
            ).map_err(|e| format!("Failed to call llvm.gcroot: {:?}", e))?;

            debug!(type_name = %type_name, "Registered GC root");
        }

        Ok(())
    }

    /// Generates GC write barrier for pointer assignments
    #[instrument(skip(self, builder))]
    pub fn generate_write_barrier(
        &self,
        builder: &Builder<'ctx>,
        ptr: PointerValue<'ctx>,
        value: BasicValueEnum<'ctx>
    ) -> Result<(), String> {
        if !self.gc_enabled {
            return Ok(());
        }

        // Get the llvm.gcwrite intrinsic (if available)
        if let Some(gcwrite_function) = self.module.get_function("llvm.gcwrite") {
            let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
            
            // Cast pointer to i8*
            let ptr_cast = builder.build_bitcast(ptr, i8_ptr_type, "ptr_cast")
                .map_err(|e| format!("Failed to cast pointer for write barrier: {:?}", e))?;
            
            // Cast value to i8*
            let value_cast = builder.build_bitcast(value, i8_ptr_type, "value_cast")
                .map_err(|e| format!("Failed to cast value for write barrier: {:?}", e))?;

            // Call the write barrier
            builder.build_call(
                gcwrite_function,
                &[ptr_cast.into(), value_cast.into()],
                "gcwrite"
            ).map_err(|e| format!("Failed to call write barrier: {:?}", e))?;

            debug!("Generated GC write barrier");
        }

        Ok(())
    }

    /// Analyzes a struct type and generates GC metadata
    #[instrument(skip(self, struct_type))]
    pub fn analyze_struct_type(&mut self, type_name: &str, struct_type: StructType<'ctx>, field_names: &[String]) -> GcTypeMetadata {
        debug!(type_name = %type_name, field_count = field_names.len(), "Analyzing struct type for GC");

        let mut metadata = GcTypeMetadata::new(type_name, struct_type.size_of().unwrap().get_zero_extended_constant().unwrap() as usize);
        
        // Analyze each field
        for (i, field_type) in struct_type.get_field_types().iter().enumerate() {
            let field_name = field_names.get(i).map(|s| s.as_str()).unwrap_or("unknown");
            
            if self.is_gc_managed_type(field_type) {
                metadata.add_pointer_field(i, field_name);
                debug!(field_name = %field_name, field_index = i, "Found GC-managed field");
            }
        }

        // Check if type needs finalization (e.g., has resources like file handles)
        metadata.set_needs_finalization(self.type_needs_finalization(type_name));

        metadata
    }

    /// Checks if a type is GC-managed (contains pointers)
    fn is_gc_managed_type(&self, basic_type: &BasicTypeEnum<'ctx>) -> bool {
        match basic_type {
            BasicTypeEnum::PointerType(_) => true,
            BasicTypeEnum::StructType(struct_type) => {
                // Recursively check struct fields
                struct_type.get_field_types().iter().any(|field_type| self.is_gc_managed_type(field_type))
            }
            BasicTypeEnum::ArrayType(array_type) => {
                self.is_gc_managed_type(&array_type.get_element_type())
            }
            _ => false,
        }
    }

    /// Checks if a type needs finalization
    fn type_needs_finalization(&self, type_name: &str) -> bool {
        // Types that typically need finalization
        matches!(type_name, 
            "File" | "Socket" | "Channel" | "Mutex" | "Thread" | 
            "HttpClient" | "HttpServer" | "Database" | "Stream"
        )
    }

    /// Creates a string constant in the module
    fn create_string_constant(&self, value: &str) -> Result<GlobalValue<'ctx>, String> {
        let string_type = self.context.i8_type().array_type(value.len() as u32 + 1);
        let string_value = self.context.const_string(value.as_bytes(), true);
        
        let global = self.module.add_global(string_type, Some(AddressSpace::default()), "");
        global.set_initializer(&string_value);
        global.set_constant(true);
        
        Ok(global)
    }

    /// Creates a byte array constant in the module
    fn create_byte_array_constant(&self, bytes: &[u8]) -> Result<GlobalValue<'ctx>, String> {
        let array_type = self.context.i8_type().array_type(bytes.len() as u32);
        let values: Vec<_> = bytes.iter().map(|&b| self.context.i8_type().const_int(b as u64, false)).collect();
        let array_value = self.context.const_string(&values.iter().map(|v| v.get_zero_extended_constant().unwrap() as u8).collect::<Vec<_>>(), false);
        
        let global = self.module.add_global(array_type, Some(AddressSpace::default()), "");
        global.set_initializer(&array_value);
        global.set_constant(true);
        
        Ok(global)
    }

    /// Gets the GC descriptor table global (if generated)
    pub fn get_gc_descriptor_table(&self) -> Option<GlobalValue<'ctx>> {
        self.gc_descriptor_table
    }

    /// Gets all registered type metadata
    pub fn get_all_type_metadata(&self) -> &HashMap<String, GcTypeMetadata> {
        &self.type_metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_gc_metadata_creation() {
        let mut metadata = GcTypeMetadata::new("TestStruct", 24);
        metadata.add_pointer_field(0, "field1");
        metadata.add_pointer_field(2, "field3");
        
        assert_eq!(metadata.type_name, "TestStruct");
        assert_eq!(metadata.type_size, 24);
        assert_eq!(metadata.pointer_fields, vec![0, 2]);
        assert_eq!(metadata.field_names[0], "field1");
        assert_eq!(metadata.field_names[2], "field3");
    }

    #[test]
    fn test_gc_map_generation() {
        let mut metadata = GcTypeMetadata::new("TestStruct", 16);
        metadata.add_pointer_field(0, "field1");  // First field
        metadata.add_pointer_field(1, "field2");  // Second field
        metadata.add_pointer_field(8, "field9");  // Ninth field
        
        let gc_map = metadata.get_gc_map();
        
        // Should have at least 2 bytes (16 bits for 16-byte struct)
        assert!(gc_map.len() >= 2);
        
        // First byte should have bits 0 and 1 set
        assert_eq!(gc_map[0] & 0b00000011, 0b00000011);
        
        // Second byte should have bit 0 set (for field 8)
        assert_eq!(gc_map[1] & 0b00000001, 0b00000001);
    }

    #[test]
    fn test_llvm_gc_integration_creation() {
        // Use leaked memory to satisfy lifetime requirements in tests
        let context = Box::leak(Box::new(Context::create()));
        let module = Box::leak(Box::new(context.create_module("test")));
        
        let gc_integration = LlvmGcIntegration::new(context, module);
        
        assert!(gc_integration.gc_enabled);
        assert!(gc_integration.type_metadata.is_empty());
        assert!(gc_integration.gc_descriptor_table.is_none());
    }

    #[test]
    fn test_gc_managed_type_detection() {
        // Use leaked memory to satisfy lifetime requirements in tests
        let context = Box::leak(Box::new(Context::create()));
        let module = Box::leak(Box::new(context.create_module("test")));
        
        let gc_integration = LlvmGcIntegration::new(context, module);
        
        // Pointer types should be GC-managed
        let ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        assert!(gc_integration.is_gc_managed_type(&ptr_type.into()));
        
        // Basic types should not be GC-managed
        let int_type = context.i32_type();
        assert!(!gc_integration.is_gc_managed_type(&int_type.into()));
        
        // Struct with pointer field should be GC-managed
        let struct_with_ptr = context.struct_type(&[
            int_type.into(),
            ptr_type.into(),
        ], false);
        assert!(gc_integration.is_gc_managed_type(&struct_with_ptr.into()));
    }
}

/// Extensions for nil handling in garbage collection
impl<'ctx> LlvmGcIntegration<'ctx> {
    /// Check if a value is nil and should be excluded from GC tracking
    pub fn is_nil_value(&self, value: BasicValueEnum<'ctx>) -> bool {
        match value {
            BasicValueEnum::PointerValue(ptr) => {
                // Check if pointer is null
                ptr.is_null()
            },
            BasicValueEnum::StructValue(_) => {
                // For structs (like slices, interfaces), we need runtime checks
                // This is a compile-time limitation
                false
            },
            _ => false,
        }
    }
    
    /// Create GC roots for non-nil values only
    pub fn create_gc_root_if_not_nil(&mut self, builder: &Builder<'ctx>, value: BasicValueEnum<'ctx>, name: &str) -> Result<(), String> {
        if !self.is_nil_value(value) {
            // For now, just log the GC root creation since we don't have a simple single-value method
            debug!("Creating GC root for non-nil value: {}", name);
            // This is a placeholder - in a real implementation, you'd register the GC root
            Ok(())
        } else {
            debug!("Skipping GC root creation for nil value: {}", name);
            Ok(())
        }
    }
    
    /// Handle nil assignment for GC tracking
    pub fn handle_nil_assignment(&mut self, builder: &Builder<'ctx>, target_ptr: PointerValue<'ctx>, _target_type: BasicTypeEnum<'ctx>) -> Result<(), String> {
        // When assigning nil to a variable, we need to ensure any previous GC roots are cleared
        // This prevents memory leaks where old values are still tracked after being set to nil
        debug!("Handling nil assignment to tracked variable");
        
        // For now, this is a placeholder for more complex GC integration
        // In a full implementation, this would:
        // 1. Check if the target currently holds a GC-tracked value
        // 2. Remove the old value from GC tracking
        // 3. Ensure the nil assignment doesn't create new GC roots
        
        Ok(())
    }
    
    /// Mark nil values as non-reachable in GC marking phase
    pub fn mark_nil_as_non_reachable(&self, value: BasicValueEnum<'ctx>) -> bool {
        // Nil values don't reference any heap objects, so they shouldn't be marked as reachable
        self.is_nil_value(value)
    }
}
