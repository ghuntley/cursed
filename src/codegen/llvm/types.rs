//! Type conversion utilities for LLVM IR generation

use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum, BasicType};
use crate::core::type_checker::Type;
use super::string_type::{CursedStringType, StringTypeUtils};
use tracing::{instrument, debug, warn};

/// Convert a Cursed AST type to an LLVM type.
#[instrument(skip(context), level = "debug")]
pub fn convert_type<'ctx>(context: &'ctx Context, ty: &Type) -> Result<BasicTypeEnum<'ctx>, String> {
    debug!("Converting CURSED type to LLVM: {:?}", ty);
    
    match ty {
        // Basic integer types
        Type::Smol => {
            debug!("Converting smol (i8) type");
            Ok(context.i8_type().into())
        }
        Type::Mid => {
            debug!("Converting mid (i16) type");
            Ok(context.i16_type().into())
        }
        Type::Normie => {
            debug!("Converting normie (i32) type");
            Ok(context.i32_type().into())
        }
        Type::Thicc => {
            debug!("Converting thicc (i64) type");
            Ok(context.i64_type().into())
        }
        
        // Floating point types
        Type::Snack => {
            debug!("Converting snack (f32) type");
            Ok(context.f32_type().into())
        }
        Type::Meal => {
            debug!("Converting meal (f64) type");
            Ok(context.f64_type().into())
        }
        
        // Boolean type
        Type::Lit => {
            debug!("Converting lit (bool) type");
            Ok(context.bool_type().into())
        }
        
        // Character types
        Type::Sip | Type::Rune => {
            debug!("Converting sip/rune (i32) type for Unicode");
            Ok(context.i32_type().into()) // Unicode code point
        }
        Type::Byte => {
            debug!("Converting byte (i8) type");
            Ok(context.i8_type().into())
        }
        
        // String type - use our new string struct
        Type::Tea => {
            debug!("Converting tea (string) type to {{i64, i8*}} struct");
            StringTypeUtils::convert_tea_type_to_llvm(context, ty)
                .map_err(|e| format!("Failed to convert string type: {}", e))
        }
        
        // Complex number type
        Type::Extra => {
            debug!("Converting extra (complex) type to {{f64, f64}} struct");
            let f64_type = context.f64_type();
            let complex_struct = context.opaque_struct_type("cursed_complex");
            complex_struct.set_body(&[f64_type.into(), f64_type.into()], false);
            Ok(complex_struct.into())
        }
        
        // Pointer types
        Type::Pointer(target_type) => {
            debug!("Converting pointer type");
            let target_llvm_type = convert_type(context, target_type)?;
            Ok(target_llvm_type.ptr_type(inkwell::AddressSpace::default()).into())
        }
        
        // Array types
        Type::Array(element_type, size) => {
            debug!("Converting array type with size {}", size);
            let element_llvm_type = convert_type(context, element_type)?;
            Ok(element_llvm_type.array_type(*size as u32).into())
        }
        
        // Slice types (represented as {ptr, len, cap})
        Type::Slice(element_type) => {
            debug!("Converting slice type to {{ptr, len, cap}} struct");
            let element_llvm_type = convert_type(context, element_type)?;
            let ptr_type = element_llvm_type.ptr_type(inkwell::AddressSpace::default());
            let len_type = context.i64_type();
            let cap_type = context.i64_type();
            
            let slice_struct = context.opaque_struct_type("cursed_slice");
            slice_struct.set_body(&[ptr_type.into(), len_type.into(), cap_type.into()], false);
            Ok(slice_struct.into())
        }
        
        // Map types (represented as {size, capacity, buckets_ptr} struct)
        Type::Map(key_type, value_type) => {
            debug!("Converting map type to {{size, capacity, buckets_ptr}} struct");
            let size_type = context.i64_type();
            let capacity_type = context.i64_type();
            let buckets_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
            
            let map_struct = context.opaque_struct_type("cursed_map");
            map_struct.set_body(&[
                size_type.into(),
                capacity_type.into(),
                buckets_ptr_type.into(),
            ], false);
            Ok(map_struct.into())
        }
        
        // Channel types (represented as pointer to runtime channel structure)
        Type::Channel(element_type) => {
            debug!("Converting channel type to runtime channel pointer");
            let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
            Ok(i8_ptr_type.into())
        }
        
        // Function types
        Type::Function(param_types, return_type) => {
            debug!("Converting function type");
            let param_llvm_types: Result<Vec<_>, _> = param_types
                .iter()
                .map(|ty| convert_type(context, ty))
                .collect();
            let param_llvm_types = param_llvm_types?;
            
            // Convert BasicTypeEnum to BasicMetadataTypeEnum
            let param_metadata_types: Vec<inkwell::types::BasicMetadataTypeEnum> = param_llvm_types
                .into_iter()
                .map(|ty| ty.into())
                .collect();
            
            let return_llvm_type = convert_type(context, return_type)?;
            let fn_type = return_llvm_type.fn_type(&param_metadata_types, false);
            Ok(fn_type.ptr_type(inkwell::AddressSpace::default()).into())
        }
        
        // Struct types (represented as opaque structs for now)
        Type::Struct(name, type_params) => {
            debug!("Converting struct type: {}", name);
            if type_params.is_empty() {
                let struct_type = context.opaque_struct_type(name);
                Ok(struct_type.into())
            } else {
                // Generic structs need special handling during monomorphization
                warn!("Generic struct type conversion not fully implemented: {}", name);
                let struct_type = context.opaque_struct_type(&format!("{}_generic", name));
                Ok(struct_type.into())
            }
        }
        
        // Interface types (represented as {data_ptr, vtable_ptr})
        Type::Interface(name, type_params) => {
            debug!("Converting interface type: {} to {{data_ptr, vtable_ptr}}", name);
            let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
            let interface_struct = context.opaque_struct_type(&format!("interface_{}", name));
            interface_struct.set_body(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
            Ok(interface_struct.into())
        }
        
        // Generic types
        Type::Generic(name, type_params) => {
            debug!("Converting generic type: {}", name);
            warn!("Generic type conversion requires monomorphization: {}", name);
            let generic_struct = context.opaque_struct_type(&format!("{}_generic", name));
            Ok(generic_struct.into())
        }
        
        // Named types (user-defined types)
        Type::Named(name) => {
            debug!("Converting named type: {}", name);
            // For named types, we create an opaque struct type
            let named_struct = context.opaque_struct_type(name);
            Ok(named_struct.into())
        }
        
        // Type parameters (used during monomorphization)
        Type::TypeParam(name) => {
            debug!("Converting type parameter: {}", name);
            warn!("Type parameter conversion requires monomorphization context: {}", name);
            // Type parameters should be resolved during monomorphization
            // For now, use an opaque pointer
            let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
            Ok(i8_ptr_type.into())
        }
        
        // Unknown type
        Type::Unknown => {
            warn!("Attempted to convert unknown type");
            Err("Cannot convert Unknown type to LLVM".to_string())
        }
    }
}

