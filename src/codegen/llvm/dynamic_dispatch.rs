//! Dynamic dispatch for interfaces in LLVM IR
//!
//! This module implements the creation and use of vtables for interfaces,
//! enabling dynamic dispatch in the CURSED language. It manages the mapping
//! between interface methods and their concrete implementations.
//!
//! The implementation follows a standard vtable-based approach:
//! 1. Each interface has a vtable structure with function pointers for all methods
//! 2. Each implementing type has its own vtable instance with pointers to its method implementations
//! 3. Interface values contain a data pointer and a vtable pointer
//! 4. Method calls use the vtable to find the correct implementation to call

use crate::code::Code;
use crate::error::Error;
use inkwell::types::{BasicTypeEnum, FunctionType, PointerType, StructType};
use inkwell::values::{BasicValueEnum, BasicValue, FunctionValue, PointerValue, BasicMetadataValueEnum};
use inkwell::AddressSpace;
use inkwell::context::Context;
use std::collections::HashMap;
use std::fmt;

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
    /// Type parameters for generic interfaces
    pub type_parameters: Vec<String>,
}

/// VTable structure for an interface
///
/// A VTable contains function pointers for each method in the interface
pub struct VTable<'ctx> {
    /// The LLVM struct type for the vtable
    pub vtable_type: StructType<'ctx>,
    /// Maps method names to their indices in the vtable
    pub method_indices: HashMap<String, usize>,
    /// Maps method indices to their signature information
    pub method_signatures: Vec<MethodSignature<'ctx>>,
}

/// Method signature information for vtable entries
pub struct MethodSignature<'ctx> {
    /// Name of the method
    pub name: String,
    /// LLVM function type
    pub function_type: FunctionType<'ctx>,
    /// Return type if any
    pub return_type: Option<BasicTypeEnum<'ctx>>,
    /// Parameter types excluding the self parameter
    pub param_types: Vec<BasicTypeEnum<'ctx>>,
}

/// Implementation of a VTable for a specific type
pub struct VTableImpl<'ctx> {
    /// The global VTable constant for this implementation
    pub vtable_global: PointerValue<'ctx>,
    /// The type that implements the interface
    pub implementing_type: CursedType,
    /// The interface being implemented
    pub interface_type: CursedType,
    /// Runtime type information for this implementation
    pub runtime_type_info: TypeInfo,
}

/// Runtime type information for interface implementations
#[derive(Clone, Debug)]
pub struct TypeInfo {
    /// Type ID for runtime type identification
    pub type_id: String,
    /// Name of the implementing type
    pub type_name: String,
    /// Type arguments for generic types
    pub type_args: Vec<String>,
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.type_args.is_empty() {
            write!(f, "{}", self.type_name)
        } else {
            write!(f, "{}[{}]", self.type_name, self.type_args.join(", "))
        }
    }
}

/// InterfaceManager handles the creation and management of interfaces and vtables
pub struct InterfaceManager<'ctx> {
    /// Maps interface names to their structure
    interfaces: HashMap<String, InterfaceStructure<'ctx>>,
    /// Maps interface names to their vtable structure
    vtables: HashMap<String, VTable<'ctx>>,
    /// Maps (interface_name, implementing_type_name) to VTable implementation
    vtable_impls: HashMap<(String, String), VTableImpl<'ctx>>,
    /// Maps type IDs to runtime type information
    type_info_map: HashMap<String, TypeInfo>,
    /// Next type ID for unique runtime type identification
    next_type_id: u64,
}

