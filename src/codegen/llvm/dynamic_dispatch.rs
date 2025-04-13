//! Dynamic dispatch for interfaces in LLVM IR
//!
//! This module implements the creation and use of vtables for interfaces,
//! enabling dynamic dispatch in the CURSED language. It manages the mapping
//! between interface methods and their concrete implementations.

use crate::code::Code;
use crate::error::Error;
use inkwell::types::{BasicTypeEnum, FunctionType, PointerType, StructType};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use std::collections::HashMap;

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::type_checker::Type as CursedType;

/// Structure of an interface value
///
/// An interface value consists of two pointers:
/// 1. Data pointer: Points to the actual object implementing the interface
/// 2. VTable pointer: Points to the vtable containing function pointers
pub struct InterfaceStructure<'ctx> {
    /// The LLVM struct type for the interface value
    pub interface_type: StructType<'ctx>,
    /// The methods that this interface declares
    pub methods: Vec<(String, FunctionType<'ctx>, Option<BasicTypeEnum<'ctx>>)>,
}

/// VTable structure for an interface
///
/// A VTable contains function pointers for each method in the interface
pub struct VTable<'ctx> {
    /// The LLVM struct type for the vtable
    pub vtable_type: StructType<'ctx>,
    /// Maps method names to their indices in the vtable
    pub method_indices: HashMap<String, usize>,
}

/// Implementation of a VTable for a specific type
pub struct VTableImpl<'ctx> {
    /// The global VTable constant for this implementation
    pub vtable_global: PointerValue<'ctx>,
    /// The type that implements the interface
    pub implementing_type: CursedType,
    /// The interface being implemented
    pub interface_type: CursedType,
}

/// InterfaceManager handles the creation and management of interfaces and vtables
pub struct InterfaceManager<'ctx> {
    /// Maps interface names to their structure
    interfaces: HashMap<String, InterfaceStructure<'ctx>>,
    /// Maps interface names to their vtable structure
    vtables: HashMap<String, VTable<'ctx>>,
    /// Maps (interface_name, implementing_type_name) to VTable implementation
    vtable_impls: HashMap<(String, String), VTableImpl<'ctx>>,
}

