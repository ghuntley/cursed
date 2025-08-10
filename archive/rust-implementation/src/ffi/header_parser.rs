//! Automatic C header parser for FFI binding generation
//!
//! This module provides capabilities to parse C header files and extract
//! function signatures, struct definitions, and type information for
//! automatic binding generation.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use crate::error::CursedError;
use super::{FfiType, FunctionSignature, Parameter};

/// Information extracted from a C header file
#[derive(Debug, Clone)]
pub struct HeaderInfo {
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
    pub typedefs: Vec<TypedefInfo>,
    pub constants: Vec<ConstantInfo>,
    pub includes: Vec<String>,
}

/// Function information from header
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<ParameterInfo>,
    pub is_variadic: bool,
    pub is_inline: bool,
    pub is_static: bool,
    pub documentation: Option<String>,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
    pub is_const: bool,
    pub is_pointer: bool,
    pub pointer_depth: usize,
    pub is_array: bool,
    pub array_size: Option<usize>,
}

/// Struct information
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub is_packed: bool,
    pub alignment: Option<usize>,
    pub documentation: Option<String>,
}

/// Field information
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_const: bool,
    pub is_pointer: bool,
    pub pointer_depth: usize,
    pub is_array: bool,
    pub array_size: Option<usize>,
    pub offset: Option<usize>,
}

/// Enum information
#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub underlying_type: Option<String>,
    pub documentation: Option<String>,
}

/// Enum value
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: Option<i64>,
}

/// Typedef information
#[derive(Debug, Clone)]
pub struct TypedefInfo {
    pub name: String,
    pub target_type: String,
    pub is_function_pointer: bool,
    pub function_signature: Option<FunctionSignature>,
}

/// Constant information
#[derive(Debug, Clone)]
pub struct ConstantInfo {
    pub name: String,
    pub value: String,
    pub type_name: Option<String>,
}

/// C header parser
pub struct HeaderParser {
    type_mappings: HashMap<String, FfiType>,
    preprocessor_definitions: HashMap<String, String>,
}

impl HeaderParser {
    /// Create a new header parser
    pub fn new() -> Self {
        let mut parser = Self {
            type_mappings: HashMap::new(),
            preprocessor_definitions: HashMap::new(),
        };
        
        parser.initialize_builtin_types();
        parser
    }
    
