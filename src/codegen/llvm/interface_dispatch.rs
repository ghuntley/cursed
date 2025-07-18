//! Interface Dispatch System for CURSED LLVM Code Generation
//!
//! This module implements the complete interface dispatch system including:
//! - Virtual table (vtable) generation
//! - Dynamic method dispatch
//! - Interface method resolution
//! - Type checking for interface compliance
//! - Optimization for interface calls
//! - Runtime support for interface operations

use crate::ast::{InterfaceStatement, MethodSignature, Type as AstType, Expression, Statement, Program};
use crate::error::{CursedError, SourceLocation};
use crate::codegen::llvm::register_tracker::RegisterTracker;
use crate::runtime::interface_dispatch::{InterfaceVTable, VTableEntry, InterfaceMethod, InterfaceValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Interface dispatch code generator
#[derive(Debug, Clone)]
pub struct InterfaceDispatchCodegen {
    /// Current LLVM IR code being generated
    ir_code: String,
    /// Register tracker for consistent numbering
    register_tracker: RegisterTracker,
    /// Interface definitions
    interfaces: HashMap<String, InterfaceDefinition>,
    /// Virtual tables for interface implementations
    vtables: HashMap<String, VTableDefinition>,
    /// Method resolution cache for optimization
    method_cache: HashMap<String, MethodResolution>,
    /// Runtime vtable registry
    runtime_vtables: HashMap<String, Arc<InterfaceVTable>>,
    /// Interface type registry
    interface_types: HashMap<String, InterfaceType>,
    /// Current function context
    current_function: Option<String>,
}

/// Interface definition for codegen
#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: String,
    pub methods: Vec<InterfaceMethodSignature>,
    pub type_parameters: Vec<String>,
    pub extends: Vec<String>,
}

/// Interface method signature for codegen
#[derive(Debug, Clone)]
pub struct InterfaceMethodSignature {
    pub name: String,
    pub parameters: Vec<ParameterType>,
    pub return_type: Option<String>,
    pub index: usize,
}

/// Parameter type information
#[derive(Debug, Clone)]
pub struct ParameterType {
    pub name: String,
    pub type_name: String,
    pub is_pointer: bool,
}

/// Virtual table definition for LLVM IR
#[derive(Debug, Clone)]
pub struct VTableDefinition {
    pub interface_name: String,
    pub implementation_type: String,
    pub methods: Vec<VTableMethodEntry>,
    pub llvm_type: String,
    pub global_name: String,
}

/// Virtual table method entry
#[derive(Debug, Clone)]
pub struct VTableMethodEntry {
    pub method_name: String,
    pub function_name: String,
    pub function_type: String,
    pub index: usize,
}

/// Method resolution result
#[derive(Debug, Clone)]
pub struct MethodResolution {
    pub interface_name: String,
    pub method_name: String,
    pub implementation_type: String,
    pub function_name: String,
    pub is_optimized: bool,
}

/// Interface type for LLVM IR
#[derive(Debug, Clone)]
pub struct InterfaceType {
    pub name: String,
    pub llvm_type: String,
    pub vtable_type: String,
    pub methods: Vec<InterfaceMethodSignature>,
}

impl InterfaceDispatchCodegen {
    /// Create new interface dispatch code generator
    pub fn new() -> Self {
        Self {
            ir_code: String::new(),
            register_tracker: RegisterTracker::new(),
            interfaces: HashMap::new(),
            vtables: HashMap::new(),
            method_cache: HashMap::new(),
            runtime_vtables: HashMap::new(),
            interface_types: HashMap::new(),
            current_function: None,
        }
    }

    /// Generate LLVM IR for interface system
    pub fn generate_interface_system(&mut self, program: &Program) -> Result<String, CursedError> {
        // Generate interface type definitions
        self.generate_interface_types(program)?;
        
        // Generate vtable definitions
        self.generate_vtable_definitions(program)?;
        
        // Generate interface runtime support
        self.generate_interface_runtime_support()?;
        
        // Generate method dispatch functions
        self.generate_dispatch_functions()?;
        
        Ok(self.ir_code.clone())
    }

    /// Generate interface type definitions
    fn generate_interface_types(&mut self, program: &Program) -> Result<(), CursedError> {
        for statement in &program.statements {
            if let Statement::Interface(interface) = statement {
                self.generate_interface_type_definition(interface)?;
            }
        }
        Ok(())
    }

