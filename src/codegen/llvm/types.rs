use crate::error::Error;
/// LLVM-related type definitions for the CURSED compiler
/// 
/// This module provides type definitions for LLVM code generation
/// that are commonly referenced but may be missing from specific modules.

use std::collections::HashMap;

/// LLVM value reference type alias for compatibility
pub type LLVMValueRef = *mut std::ffi::c_void;

/// LLVM type reference type alias for compatibility  
pub type LLVMTypeRef = *mut std::ffi::c_void;

/// LLVM context reference type alias for compatibility
pub type LLVMContextRef = *mut std::ffi::c_void;

/// LLVM module reference type alias for compatibility
pub type LLVMModuleRef = *mut std::ffi::c_void;

/// LLVM builder reference type alias for compatibility
pub type LLVMBuilderRef = *mut std::ffi::c_void;

/// LLVM value wrapper with type information
#[derive(Debug, Clone)]
pub struct LlvmValue {
    /// The LLVM value reference
    pub value: LLVMValueRef,
    /// Type information for this value
    pub value_type: LlvmType,
    /// Name of the value (for debugging)
    pub name: Option<String>,
}

impl LlvmValue {
    /// Create a new LLVM value
    pub fn new(value: LLVMValueRef, value_type: LlvmType) -> Self {
        Self {
            value,
            value_type,
            name: None,
        }
    }

    /// Create a new LLVM value with name
    pub fn with_name(value: LLVMValueRef, value_type: LlvmType, name: String) -> Self {
        Self {
            value,
            value_type,
            name: Some(name),
        }
    }

    /// Get the LLVM value reference
    pub fn get_value(&self) -> LLVMValueRef {
        self.value
    }

    /// Get the type of this value
    pub fn get_type(&self) -> &LlvmType {
        &self.value_type
    }

    /// Get the name of this value
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Check if this value is null
    pub fn is_null(&self) -> bool {
        self.value.is_null()
    }
}

/// LLVM type wrapper
#[derive(Debug, Clone)]
pub enum LlvmType {
    /// Void type
    Void,
    /// Integer type with bit width
    Integer(u32),
    /// Floating point type
    Float,
    /// Double precision floating point
    Double,
    /// Pointer type
    Pointer(Box<LlvmType>),
    /// Array type
    Array(Box<LlvmType>, usize),
    /// Structure type
    Struct(Vec<LlvmType>),
    /// Function type
    Function {
        return_type: Box<LlvmType>,
        parameter_types: Vec<LlvmType>,
        is_var_arg: bool,
    },
    /// Named type (for user-defined types)
    Named(String),
}

impl LlvmType {
    /// Create an integer type
    pub fn i32() -> Self {
        LlvmType::Integer(32)
    }

    /// Create a 64-bit integer type
    pub fn i64() -> Self {
        LlvmType::Integer(64)
    }

    /// Create a boolean type (i1)
    pub fn bool() -> Self {
        LlvmType::Integer(1)
    }

    /// Create a float type
    pub fn float() -> Self {
        LlvmType::Float
    }

    /// Create a double type
    pub fn double() -> Self {
        LlvmType::Double
    }

    /// Create a pointer type
    pub fn pointer(pointee_type: LlvmType) -> Self {
        LlvmType::Pointer(Box::new(pointee_type))
    }

    /// Create an array type
    pub fn array(element_type: LlvmType, length: usize) -> Self {
        LlvmType::Array(Box::new(element_type), length)
    }

    /// Create a struct type
    pub fn struct_type(field_types: Vec<LlvmType>) -> Self {
        LlvmType::Struct(field_types)
    }

    /// Create a function type
    pub fn function(return_type: LlvmType, parameter_types: Vec<LlvmType>) -> Self {
        LlvmType::Function {
            return_type: Box::new(return_type),
            parameter_types,
            is_var_arg: false,
        }
    }

    /// Create a variadic function type
    pub fn var_arg_function(return_type: LlvmType, parameter_types: Vec<LlvmType>) -> Self {
        LlvmType::Function {
            return_type: Box::new(return_type),
            parameter_types,
            is_var_arg: true,
        }
    }