impl<'ctx> InterfaceManager<'ctx> {
    /// Create a new interface manager
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            vtables: HashMap::new(),
            vtable_impls: HashMap::new(),
            type_info_map: HashMap::new(),
            next_type_id: 1, // Start with 1 so 0 can represent null or invalid
        }
    }
    
    /// Generate a unique type ID for runtime type information
    fn generate_type_id(&mut self) -> String {
        let id = self.next_type_id;
        self.next_type_id += 1;
        format!("type_{}", id)
    }
    
    /// Register type information for a given type
    fn register_type_info(&mut self, type_: &CursedType) -> TypeInfo {
        match type_ {
            CursedType::Struct(name, type_args) => {
                let type_arg_strings: Vec<String> = type_args.iter()
                    .map(|arg| arg.to_string())
                    .collect();
                
                let type_id = self.generate_type_id();
                let type_info = TypeInfo {
                    type_id: type_id.clone(),
                    type_name: name.clone(),
                    type_args: type_arg_strings,
                };
                
                self.type_info_map.insert(type_id.clone(), type_info.clone());
                type_info
            },
            _ => {
                // For non-struct types, use a simpler approach
                let type_id = self.generate_type_id();
                let type_info = TypeInfo {
                    type_id: type_id.clone(),
                    type_name: type_.to_string(),
                    type_args: Vec::new(),
                };
                
                self.type_info_map.insert(type_id.clone(), type_info.clone());
                type_info
            }
        }
    }

    /// Register an interface with the manager
    pub fn register_interface(
        &mut self,
        context: &'ctx Context,
        interface_name: &str,
        methods: Vec<(String, Vec<CursedType>, Option<CursedType>)>,
        type_params: Vec<String>,
    ) -> Result<(), Error> {
        // Convert CursedType params and return types to LLVM types
        let mut llvm_methods = Vec::new();
        let mut method_signatures = Vec::new();

        for (method_name, param_types, return_type) in methods {
            // Convert parameter types to LLVM types
            let mut llvm_param_types = Vec::new();
            let mut param_type_values = Vec::new();
            
            // For each parameter type, get the corresponding LLVM type
            for param_type in &param_types {
                // Convert CursedType to LLVM type - simplified for now
                let llvm_type = match param_type {
                    CursedType::Tea => context.i8_type().ptr_type(AddressSpace::default()).into(),
                    CursedType::Normie => context.i32_type().into(),
                    CursedType::Lit => context.bool_type().into(),
                    CursedType::Meal => context.f64_type().into(),
                    // Handle other types as needed - simplified for now
                    _ => context.i8_type().ptr_type(AddressSpace::default()).into(),
                };
                
                llvm_param_types.push(llvm_type);
                param_type_values.push(llvm_type);
            }

            // Convert return type to LLVM type if present
            let llvm_return_type = match &return_type {
                Some(ret_type) => {
                    // Convert CursedType to LLVM type - simplified for now
                    let ret_llvm_type = match ret_type {
                        CursedType::Tea => context.i8_type().ptr_type(AddressSpace::default()).into(),
                        CursedType::Normie => context.i32_type().into(),
                        CursedType::Lit => context.bool_type().into(), 
                        CursedType::Meal => context.f64_type().into(),
                        // Handle other types as needed - simplified for now
                        _ => context.i8_type().ptr_type(AddressSpace::default()).into(),
                    };
                    Some(ret_llvm_type)
                },
                None => None,
            };

            // Create function type using the appropriate LLVM API
            let fn_type = if let Some(ret_type) = llvm_return_type {
                match ret_type {
                    BasicTypeEnum::IntType(int_type) => int_type.fn_type(&llvm_param_types, false),
                    BasicTypeEnum::FloatType(float_type) => float_type.fn_type(&llvm_param_types, false),
                    BasicTypeEnum::PointerType(ptr_type) => ptr_type.fn_type(&llvm_param_types, false),
                    BasicTypeEnum::StructType(struct_type) => struct_type.fn_type(&llvm_param_types, false),
                    BasicTypeEnum::ArrayType(array_type) => array_type.fn_type(&llvm_param_types, false),
                    BasicTypeEnum::VectorType(vector_type) => vector_type.fn_type(&llvm_param_types, false),
                }
            } else {
                context.void_type().fn_type(&llvm_param_types, false)
            };

            llvm_methods.push((method_name.clone(), fn_type, llvm_return_type));
            
            // Create method signature
            let method_signature = MethodSignature {
                name: method_name.clone(),
                function_type: fn_type,
                return_type: llvm_return_type,
                param_types: param_type_values.into_iter().map(|t| t.try_into().unwrap()).collect(),
            };
            
            method_signatures.push(method_signature);
        }

        // Create interface structure: { data_ptr, vtable_ptr }
        let interface_type = context.struct_type(
            &[
                context.i8_type().ptr_type(AddressSpace::default()).into(), // data pointer
                context.i8_type().ptr_type(AddressSpace::default()).into(), // vtable pointer
            ],
            false,
        );

        let interface_structure = InterfaceStructure {
            interface_type,
            methods: llvm_methods.clone(),
            type_parameters: type_params.clone(),
        };

        self.interfaces.insert(interface_name.to_string(), interface_structure);

        // Create VTable structure for this interface
        let vtable_fields: Vec<BasicTypeEnum<'ctx>> = llvm_methods
            .iter()
            .map(|(_, fn_type, _)| {
                fn_type.ptr_type(AddressSpace::default()).into()
            })
            .collect();

        let vtable_type = context.struct_type(&vtable_fields, false);

        let mut method_indices = HashMap::new();
        for (i, (method_name, _, _)) in llvm_methods.iter().enumerate() {
            method_indices.insert(method_name.clone(), i);
        }

        let vtable = VTable {
            vtable_type,
            method_indices,
            method_signatures,
        };

        self.vtables.insert(interface_name.to_string(), vtable);

        Ok(())
    }
    
    /// Parse a potentially generic type name into base name and type parameters
    fn parse_generic_name(name: &str) -> (String, Vec<String>) {
        if let Some(open_bracket) = name.find('<') {
            if let Some(close_bracket) = name.rfind('>') {
                let base_name = name[0..open_bracket].to_string();
                let type_params_str = &name[open_bracket+1..close_bracket];
                let type_params = type_params_str.split(',').map(|s| s.trim().to_string()).collect();
                return (base_name, type_params);
            }
        }
        (name.to_string(), Vec::new())
    }

    /// Create a vtable for a type that implements an interface
    pub fn create_vtable_for_implementation(
        &mut self,
        context: &'ctx Context,
        module: &inkwell::module::Module<'ctx>,
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

        // Extract the implementing type name (we've already checked it's a struct earlier)
        let type_name = match implementing_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Only structs can implement interfaces")),
        };
        
        // Create a TypeInfo manually since we can't borrow self mutably here
        let type_id = format!("type_{}", self.next_type_id);
        self.next_type_id += 1;
        
        let type_args: Vec<String> = match implementing_type {
            CursedType::Struct(_, type_args) => type_args.iter()
                .map(|arg| arg.to_string())
                .collect(),
            _ => Vec::new(),
        };
        
        let type_info = TypeInfo {
            type_id: type_id.clone(),
            type_name: type_name.clone(),
            type_args,
        };
        
        // Register in the type info map
        self.type_info_map.insert(type_id, type_info.clone());
        
        // Extract interface type arguments if it's generic
        let (base_interface_name, interface_type_args) = Self::parse_generic_name(interface_name);
        
        // Create the vtable name, including type arguments for generic interfaces
        let vtable_name = format!("vtable.{}.{}", interface_name, type_name);
        
        let vtable_const = module.add_global(
            vtable.vtable_type,
            Some(AddressSpace::default()),
            &vtable_name,
        );

        let vtable_struct = context.const_struct(&vtable_values, false);
        vtable_const.set_initializer(&vtable_struct);
        vtable_const.set_constant(true);
        vtable_const.set_linkage(inkwell::module::Linkage::Private);

        // Construct interface type with the right type parameters
        let interface_type = if interface_type_args.is_empty() {
            CursedType::Interface(interface_name.to_string(), Vec::new())
        } else {
            // Convert type args strings to CursedType
            let type_args = interface_type_args.iter()
                .map(|arg| Box::new(CursedType::Named(arg.clone())))
                .collect();
                
            CursedType::Interface(base_interface_name, type_args)
        };
        
        // Save this vtable implementation
        let vtable_impl = VTableImpl {
            vtable_global: vtable_const.as_pointer_value(),
            implementing_type: implementing_type.clone(),
            interface_type,
            runtime_type_info: type_info,
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
        context: &'ctx Context,
        builder: &inkwell::builder::Builder<'ctx>,
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
        let interface_ptr = builder.build_alloca(
            interface.interface_type,
            "interface_value",
        ).map_err(|e| Error::from_str(&format!("Failed to allocate interface value: {}", e)))?;

        // Cast value pointer to i8*
        let value_i8_ptr = builder.build_bitcast(
            value,
            context.i8_type().ptr_type(AddressSpace::default()),
            "value_i8_ptr",
        ).expect("Failed to cast value to i8*").into_pointer_value();

        // Get data pointer pointer (first field)
        let data_ptr_ptr = unsafe {
            builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                0, 
                "data_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get data pointer: {}", e)))?;
        builder.build_store(data_ptr_ptr, value_i8_ptr)
            .map_err(|e| Error::from_str(&format!("Failed to store data pointer: {}", e)))?;

        // Cast vtable global to i8*
        let vtable_i8_ptr = builder.build_bitcast(
            vtable_impl.vtable_global,
            context.i8_type().ptr_type(AddressSpace::default()),
            "vtable_i8_ptr",
        ).expect("Failed to cast vtable global to i8*").into_pointer_value();

        // Get vtable pointer pointer (second field)
        let vtable_ptr_ptr = unsafe {
            builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                1, 
                "vtable_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get vtable pointer: {}", e)))?;
        builder.build_store(vtable_ptr_ptr, vtable_i8_ptr)
            .map_err(|e| Error::from_str(&format!("Failed to store vtable pointer: {}", e)))?;
        
        // Add debug info about the interface implementation
        tracing::debug!("Created interface value for {} implementing {}", 
            vtable_impl.runtime_type_info.to_string(), 
            interface_name
        );

        Ok(interface_ptr)
    }
    
    /// Call a method on an interface value (dynamic dispatch)
    pub fn call_interface_method(
        &self,
        context: &'ctx Context,
        builder: &inkwell::builder::Builder<'ctx>,
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

        // Get the method's signature information
        let method_signature = match vtable.method_signatures.get(method_index) {
            Some(signature) => signature,
            None => return Err(Error::from_str(&format!(
                "Method signature not found for '{}' in interface: {}", 
                method_name, 
                interface_name
            ))),
        };

        // Load the data pointer from the interface value
        let data_ptr_ptr = unsafe {
            builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                0, 
                "data_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get data pointer: {}", e)))?;
        
        // Get element type for data pointer
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        
        let data_ptr = builder
            .build_load(i8_ptr_type, data_ptr_ptr, "data_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load data pointer: {}", e)))?;
        
        let data_ptr = data_ptr.into_pointer_value();

        // Load the vtable pointer from the interface value
        let vtable_ptr_ptr = unsafe {
            builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                1, 
                "vtable_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get vtable pointer: {}", e)))?;
        
        // Get element type for vtable pointer
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        
        let vtable_ptr = builder
            .build_load(i8_ptr_type, vtable_ptr_ptr, "vtable_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load vtable pointer: {}", e)))?;
            
        let vtable_ptr = vtable_ptr.into_pointer_value();

        // Cast the vtable pointer to the correct type
        let typed_vtable_ptr = builder
            .build_bitcast(
                vtable_ptr,
                vtable.vtable_type.ptr_type(AddressSpace::default()),
                "typed_vtable_ptr",
            )
            .expect("Failed to cast vtable pointer")
            .into_pointer_value();

        // Get the function pointer from the vtable
        let fn_ptr_ptr = unsafe {
            builder.build_struct_gep(
                vtable.vtable_type,
                typed_vtable_ptr, 
                method_index as u32, 
                "fn_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get function pointer: {}", e)))?;
        
        // Get the method signature function type pointer for correct loading
        let fn_ptr_type = method_signature.function_type.ptr_type(AddressSpace::default());
        
        let fn_ptr = builder
            .build_load(fn_ptr_type, fn_ptr_ptr, "fn_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load function pointer: {}", e)))?;
            
        let fn_ptr = fn_ptr.into_pointer_value();

        // Cast the function pointer to the correct function type
        let fn_ptr_typed = builder
            .build_bitcast(
                fn_ptr,
                method_signature.function_type.ptr_type(AddressSpace::default()),
                "fn_ptr_typed",
            )
            .expect("Failed to cast function pointer")
            .into_pointer_value();

        // Create a new array of arguments with the data pointer as the first argument (self pointer)
        let mut real_args = vec![data_ptr.into()];
        real_args.extend_from_slice(args);

        // Convert BasicValueEnum to BasicMetadataValueEnum for the arguments
        let metadata_args: Vec<_> = real_args.iter().map(|arg| {
            (*arg).into()
        }).collect();

        // Add tracing for debugging dynamic dispatch calls
        tracing::debug!("Calling interface method {} on {}", method_name, interface_name);

        // Call the function through the function pointer
        let call_site = builder.build_indirect_call(
            method_signature.function_type,
            fn_ptr_typed,
            &metadata_args,
            "interface_call"
        ).map_err(|e| Error::from_str(&format!("Failed to call interface method: {}", e)))?;

        // Return the result if the function has a return type
        Ok(call_site.try_as_basic_value().left())
    }
    
    /// Check if a value implements an interface at runtime (for type assertions)
    pub fn check_instance_of(
        &self,
        context: &'ctx Context,
        builder: &inkwell::builder::Builder<'ctx>,
        interface_value: PointerValue<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Load the vtable pointer from the interface value
        let vtable_ptr_ptr = unsafe {
            // Assuming interface value is { data_ptr, vtable_ptr }
            let interface_type = context.struct_type(&[
                context.i8_type().ptr_type(AddressSpace::default()).into(),
                context.i8_type().ptr_type(AddressSpace::default()).into()
            ], false);
            
            builder.build_struct_gep(
                interface_type,
                interface_value, 
                1, 
                "vtable_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get vtable pointer: {}", e)))?;
        
        // Get element type for vtable pointer
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        
        let vtable_ptr = builder
            .build_load(i8_ptr_type, vtable_ptr_ptr, "vtable_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load vtable pointer: {}", e)))?;
            
        let vtable_ptr = vtable_ptr.into_pointer_value();
        
        // In a real implementation, we'd check a type ID field in the vtable
        // or load runtime type info, but for now we'll use a simple approach.
        
        // Check all vtable implementations to see if any match our target type
        let matches = self.vtable_impls.iter().any(|((_, impl_type_name), _)| {
            impl_type_name == target_type_name
        });
        
        // Create a boolean result
        let result = context.bool_type().const_int(if matches { 1 } else { 0 }, false);
        
        Ok(result.into())
    }
}