//! Automatic binding generator for FFI functions
//!
//! This module generates CURSED bindings from parsed C header information,
//! creating type-safe wrapper functions and appropriate marshalling code.

use std::collections::HashMap;
use std::fmt::Write;
use crate::error::CursedError;
use super::header_parser::*;
use super::{FfiType, FunctionSignature, Parameter, FfiFunctionWrapper};

/// Generated bindings from header parsing
#[derive(Debug, Clone)]
pub struct GeneratedBindings {
    pub functions: HashMap<String, Box<dyn FfiFunctionWrapper>>,
    pub types: HashMap<String, FfiType>,
    pub constants: HashMap<String, String>,
    pub cursed_code: String,
    pub rust_code: String,
}

/// Code generation options
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    pub generate_cursed_wrapper: bool,
    pub generate_rust_wrapper: bool,
    pub generate_documentation: bool,
    pub use_safe_wrappers: bool,
    pub handle_null_pointers: bool,
    pub generate_type_checks: bool,
    pub include_error_handling: bool,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            generate_cursed_wrapper: true,
            generate_rust_wrapper: true,
            generate_documentation: true,
            use_safe_wrappers: true,
            handle_null_pointers: true,
            generate_type_checks: true,
            include_error_handling: true,
        }
    }
}

/// Binding generator for creating FFI wrappers
pub struct BindingGenerator {
    options: CodeGenOptions,
    type_mappings: HashMap<String, String>,
    error_handling_templates: HashMap<String, String>,
}

impl BindingGenerator {
    /// Create a new binding generator
    pub fn new() -> Self {
        let mut generator = Self {
            options: CodeGenOptions::default(),
            type_mappings: HashMap::new(),
            error_handling_templates: HashMap::new(),
        };
        
        generator.initialize_type_mappings();
        generator.initialize_error_handling_templates();
        generator
    }
    
    /// Create binding generator with custom options
    pub fn with_options(options: CodeGenOptions) -> Self {
        let mut generator = Self {
            options,
            type_mappings: HashMap::new(),
            error_handling_templates: HashMap::new(),
        };
        
        generator.initialize_type_mappings();
        generator.initialize_error_handling_templates();
        generator
    }
    
    /// Initialize type mappings from C to CURSED
    fn initialize_type_mappings(&mut self) {
        // Basic types
        self.type_mappings.insert("void".to_string(), "cringe".to_string());
        self.type_mappings.insert("int".to_string(), "normie".to_string());
        self.type_mappings.insert("char".to_string(), "sip".to_string());
        self.type_mappings.insert("unsigned char".to_string(), "byte".to_string());
        self.type_mappings.insert("short".to_string(), "smol".to_string());
        self.type_mappings.insert("unsigned short".to_string(), "mid".to_string());
        self.type_mappings.insert("long".to_string(), "thicc".to_string());
        self.type_mappings.insert("unsigned long".to_string(), "thicc".to_string());
        self.type_mappings.insert("long long".to_string(), "thicc".to_string());
        self.type_mappings.insert("unsigned long long".to_string(), "thicc".to_string());
        self.type_mappings.insert("float".to_string(), "drip".to_string());
        self.type_mappings.insert("double".to_string(), "meal".to_string());
        self.type_mappings.insert("char*".to_string(), "tea".to_string());
        self.type_mappings.insert("bool".to_string(), "lit".to_string());
        self.type_mappings.insert("_Bool".to_string(), "lit".to_string());
        
        // Standard integer types
        self.type_mappings.insert("int8_t".to_string(), "smol".to_string());
        self.type_mappings.insert("int16_t".to_string(), "smol".to_string());
        self.type_mappings.insert("int32_t".to_string(), "normie".to_string());
        self.type_mappings.insert("int64_t".to_string(), "thicc".to_string());
        self.type_mappings.insert("uint8_t".to_string(), "byte".to_string());
        self.type_mappings.insert("uint16_t".to_string(), "mid".to_string());
        self.type_mappings.insert("uint32_t".to_string(), "normie".to_string());
        self.type_mappings.insert("uint64_t".to_string(), "thicc".to_string());
        self.type_mappings.insert("size_t".to_string(), "thicc".to_string());
        self.type_mappings.insert("ssize_t".to_string(), "thicc".to_string());
        
        // Pointer types
        self.type_mappings.insert("void*".to_string(), "*cringe".to_string());
        self.type_mappings.insert("int*".to_string(), "*normie".to_string());
        self.type_mappings.insert("char**".to_string(), "*tea".to_string());
    }
    