    /// Get the string representation of this type
    pub fn to_string(&self) -> String {
        match self {
            LlvmType::Void => "void".to_string(),
            LlvmType::Integer(bits) => format!("i{}", bits),
            LlvmType::Float => "float".to_string(),
            LlvmType::Double => "double".to_string(),
            LlvmType::Pointer(pointee) => format!("{}*", pointee.to_string()),
            LlvmType::Array(element, length) => format!("[{} x {}]", length, element.to_string()),
            LlvmType::Struct(fields) => {
                let field_strs: Vec<String> = fields.iter().map(|f| f.to_string()).collect();
                format!("{{ {} }}", field_strs.join(", "))
            }
            LlvmType::Function { return_type, parameter_types, is_var_arg } => {
                let param_strs: Vec<String> = parameter_types.iter().map(|p| p.to_string()).collect();
                let params = if *is_var_arg {
                    format!("{}, ...", param_strs.join(", "))
                } else {
                    param_strs.join(", ")
                };
                format!("{} ({})", return_type.to_string(), params)
            }
            LlvmType::Named(name) => name.clone(),
        }
    }

    /// Check if this is a pointer type
    pub fn is_pointer(&self) -> bool {
        matches!(self, LlvmType::Pointer(_))
    }

    /// Check if this is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(self, LlvmType::Integer(_))
    }

    /// Check if this is a floating point type
    pub fn is_float(&self) -> bool {
        matches!(self, LlvmType::Float | LlvmType::Double)
    }

    /// Check if this is a function type
    pub fn is_function(&self) -> bool {
        matches!(self, LlvmType::Function { .. })
    }
}

/// LLVM compilation context
#[derive(Debug)]
pub struct LlvmContext {
    /// LLVM context reference
    pub context: LLVMContextRef,
    /// Module being compiled
    pub module: LLVMModuleRef,
    /// IR builder
    pub builder: LLVMBuilderRef,
    /// Named values (variables, functions, etc.)
    pub named_values: HashMap<String, LlvmValue>,
    /// Type definitions
    pub type_definitions: HashMap<String, LlvmType>,
}

impl LlvmContext {
    /// Create a new LLVM context
    pub fn new(context: LLVMContextRef, module: LLVMModuleRef, builder: LLVMBuilderRef) -> Self {
        Self {
            context,
            module,
            builder,
            named_values: HashMap::new(),
            type_definitions: HashMap::new(),
        }
    }

    /// Add a named value to the context
    pub fn add_named_value(&mut self, name: String, value: LlvmValue) {
        self.named_values.insert(name, value);
    }

    /// Get a named value from the context
    pub fn get_named_value(&self, name: &str) -> Option<&LlvmValue> {
        self.named_values.get(name)
    }

    /// Add a type definition
    pub fn add_type_definition(&mut self, name: String, llvm_type: LlvmType) {
        self.type_definitions.insert(name, llvm_type);
    }

    /// Get a type definition
    pub fn get_type_definition(&self, name: &str) -> Option<&LlvmType> {
        self.type_definitions.get(name)
    }

    /// Get the LLVM context reference
    pub fn get_context(&self) -> LLVMContextRef {
        self.context
    }

    /// Get the module reference
    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
    }

    /// Get the builder reference
    pub fn get_builder(&self) -> LLVMBuilderRef {
        self.builder
    }
}

/// LLVM function signature
#[derive(Debug, Clone)]
pub struct LlvmFunctionSignature {
    /// Function name
    pub name: String,
    /// Return type
    pub return_type: LlvmType,
    /// Parameter types
    pub parameter_types: Vec<LlvmType>,
    /// Parameter names
    pub parameter_names: Vec<String>,
    /// Whether the function is variadic
    pub is_var_arg: bool,
    /// Whether the function is external (declaration only)
    pub is_external: bool,
}

impl LlvmFunctionSignature {
    /// Create a new function signature
    pub fn new(name: String, return_type: LlvmType) -> Self {
        Self {
            name,
            return_type,
            parameter_types: Vec::new(),
            parameter_names: Vec::new(),
            is_var_arg: false,
            is_external: false,
        }
    }