    /// Initialize built-in C type mappings
    fn initialize_builtin_types(&mut self) {
        self.type_mappings.insert("void".to_string(), FfiType::Void);
        self.type_mappings.insert("char".to_string(), FfiType::SignedInteger(8));
        self.type_mappings.insert("signed char".to_string(), FfiType::SignedInteger(8));
        self.type_mappings.insert("unsigned char".to_string(), FfiType::UnsignedInteger(8));
        self.type_mappings.insert("short".to_string(), FfiType::SignedInteger(16));
        self.type_mappings.insert("unsigned short".to_string(), FfiType::UnsignedInteger(16));
        self.type_mappings.insert("int".to_string(), FfiType::SignedInteger(32));
        self.type_mappings.insert("unsigned int".to_string(), FfiType::UnsignedInteger(32));
        self.type_mappings.insert("long".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("unsigned long".to_string(), FfiType::UnsignedInteger(64));
        self.type_mappings.insert("long long".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("unsigned long long".to_string(), FfiType::UnsignedInteger(64));
        self.type_mappings.insert("float".to_string(), FfiType::Float(32));
        self.type_mappings.insert("double".to_string(), FfiType::Float(64));
        self.type_mappings.insert("long double".to_string(), FfiType::Float(128));
        self.type_mappings.insert("size_t".to_string(), FfiType::UnsignedInteger(64));
        self.type_mappings.insert("ssize_t".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("ptrdiff_t".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("intptr_t".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("uintptr_t".to_string(), FfiType::UnsignedInteger(64));
        
        // Standard integer types
        self.type_mappings.insert("int8_t".to_string(), FfiType::SignedInteger(8));
        self.type_mappings.insert("int16_t".to_string(), FfiType::SignedInteger(16));
        self.type_mappings.insert("int32_t".to_string(), FfiType::SignedInteger(32));
        self.type_mappings.insert("int64_t".to_string(), FfiType::SignedInteger(64));
        self.type_mappings.insert("uint8_t".to_string(), FfiType::UnsignedInteger(8));
        self.type_mappings.insert("uint16_t".to_string(), FfiType::UnsignedInteger(16));
        self.type_mappings.insert("uint32_t".to_string(), FfiType::UnsignedInteger(32));
        self.type_mappings.insert("uint64_t".to_string(), FfiType::UnsignedInteger(64));
        
        // Boolean type
        self.type_mappings.insert("bool".to_string(), FfiType::Boolean);
        self.type_mappings.insert("_Bool".to_string(), FfiType::Boolean);
    }
    
    /// Parse a C header file
    pub fn parse_file(&self, header_path: &str) -> Result<HeaderInfo, CursedError> {
        let path = Path::new(header_path);
        if !path.exists() {
            return Err(CursedError::General(format!("Header file not found: {}", header_path)));
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::General(format!("Failed to read header file: {}", e)))?;
        
        self.parse_content(&content)
    }
    
    /// Parse header content
    pub fn parse_content(&self, content: &str) -> Result<HeaderInfo, CursedError> {
        let mut header_info = HeaderInfo {
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            typedefs: Vec::new(),
            constants: Vec::new(),
            includes: Vec::new(),
        };
        
        // Preprocess the content
        let preprocessed = self.preprocess_content(content)?;
        
        // Parse different constructs
        header_info.functions = self.parse_functions(&preprocessed)?;
        header_info.structs = self.parse_structs(&preprocessed)?;
        header_info.enums = self.parse_enums(&preprocessed)?;
        header_info.typedefs = self.parse_typedefs(&preprocessed)?;
        header_info.constants = self.parse_constants(&preprocessed)?;
        header_info.includes = self.parse_includes(&preprocessed)?;
        
        Ok(header_info)
    }
    
    /// Preprocess content (handle comments, macros, etc.)
    fn preprocess_content(&self, content: &str) -> Result<String, CursedError> {
        let mut result = content.to_string();
        
        // Remove C-style comments
        let comment_regex = Regex::new(r"/\*.*?\*/").unwrap();
        result = comment_regex.replace_all(&result, "").to_string();
        
        // Remove C++ style comments
        let cpp_comment_regex = Regex::new(r"//.*$").unwrap();
        result = cpp_comment_regex.replace_all(&result, "").to_string();
        
        // Handle multi-line statements
        result = result.replace("\\\n", " ");
        
        // Normalize whitespace
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        result = whitespace_regex.replace_all(&result, " ").to_string();
        
        Ok(result)
    }
    
    /// Parse function declarations
    fn parse_functions(&self, content: &str) -> Result<Vec<FunctionInfo>, CursedError> {
        let mut functions = Vec::new();
        
        // Regex to match function declarations
        let func_regex = Regex::new(
            r"(?m)^(?:(static|inline|extern)\s+)?([\w\s\*]+)\s+(\w+)\s*\(([^)]*)\)\s*;"
        ).unwrap();
        
        for cap in func_regex.captures_iter(content) {
            let modifiers = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let return_type = cap.get(2).unwrap().as_str().trim();
            let name = cap.get(3).unwrap().as_str();
            let params_str = cap.get(4).unwrap().as_str();
            
            let parameters = self.parse_parameters(params_str)?;
            let is_variadic = params_str.contains("...");
            
            functions.push(FunctionInfo {
                name: name.to_string(),
                return_type: return_type.to_string(),
                parameters,
                is_variadic,
                is_inline: modifiers.contains("inline"),
                is_static: modifiers.contains("static"),
                documentation: None,
            });
        }
        
        Ok(functions)
    }
    
    /// Parse function parameters
    fn parse_parameters(&self, params_str: &str) -> Result<Vec<ParameterInfo>, CursedError> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() || params_str.trim() == "void" {
            return Ok(parameters);
        }
        
        // Split parameters by comma, but handle nested parentheses
        let param_parts = self.split_parameters(params_str);
        
        for param in param_parts {
            let param = param.trim();
            if param == "..." {
                continue; // Skip variadic marker
            }
            
            let param_info = self.parse_parameter(param)?;
            parameters.push(param_info);
        }
        
        Ok(parameters)
    }
    
    /// Split parameters handling nested parentheses
    fn split_parameters(&self, params_str: &str) -> Vec<String> {
        let mut parameters = Vec::new();
        let mut current_param = String::new();
        let mut paren_depth = 0;
        
        for ch in params_str.chars() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current_param.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current_param.push(ch);
                }
                ',' if paren_depth == 0 => {
                    if !current_param.trim().is_empty() {
                        parameters.push(current_param.trim().to_string());
                        current_param.clear();
                    }
                }
                _ => {
                    current_param.push(ch);
                }
            }
        }
        
        if !current_param.trim().is_empty() {
            parameters.push(current_param.trim().to_string());
        }
        
        parameters
    }
    
    /// Parse a single parameter
    fn parse_parameter(&self, param_str: &str) -> Result<ParameterInfo, CursedError> {
        let parts: Vec<&str> = param_str.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CursedError::General("Empty parameter".to_string()));
        }
        