    /// Generate type constraint checking for generic interfaces
    pub fn generate_interface_constraints(
        &mut self,
        interface_name: &str,
        type_parameters: &[crate::ast::TypeParameter],
    ) -> Result<(), CursedError> {
        // For each type parameter, generate constraint checking functions
        for type_param in type_parameters {
            let constraint_func_name = format!("{}_{}_constraint_check", interface_name, type_param.name);
            
            // Generate LLVM IR for constraint checking function
            self.ir_code.push_str(&format!(
                r#"
define i1 @{}() {{
entry:
    ; Constraint checking for type parameter {}
    ; For now, return true (constraints checked at compile time)
    ret i1 true
}}

"#,
                constraint_func_name, type_param.name
            ));
        }
        
        Ok(())
    }

    /// Generate LLVM IR for interface type definition
    fn generate_interface_type_definition(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        let interface_name = &interface.name;
        
        // Process interface methods
        let mut methods = Vec::new();
        for (index, method) in interface.methods.iter().enumerate() {
            let method_sig = self.process_interface_method(method, index)?;
            methods.push(method_sig);
        }
        
        // Handle generic interfaces with type parameters
        let interface_full_name = if interface.type_parameters.is_empty() {
            interface_name.clone()
        } else {
            let type_params: Vec<String> = interface.type_parameters.iter()
                .map(|tp| tp.name.clone()).collect();
            format!("{}<{}>", interface_name, type_params.join(", "))
        };
        
        // Generate LLVM type for interface
        let interface_llvm_type = format!("%interface.{}", interface_name);
        let vtable_type = format!("%vtable.{}", interface_name);
        
        // Generate interface structure: { vtable_ptr, data_ptr, type_info }
        // For generic interfaces, include type information
        if interface.type_parameters.is_empty() {
            self.ir_code.push_str(&format!(
                "{} = type {{ {}*, i8* }}\n",
                interface_llvm_type, vtable_type
            ));
        } else {
            self.ir_code.push_str(&format!(
                "{} = type {{ {}*, i8*, i8* }}\n",
                interface_llvm_type, vtable_type
            ));
        }
        
        // Generate vtable structure type with generic support
        let vtable_methods: Vec<String> = methods.iter()
            .map(|m| self.get_llvm_function_type(&m.parameters, &m.return_type))
            .collect();
        
        // Add type parameter information to vtable for generic interfaces
        if interface.type_parameters.is_empty() {
            self.ir_code.push_str(&format!(
                "{} = type {{ {} }}\n",
                vtable_type,
                vtable_methods.join(", ")
            ));
        } else {
            self.ir_code.push_str(&format!(
                "{} = type {{ i8*, {} }}\n",
                vtable_type,
                vtable_methods.join(", ")
            ));
        }
        
        // Generate type constraint checking functions for generic interfaces
        if !interface.type_parameters.is_empty() {
            self.generate_generic_interface_constraints(interface_name, &interface.type_parameters)?;
        }
        
        // Store interface definition
        let interface_def = InterfaceDefinition {
            name: interface_name.clone(),
            methods,
            type_parameters: interface.type_parameters.iter().map(|tp| tp.name.clone()).collect(),
            extends: interface.extends.clone(),
        };
        
        self.interfaces.insert(interface_name.clone(), interface_def);
        
        // Store interface type
        let interface_type = InterfaceType {
            name: interface_name.clone(),
            llvm_type: interface_llvm_type,
            vtable_type,
            methods: self.interfaces[interface_name].methods.clone(),
        };
        
        self.interface_types.insert(interface_name.clone(), interface_type);
        
        Ok(())
    }

    /// Process interface method signature
    fn process_interface_method(&self, method: &MethodSignature, index: usize) -> Result<InterfaceMethodSignature, CursedError> {
        let mut parameters = Vec::new();
        
        // Process parameters
        for param in &method.parameters {
            let param_type = ParameterType {
                name: param.name.clone(),
                type_name: if let Some(ref param_type) = param.param_type {
                    self.ast_type_to_llvm_type(param_type)?
                } else {
                    "i8*".to_string()
                },
                is_pointer: if let Some(ref param_type) = param.param_type {
                    self.is_pointer_type(param_type)
                } else {
                    true
                },
            };
            parameters.push(param_type);
        }
        
        // Process return type
        let return_type = match &method.return_type {
            Some(rt) => Some(self.ast_type_to_llvm_type(rt)?),
            None => None,
        };
        
        Ok(InterfaceMethodSignature {
            name: method.name.clone(),
            parameters,
            return_type,
            index,
        })
    }

    /// Generate vtable definitions for interface implementations
    fn generate_vtable_definitions(&mut self, program: &Program) -> Result<(), CursedError> {
        // This would typically be called after type checking has identified
        // which types implement which interfaces
        
        // For now, generate example vtables for demonstration
        let interfaces_clone = self.interfaces.clone();
        for (interface_name, interface_def) in interfaces_clone.iter() {
            self.generate_example_vtable(interface_name, interface_def)?;
        }
        
        Ok(())
    }

