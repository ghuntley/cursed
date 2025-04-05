use crate::ast;
use crate::core::type_checker::Type;
use crate::core::generic_instantiation::GenericInstantiator;
use crate::error::Error;
use std::collections::HashMap;

/// Helper for generating LLVM code for generic type instantiations
pub struct LlvmGenericCodegen {
    // Maps type parameter names to their concrete LLVM types
    type_map: HashMap<String, String>,
    
    // Maps generic function/struct names to their instantiated versions
    instantiated: HashMap<String, Vec<String>>,
}

impl LlvmGenericCodegen {
    /// Create a new LLVM generic code generator
    pub fn new() -> Self {
        LlvmGenericCodegen {
            type_map: HashMap::new(),
            instantiated: HashMap::new(),
        }
    }
    
    /// Register a concrete instantiation of a generic type or function
    pub fn register_instantiation(&mut self, generic_name: &str, concrete_name: &str) {
        let instantiations = self.instantiated.entry(generic_name.to_string())
            .or_insert_with(Vec::new);
        
        // Only add if not already present
        if !instantiations.contains(&concrete_name.to_string()) {
            instantiations.push(concrete_name.to_string());
        }
    }
    
    /// Generate a unique name for a concrete instantiation
    pub fn generate_instantiation_name(&self, generic_name: &str, type_args: &[Type]) -> String {
        // Generate a name like: generic_name_T1_T2_...
        let type_suffix = type_args.iter()
            .map(|t| t.to_string().replace("[", "_").replace("]", "_").replace(", ", "_"))
            .collect::<Vec<String>>()
            .join("_");
        
        format!("{}__{}", generic_name, type_suffix)
    }
    
    /// Generate LLVM type for a CURSED type
    pub fn generate_llvm_type(&self, ty: &Type) -> Result<String, Error> {
        match ty {
            Type::Lit => Ok("i1".to_string()),                       // boolean -> i1
            Type::Smol => Ok("i8".to_string()),                      // int8 -> i8
            Type::Mid => Ok("i16".to_string()),                      // int16 -> i16
            Type::Normie => Ok("i32".to_string()),                   // int32 -> i32
            Type::Thicc => Ok("i64".to_string()),                    // int64 -> i64
            Type::Snack => Ok("float".to_string()),                  // float32 -> float
            Type::Meal => Ok("double".to_string()),                  // float64 -> double
            Type::Tea => Ok("%String".to_string()),                  // string -> %String
            Type::Sip => Ok("i32".to_string()),                      // char/rune -> i32
            Type::Byte => Ok("i8".to_string()),                      // byte -> i8
            
            Type::Pointer(target) => {
                let llvm_target = self.generate_llvm_type(target)?;
                Ok(format!("{}*", llvm_target))
            },
            
            Type::TypeParam(name) => {
                // Look up the concrete LLVM type for this parameter
                if let Some(llvm_type) = self.type_map.get(name) {
                    Ok(llvm_type.clone())
                } else {
                    Err(Error::from_str(&format!("Unknown type parameter in LLVM codegen: {}", name)))
                }
            },
            
            // For other complex types, we would need to generate appropriate LLVM struct types
            // This is a simplified implementation
            _ => Err(Error::from_str(&format!("Unsupported type in LLVM codegen: {}", ty.to_string()))),
        }
    }
    
    /// Instantiate a generic function with concrete type arguments
    pub fn instantiate_generic_function(
        &mut self,
        generic_function: &ast::FunctionLiteral,
        type_args: &[Type]
    ) -> Result<String, Error> {
        // Check if we have the right number of type arguments
        if generic_function.type_parameters.len() != type_args.len() {
            return Err(Error::from_str(
                &format!("Expected {} type arguments, got {}",
                    generic_function.type_parameters.len(),
                    type_args.len())
            ));
        }
        
        // Create type parameter mapping
        let mut instantiator = GenericInstantiator::new();
        for (param, arg) in generic_function.type_parameters.iter().zip(type_args.iter()) {
            instantiator.add_type_param(&param.value, arg.clone());
            
            // Also add to the LLVM type map
            let llvm_type = self.generate_llvm_type(arg)?;
            self.type_map.insert(param.value.clone(), llvm_type);
        }
        
        // Generate a unique name for this instantiation
        let concrete_name = self.generate_instantiation_name(
            &generic_function.token.token_literal(),
            type_args
        );
        
        // Register this instantiation
        self.register_instantiation(&generic_function.token.token_literal(), &concrete_name);
        
        // Return the instantiated function name
        Ok(concrete_name)
    }
}

/// Trait for LLVM code generators that support generics
pub trait LlvmGenericCodeGenerator {
    /// Generate LLVM code for a generic type instantiation
    fn generate_generic_type(
        &mut self,
        generic_type: &Type,
        type_args: &[Type]
    ) -> Result<String, Error>;
    
    /// Generate LLVM code for a generic function instantiation
    fn generate_generic_function(
        &mut self,
        generic_function: &ast::FunctionLiteral,
        type_args: &[Type]
    ) -> Result<String, Error>;
}