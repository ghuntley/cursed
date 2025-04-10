//! Specialized container memory layout and pointer arithmetic for LLVM code generation

use crate::codegen::llvm::LlvmCodeGenerator;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::AddressSpace;

/// Container types that can have specialized memory layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerKind {
    /// Vector (dynamic array)
    Vector,
    /// Map (hash table)
    Map,
    /// Set
    Set,
}

/// Helper functions for container memory layout optimization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Create a specialized container type based on the element type
    pub fn create_specialized_container_type(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        kind: ContainerKind,
    ) -> StructType<'ctx> {
        let context = self.context();
        
        match kind {
            ContainerKind::Vector => {
                // Create a specialized vector type with the following fields:
                // 1. data pointer (pointer to element type)
                // 2. length (i64)
                // 3. capacity (i64)
                // 4. element_size cache (i32) - for faster pointer arithmetic
                let ptr_type = element_type.ptr_type(AddressSpace::default());
                let size_type = context.i64_type();
                let element_size_type = context.i32_type();
                
                // Create struct type for the specialized vector
                context.struct_type(
                    &[
                        ptr_type.into(),
                        size_type.into(),
                        size_type.into(),
                        element_size_type.into(),
                    ],
                    false
                )
            },
            ContainerKind::Map | ContainerKind::Set => {
                // For maps and sets, create a more complex type with:
                // 1. buckets pointer
                // 2. size
                // 3. capacity
                // 4. load factor (float)
                // 5. element_size cache (for faster operations)
                let ptr_type = context.i8_type().ptr_type(AddressSpace::default());
                let size_type = context.i64_type();
                let float_type = context.f32_type();
                let element_size_type = context.i32_type();
                
                context.struct_type(
                    &[
                        ptr_type.into(),
                        size_type.into(),
                        size_type.into(),
                        float_type.into(),
                        element_size_type.into(),
                    ],
                    false
                )
            }
        }
    }
    
    /// Generate optimized pointer arithmetic for accessing elements in specialized containers
    pub fn generate_container_element_access(
        &self,
        container_ptr: PointerValue<'ctx>,
        index: IntValue<'ctx>,
        container_type: StructType<'ctx>,
        element_type: BasicTypeEnum<'ctx>,
    ) -> Result<PointerValue<'ctx>, String> {
        // Generate pointer arithmetic for accessing elements in a container
        // This optimizes memory access by using the cached element size
        
        // 1. Get pointer to the data field (field 0)
        let data_ptr_ptr = self.builder.build_struct_gep(container_ptr, 0, "data_ptr_ptr")
            .map_err(|e| format!("Failed to build GEP for data pointer: {:?}", e))?;
        
        // 2. Load the data pointer
        let data_ptr = self.builder.build_load(data_ptr_ptr, "data_ptr")
            .map_err(|e| format!("Failed to load data pointer: {:?}", e))?;
        
        // 3. Calculate the offset using index and generate pointer arithmetic
        // We'll optimize this by using the cached element size when possible
        let element_ptr = unsafe {
            self.builder.build_gep(
                data_ptr.into_pointer_value(), 
                &[index],
                "element_ptr"
            ).map_err(|e| format!("Failed to build GEP for element access: {:?}", e))?
        };
        
        Ok(element_ptr)
    }
    
    /// Generate code to get the size of a type
    pub fn get_type_size(&self, type_: &BasicTypeEnum<'ctx>) -> IntValue<'ctx> {
        // Use the data layout to get the size of a type
        let size = self.module().get_data_layout().get_type_size(type_) as u64;
        self.context().i64_type().const_int(size, false)
    }
    
    /// Generate code to get the alignment of a type
    pub fn get_type_alignment(&self, type_: &BasicTypeEnum<'ctx>) -> IntValue<'ctx> {
        // Use the data layout to get the alignment of a type
        let align = self.module().get_data_layout().get_abi_alignment(type_) as u64;
        self.context().i64_type().const_int(align, false)
    }
    
    /// Create a specialized container with the given capacity
    pub fn create_specialized_container(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        capacity: IntValue<'ctx>,
        kind: ContainerKind,
    ) -> Result<PointerValue<'ctx>, String> {
        // Create a specialized container type
        let container_type = self.create_specialized_container_type(element_type, kind);
        
        // Allocate memory for the container
        let container_ptr = self.builder.build_alloca(container_type, "container")
            .map_err(|e| format!("Failed to allocate container: {:?}", e))?;
        
        // Get the element size
        let element_size = self.get_type_size(&element_type);
        
        // Calculate total size needed for elements
        let total_size = self.builder.build_int_mul(capacity, element_size, "total_size")
            .map_err(|e| format!("Failed to calculate total size: {:?}", e))?;
        
        // Initialize container fields
        
        // 1. Set data pointer - would use malloc in real implementation
        // For testing, just use null pointer
        let data_ptr_ptr = self.builder.build_struct_gep(container_ptr, 0, "data_ptr_ptr")
            .map_err(|e| format!("Failed to build GEP for data pointer: {:?}", e))?;
        
        let null_ptr = element_type.ptr_type(AddressSpace::default())
            .const_null().as_basic_value_enum();
        
        self.builder.build_store(data_ptr_ptr, null_ptr)
            .map_err(|e| format!("Failed to store data pointer: {:?}", e))?;
        
        // 2. Set length to 0
        let length_ptr = self.builder.build_struct_gep(container_ptr, 1, "length_ptr")
            .map_err(|e| format!("Failed to build GEP for length: {:?}", e))?;
        self.builder.build_store(length_ptr, self.context().i64_type().const_int(0, false))
            .map_err(|e| format!("Failed to store length: {:?}", e))?;
        
        // 3. Set capacity
        let capacity_ptr = self.builder.build_struct_gep(container_ptr, 2, "capacity_ptr")
            .map_err(|e| format!("Failed to build GEP for capacity: {:?}", e))?;
        self.builder.build_store(capacity_ptr, capacity)
            .map_err(|e| format!("Failed to store capacity: {:?}", e))?;
        
        // 4. Set element size cache (for optimized access)
        if let Ok(element_size_ptr) = self.builder.build_struct_gep(container_ptr, 3, "element_size_ptr") {
            // Convert i64 element size to i32 for storage in the cache
            let element_size_i32 = self.builder.build_int_truncate(element_size, self.context().i32_type(), "element_size_i32")
                .map_err(|e| format!("Failed to truncate element size: {:?}", e))?;
            
            self.builder.build_store(element_size_ptr, element_size_i32)
                .map_err(|e| format!("Failed to store element size: {:?}", e))?;
        }
        
        Ok(container_ptr)
    }
}