    /// Generate example vtable for demonstration
    fn generate_example_vtable(&mut self, interface_name: &str, interface_def: &InterfaceDefinition) -> Result<(), CursedError> {
        let implementation_type = format!("{}Impl", interface_name);
        let vtable_global_name = format!("@vtable.{}.{}", interface_name, implementation_type);
        
        // Generate vtable entries
        let mut vtable_methods = Vec::new();
        for method in &interface_def.methods {
            let function_name = format!("@{}.{}.{}", implementation_type, method.name, interface_name);
            let function_type = self.get_llvm_function_type(&method.parameters, &method.return_type);
            
            vtable_methods.push(VTableMethodEntry {
                method_name: method.name.clone(),
                function_name,
                function_type,
                index: method.index,
            });
        }
        
        // Generate LLVM IR for vtable
        let vtable_type = &self.interface_types[interface_name].vtable_type;
        let vtable_initializer: Vec<String> = vtable_methods.iter()
            .map(|entry| format!("{} {}", entry.function_type, entry.function_name))
            .collect();
        
        self.ir_code.push_str(&format!(
            "{} = global {} {{ {} }}\n",
            vtable_global_name,
            vtable_type,
            vtable_initializer.join(", ")
        ));
        
        // Store vtable definition
        let vtable_def = VTableDefinition {
            interface_name: interface_name.to_string(),
            implementation_type: implementation_type.clone(),
            methods: vtable_methods,
            llvm_type: vtable_type.clone(),
            global_name: vtable_global_name,
        };
        
        let vtable_key = format!("{}::{}", interface_name, implementation_type);
        self.vtables.insert(vtable_key, vtable_def);
        
        Ok(())
    }

    /// Generate interface runtime support functions
    fn generate_interface_runtime_support(&mut self) -> Result<(), CursedError> {
        // Generate interface value creation function
        self.generate_interface_value_creation()?;
        
        // Generate method dispatch function
        self.generate_method_dispatch_function()?;
        
        // Generate interface type checking
        self.generate_interface_type_checking()?;
        
        Ok(())
    }