    /// Initialize error handling templates
    fn initialize_error_handling_templates(&mut self) {
        // Error handling for null pointers
        self.error_handling_templates.insert(
            "null_check".to_string(),
            "lowkey {param_name} == cringe {{ yikes \"Null pointer passed to {function_name}\" }}".to_string()
        );
        
        // Error handling for invalid ranges
        self.error_handling_templates.insert(
            "range_check".to_string(),
            "lowkey {param_name} < 0 || {param_name} > {max_value} {{ yikes \"Invalid range for {param_name}\" }}".to_string()
        );
        
        // Error handling for buffer overflow
        self.error_handling_templates.insert(
            "buffer_check".to_string(),
            "lowkey {buffer_name}_len < {required_size} {{ yikes \"Buffer too small for {function_name}\" }}".to_string()
        );
    }
    
    /// Generate bindings from header information
    pub fn generate_bindings(&self, header_info: &HeaderInfo) -> Result<GeneratedBindings, CursedError> {
        let mut bindings = GeneratedBindings {
            functions: HashMap::new(),
            types: HashMap::new(),
            constants: HashMap::new(),
            cursed_code: String::new(),
            rust_code: String::new(),
        };
        
        // Generate function bindings
        for function in &header_info.functions {
            self.generate_function_binding(function, &mut bindings)?;
        }
        
        // Generate struct bindings
        for struct_info in &header_info.structs {
            self.generate_struct_binding(struct_info, &mut bindings)?;
        }
        
        // Generate enum bindings
        for enum_info in &header_info.enums {
            self.generate_enum_binding(enum_info, &mut bindings)?;
        }
        
        // Generate constant bindings
        for constant in &header_info.constants {
            self.generate_constant_binding(constant, &mut bindings)?;
        }
        
        // Generate typedef bindings
        for typedef_info in &header_info.typedefs {
            self.generate_typedef_binding(typedef_info, &mut bindings)?;
        }
        
        Ok(bindings)
    }
    
    /// Generate binding for a function
    fn generate_function_binding(
        &self,
        function: &FunctionInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        if self.options.generate_cursed_wrapper {
            self.generate_cursed_function_wrapper(function, bindings)?;
        }
        
        if self.options.generate_rust_wrapper {
            self.generate_rust_function_wrapper(function, bindings)?;
        }
        
        Ok(())
    }
    
    /// Generate CURSED wrapper function
    fn generate_cursed_function_wrapper(
        &self,
        function: &FunctionInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut wrapper_code = String::new();
        
        // Generate documentation if enabled
        if self.options.generate_documentation {
            if let Some(doc) = &function.documentation {
                writeln!(wrapper_code, "/// {}", doc)?;
            }
            writeln!(wrapper_code, "/// FFI wrapper for C function: {}", function.name)?;
        }
        
        // Generate function signature
        let cursed_return_type = self.map_c_type_to_cursed(&function.return_type);
        write!(wrapper_code, "slay {}(", function.name)?;
        
        // Generate parameters
        for (i, param) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(wrapper_code, ", ")?;
            }
            