    /// Add a parameter to the function signature
    pub fn add_parameter(&mut self, name: String, param_type: LlvmType) {
        self.parameter_names.push(name);
        self.parameter_types.push(param_type);
    }

    /// Mark as variadic function
    pub fn set_var_arg(mut self, is_var_arg: bool) -> Self {
        self.is_var_arg = is_var_arg;
        self
    }

    /// Mark as external function
    pub fn set_external(mut self, is_external: bool) -> Self {
        self.is_external = is_external;
        self
    }

    /// Get the function type
    pub fn get_function_type(&self) -> LlvmType {
        LlvmType::Function {
            return_type: Box::new(self.return_type.clone()),
            parameter_types: self.parameter_types.clone(),
            is_var_arg: self.is_var_arg,
        }
    }
}

/// LLVM compilation options
#[derive(Debug, Clone)]
pub struct LlvmCompileOptions {
    /// Optimization level (0-3)
    pub optimization_level: u32,
    /// Target triple
    pub target_triple: Option<String>,
    /// Whether to emit debug information
    pub emit_debug_info: bool,
    /// Whether to enable fast math optimizations
    pub fast_math: bool,
    /// Additional LLVM passes to run
    pub additional_passes: Vec<String>,
}

impl Default for LlvmCompileOptions {
    fn default() -> Self {
        Self {
            optimization_level: 2,
            target_triple: None,
            emit_debug_info: false,
            fast_math: false,
            additional_passes: Vec::new(),
        }
    }
}

/// LLVM compilation result
#[derive(Debug)]
pub struct LlvmCompileResult {
    /// Whether compilation was successful
    pub success: bool,
    /// Generated LLVM IR code
    pub llvm_ir: Option<String>,
    /// Error messages if compilation failed
    pub errors: Vec<String>,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Compilation statistics
    pub stats: LlvmCompileStats,
}

impl LlvmCompileResult {
    /// Create a successful compilation result
    pub fn success(llvm_ir: String) -> Self {
        Self {
            success: true,
            llvm_ir: Some(llvm_ir),
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: LlvmCompileStats::default(),
        }
    }

    /// Create a failed compilation result
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            success: false,
            llvm_ir: None,
            errors,
            warnings: Vec::new(),
            stats: LlvmCompileStats::default(),
        }
    }

    /// Add a warning message
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// LLVM compilation statistics
#[derive(Debug, Default)]
pub struct LlvmCompileStats {
    /// Number of functions compiled
    pub functions_compiled: usize,
    /// Number of instructions generated
    pub instructions_generated: usize,
    /// Compilation time in milliseconds
    pub compile_time_ms: u64,
    /// IR code size in bytes
    pub ir_size_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llvm_type_creation() {
        let int_type = LlvmType::i32();
        assert!(int_type.is_integer());
        assert_eq!(int_type.to_string(), "i32");

        let ptr_type = LlvmType::pointer(LlvmType::i64());
        assert!(ptr_type.is_pointer());
        assert_eq!(ptr_type.to_string(), "i64*");

        let func_type = LlvmType::function(LlvmType::i32(), vec![LlvmType::i32(), LlvmType::i32()]);
        assert!(func_type.is_function());
        assert_eq!(func_type.to_string(), "i32 (i32, i32)");
    }

    #[test]
    fn test_llvm_value_creation() {
        let value = LlvmValue::new(std::ptr::null_mut(), LlvmType::i32());
        assert!(value.is_null());
        assert_eq!(value.get_type().to_string(), "i32");

        let named_value = LlvmValue::with_name(std::ptr::null_mut(), LlvmType::double(), "test".to_string());
        assert_eq!(named_value.get_name(), Some("test"));
    }

    #[test]
    fn test_function_signature() {
        let mut sig = LlvmFunctionSignature::new("test_func".to_string(), LlvmType::i32());
        sig.add_parameter("x".to_string(), LlvmType::i32());
        sig.add_parameter("y".to_string(), LlvmType::i32());

        assert_eq!(sig.parameter_types.len(), 2);
        assert_eq!(sig.parameter_names.len(), 2);

        let func_type = sig.get_function_type();
        assert!(func_type.is_function());
    }
}