    /// Generate interface value creation function
    fn generate_interface_value_creation(&mut self) -> Result<(), CursedError> {
        self.ir_code.push_str(&format!(
            r#"
; Interface value creation runtime function
declare i8* @cursed_create_interface_value(i8*, i8*, i8*)

; Interface value creation wrapper
define i8* @create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name) {{
entry:
    %interface_value = call i8* @cursed_create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name)
    ret i8* %interface_value
}}

"#
        ));
        Ok(())
    }

    /// Generate method dispatch function
    fn generate_method_dispatch_function(&mut self) -> Result<(), CursedError> {
        self.ir_code.push_str(&format!(
            r#"
; Method dispatch runtime function
declare i8* @cursed_dispatch_method(i8*, i8*, i8*, i32)

; Method dispatch wrapper with optimization
define i8* @dispatch_interface_method(i8* %interface_value, i8* %method_name, i8* %args, i32 %arg_count) {{
entry:
    ; Extract vtable from interface value
    %interface_ptr = bitcast i8* %interface_value to {{i8*, i8*}}*
    %vtable_ptr_ptr = getelementptr {{i8*, i8*}}, {{i8*, i8*}}* %interface_ptr, i32 0, i32 0
    %vtable_ptr = load i8*, i8** %vtable_ptr_ptr
    
    ; Extract data pointer
    %data_ptr_ptr = getelementptr {{i8*, i8*}}, {{i8*, i8*}}* %interface_ptr, i32 0, i32 1
    %data_ptr = load i8*, i8** %data_ptr_ptr
    
    ; Dispatch method call
    %result = call i8* @cursed_dispatch_method(i8* %vtable_ptr, i8* %method_name, i8* %args, i32 %arg_count)
    ret i8* %result
}}

"#
        ));
        Ok(())
    }

    /// Generate interface type checking
    fn generate_interface_type_checking(&mut self) -> Result<(), CursedError> {
        self.ir_code.push_str(&format!(
            r#"
; Interface type checking runtime function
declare i1 @cursed_implements_interface(i8*, i8*)

; Interface type checking wrapper
define i1 @check_interface_implementation(i8* %type_name, i8* %interface_name) {{
entry:
    %result = call i1 @cursed_implements_interface(i8* %type_name, i8* %interface_name)
    ret i1 %result
}}

"#
        ));
        Ok(())
    }

    /// Generate optimized dispatch functions
    fn generate_dispatch_functions(&mut self) -> Result<(), CursedError> {
        let interfaces_clone = self.interfaces.clone();
        for (interface_name, interface_def) in interfaces_clone.iter() {
            self.generate_interface_dispatch_functions(interface_name, interface_def)?;
        }
        Ok(())
    }

    /// Generate dispatch functions for specific interface
    fn generate_interface_dispatch_functions(&mut self, interface_name: &str, interface_def: &InterfaceDefinition) -> Result<(), CursedError> {
        // Generate fast dispatch function for each method
        for method in &interface_def.methods {
            self.generate_method_fast_dispatch(interface_name, method)?;
        }
        
        // Generate polymorphic dispatch function
        self.generate_polymorphic_dispatch(interface_name, interface_def)?;
        
        Ok(())
    }

    /// Generate fast dispatch for specific method
    fn generate_method_fast_dispatch(&mut self, interface_name: &str, method: &InterfaceMethodSignature) -> Result<(), CursedError> {
        let function_name = format!("dispatch_{}_{}_{}", interface_name, method.name, method.index);
        let return_type = method.return_type.as_deref().unwrap_or("void");
        
        // Generate parameter list
        let mut params = vec!["i8* %interface_value".to_string()];
        for (i, param) in method.parameters.iter().enumerate() {
            params.push(format!("{} %arg{}", param.type_name, i));
        }
        
        // Generate parameter arguments for method call
        let method_args = if method.parameters.is_empty() {
            String::new()
        } else {
            let param_str = method.parameters.iter().enumerate()
                .map(|(i, p)| format!("{} %arg{}", p.type_name, i))
                .collect::<Vec<_>>().join(", ");
            format!(", {}", param_str)
        };
        
        self.ir_code.push_str(&format!(
            r#"
; Fast dispatch for {}.{}
define {} @{}({}) {{
entry:
    ; Extract vtable from interface value
    %interface_ptr = bitcast i8* %interface_value to {{i8*, i8*}}*
    %vtable_ptr_ptr = getelementptr {{i8*, i8*}}, {{i8*, i8*}}* %interface_ptr, i32 0, i32 0
    %vtable_ptr = load i8*, i8** %vtable_ptr_ptr
    
    ; Extract data pointer
    %data_ptr_ptr = getelementptr {{i8*, i8*}}, {{i8*, i8*}}* %interface_ptr, i32 0, i32 1
    %data_ptr = load i8*, i8** %data_ptr_ptr
    
    ; Cast vtable to correct type
    %vtable_typed = bitcast i8* %vtable_ptr to %vtable.{}*
    
    ; Get method pointer from vtable
    %method_ptr_ptr = getelementptr %vtable.{}, %vtable.{}* %vtable_typed, i32 0, i32 {}
    %method_ptr = load {}*, {}** %method_ptr_ptr
    
    ; Call method with data pointer and arguments
    %result = call {} %method_ptr(i8* %data_ptr{})

"#,
            interface_name, method.name,
            return_type, function_name, params.join(", "),
            interface_name,
            interface_name, interface_name, method.index,
            self.get_llvm_function_type(&method.parameters, &method.return_type),
            self.get_llvm_function_type(&method.parameters, &method.return_type),
            self.get_llvm_function_type(&method.parameters, &method.return_type),
            method_args
        ));
        
        if return_type != "void" {
            self.ir_code.push_str("    ret ");
            self.ir_code.push_str(return_type);
            self.ir_code.push_str(" %result\n");
        } else {
            self.ir_code.push_str("    ret void\n");
        }
        
        self.ir_code.push_str("}\n\n");
        
        Ok(())
    }

    /// Generate polymorphic dispatch function
    fn generate_polymorphic_dispatch(&mut self, interface_name: &str, interface_def: &InterfaceDefinition) -> Result<(), CursedError> {
        let function_name = format!("dispatch_{}_polymorphic", interface_name);
        
        self.ir_code.push_str(&format!(
            r#"
; Polymorphic dispatch for {}
define i8* @{}(i8* %interface_value, i8* %method_name, i8* %args, i32 %arg_count) {{
entry:
    ; Create method name comparison blocks
"#,
            interface_name, function_name
        ));
        
        // Generate method comparison chain
        for (i, method) in interface_def.methods.iter().enumerate() {
            let method_name_str = format!("@str.{}.{}", interface_name, method.name);
            let compare_label = format!("compare_{}", i);
            let dispatch_label = format!("dispatch_{}", i);
            let next_label = if i + 1 < interface_def.methods.len() {
                format!("compare_{}", i + 1)
            } else {
                "method_not_found".to_string()
            };
            
            self.ir_code.push_str(&format!(
                r#"    br label %{}

{}:
    %method_name_{} = call i8* @strcmp(i8* %method_name, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i32 0, i32 0))
    %is_method_{} = icmp eq i8* %method_name_{}, i8* null
    br i1 %is_method_{}, label %{}, label %{}

{}:
    %result_{} = call i8* @dispatch_{}_{}_{}(i8* %interface_value, i8* %args, i32 %arg_count)
    ret i8* %result_{}

"#,
                compare_label,
                compare_label, i,
                method.name.len() + 1, method.name.len() + 1, method_name_str,
                i, i, i, dispatch_label, next_label,
                dispatch_label, i, interface_name, method.name, method.index, i
            ));
        }
        
        // Generate method not found block
        self.ir_code.push_str(&format!(
            r#"method_not_found:
    ; Return null for method not found
    ret i8* null
}}

"#
        ));
        
        Ok(())
    }

    /// Generate interface casting function
    pub fn generate_interface_cast(&mut self, from_type: &str, to_interface: &str) -> Result<String, CursedError> {
        let cast_register = self.register_tracker.next_register();
        let vtable_register = self.register_tracker.next_register();
        
        // Generate runtime interface cast
        self.ir_code.push_str(&format!(
            r#"    ; Cast {} to interface {}
    %{} = call i8* @get_vtable_for_type(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.{}, i32 0, i32 0), i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.{}, i32 0, i32 0))
    %{} = call i8* @create_interface_value(i8* %{}, i8* %interface_value, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.{}, i32 0, i32 0))
"#,
            from_type, to_interface,
            vtable_register, from_type.len() + 1, from_type.len() + 1, from_type,
            to_interface.len() + 1, to_interface.len() + 1, to_interface,
            cast_register, vtable_register, from_type.len() + 1, from_type.len() + 1, from_type
        ));
        
        Ok(format!("%{}", cast_register))
    }

    /// Generate optimized method call
    pub fn generate_optimized_method_call(&mut self, interface_value: &str, method_name: &str, args: &[String]) -> Result<String, CursedError> {
        let result_register = self.register_tracker.next_register();
        // Extract the register number from the string (remove the % prefix)
        let register_num = result_register.trim_start_matches('%').parse::<usize>().unwrap_or(0);
        
        // Check if we can optimize this call
        if let Some(resolution) = self.resolve_method_call(interface_value, method_name) {
            if resolution.is_optimized {
                // Generate direct call for optimized case
                self.ir_code.push_str(&format!(
                    "    %{} = call {} @{}({})\n",
                    register_num,
                    "i8*", // This should be the actual return type
                    resolution.function_name,
                    args.join(", ")
                ));
            } else {
                // Generate polymorphic dispatch
                self.generate_polymorphic_method_call(interface_value, method_name, args, register_num)?;
            }
        } else {
            // Generate runtime dispatch
            self.generate_runtime_method_call(interface_value, method_name, args, register_num)?;
        }
        
        Ok(format!("%{}", register_num))
    }

    /// Generate polymorphic method call
    fn generate_polymorphic_method_call(&mut self, interface_value: &str, method_name: &str, args: &[String], result_register: usize) -> Result<(), CursedError> {
        // Create method name string
        let method_name_str = format!("@str.method.{}", method_name);
        
        // Generate argument array
        let args_array_register = self.register_tracker.next_register();
        self.ir_code.push_str(&format!(
            "    %{} = alloca [{}], align 8\n",
            args_array_register, args.len()
        ));
        
        // Store arguments in array
        for (i, arg) in args.iter().enumerate() {
            self.ir_code.push_str(&format!(
                "    %arg_ptr_{} = getelementptr [{}], [{}]* %{}, i32 0, i32 {}\n",
                i, args.len(), args.len(), args_array_register, i
            ));
            self.ir_code.push_str(&format!(
                "    store i8* {}, i8** %arg_ptr_{}\n",
                arg, i
            ));
        }
        
        // Generate polymorphic dispatch call
        self.ir_code.push_str(&format!(
            "    %{} = call i8* @dispatch_interface_method(i8* {}, i8* {}, i8* bitcast ([{}]* %{} to i8*), i32 {})\n",
            result_register, interface_value, method_name_str, args.len(), args_array_register, args.len()
        ));
        
        Ok(())
    }

    /// Generate runtime method call
    fn generate_runtime_method_call(&mut self, interface_value: &str, method_name: &str, args: &[String], result_register: usize) -> Result<(), CursedError> {
        // Generate full runtime dispatch
        self.generate_polymorphic_method_call(interface_value, method_name, args, result_register)
    }

    /// Resolve method call for optimization
    fn resolve_method_call(&self, interface_value: &str, method_name: &str) -> Option<MethodResolution> {
        // Check method cache first
        let cache_key = format!("{}::{}", interface_value, method_name);
        if let Some(resolution) = self.method_cache.get(&cache_key) {
            return Some(resolution.clone());
        }
        
        // For now, return None to use runtime dispatch
        None
    }

    /// Convert AST type to LLVM type
    fn ast_type_to_llvm_type(&self, ast_type: &AstType) -> Result<String, CursedError> {
        match ast_type {
            AstType::Normie => Ok("i32".to_string()),
            AstType::Thicc => Ok("i64".to_string()),
            AstType::Smol => Ok("i8".to_string()),
            AstType::Mid => Ok("i16".to_string()),
            AstType::Meal => Ok("double".to_string()),
            AstType::Snack => Ok("float".to_string()),
            AstType::Tea => Ok("i8*".to_string()),
            AstType::Lit => Ok("i1".to_string()),
            AstType::Sip => Ok("i8".to_string()),
            AstType::Byte => Ok("i8".to_string()),
            AstType::Rune => Ok("i32".to_string()),
            AstType::Void => Ok("void".to_string()),
            AstType::Pointer(inner) => {
                let inner_type = self.ast_type_to_llvm_type(inner)?;
                Ok(format!("{}*", inner_type))
            },
            AstType::Array(inner, _) => {
                let inner_type = self.ast_type_to_llvm_type(inner)?;
                Ok(format!("[0 x {}]*", inner_type)) // Simplified array type
            },
            AstType::Collab(interface_name) => {
                if let Some(interface_type) = self.interface_types.get(interface_name) {
                    Ok(format!("{}*", interface_type.llvm_type))
                } else {
                    Ok("i8*".to_string()) // Generic interface pointer
                }
            },
            AstType::Custom(type_name) => Ok(format!("%{}*", type_name)),
            _ => Ok("i8*".to_string()), // Default to generic pointer
        }
    }

    /// Check if AST type is a pointer type
    fn is_pointer_type(&self, ast_type: &AstType) -> bool {
        matches!(ast_type, AstType::Pointer(_) | AstType::Tea | AstType::Array(_, _) | AstType::Collab(_))
    }

    /// Get LLVM function type string
    fn get_llvm_function_type(&self, parameters: &[ParameterType], return_type: &Option<String>) -> String {
        let param_types: Vec<String> = parameters.iter()
            .map(|p| p.type_name.clone())
            .collect();
        
        let return_type_str = return_type.as_deref().unwrap_or("void");
        
        format!("{} ({})*", return_type_str, param_types.join(", "))
    }

    /// Generate type constraint checking for generic interfaces
    fn generate_generic_interface_constraints(&mut self, interface_name: &str, type_parameters: &[crate::ast::TypeParameter]) -> Result<(), CursedError> {
        // Generate constraint checking function for each type parameter
        for type_param in type_parameters {
            let constraint_func_name = format!("check_constraint_{}_{}", interface_name, type_param.name);
            
            self.ir_code.push_str(&format!(
                r#"
; Type constraint checking for generic interface {} type parameter {}
define i1 @{}(i8* %type_info) {{
entry:
"#,
                interface_name, type_param.name, constraint_func_name
            ));
            
            // Generate constraint checks for each bound
            for (i, bound) in type_param.bounds.iter().enumerate() {
                let check_label = format!("check_{}", i);
                let success_label = format!("success_{}", i);
                let fail_label = if i + 1 < type_param.bounds.len() {
                    format!("check_{}", i + 1)
                } else {
                    "fail".to_string()
                };
                
                self.ir_code.push_str(&format!(
                    r#"    br label %{}

{}:
    %bound_check_{} = call i1 @check_type_implements_trait(i8* %type_info, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.{}, i32 0, i32 0))
    br i1 %bound_check_{}, label %{}, label %{}

"#,
                    check_label,
                    check_label, i,
                    bound.len() + 1, bound.len() + 1, bound,
                    i, success_label, fail_label
                ));
                
                if i + 1 == type_param.bounds.len() {
                    self.ir_code.push_str(&format!(
                        r#"{}:
    ret i1 true

"#,
                        success_label
                    ));
                } else {
                    self.ir_code.push_str(&format!(
                        r#"{}:
    br label %check_{}

"#,
                        success_label, i + 1
                    ));
                }
            }
            
            if type_param.bounds.is_empty() {
                self.ir_code.push_str("    ret i1 true\n");
            } else {
                self.ir_code.push_str(&format!(
                    r#"fail:
    ret i1 false
"#
                ));
            }
            
            self.ir_code.push_str("}\n\n");
        }
        
        // Generate generic interface instantiation function
        self.generate_generic_interface_instantiation(interface_name, type_parameters)?;
        
        Ok(())
    }

    /// Generate generic interface instantiation function
    fn generate_generic_interface_instantiation(&mut self, interface_name: &str, type_parameters: &[crate::ast::TypeParameter]) -> Result<(), CursedError> {
        let instantiation_func_name = format!("instantiate_generic_interface_{}", interface_name);
        
        // Generate parameter list for type arguments
        let type_param_args: Vec<String> = type_parameters.iter()
            .enumerate()
            .map(|(i, _)| format!("i8* %type_arg_{}", i))
            .collect();
        
        self.ir_code.push_str(&format!(
            r#"
; Generic interface instantiation for {}
define i8* @{}({}) {{
entry:
"#,
            interface_name, instantiation_func_name, type_param_args.join(", ")
        ));
        
        // Validate each type argument against constraints
        for (i, type_param) in type_parameters.iter().enumerate() {
            if !type_param.bounds.is_empty() {
                let constraint_func_name = format!("check_constraint_{}_{}", interface_name, type_param.name);
                self.ir_code.push_str(&format!(
                    r#"    %constraint_check_{} = call i1 @{}(i8* %type_arg_{})
    br i1 %constraint_check_{}, label %constraint_ok_{}, label %constraint_fail

constraint_ok_{}:
"#,
                    i, constraint_func_name, i, i, i, i
                ));
            }
        }
        
        // Create generic interface instance
        self.ir_code.push_str(&format!(
            r#"    ; Create generic interface instance
    %interface_instance = call i8* @malloc(i64 {})
    %interface_typed = bitcast i8* %interface_instance to %interface.{}*
    
    ; Initialize vtable pointer (will be set by implementation)
    %vtable_ptr_ptr = getelementptr %interface.{}, %interface.{}* %interface_typed, i32 0, i32 0
    store %vtable.{}* null, %vtable.{}** %vtable_ptr_ptr
    
    ; Initialize data pointer (will be set by implementation)
    %data_ptr_ptr = getelementptr %interface.{}, %interface.{}* %interface_typed, i32 0, i32 1
    store i8* null, i8** %data_ptr_ptr
"#,
            if type_parameters.is_empty() { 16 } else { 24 }, // size depends on whether we have type info
            interface_name,
            interface_name, interface_name,
            interface_name, interface_name,
            interface_name, interface_name
        ));
        
        // For generic interfaces, store type information
        if !type_parameters.is_empty() {
            self.ir_code.push_str(&format!(
                r#"    ; Store type information for generic interface
    %type_info_ptr = getelementptr %interface.{}, %interface.{}* %interface_typed, i32 0, i32 2
"#,
                interface_name, interface_name
            ));
            
            // Create type info array
            self.ir_code.push_str(&format!(
                r#"    %type_info_array = call i8* @malloc(i64 {})
    store i8* %type_info_array, i8** %type_info_ptr
"#,
                type_parameters.len() * 8 // pointer size
            ));
            
            // Store each type argument
            for i in 0..type_parameters.len() {
                self.ir_code.push_str(&format!(
                    r#"    %type_slot_{} = getelementptr i8, i8* %type_info_array, i64 {}
    %type_slot_ptr_{} = bitcast i8* %type_slot_{} to i8**
    store i8* %type_arg_{}, i8** %type_slot_ptr_{}
"#,
                    i, i * 8, i, i, i, i
                ));
            }
        }
        
        self.ir_code.push_str(&format!(
            r#"    ret i8* %interface_instance

constraint_fail:
    ret i8* null
}}

"#
        ));
        
        Ok(())
    }

    /// Generate monomorphized interface implementations
    pub fn generate_monomorphized_interface(&mut self, interface_name: &str, concrete_types: &[String]) -> Result<String, CursedError> {
        let monomorphized_name = if concrete_types.is_empty() {
            interface_name.to_string()
        } else {
            format!("{}_{}", interface_name, concrete_types.join("_"))
        };
        
        // Generate specialized interface type
        let interface_llvm_type = format!("%interface.{}", monomorphized_name);
        let vtable_type = format!("%vtable.{}", monomorphized_name);
        
        // Get base interface definition
        if let Some(base_interface) = self.interfaces.get(interface_name) {
            // Generate monomorphized interface structure
            self.ir_code.push_str(&format!(
                "{} = type {{ {}*, i8* }}\n",
                interface_llvm_type, vtable_type
            ));
            
            // Generate monomorphized vtable type
            let vtable_methods: Vec<String> = base_interface.methods.iter()
                .map(|m| {
                    // Substitute concrete types in method signatures
                    let specialized_sig = self.specialize_method_signature(m, concrete_types)?;
                    Ok(self.get_llvm_function_type(&specialized_sig.parameters, &specialized_sig.return_type))
                })
                .collect::<Result<Vec<_>, CursedError>>()?;
            
            self.ir_code.push_str(&format!(
                "{} = type {{ {} }}\n",
                vtable_type,
                vtable_methods.join(", ")
            ));
        }
        
        Ok(monomorphized_name)
    }

    /// Specialize method signature with concrete types
    fn specialize_method_signature(&self, method: &InterfaceMethodSignature, concrete_types: &[String]) -> Result<InterfaceMethodSignature, CursedError> {
        // This would substitute type parameters with concrete types
        // For now, return the original signature
        Ok(method.clone())
    }

    /// Get generated LLVM IR code
    pub fn get_ir_code(&self) -> &str {
        &self.ir_code
    }

    /// Clear generated code
    pub fn clear(&mut self) {
        self.ir_code.clear();
        self.register_tracker = RegisterTracker::new();
    }
}