            let cursed_type = self.map_c_type_to_cursed(&param.type_name);
            write!(wrapper_code, "{} {}", param.name, cursed_type)?;
        }
        
        if function.is_variadic {
            if !function.parameters.is_empty() {
                write!(wrapper_code, ", ")?;
            }
            write!(wrapper_code, "...args")?;
        }
        
        writeln!(wrapper_code, ") {} {{", cursed_return_type)?;
        
        // Generate safety checks if enabled
        if self.options.use_safe_wrappers {
            self.generate_safety_checks(function, &mut wrapper_code)?;
        }
        
        // Generate the actual FFI call
        write!(wrapper_code, "    ")?;
        if function.return_type != "void" {
            write!(wrapper_code, "sus result {} = ", cursed_return_type)?;
        }
        
        write!(wrapper_code, "ffi_call(\"{}\", ", function.name)?;
        
        // Generate argument list
        write!(wrapper_code, "[")?;
        for (i, param) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(wrapper_code, ", ")?;
            }
            write!(wrapper_code, "{}", param.name)?;
        }
        write!(wrapper_code, "]")?;
        
        if function.is_variadic {
            write!(wrapper_code, ", args")?;
        }
        
        writeln!(wrapper_code, ")")?;
        
        // Generate error handling if enabled
        if self.options.include_error_handling {
            self.generate_error_handling(function, &mut wrapper_code)?;
        }
        
        // Generate return statement
        if function.return_type != "void" {
            writeln!(wrapper_code, "    damn result")?;
        }
        
        writeln!(wrapper_code, "}}")?;
        writeln!(wrapper_code)?;
        
        bindings.cursed_code.push_str(&wrapper_code);
        
        Ok(())
    }
    
    /// Generate Rust wrapper function
    fn generate_rust_function_wrapper(
        &self,
        function: &FunctionInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut wrapper_code = String::new();
        
        // Generate documentation
        if self.options.generate_documentation {
            if let Some(doc) = &function.documentation {
                writeln!(wrapper_code, "/// {}", doc)?;
            }
            writeln!(wrapper_code, "/// FFI wrapper for C function: {}", function.name)?;
        }
        
        // Generate function signature
        let rust_return_type = self.map_c_type_to_rust(&function.return_type);
        write!(wrapper_code, "pub fn {}(", function.name)?;
        
        // Generate parameters
        for (i, param) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(wrapper_code, ", ")?;
            }
            
            let rust_type = self.map_c_type_to_rust(&param.type_name);
            write!(wrapper_code, "{}: {}", param.name, rust_type)?;
        }
        
        writeln!(wrapper_code, ") -> Result<{}, CursedError> {{", rust_return_type)?;
        
        // Generate safety checks
        if self.options.use_safe_wrappers {
            self.generate_rust_safety_checks(function, &mut wrapper_code)?;
        }
        
        // Generate the actual FFI call
        writeln!(wrapper_code, "    unsafe {{")?;
        write!(wrapper_code, "        ")?;
        
        if function.return_type != "void" {
            write!(wrapper_code, "let result = ")?;
        }
        
        write!(wrapper_code, "{}(", function.name)?;
        
        // Generate argument list
        for (i, param) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(wrapper_code, ", ")?;
            }
            write!(wrapper_code, "{}", param.name)?;
        }
        
        writeln!(wrapper_code, ");")?;
        
        // Generate return statement
        if function.return_type != "void" {
            writeln!(wrapper_code, "        Ok(result)")?;
        } else {
            writeln!(wrapper_code, "        Ok(())")?;
        }
        
        writeln!(wrapper_code, "    }}")?;
        writeln!(wrapper_code, "}}")?;
        writeln!(wrapper_code)?;
        
        bindings.rust_code.push_str(&wrapper_code);
        
        Ok(())
    }
    
    /// Generate struct binding
    fn generate_struct_binding(
        &self,
        struct_info: &StructInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut struct_code = String::new();
        
        // Generate documentation
        if self.options.generate_documentation {
            if let Some(doc) = &struct_info.documentation {
                writeln!(struct_code, "/// {}", doc)?;
            }
            writeln!(struct_code, "/// FFI struct binding for C struct: {}", struct_info.name)?;
        }
        
        // Generate struct definition
        writeln!(struct_code, "struct {} {{", struct_info.name)?;
        
        for field in &struct_info.fields {
            let cursed_type = self.map_c_type_to_cursed(&field.type_name);
            writeln!(struct_code, "    {} {}", field.name, cursed_type)?;
        }
        
        writeln!(struct_code, "}}")?;
        writeln!(struct_code)?;
        
        // Generate constructor function
        writeln!(struct_code, "slay new_{}(", struct_info.name)?;
        for (i, field) in struct_info.fields.iter().enumerate() {
            if i > 0 {
                write!(struct_code, ", ")?;
            }
            let cursed_type = self.map_c_type_to_cursed(&field.type_name);
            write!(struct_code, "{} {}", field.name, cursed_type)?;
        }
        writeln!(struct_code, ") {} {{", struct_info.name)?;
        
        writeln!(struct_code, "    {} {{", struct_info.name)?;
        for field in &struct_info.fields {
            writeln!(struct_code, "        {}: {},", field.name, field.name)?;
        }
        writeln!(struct_code, "    }}")?;
        writeln!(struct_code, "}}")?;
        writeln!(struct_code)?;
        
        bindings.cursed_code.push_str(&struct_code);
        
        Ok(())
    }
    
    /// Generate enum binding
    fn generate_enum_binding(
        &self,
        enum_info: &EnumInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut enum_code = String::new();
        
        // Generate documentation
        if self.options.generate_documentation {
            if let Some(doc) = &enum_info.documentation {
                writeln!(enum_code, "/// {}", doc)?;
            }
            writeln!(enum_code, "/// FFI enum binding for C enum: {}", enum_info.name)?;
        }
        
        // Generate enum definition
        writeln!(enum_code, "enum {} {{", enum_info.name)?;
        
        for value in &enum_info.values {
            if let Some(val) = value.value {
                writeln!(enum_code, "    {} = {},", value.name, val)?;
            } else {
                writeln!(enum_code, "    {},", value.name)?;
            }
        }
        
        writeln!(enum_code, "}}")?;
        writeln!(enum_code)?;
        
        bindings.cursed_code.push_str(&enum_code);
        
        Ok(())
    }
    
    /// Generate constant binding
    fn generate_constant_binding(
        &self,
        constant: &ConstantInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut constant_code = String::new();
        
        // Generate constant definition
        writeln!(constant_code, "facts {} = {}", constant.name, constant.value)?;
        
        bindings.cursed_code.push_str(&constant_code);
        bindings.constants.insert(constant.name.clone(), constant.value.clone());
        
        Ok(())
    }
    
    /// Generate typedef binding
    fn generate_typedef_binding(
        &self,
        typedef_info: &TypedefInfo,
        bindings: &mut GeneratedBindings,
    ) -> Result<(), CursedError> {
        let mut typedef_code = String::new();
        
        if typedef_info.is_function_pointer {
            // Generate function pointer type
            if let Some(signature) = &typedef_info.function_signature {
                writeln!(typedef_code, "be_like {} = slay(", typedef_info.name)?;
                
                for (i, param) in signature.parameters.iter().enumerate() {
                    if i > 0 {
                        write!(typedef_code, ", ")?;
                    }
                    let cursed_type = self.map_ffi_type_to_cursed(&param.param_type);
                    write!(typedef_code, "{} {}", param.name, cursed_type)?;
                }
                
                let return_type = self.map_ffi_type_to_cursed(&signature.return_type);
                writeln!(typedef_code, ") -> {}", return_type)?;
            }
        } else {
            // Generate type alias
            let cursed_type = self.map_c_type_to_cursed(&typedef_info.target_type);
            writeln!(typedef_code, "be_like {} = {}", typedef_info.name, cursed_type)?;
        }
        
        bindings.cursed_code.push_str(&typedef_code);
        
        Ok(())
    }
    
    /// Generate safety checks for CURSED wrapper
    fn generate_safety_checks(
        &self,
        function: &FunctionInfo,
        wrapper_code: &mut String,
    ) -> Result<(), CursedError> {
        if !self.options.handle_null_pointers {
            return Ok(());
        }
        
        for param in &function.parameters {
            if param.is_pointer {
                let null_check = self.error_handling_templates.get("null_check")
                    .unwrap()
                    .replace("{param_name}", &param.name)
                    .replace("{function_name}", &function.name);
                
                writeln!(wrapper_code, "    {}", null_check)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate Rust safety checks
    fn generate_rust_safety_checks(
        &self,
        function: &FunctionInfo,
        wrapper_code: &mut String,
    ) -> Result<(), CursedError> {
        if !self.options.handle_null_pointers {
            return Ok(());
        }
        
        for param in &function.parameters {
            if param.is_pointer {
                writeln!(wrapper_code, "    if {}.is_null() {{", param.name)?;
                writeln!(wrapper_code, "        return Err(CursedError::General(\"Null pointer passed to {}\".to_string()));", function.name)?;
                writeln!(wrapper_code, "    }}")?;
            }
        }
        
        Ok(())
    }
    
    /// Generate error handling code
    fn generate_error_handling(
        &self,
        function: &FunctionInfo,
        wrapper_code: &mut String,
    ) -> Result<(), CursedError> {
        if !self.options.include_error_handling {
            return Ok(());
        }
        
        // Generate error checking for common error patterns
        writeln!(wrapper_code, "    // Check for common error conditions")?;
        writeln!(wrapper_code, "    lowkey result < 0 {{")?;
        writeln!(wrapper_code, "        yikes \"Function {} failed with error code: {{result}}\"")?;
        writeln!(wrapper_code, "    }}")?;
        
        Ok(())
    }
    
    /// Map C type to CURSED type
    fn map_c_type_to_cursed(&self, c_type: &str) -> String {
        // Handle pointer types
        if c_type.contains('*') {
            let base_type = c_type.replace('*', "").trim();
            let cursed_base = self.type_mappings.get(base_type)
                .cloned()
                .unwrap_or_else(|| format!("*{}", base_type));
            return format!("*{}", cursed_base);
        }
        
        // Handle array types
        if c_type.contains('[') {
            let base_type = c_type.split('[').next().unwrap().trim();
            let cursed_base = self.type_mappings.get(base_type)
                .cloned()
                .unwrap_or_else(|| base_type.to_string());
            return format!("[{}]", cursed_base);
        }
        
        // Handle const types
        let clean_type = c_type.replace("const ", "").trim().to_string();
        
        self.type_mappings.get(&clean_type)
            .cloned()
            .unwrap_or_else(|| clean_type)
    }
    
    /// Map C type to Rust type
    fn map_c_type_to_rust(&self, c_type: &str) -> String {
        match c_type {
            "void" => "()".to_string(),
            "int" => "i32".to_string(),
            "char" => "i8".to_string(),
            "unsigned char" => "u8".to_string(),
            "short" => "i16".to_string(),
            "unsigned short" => "u16".to_string(),
            "long" => "i64".to_string(),
            "unsigned long" => "u64".to_string(),
            "long long" => "i64".to_string(),
            "unsigned long long" => "u64".to_string(),
            "float" => "f32".to_string(),
            "double" => "f64".to_string(),
            "char*" => "*const i8".to_string(),
            "void*" => "*mut std::ffi::c_void".to_string(),
            "bool" | "_Bool" => "bool".to_string(),
            _ => {
                if c_type.contains('*') {
                    let base_type = c_type.replace('*', "").trim();
                    let rust_base = self.map_c_type_to_rust(base_type);
                    format!("*mut {}", rust_base)
                } else {
                    c_type.to_string()
                }
            }
        }
    }
    
    /// Map FFI type to CURSED type
    fn map_ffi_type_to_cursed(&self, ffi_type: &FfiType) -> String {
        match ffi_type {
            FfiType::Void => "cringe".to_string(),
            FfiType::SignedInteger(8) => "smol".to_string(),
            FfiType::SignedInteger(16) => "smol".to_string(),
            FfiType::SignedInteger(32) => "normie".to_string(),
            FfiType::SignedInteger(64) => "thicc".to_string(),
            FfiType::UnsignedInteger(8) => "byte".to_string(),
            FfiType::UnsignedInteger(16) => "mid".to_string(),
            FfiType::UnsignedInteger(32) => "normie".to_string(),
            FfiType::UnsignedInteger(64) => "thicc".to_string(),
            FfiType::Float(32) => "drip".to_string(),
            FfiType::Float(64) => "meal".to_string(),
            FfiType::Boolean => "lit".to_string(),
            FfiType::Character => "sip".to_string(),
            FfiType::String => "tea".to_string(),
            FfiType::CString => "tea".to_string(),
            FfiType::Pointer(inner) => format!("*{}", self.map_ffi_type_to_cursed(inner)),
            FfiType::Array(inner, size) => {
                if let Some(s) = size {
                    format!("[{}; {}]", self.map_ffi_type_to_cursed(inner), s)
                } else {
                    format!("[{}]", self.map_ffi_type_to_cursed(inner))
                }
            }
            FfiType::Struct(_) => "struct".to_string(),
            FfiType::Function(_) => "slay".to_string(),
            _ => "cringe".to_string(),
        }
    }
    
    /// Set code generation options
    pub fn set_options(&mut self, options: CodeGenOptions) {
        self.options = options;
    }
    
    /// Add custom type mapping
    pub fn add_type_mapping(&mut self, c_type: String, cursed_type: String) {
        self.type_mappings.insert(c_type, cursed_type);
    }
    
    /// Add custom error handling template
    pub fn add_error_handling_template(&mut self, name: String, template: String) {
        self.error_handling_templates.insert(name, template);
    }
}

impl Default for BindingGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// FFI function wrapper trait
pub trait FfiFunctionWrapper: Send + Sync {
    fn call(&self, args: &[super::FfiValue]) -> Result<super::FfiValue, CursedError>;
    fn signature(&self) -> &FunctionSignature;
    fn name(&self) -> &str;
}

/// Simple FFI function wrapper implementation
#[derive(Debug)]
pub struct SimpleFfiFunctionWrapper {
    name: String,
    signature: FunctionSignature,
    function_ptr: *mut std::ffi::c_void,
}

impl SimpleFfiFunctionWrapper {
    pub fn new(name: String, signature: FunctionSignature, function_ptr: *mut std::ffi::c_void) -> Self {
        Self {
            name,
            signature,
            function_ptr,
        }
    }
}

impl FfiFunctionWrapper for SimpleFfiFunctionWrapper {
    fn call(&self, args: &[super::FfiValue]) -> Result<super::FfiValue, CursedError> {
        // This would contain the actual FFI call logic
        // For now, return a placeholder
        Ok(super::FfiValue::Void)
    }
    
    fn signature(&self) -> &FunctionSignature {
        &self.signature
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

unsafe impl Send for SimpleFfiFunctionWrapper {}
unsafe impl Sync for SimpleFfiFunctionWrapper {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::header_parser::*;
    
    #[test]
    fn test_generate_function_binding() {
        let generator = BindingGenerator::new();
        
        let function = FunctionInfo {
            name: "add".to_string(),
            return_type: "int".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "a".to_string(),
                    type_name: "int".to_string(),
                    is_const: false,
                    is_pointer: false,
                    pointer_depth: 0,
                    is_array: false,
                    array_size: None,
                },
                ParameterInfo {
                    name: "b".to_string(),
                    type_name: "int".to_string(),
                    is_const: false,
                    is_pointer: false,
                    pointer_depth: 0,
                    is_array: false,
                    array_size: None,
                },
            ],
            is_variadic: false,
            is_inline: false,
            is_static: false,
            documentation: Some("Adds two integers".to_string()),
        };
        
        let mut bindings = GeneratedBindings {
            functions: HashMap::new(),
            types: HashMap::new(),
            constants: HashMap::new(),
            cursed_code: String::new(),
            rust_code: String::new(),
        };
        
        let result = generator.generate_function_binding(&function, &mut bindings);
        assert!(result.is_ok());
        assert!(bindings.cursed_code.contains("slay add("));
        assert!(bindings.cursed_code.contains("a normie, b normie"));
    }
    
    #[test]
    fn test_type_mapping() {
        let generator = BindingGenerator::new();
        
        assert_eq!(generator.map_c_type_to_cursed("int"), "normie");
        assert_eq!(generator.map_c_type_to_cursed("char*"), "*tea");
        assert_eq!(generator.map_c_type_to_cursed("void"), "cringe");
        assert_eq!(generator.map_c_type_to_cursed("double"), "meal");
    }
    
    #[test]
    fn test_generate_struct_binding() {
        let generator = BindingGenerator::new();
        
        let struct_info = StructInfo {
            name: "Point".to_string(),
            fields: vec![
                FieldInfo {
                    name: "x".to_string(),
                    type_name: "int".to_string(),
                    is_const: false,
                    is_pointer: false,
                    pointer_depth: 0,
                    is_array: false,
                    array_size: None,
                    offset: None,
                },
                FieldInfo {
                    name: "y".to_string(),
                    type_name: "int".to_string(),
                    is_const: false,
                    is_pointer: false,
                    pointer_depth: 0,
                    is_array: false,
                    array_size: None,
                    offset: None,
                },
            ],
            is_packed: false,
            alignment: None,
            documentation: Some("2D point structure".to_string()),
        };
        
        let mut bindings = GeneratedBindings {
            functions: HashMap::new(),
            types: HashMap::new(),
            constants: HashMap::new(),
            cursed_code: String::new(),
            rust_code: String::new(),
        };
        
        let result = generator.generate_struct_binding(&struct_info, &mut bindings);
        assert!(result.is_ok());
        assert!(bindings.cursed_code.contains("struct Point"));
        assert!(bindings.cursed_code.contains("x normie"));
        assert!(bindings.cursed_code.contains("y normie"));
    }
}