/// Create LLVM type for basic Cursed types like smol, mid, etc.
#[instrument(skip(context), level = "debug")]
pub fn create_basic_type<'ctx>(context: &'ctx Context, type_name: &str) -> Result<BasicTypeEnum<'ctx>, String> {
    debug!("Creating basic LLVM type for: {}", type_name);
    
    match type_name {
        "lit" => {
            debug!("Creating lit (bool) type");
            Ok(context.bool_type().into())
        }
        "smol" => {
            debug!("Creating smol (i8) type");
            Ok(context.i8_type().into())
        }
        "mid" => {
            debug!("Creating mid (i16) type");
            Ok(context.i16_type().into())
        }
        "normie" => {
            debug!("Creating normie (i32) type");
            Ok(context.i32_type().into())
        }
        "thicc" => {
            debug!("Creating thicc (i64) type");
            Ok(context.i64_type().into())
        }
        "snack" => {
            debug!("Creating snack (f32) type");
            Ok(context.f32_type().into())
        }
        "meal" => {
            debug!("Creating meal (f64) type");
            Ok(context.f64_type().into())
        }
        "tea" => {
            debug!("Creating tea (string) type as {{i64, i8*}} struct");
            let string_type = CursedStringType::new(context);
            Ok(string_type.as_basic_type())
        }
        "sip" | "rune" => {
            debug!("Creating sip/rune (i32) type for Unicode");
            Ok(context.i32_type().into())
        }
        "byte" => {
            debug!("Creating byte (i8) type");
            Ok(context.i8_type().into())
        }
        "extra" => {
            debug!("Creating extra (complex) type as {{f64, f64}} struct");
            let f64_type = context.f64_type();
            let complex_struct = context.opaque_struct_type("cursed_complex");
            complex_struct.set_body(&[f64_type.into(), f64_type.into()], false);
            Ok(complex_struct.into())
        }
        _ => {
            warn!("Unsupported basic type: {}", type_name);
            Err(format!("Unsupported type: {}", type_name))
        }
    }
}