        let mut is_const = false;
        let mut type_parts = Vec::new();
        let mut name = String::new();
        
        // Parse type and name
        for (i, part) in parts.iter().enumerate() {
            if *part == "const" {
                is_const = true;
            } else if i == parts.len() - 1 {
                // Last part is the parameter name
                name = part.to_string();
            } else {
                type_parts.push(*part);
            }
        }
        
        let type_name = type_parts.join(" ");
        let pointer_depth = type_name.chars().filter(|&c| c == '*').count();
        let is_pointer = pointer_depth > 0;
        
        // Handle array notation
        let (is_array, array_size) = if name.contains('[') {
            let array_regex = Regex::new(r"(\w+)\[(\d*)\]").unwrap();
            if let Some(cap) = array_regex.captures(&name) {
                name = cap.get(1).unwrap().as_str().to_string();
                let size_str = cap.get(2).unwrap().as_str();
                let size = if size_str.is_empty() {
                    None
                } else {
                    Some(size_str.parse().unwrap_or(0))
                };
                (true, size)
            } else {
                (false, None)
            }
        } else {
            (false, None)
        };
        
        Ok(ParameterInfo {
            name,
            type_name,
            is_const,
            is_pointer,
            pointer_depth,
            is_array,
            array_size,
        })
    }
    
    /// Parse struct definitions
    fn parse_structs(&self, content: &str) -> Result<Vec<StructInfo>, CursedError> {
        let mut structs = Vec::new();
        
        // Regex to match struct definitions
        let struct_regex = Regex::new(
            r"(?s)struct\s+(\w+)\s*\{([^}]*)\}"
        ).unwrap();
        
        for cap in struct_regex.captures_iter(content) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let fields_str = cap.get(2).unwrap().as_str();
            
            let fields = self.parse_struct_fields(fields_str)?;
            
            structs.push(StructInfo {
                name,
                fields,
                is_packed: false, // Could be enhanced to detect __attribute__((packed))
                alignment: None,
                documentation: None,
            });
        }
        
        Ok(structs)
    }
    
    /// Parse struct fields
    fn parse_struct_fields(&self, fields_str: &str) -> Result<Vec<FieldInfo>, CursedError> {
        let mut fields = Vec::new();
        
        // Split by semicolon to get individual field declarations
        let field_declarations: Vec<&str> = fields_str.split(';').collect();
        
        for field_decl in field_declarations {
            let field_decl = field_decl.trim();
            if field_decl.is_empty() {
                continue;
            }
            
            // Parse field similar to parameter parsing
            let field_info = self.parse_field(field_decl)?;
            fields.push(field_info);
        }
        
        Ok(fields)
    }
    
    /// Parse a struct field
    fn parse_field(&self, field_str: &str) -> Result<FieldInfo, CursedError> {
        let parts: Vec<&str> = field_str.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(CursedError::General(format!("Invalid field declaration: {}", field_str)));
        }
        
        let mut is_const = false;
        let mut type_parts = Vec::new();
        let mut name = String::new();
        
        // Parse type and name
        for (i, part) in parts.iter().enumerate() {
            if *part == "const" {
                is_const = true;
            } else if i == parts.len() - 1 {
                // Last part is the field name
                name = part.to_string();
            } else {
                type_parts.push(*part);
            }
        }
        
        let type_name = type_parts.join(" ");
        let pointer_depth = type_name.chars().filter(|&c| c == '*').count();
        let is_pointer = pointer_depth > 0;
        
        // Handle array notation
        let (is_array, array_size) = if name.contains('[') {
            let array_regex = Regex::new(r"(\w+)\[(\d*)\]").unwrap();
            if let Some(cap) = array_regex.captures(&name) {
                name = cap.get(1).unwrap().as_str().to_string();
                let size_str = cap.get(2).unwrap().as_str();
                let size = if size_str.is_empty() {
                    None
                } else {
                    Some(size_str.parse().unwrap_or(0))
                };
                (true, size)
            } else {
                (false, None)
            }
        } else {
            (false, None)
        };
        
        Ok(FieldInfo {
            name,
            type_name,
            is_const,
            is_pointer,
            pointer_depth,
            is_array,
            array_size,
            offset: None, // Could be calculated based on field order and alignment
        })
    }
    
    /// Parse enum definitions
    fn parse_enums(&self, content: &str) -> Result<Vec<EnumInfo>, CursedError> {
        let mut enums = Vec::new();
        
        // Regex to match enum definitions
        let enum_regex = Regex::new(
            r"(?s)enum\s+(\w+)\s*\{([^}]*)\}"
        ).unwrap();
        
        for cap in enum_regex.captures_iter(content) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let values_str = cap.get(2).unwrap().as_str();
            
            let values = self.parse_enum_values(values_str)?;
            
            enums.push(EnumInfo {
                name,
                values,
                underlying_type: None,
                documentation: None,
            });
        }
        
        Ok(enums)
    }
    
    /// Parse enum values
    fn parse_enum_values(&self, values_str: &str) -> Result<Vec<EnumValue>, CursedError> {
        let mut values = Vec::new();
        let mut current_value = 0i64;
        
        // Split by comma to get individual enum values
        let value_declarations: Vec<&str> = values_str.split(',').collect();
        
        for value_decl in value_declarations {
            let value_decl = value_decl.trim();
            if value_decl.is_empty() {
                continue;
            }
            
            // Check if value has explicit assignment
            if value_decl.contains('=') {
                let parts: Vec<&str> = value_decl.split('=').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let value_str = parts[1].trim();
                    
                    // Try to parse the value
                    if let Ok(val) = value_str.parse::<i64>() {
                        current_value = val;
                        values.push(EnumValue {
                            name,
                            value: Some(current_value),
                        });
                        current_value += 1;
                    } else {
                        // Handle complex expressions (simplified)
                        values.push(EnumValue {
                            name,
                            value: None,
                        });
                    }
                } else {
                    return Err(CursedError::General(format!("Invalid enum value: {}", value_decl)));
                }
            } else {
                // Use current auto-increment value
                values.push(EnumValue {
                    name: value_decl.to_string(),
                    value: Some(current_value),
                });
                current_value += 1;
            }
        }
        
        Ok(values)
    }
    
    /// Parse typedef declarations
    fn parse_typedefs(&self, content: &str) -> Result<Vec<TypedefInfo>, CursedError> {
        let mut typedefs = Vec::new();
        
        // Regex to match typedef declarations
        let typedef_regex = Regex::new(
            r"typedef\s+([^;]+);"
        ).unwrap();
        
        for cap in typedef_regex.captures_iter(content) {
            let typedef_content = cap.get(1).unwrap().as_str().trim();
            
            // Check if it's a function pointer typedef
            if typedef_content.contains('(') && typedef_content.contains(')') {
                let typedef_info = self.parse_function_pointer_typedef(typedef_content)?;
                typedefs.push(typedef_info);
            } else {
                // Simple type alias
                let parts: Vec<&str> = typedef_content.split_whitespace().collect();
                if parts.len() >= 2 {
                    let target_type = parts[..parts.len()-1].join(" ");
                    let name = parts.last().unwrap().to_string();
                    
                    typedefs.push(TypedefInfo {
                        name,
                        target_type,
                        is_function_pointer: false,
                        function_signature: None,
                    });
                }
            }
        }
        
        Ok(typedefs)
    }
    
    /// Parse function pointer typedef
    fn parse_function_pointer_typedef(&self, typedef_content: &str) -> Result<TypedefInfo, CursedError> {
        // Example: int (*callback)(int, char*)
        let func_ptr_regex = Regex::new(
            r"([^(]+)\(\s*\*\s*(\w+)\s*\)\s*\(([^)]*)\)"
        ).unwrap();
        
        if let Some(cap) = func_ptr_regex.captures(typedef_content) {
            let return_type = cap.get(1).unwrap().as_str().trim();
            let name = cap.get(2).unwrap().as_str();
            let params_str = cap.get(3).unwrap().as_str();
            
            let parameters = self.parse_parameters(params_str)?;
            
            let signature = FunctionSignature {
                name: name.to_string(),
                return_type: self.map_c_type_to_ffi(return_type),
                parameters: parameters.into_iter().map(|p| Parameter {
                    name: p.name,
                    param_type: self.map_c_type_to_ffi(&p.type_name),
                    is_const: p.is_const,
                    is_nullable: p.is_pointer,
                }).collect(),
                is_variadic: params_str.contains("..."),
            };
            
            Ok(TypedefInfo {
                name: name.to_string(),
                target_type: "function_pointer".to_string(),
                is_function_pointer: true,
                function_signature: Some(signature),
            })
        } else {
            Err(CursedError::General(format!("Invalid function pointer typedef: {}", typedef_content)))
        }
    }
    
    /// Parse constants and defines
    fn parse_constants(&self, content: &str) -> Result<Vec<ConstantInfo>, CursedError> {
        let mut constants = Vec::new();
        
        // Regex to match #define constants
        let define_regex = Regex::new(
            r"#define\s+(\w+)\s+(.+)"
        ).unwrap();
        
        for cap in define_regex.captures_iter(content) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).unwrap().as_str().trim().to_string();
            
            constants.push(ConstantInfo {
                name,
                value,
                type_name: None, // Could be inferred from value
            });
        }
        
        Ok(constants)
    }
    
    /// Parse include statements
    fn parse_includes(&self, content: &str) -> Result<Vec<String>, CursedError> {
        let mut includes = Vec::new();
        
        // Regex to match #include statements
        let include_regex = Regex::new(
            r#"#include\s+[<"]([^>"]+)[>"]"#
        ).unwrap();
        
        for cap in include_regex.captures_iter(content) {
            let include_path = cap.get(1).unwrap().as_str().to_string();
            includes.push(include_path);
        }
        
        Ok(includes)
    }
    
    /// Map C type to FFI type
    fn map_c_type_to_ffi(&self, c_type: &str) -> FfiType {
        let normalized_type = c_type.trim().replace("const ", "").replace("*", "");
        
        self.type_mappings.get(&normalized_type)
            .cloned()
            .unwrap_or_else(|| {
                // Handle pointer types
                if c_type.contains('*') {
                    let base_type = c_type.replace('*', "").trim().to_string();
                    let base_ffi_type = self.map_c_type_to_ffi(&base_type);
                    FfiType::Pointer(Box::new(base_ffi_type))
                } else {
                    // Default to void for unknown types
                    FfiType::Void
                }
            })
    }
    
    /// Add custom type mapping
    pub fn add_type_mapping(&mut self, c_type: String, ffi_type: FfiType) {
        self.type_mappings.insert(c_type, ffi_type);
    }
    
    /// Add preprocessor definition
    pub fn add_preprocessor_definition(&mut self, name: String, value: String) {
        self.preprocessor_definitions.insert(name, value);
    }
}