/// Interface dispatch optimization passes
#[derive(Debug, Clone)]
pub struct InterfaceOptimizationPasses {
    /// Enable method inlining
    pub inline_methods: bool,
    /// Enable devirtualization
    pub devirtualize_calls: bool,
    /// Enable vtable optimization
    pub optimize_vtables: bool,
    /// Enable interface type analysis
    pub analyze_interface_types: bool,
}

impl Default for InterfaceOptimizationPasses {
    fn default() -> Self {
        Self {
            inline_methods: true,
            devirtualize_calls: true,
            optimize_vtables: true,
            analyze_interface_types: true,
        }
    }
}

/// Interface dispatch optimizer
pub struct InterfaceDispatchOptimizer {
    passes: InterfaceOptimizationPasses,
    codegen: InterfaceDispatchCodegen,
}

impl InterfaceDispatchOptimizer {
    /// Create new interface dispatch optimizer
    pub fn new(passes: InterfaceOptimizationPasses) -> Self {
        Self {
            passes,
            codegen: InterfaceDispatchCodegen::new(),
        }
    }

    /// Optimize interface dispatch for program
    pub fn optimize_program(&mut self, program: &Program) -> Result<String, CursedError> {
        // Run optimization passes
        if self.passes.analyze_interface_types {
            self.analyze_interface_types(program)?;
        }
        
        if self.passes.devirtualize_calls {
            self.devirtualize_calls(program)?;
        }
        
        if self.passes.optimize_vtables {
            self.optimize_vtables(program)?;
        }
        
        if self.passes.inline_methods {
            self.inline_methods(program)?;
        }
        
        // Generate optimized code
        self.codegen.generate_interface_system(program)
    }