/// Get the size in bytes of a CURSED type
#[instrument(level = "trace")]
pub fn get_type_size(ty: &Type) -> usize {
    match ty {
        Type::Lit => 1,         // bool
        Type::Smol => 1,        // i8
        Type::Byte => 1,        // i8
        Type::Mid => 2,         // i16
        Type::Normie => 4,      // i32
        Type::Sip | Type::Rune => 4, // i32 for Unicode
        Type::Snack => 4,       // f32
        Type::Thicc => 8,       // i64
        Type::Meal => 8,        // f64
        Type::Tea => StringTypeUtils::string_type_size(), // {i64, i8*} = 16 bytes
        Type::Extra => 16,      // {f64, f64} = 16 bytes
        Type::Pointer(_) => 8,  // 64-bit pointer
        Type::Array(element_type, size) => get_type_size(element_type) * size,
        Type::Slice(_) => 24,   // {ptr, len, cap} = 8 + 8 + 8 = 24 bytes
        Type::Map(_, _) => 8,   // pointer to runtime map
        Type::Channel(_) => 8,  // pointer to runtime channel
        Type::Function(_, _) => 8, // function pointer
        Type::Interface(_, _) => 16, // {data_ptr, vtable_ptr} = 8 + 8 = 16 bytes
        Type::Struct(_, _) => 8,     // opaque struct pointer
        Type::Generic(_, _) => 8,    // generic type pointer
        Type::Named(_) => 8,         // named type pointer
        Type::TypeParam(_) => 8,     // type parameter pointer
        Type::Unknown => 0,          // unknown size
    }
}

/// Check if a type is a string type
#[instrument(level = "trace")]
pub fn is_string_type(ty: &Type) -> bool {
    StringTypeUtils::is_string_type(ty)
}

/// Validate that two types are compatible for the given operation
#[instrument(level = "debug")]
pub fn validate_type_compatibility(left: &Type, right: &Type, operation: &str) -> Result<(), String> {
    match operation {
        // String operations
        "+" | "==" | "!=" | "<" | ">" | "<=" | ">=" if is_string_type(left) || is_string_type(right) => {
            StringTypeUtils::validate_string_operation_types(left, right, operation)
                .map_err(|e| e.to_string())
        }
        
        // Arithmetic operations
        "+" | "-" | "*" | "/" | "%" => {
            if is_numeric_type(left) && is_numeric_type(right) {
                Ok(())
            } else {
                Err(format!("Arithmetic operation {} requires numeric types, got {:?} and {:?}", 
                    operation, left, right))
            }
        }
        
        // Comparison operations
        "==" | "!=" => {
            if left == right {
                Ok(())
            } else {
                Err(format!("Equality comparison requires same types, got {:?} and {:?}", left, right))
            }
        }
        
        // Logical operations
        "&&" | "||" => {
            if matches!(left, Type::Lit) && matches!(right, Type::Lit) {
                Ok(())
            } else {
                Err(format!("Logical operation {} requires boolean types", operation))
            }
        }
        
        _ => Err(format!("Unsupported operation: {}", operation))
    }
}

/// Check if a type is numeric
#[instrument(level = "trace")]
pub fn is_numeric_type(ty: &Type) -> bool {
    matches!(ty, 
        Type::Smol | Type::Mid | Type::Normie | Type::Thicc |
        Type::Snack | Type::Meal | Type::Extra | Type::Byte |
        Type::Sip | Type::Rune
    )
}

/// Get the result type of a binary operation
#[instrument(level = "debug")]
pub fn get_binary_operation_result_type(left: &Type, right: &Type, operation: &str) -> Result<Type, String> {
    match operation {
        // String concatenation
        "+" if is_string_type(left) && is_string_type(right) => Ok(Type::Tea),
        
        // String comparisons return boolean
        "==" | "!=" | "<" | ">" | "<=" | ">=" if is_string_type(left) && is_string_type(right) => {
            Ok(Type::Lit)
        }
        
        // Arithmetic operations between numeric types
        "+" | "-" | "*" | "/" | "%" if is_numeric_type(left) && is_numeric_type(right) => {
            // Return the "wider" type
            match (left, right) {
                (Type::Extra, _) | (_, Type::Extra) => Ok(Type::Extra),
                (Type::Meal, _) | (_, Type::Meal) => Ok(Type::Meal),
                (Type::Snack, _) | (_, Type::Snack) => Ok(Type::Snack),
                (Type::Thicc, _) | (_, Type::Thicc) => Ok(Type::Thicc),
                (Type::Normie, _) | (_, Type::Normie) => Ok(Type::Normie),
                (Type::Mid, _) | (_, Type::Mid) => Ok(Type::Mid),
                _ => Ok(Type::Smol),
            }
        }
        
        // Comparison operations return boolean
        "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(Type::Lit),
        
        // Logical operations return boolean
        "&&" | "||" => Ok(Type::Lit),
        
        _ => Err(format!("Cannot determine result type for operation: {}", operation))
    }
}