impl Default for HeaderParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_function() {
        let parser = HeaderParser::new();
        let content = "int add(int a, int b);";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.functions.len(), 1);
        let func = &header_info.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.return_type, "int");
        assert_eq!(func.parameters.len(), 2);
        assert_eq!(func.parameters[0].name, "a");
        assert_eq!(func.parameters[0].type_name, "int");
        assert_eq!(func.parameters[1].name, "b");
        assert_eq!(func.parameters[1].type_name, "int");
    }
    
    #[test]
    fn test_parse_struct() {
        let parser = HeaderParser::new();
        let content = "struct Point { int x; int y; };";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.structs.len(), 1);
        let struct_info = &header_info.structs[0];
        assert_eq!(struct_info.name, "Point");
        assert_eq!(struct_info.fields.len(), 2);
        assert_eq!(struct_info.fields[0].name, "x");
        assert_eq!(struct_info.fields[0].type_name, "int");
        assert_eq!(struct_info.fields[1].name, "y");
        assert_eq!(struct_info.fields[1].type_name, "int");
    }
    
    #[test]
    fn test_parse_enum() {
        let parser = HeaderParser::new();
        let content = "enum Color { RED, GREEN = 5, BLUE };";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.enums.len(), 1);
        let enum_info = &header_info.enums[0];
        assert_eq!(enum_info.name, "Color");
        assert_eq!(enum_info.values.len(), 3);
        assert_eq!(enum_info.values[0].name, "RED");
        assert_eq!(enum_info.values[0].value, Some(0));
        assert_eq!(enum_info.values[1].name, "GREEN");
        assert_eq!(enum_info.values[1].value, Some(5));
        assert_eq!(enum_info.values[2].name, "BLUE");
        assert_eq!(enum_info.values[2].value, Some(6));
    }
    
    #[test]
    fn test_parse_function_pointer_typedef() {
        let parser = HeaderParser::new();
        let content = "typedef int (*callback_t)(int, char*);";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.typedefs.len(), 1);
        let typedef_info = &header_info.typedefs[0];
        assert_eq!(typedef_info.name, "callback_t");
        assert!(typedef_info.is_function_pointer);
        assert!(typedef_info.function_signature.is_some());
    }
    
    #[test]
    fn test_parse_includes() {
        let parser = HeaderParser::new();
        let content = "#include <stdio.h>\n#include \"custom.h\"";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.includes.len(), 2);
        assert_eq!(header_info.includes[0], "stdio.h");
        assert_eq!(header_info.includes[1], "custom.h");
    }
    
    #[test]
    fn test_parse_constants() {
        let parser = HeaderParser::new();
        let content = "#define MAX_SIZE 1024\n#define PI 3.14159";
        
        let header_info = parser.parse_content(content).unwrap();
        
        assert_eq!(header_info.constants.len(), 2);
        assert_eq!(header_info.constants[0].name, "MAX_SIZE");
        assert_eq!(header_info.constants[0].value, "1024");
        assert_eq!(header_info.constants[1].name, "PI");
        assert_eq!(header_info.constants[1].value, "3.14159");
    }
}