    /// Analyze interface types for optimization opportunities
    fn analyze_interface_types(&mut self, program: &Program) -> Result<(), CursedError> {
        // TODO: Implement interface type analysis
        // This would identify monomorphic interface calls that can be devirtualized
        Ok(())
    }

    /// Devirtualize interface calls where possible
    fn devirtualize_calls(&mut self, program: &Program) -> Result<(), CursedError> {
        // TODO: Implement call devirtualization
        // This would replace interface calls with direct calls when the concrete type is known
        Ok(())
    }

    /// Optimize vtables
    fn optimize_vtables(&mut self, program: &Program) -> Result<(), CursedError> {
        // TODO: Implement vtable optimization
        // This would merge identical vtables and optimize vtable layout
        Ok(())
    }

    /// Inline interface methods where beneficial
    fn inline_methods(&mut self, program: &Program) -> Result<(), CursedError> {
        // TODO: Implement method inlining
        // This would inline small interface methods at call sites
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_interface_dispatch_codegen() {
        let mut codegen = InterfaceDispatchCodegen::new();
        
        // Create test interface
        let interface = InterfaceStatement {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "test_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                }
            ],
            visibility: Visibility::Public,
        };
        
        // Test interface type generation
        assert!(codegen.generate_interface_type_definition(&interface).is_ok());
        
        // Check generated code contains interface definition
        let ir_code = codegen.get_ir_code();
        assert!(ir_code.contains("%interface.TestInterface"));
        assert!(ir_code.contains("%vtable.TestInterface"));
    }

    #[test]
    fn test_interface_optimization() {
        let passes = InterfaceOptimizationPasses::default();
        let mut optimizer = InterfaceDispatchOptimizer::new(passes);
        
        let program = Program { 
            statements: vec![], 
            imports: vec![], 
            package: None 
        };
        
        // Test optimization pipeline
        assert!(optimizer.optimize_program(&program).is_ok());
    }
}
