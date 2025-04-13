//! Container layout implementation for optimized memory representation
//! This module provides specialized container memory layouts and access patterns

use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use crate::error::Error;
use super::context::LlvmCodeGenerator;

/// Enum representing the type of container
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerKind {
    /// A vector/array container with sequential memory layout
    Vector,
    /// A hash table container with key-value pairs
    HashMap,
    /// A linked list container
    LinkedList,
}

/// Container layout manager trait for creating and managing container memory layouts
pub trait ContainerLayout<'ctx> {
    /// Create a specialized container type for a given element type and container kind
    fn create_specialized_container_type(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<StructType<'ctx>, Error>;

    /// Create a specialized container for a given element type and container kind
    fn create_specialized_container(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<BasicTypeEnum<'ctx>, Error>;

    /// Create a container instance with the given element type, capacity, and kind
    fn create_container_instance(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        capacity: u64,
        container_kind: ContainerKind,
    ) -> Result<PointerValue<'ctx>, Error>;

    /// Get a pointer to an element in the container
    fn get_element_pointer(
        &self,
        container_ptr: PointerValue<'ctx>,
        index: BasicValueEnum<'ctx>,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<PointerValue<'ctx>, Error>;
}

/// Container layout manager for the LLVM code generator
pub struct ContainerLayoutManager<'a, 'ctx> {
    generator: &'a mut LlvmCodeGenerator<'ctx>,
}

impl<'a, 'ctx> ContainerLayoutManager<'a, 'ctx> {
    /// Create a new container layout manager
    pub fn new(generator: &'a mut LlvmCodeGenerator<'ctx>) -> Self {
        Self { generator }
    }
}

impl<'a, 'ctx> ContainerLayout<'ctx> for ContainerLayoutManager<'a, 'ctx> {
    fn create_specialized_container_type(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<StructType<'ctx>, Error> {
        let context = self.generator.context();
        
        // Create different layouts based on container kind
        match container_kind {
            ContainerKind::Vector => {
                // Vector layout: [data_ptr, length, capacity, element_type_metadata]
                let fields = vec![
                    // Pointer to data array
                    element_type
                        .ptr_type(inkwell::AddressSpace::default())
                        .into(),
                    // Length
                    context.i64_type().into(),
                    // Capacity
                    context.i64_type().into(),
                    // Element type metadata (could store type info for runtime)
                    context.i64_type().into(),
                ];
                
                Ok(context.struct_type(&fields, false))
            },
            ContainerKind::HashMap => {
                // HashMap layout: more complex with buckets, etc.
                // Simplified for this implementation
                let fields = vec![
                    // Buckets array pointer
                    context.i8_type()
                        .ptr_type(inkwell::AddressSpace::default())
                        .into(),
                    // Size
                    context.i64_type().into(),
                    // Capacity
                    context.i64_type().into(),
                    // Load factor (as int)
                    context.i32_type().into(),
                ];
                
                Ok(context.struct_type(&fields, false))
            },
            ContainerKind::LinkedList => {
                // LinkedList would have head/tail pointers
                let fields = vec![
                    // Head pointer
                    context.i8_type()
                        .ptr_type(inkwell::AddressSpace::default())
                        .into(),
                    // Tail pointer
                    context.i8_type()
                        .ptr_type(inkwell::AddressSpace::default())
                        .into(),
                    // Length
                    context.i64_type().into(),
                ];
                
                Ok(context.struct_type(&fields, false))
            }
        }
    }
    
    fn create_specialized_container(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Create the container type
        let container_type = self.create_specialized_container_type(
            element_type,
            container_kind,
        )?;
        
        // Return the type as a basic type enum
        Ok(container_type.into())
    }
    
    fn create_container_instance(
        &self,
        element_type: BasicTypeEnum<'ctx>,
        capacity: u64,
        container_kind: ContainerKind,
    ) -> Result<PointerValue<'ctx>, Error> {
        let context = self.generator.context();
        let builder = self.generator.builder();
        
        // Get the specialized container type
        let container_type = self
            .create_specialized_container_type(element_type, container_kind)?;
            
        // Allocate the container
        let container_ptr = if let Some(function) = self.generator.current_function() {
            // If we're in a function, allocate on the stack
            let builder = self.generator.builder();
            let alloca = builder.build_alloca(container_type, "container")?;
            alloca
        } else {
            // Otherwise, we'd allocate globally or through a helper
            return Err(Error::codegen("Cannot create container instance outside of a function".to_string()));
        };
        
        // For Vector containers, allocate the data array
        if container_kind == ContainerKind::Vector {
            // Allocate memory for the elements based on capacity
            let data_type = element_type.array_type(capacity as u32);
            let data_ptr = self.generator.builder()
                .build_alloca(data_type, "container_data")?;
                
            // Store the data pointer in the container
            let gep = self.generator.builder()
                .build_struct_gep(container_type, container_ptr, 0, "data_ptr_slot")?;
            
            // Convert to the expected pointer type
            let data_ptr_cast = self.generator.builder()
                .build_bitcast(
                    data_ptr,
                    element_type.ptr_type(inkwell::AddressSpace::default()),
                    "data_ptr_cast",
                )?;
                
            self.generator.builder()
                .build_store(gep, data_ptr_cast)?;
                
            // Initialize length to 0
            let length_gep = self.generator.builder()
                .build_struct_gep(container_type, container_ptr, 1, "length_slot")?;
            self.generator.builder()
                .build_store(length_gep, context.i64_type().const_int(0, false))?;
                
            // Initialize capacity
            let capacity_gep = self.generator.builder()
                .build_struct_gep(container_type, container_ptr, 2, "capacity_slot")?;
            self.generator.builder()
                .build_store(capacity_gep, context.i64_type().const_int(capacity, false))?;
        }
        
        Ok(container_ptr)
    }
    
    fn get_element_pointer(
        &self,
        container_ptr: PointerValue<'ctx>,
        index: BasicValueEnum<'ctx>,
        element_type: BasicTypeEnum<'ctx>,
        container_kind: ContainerKind,
    ) -> Result<PointerValue<'ctx>, Error> {
        let context = self.generator.context();
        let builder = self.generator.builder();
        
        match container_kind {
            ContainerKind::Vector => {
                // Get the data pointer from the container
                // We need to determine the container type
                // Since we can't get the element type directly from the pointer in this version of inkwell,
                // we'll have to use a placeholder or tracked type
                // For test purposes, we'll create a representative struct type
                let context = self.generator.context();
                let container_type = context.struct_type(
                    &[
                        context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
                        context.i64_type().into(),
                        context.i64_type().into(),
                        context.i64_type().into(),
                    ],
                    false
                );
                let data_ptr_gep = builder
                    .build_struct_gep(container_type, container_ptr, 0, "data_ptr_slot")?;
                let data_ptr = builder
                    .build_load(
                        element_type.ptr_type(inkwell::AddressSpace::default()),
                        data_ptr_gep,
                        "data_ptr",
                    )?
                    .into_pointer_value();
                    
                // Get a pointer to the element at the index
                // Convert the BasicValueEnum index to IntValue if needed
                let index_value = if let BasicValueEnum::IntValue(i) = index {
                    i
                } else {
                    return Err(Error::codegen("Index must be an integer value".to_string()));
                };
                
                let ptr = unsafe {
                    builder.build_gep(
                        element_type,
                        data_ptr,
                        &[index_value],
                        "element_ptr",
                    )?
                };
                
                Ok(ptr)
            },
            // Other container types would have their own access patterns
            _ => Err(Error::codegen(format!(
                "get_element_pointer not implemented for {:?}",
                container_kind
            ))),
        }
    }
}

/// Extension trait for LlvmCodeGenerator to provide container layout functionality
pub trait ContainerLayoutExtension<'a, 'ctx> {
    /// Get the container layout manager
    fn container_layout_manager(&'a mut self) -> ContainerLayoutManager<'a, 'ctx>;
}

impl<'a, 'ctx> ContainerLayoutExtension<'a, 'ctx> for LlvmCodeGenerator<'ctx> {
    fn container_layout_manager(&'a mut self) -> ContainerLayoutManager<'a, 'ctx> {
        ContainerLayoutManager::new(self)
    }
}