impl<'ctx> InterfaceManager<'ctx> {
    /// Create a new interface manager
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            vtables: HashMap::new(),
            vtable_impls: HashMap::new(),
        }
    }

    /// Register an interface with the manager
    pub fn register_interface(
        &mut self,
        generator: &LlvmCodeGenerator<'ctx>,
        interface_name: &str,
        methods: Vec<(String, Vec<CursedType>, Option<CursedType>)>,
    ) -> Result<(), Error> {
        // Create LLVM function types for each method
        let mut llvm_methods = Vec::new();

        for (method_name, param_types, return_type) in methods {
            // Convert CURSED types to LLVM types
            let mut llvm_param_types = Vec::new();
            for param_type in param_types {
                let llvm_type = generator.convert_type_to_llvm_type(&param_type)?;
                llvm_param_types.push(llvm_type);
            }

            // Convert return type
            let llvm_return_type = match &return_type {
                Some(ret_type) => Some(generator.convert_type_to_llvm_type(ret_type)?),
                None => None,
            };

            // Create function type
            let fn_type = match llvm_return_type {
                Some(ret_type) => generator.context.function_type(
                    ret_type.into_basic_type_enum(),
                    &llvm_param_types.as_slice(),
                    false,
                ),
                None => generator.context.void_type().fn_type(
                    &llvm_param_types.as_slice(),
                    false,
                ),
            };

            llvm_methods.push((method_name, fn_type, llvm_return_type));
        }

        // Create interface structure: { data_ptr, vtable_ptr }
        let interface_type = generator.context.struct_type(
            &[
                generator.context.i8_type().ptr_type(AddressSpace::default()).into(), // data pointer
                generator.context.i8_type().ptr_type(AddressSpace::default()).into(), // vtable pointer
            ],
            false,
        );

        let interface_structure = InterfaceStructure {
            interface_type,
            methods: llvm_methods.clone(),
        };

        self.interfaces.insert(interface_name.to_string(), interface_structure);

        // Create VTable structure for this interface
        let vtable_fields: Vec<BasicTypeEnum<'ctx>> = llvm_methods
            .iter()
            .map(|(_, fn_type, _)| {
                fn_type.ptr_type(AddressSpace::default()).into()
            })
            .collect();

        let vtable_type = generator.context.struct_type(&vtable_fields, false);

        let mut method_indices = HashMap::new();
        for (i, (method_name, _, _)) in llvm_methods.iter().enumerate() {
            method_indices.insert(method_name.clone(), i);
        }

        let vtable = VTable {
            vtable_type,
            method_indices,
        };

        self.vtables.insert(interface_name.to_string(), vtable);

        Ok(())
    }

    /// Create a vtable for a type that implements an interface
    pub fn create_vtable_for_implementation(
        &mut self,
        generator: &LlvmCodeGenerator<'ctx>,
        interface_name: &str,
        implementing_type: &CursedType,
        implementation_methods: HashMap<String, FunctionValue<'ctx>>,
    ) -> Result<(), Error> {
        // Get the interface and vtable structures
        let interface = match self.interfaces.get(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };

        let vtable = match self.vtables.get(interface_name) {
            Some(vtable) => vtable,
            None => return Err(Error::from_str(&format!(
                "No vtable found for interface: {}", 
                interface_name
            ))),
        };

        // Create an array of function pointers for the vtable
        let mut vtable_values = Vec::new();

        for (method_name, _, _) in &interface.methods {
            let fn_value = match implementation_methods.get(method_name) {
                Some(fn_val) => *fn_val,
                None => return Err(Error::from_str(&format!(
                    "Implementation for '{}' doesn't provide method: {}", 
                    implementing_type.to_string(), 
                    method_name
                ))),
            };

            vtable_values.push(fn_value.as_global_value().as_pointer_value().into());
        }

        // Create a global constant for the vtable
        let type_name = match implementing_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Only structs can implement interfaces")),
        };

        let vtable_name = format!("vtable.{}.{}", interface_name, type_name);
        let vtable_const = generator.module.add_global(
            vtable.vtable_type,
            Some(AddressSpace::default()),
            &vtable_name,
        );

        let vtable_struct = generator.context.const_struct(&vtable_values, false);
        vtable_const.set_initializer(&vtable_struct);
        vtable_const.set_constant(true);
        vtable_const.set_linkage(inkwell::module::Linkage::Private);

        // Save this vtable implementation
        let vtable_impl = VTableImpl {
            vtable_global: vtable_const.as_pointer_value(),
            implementing_type: implementing_type.clone(),
            interface_type: CursedType::Interface(interface_name.to_string(), Vec::new()),
        };

        self.vtable_impls.insert(
            (interface_name.to_string(), type_name.clone()),
            vtable_impl,
        );

        Ok(())
    }

    /// Get the interface structure for a given interface name
    pub fn get_interface(&self, interface_name: &str) -> Option<&InterfaceStructure<'ctx>> {
        self.interfaces.get(interface_name)
    }

    /// Get the vtable structure for a given interface name
    pub fn get_vtable(&self, interface_name: &str) -> Option<&VTable<'ctx>> {
        self.vtables.get(interface_name)
    }

    /// Get the vtable implementation for a given interface and implementing type
    pub fn get_vtable_impl(
        &self,
        interface_name: &str,
        implementing_type_name: &str,
    ) -> Option<&VTableImpl<'ctx>> {
        self.vtable_impls.get(&(interface_name.to_string(), implementing_type_name.to_string()))
    }

    /// Create an interface value from a concrete type
    pub fn create_interface_value(
        &self,
        generator: &LlvmCodeGenerator<'ctx>,
        value: PointerValue<'ctx>,
        value_type: &CursedType,
        interface_name: &str,
    ) -> Result<PointerValue<'ctx>, Error> {
        // Get the interface structure
        let interface = match self.interfaces.get(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };

        // Get the implementing type name
        let type_name = match value_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Only structs can implement interfaces")),
        };

        // Get the vtable implementation
        let vtable_impl = match self.vtable_impls.get(&(interface_name.to_string(), type_name.clone())) {
            Some(vtable_impl) => vtable_impl,
            None => return Err(Error::from_str(&format!(
                "No vtable implementation found for {} implementing {}", 
                type_name, 
                interface_name
            ))),
        };

        // Allocate memory for the interface value
        let interface_ptr = generator.builder.build_alloca(
            interface.interface_type,
            "interface_value",
        );

        // Set the data pointer (first field)
        let data_ptr_ptr = generator.builder.build_struct_gep(
            interface_ptr,
            0,
            "data_ptr_ptr",
        )?;

        // Cast the value pointer to i8*
        let value_i8_ptr = generator.builder.build_bitcast(
            value,
            generator.context.i8_type().ptr_type(AddressSpace::default()),
            "value_i8_ptr",
        ).into_pointer_value();

        generator.builder.build_store(data_ptr_ptr, value_i8_ptr);

        // Set the vtable pointer (second field)
        let vtable_ptr_ptr = generator.builder.build_struct_gep(
            interface_ptr,
            1,
            "vtable_ptr_ptr",
        )?;

        // Cast the vtable global to i8*
        let vtable_i8_ptr = generator.builder.build_bitcast(
            vtable_impl.vtable_global,
            generator.context.i8_type().ptr_type(AddressSpace::default()),
            "vtable_i8_ptr",
        ).into_pointer_value();

        generator.builder.build_store(vtable_ptr_ptr, vtable_i8_ptr);

        Ok(interface_ptr)
    }

    /// Call a method on an interface value (dynamic dispatch)
    pub fn call_interface_method(
        &self,
        generator: &LlvmCodeGenerator<'ctx>,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // Get the interface and vtable structures
        let interface = match self.interfaces.get(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };

        let vtable = match self.vtables.get(interface_name) {
            Some(vtable) => vtable,
            None => return Err(Error::from_str(&format!(
                "No vtable found for interface: {}", 
                interface_name
            ))),
        };

        // Get the method index in the vtable
        let method_index = match vtable.method_indices.get(method_name) {
            Some(index) => *index,
            None => return Err(Error::from_str(&format!(
                "Interface '{}' does not have method: {}", 
                interface_name, 
                method_name
            ))),
        };

        // Get the method's function type and return type
        let (_, fn_type, return_type) = match interface.methods.get(method_index) {
            Some(method) => method,
            None => return Err(Error::from_str(&format!(
                "Method index out of bounds for interface: {}", 
                interface_name
            ))),
        };

        // Load the data pointer from the interface value
        let data_ptr_ptr = generator.builder.build_struct_gep(
            interface_ptr,
            0,
            "data_ptr_ptr",
        )?;

        let data_ptr = generator.builder.build_load(
            data_ptr_ptr,
            "data_ptr",
        ).into_pointer_value();

        // Load the vtable pointer from the interface value
        let vtable_ptr_ptr = generator.builder.build_struct_gep(
            interface_ptr,
            1,
            "vtable_ptr_ptr",
        )?;

        let vtable_ptr = generator.builder.build_load(
            vtable_ptr_ptr,
            "vtable_ptr",
        ).into_pointer_value();

        // Cast the vtable pointer to the correct type
        let typed_vtable_ptr = generator.builder.build_bitcast(
            vtable_ptr,
            vtable.vtable_type.ptr_type(AddressSpace::default()),
            "typed_vtable_ptr",
        ).into_pointer_value();

        // Load the function pointer from the vtable
        let fn_ptr_ptr = generator.builder.build_struct_gep(
            typed_vtable_ptr,
            method_index as u32,
            "fn_ptr_ptr",
        )?;

        let fn_ptr = generator.builder.build_load(
            fn_ptr_ptr,
            "fn_ptr",
        ).into_pointer_value();

        // Cast the function pointer to the correct function type
        let fn_ptr_typed = generator.builder.build_bitcast(
            fn_ptr,
            fn_type.ptr_type(AddressSpace::default()),
            "fn_ptr_typed",
        ).into_pointer_value();

        // Create a new array of arguments with the data pointer as the first argument (this pointer)
        let mut real_args = vec![data_ptr.into()];
        real_args.extend_from_slice(args);

        // Call the function through the function pointer
        let call_result = generator.builder.build_call(
            fn_type,
            fn_ptr_typed,
            &real_args,
            "interface_call",
        );

        // Return the result if the function has a return type
        Ok(call_result.try_as_basic_value().left())
    }
}