use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use crate::ast::*;
use crate::codegen::CodeGenerator;

/// Enhanced debug information generator for CURSED
#[derive(Debug, Clone)]
pub struct DebugInfoGenerator {
    pub config: DebugConfig,
    pub debug_builder: Option<LLVMDIBuilderRef>,
    pub debug_context: DebugContext,
    pub source_files: HashMap<String, DebugFile>,
    pub line_table: HashMap<usize, DebugLocation>,
}

#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub debug_level: DebugLevel,
    pub include_source: bool,
    pub include_variables: bool,
    pub include_types: bool,
    pub include_inlined_functions: bool,
    pub dwarf_version: u32,
    pub output_path: PathBuf,
    pub source_map: bool,
    pub optimization_level: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebugLevel {
    None,
    LineNumbers,
    Full,
    Optimized,
}

#[derive(Debug, Clone)]
pub struct DebugContext {
    pub compile_unit: Option<LLVMMetadataRef>,
    pub current_scope: Option<LLVMMetadataRef>,
    pub current_function: Option<LLVMMetadataRef>,
    pub type_cache: HashMap<String, LLVMMetadataRef>,
    pub variable_cache: HashMap<String, DebugVariable>,
}

#[derive(Debug, Clone)]
pub struct DebugFile {
    pub path: String,
    pub directory: String,
    pub checksum: String,
    pub source_content: Option<String>,
    pub debug_file: Option<LLVMMetadataRef>,
}

#[derive(Debug, Clone)]
pub struct DebugLocation {
    pub line: u32,
    pub column: u32,
    pub file: String,
    pub scope: Option<LLVMMetadataRef>,
}

#[derive(Debug, Clone)]
pub struct DebugVariable {
    pub name: String,
    pub var_type: String,
    pub scope: Option<LLVMMetadataRef>,
    pub location: DebugLocation,
    pub debug_variable: Option<LLVMMetadataRef>,
}

#[derive(Debug, Clone)]
pub struct DebugType {
    pub name: String,
    pub size: u64,
    pub align: u64,
    pub encoding: DebugEncoding,
    pub debug_type: Option<LLVMMetadataRef>,
}

#[derive(Debug, Clone)]
pub enum DebugEncoding {
    Address,
    Boolean,
    Float,
    Signed,
    Unsigned,
    UTF,
}

impl DebugInfoGenerator {
    /// Create new debug info generator
    pub fn new(config: DebugConfig) -> Self {
        Self {
            config,
            debug_builder: None,
            debug_context: DebugContext::new(),
            source_files: HashMap::new(),
            line_table: HashMap::new(),
        }
    }

    /// Initialize debug information generation
    pub fn initialize(&mut self, module: LLVMModuleRef, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.debug_level == DebugLevel::None {
            return Ok(());
        }

        println!("🔧 Initializing debug information generation...");

        unsafe {
            // Create debug builder
            let debug_builder = LLVMCreateDIBuilder(module);
            self.debug_builder = Some(debug_builder);

            // Create compile unit
            let filename = std::ffi::CString::new(
                PathBuf::from(source_file)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_bytes()
            )?;
            
            let directory = std::ffi::CString::new(
                PathBuf::from(source_file)
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .to_string_lossy()
                    .as_bytes()
            )?;

            let producer = std::ffi::CString::new("CURSED Compiler v1.0")?;
            let flags = std::ffi::CString::new("")?;
            let split_name = std::ffi::CString::new("")?;

            let compile_unit = LLVMDIBuilderCreateCompileUnit(
                debug_builder,
                LLVMDWARFSourceLanguageC, // Use C for now, could create custom language
                LLVMDIBuilderCreateFile(
                    debug_builder,
                    filename.as_ptr(),
                    filename.as_bytes().len(),
                    directory.as_ptr(),
                    directory.as_bytes().len(),
                ),
                producer.as_ptr(),
                producer.as_bytes().len(),
                if self.config.optimization_level > 0 { 1 } else { 0 },
                flags.as_ptr(),
                flags.as_bytes().len(),
                0, // Runtime version
                split_name.as_ptr(),
                split_name.as_bytes().len(),
                llvm_sys::debuginfo::LLVMDWARFEmissionKind::LLVMDWARFEmissionFull,
                0, // DWO ID
                1, // Split debug inlining
                0, // Debug info for profiling
            );

            self.debug_context.compile_unit = Some(compile_unit);
            self.debug_context.current_scope = Some(compile_unit);

            // Register source file
            let debug_file = DebugFile {
                path: source_file.to_string(),
                directory: directory.to_string_lossy().to_string(),
                checksum: self.calculate_file_checksum(source_file)?,
                source_content: if self.config.include_source {
                    Some(fs::read_to_string(source_file)?)
                } else {
                    None
                },
                debug_file: Some(compile_unit),
            };

            self.source_files.insert(source_file.to_string(), debug_file);
        }

        println!("✅ Debug information initialized");
        Ok(())
    }

    /// Generate debug information for function
    pub fn generate_function_debug(&mut self, function: &FunctionDeclaration, llvm_function: LLVMValueRef) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.debug_level == DebugLevel::None {
            return Ok(());
        }

        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;

        unsafe {
            // Create function type
            let return_type = self.create_debug_type(&function.return_type)?;
            let mut param_types = Vec::new();
            
            for param in &function.parameters {
                let param_type = self.create_debug_type(&Some(param.param_type.clone()))?;
                param_types.push(param_type);
            }

            let function_type = LLVMDIBuilderCreateSubroutineType(
                debug_builder,
                LLVMDIBuilderCreateFile(
                    debug_builder,
                    std::ffi::CString::new(function.file_path.as_bytes())?.as_ptr(),
                    function.file_path.len(),
                    std::ffi::CString::new(".")?.as_ptr(),
                    1,
                ),
                param_types.as_ptr(),
                param_types.len() as u32,
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
            );

            // Create function debug info
            let func_name = std::ffi::CString::new(function.name.as_bytes())?;
            let linkage_name = std::ffi::CString::new(function.name.as_bytes())?;
            
            let debug_function = LLVMDIBuilderCreateFunction(
                debug_builder,
                self.debug_context.current_scope.unwrap(),
                func_name.as_ptr(),
                func_name.as_bytes().len(),
                linkage_name.as_ptr(),
                linkage_name.as_bytes().len(),
                self.source_files.values().next().unwrap().debug_file.unwrap(),
                function.line_number as u32,
                function_type,
                1, // is local to unit
                1, // is definition
                function.line_number as u32,
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
                if self.config.optimization_level > 0 { 1 } else { 0 },
            );

            // Set function metadata
            LLVMSetSubprogram(llvm_function, debug_function);
            self.debug_context.current_function = Some(debug_function);
            self.debug_context.current_scope = Some(debug_function);

            // Generate debug info for parameters
            for (i, param) in function.parameters.iter().enumerate() {
                self.generate_parameter_debug(param, i + 1, llvm_function)?;
            }
        }

        Ok(())
    }

    /// Generate debug information for variable
    pub fn generate_variable_debug(&mut self, var_name: &str, var_type: &str, line: u32, alloca: LLVMValueRef) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.debug_level == DebugLevel::None || !self.config.include_variables {
            return Ok(());
        }

        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;

        unsafe {
            let debug_type = self.get_or_create_basic_type(var_type)?;
            let var_name_cstr = std::ffi::CString::new(var_name)?;
            
            let debug_variable = LLVMDIBuilderCreateAutoVariable(
                debug_builder,
                self.debug_context.current_scope.unwrap(),
                var_name_cstr.as_ptr(),
                var_name_cstr.as_bytes().len(),
                self.source_files.values().next().unwrap().debug_file.unwrap(),
                line,
                debug_type,
                1, // always preserve
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
                0, // alignment
            );

            // Create debug location
            let debug_location = LLVMDIBuilderCreateDebugLocation(
                LLVMGetGlobalContext(),
                line,
                0, // column
                self.debug_context.current_scope.unwrap(),
                std::ptr::null_mut(),
            );

            // Insert declare
            LLVMDIBuilderInsertDeclareAtEnd(
                debug_builder,
                alloca,
                debug_variable,
                LLVMDIBuilderCreateExpression(debug_builder, std::ptr::null_mut(), 0),
                debug_location,
                LLVMGetInsertBlock(LLVMCreateBuilderInContext(LLVMGetGlobalContext())),
            );

            // Cache variable
            let debug_var = DebugVariable {
                name: var_name.to_string(),
                var_type: var_type.to_string(),
                scope: self.debug_context.current_scope,
                location: DebugLocation {
                    line,
                    column: 0,
                    file: self.source_files.keys().next().unwrap().clone(),
                    scope: self.debug_context.current_scope,
                },
                debug_variable: Some(debug_variable),
            };

            self.debug_context.variable_cache.insert(var_name.to_string(), debug_var);
        }

        Ok(())
    }

    /// Generate debug information for type
    pub fn generate_type_debug(&mut self, type_name: &str, type_info: &TypeInfo) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        if let Some(cached_type) = self.debug_context.type_cache.get(type_name) {
            return Ok(*cached_type);
        }

        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        let debug_type = match type_info {
            TypeInfo::Basic(basic_type) => {
                self.create_basic_debug_type(basic_type)?
            }
            TypeInfo::Struct(struct_info) => {
                self.create_struct_debug_type(struct_info)?
            }
            TypeInfo::Array(array_info) => {
                self.create_array_debug_type(array_info)?
            }
            TypeInfo::Pointer(pointer_info) => {
                self.create_pointer_debug_type(pointer_info)?
            }
            TypeInfo::Function(function_info) => {
                self.create_function_debug_type(function_info)?
            }
        };

        self.debug_context.type_cache.insert(type_name.to_string(), debug_type);
        Ok(debug_type)
    }

    /// Set debug location for instruction
    pub fn set_debug_location(&mut self, line: u32, column: u32, builder: LLVMBuilderRef) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.debug_level == DebugLevel::None {
            return Ok(());
        }

        unsafe {
            let debug_location = LLVMDIBuilderCreateDebugLocation(
                LLVMGetGlobalContext(),
                line,
                column,
                self.debug_context.current_scope.unwrap(),
                std::ptr::null_mut(),
            );

            LLVMSetCurrentDebugLocation(builder, debug_location);
        }

        // Cache location
        self.line_table.insert(line as usize, DebugLocation {
            line,
            column,
            file: self.source_files.keys().next().unwrap().clone(),
            scope: self.debug_context.current_scope,
        });

        Ok(())
    }

    /// Finalize debug information
    pub fn finalize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(debug_builder) = self.debug_builder {
            unsafe {
                LLVMDIBuilderFinalize(debug_builder);
            }
            
            // Generate debug info summary
            self.generate_debug_summary()?;
        }

        Ok(())
    }

    /// Generate parameter debug info
    fn generate_parameter_debug(&mut self, param: &Parameter, arg_no: usize, function: LLVMValueRef) -> Result<(), Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;

        unsafe {
            let debug_type = self.get_or_create_basic_type(&format!("{:?}", param.param_type))?;
            let param_name = std::ffi::CString::new(param.name.as_bytes())?;
            
            let debug_param = LLVMDIBuilderCreateParameterVariable(
                debug_builder,
                self.debug_context.current_scope.unwrap(),
                param_name.as_ptr(),
                param_name.as_bytes().len(),
                arg_no as u32,
                self.source_files.values().next().unwrap().debug_file.unwrap(),
                1, // line
                debug_type,
                1, // always preserve
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
            );

            // Get parameter value
            let param_value = LLVMGetParam(function, (arg_no - 1) as u32);
            
            // Create debug location
            let debug_location = LLVMDIBuilderCreateDebugLocation(
                LLVMGetGlobalContext(),
                1,
                0,
                self.debug_context.current_scope.unwrap(),
                std::ptr::null_mut(),
            );

            // Insert declare for parameter
            LLVMDIBuilderInsertDeclareAtEnd(
                debug_builder,
                param_value,
                debug_param,
                LLVMDIBuilderCreateExpression(debug_builder, std::ptr::null_mut(), 0),
                debug_location,
                LLVMGetEntryBasicBlock(function),
            );
        }

        Ok(())
    }

    /// Create debug type from type info
    fn create_debug_type(&mut self, type_info: &Option<TypeInfo>) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        if let Some(type_info) = type_info {
            match type_info {
                TypeInfo::Basic(basic_type) => self.create_basic_debug_type(basic_type),
                TypeInfo::Struct(struct_info) => self.create_struct_debug_type(struct_info),
                TypeInfo::Array(array_info) => self.create_array_debug_type(array_info),
                TypeInfo::Pointer(pointer_info) => self.create_pointer_debug_type(pointer_info),
                TypeInfo::Function(function_info) => self.create_function_debug_type(function_info),
            }
        } else {
            // Void type
            let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
            unsafe {
                Ok(LLVMDIBuilderCreateBasicType(
                    debug_builder,
                    std::ffi::CString::new("void")?.as_ptr(),
                    4,
                    0,
                    llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_address,
                    llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
                ))
            }
        }
    }

    /// Create basic debug type
    fn create_basic_debug_type(&mut self, basic_type: &BasicType) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        let (name, size, encoding) = match basic_type {
            BasicType::Int32 => ("i32", 32, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_signed),
            BasicType::Int64 => ("i64", 64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_signed),
            BasicType::Float32 => ("f32", 32, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_float),
            BasicType::Float64 => ("f64", 64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_float),
            BasicType::Bool => ("bool", 1, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_boolean),
            BasicType::String => ("string", 64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_UTF),
            BasicType::Char => ("char", 8, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_UTF),
        };

        unsafe {
            Ok(LLVMDIBuilderCreateBasicType(
                debug_builder,
                std::ffi::CString::new(name)?.as_ptr(),
                name.len(),
                size,
                encoding,
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
            ))
        }
    }

    /// Create struct debug type
    fn create_struct_debug_type(&mut self, struct_info: &StructInfo) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        unsafe {
            let struct_name = std::ffi::CString::new(struct_info.name.as_bytes())?;
            
            // Create struct type
            let struct_type = LLVMDIBuilderCreateStructType(
                debug_builder,
                self.debug_context.current_scope.unwrap(),
                struct_name.as_ptr(),
                struct_name.as_bytes().len(),
                self.source_files.values().next().unwrap().debug_file.unwrap(),
                struct_info.line_number as u32,
                struct_info.size * 8, // size in bits
                struct_info.alignment * 8, // alignment in bits
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            // Create member types
            let mut member_types = Vec::new();
            for (i, field) in struct_info.fields.iter().enumerate() {
                let field_type = self.create_debug_type(&Some(field.field_type.clone()))?;
                let field_name = std::ffi::CString::new(field.name.as_bytes())?;
                
                let member_type = LLVMDIBuilderCreateMemberType(
                    debug_builder,
                    struct_type,
                    field_name.as_ptr(),
                    field_name.as_bytes().len(),
                    self.source_files.values().next().unwrap().debug_file.unwrap(),
                    field.line_number as u32,
                    field.size * 8, // size in bits
                    field.alignment * 8, // alignment in bits
                    field.offset * 8, // offset in bits
                    llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
                    field_type,
                );
                
                member_types.push(member_type);
            }

            // Set member types
            LLVMDIBuilderReplaceArrays(
                debug_builder,
                struct_type,
                member_types.as_ptr(),
                member_types.len(),
            );

            Ok(struct_type)
        }
    }

    /// Create array debug type
    fn create_array_debug_type(&mut self, array_info: &ArrayInfo) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        unsafe {
            let element_type = self.create_debug_type(&Some(array_info.element_type.clone()))?;
            
            // Create array subscript
            let subscript = LLVMDIBuilderGetOrCreateSubrange(
                debug_builder,
                0,
                array_info.size as i64,
            );

            Ok(LLVMDIBuilderCreateArrayType(
                debug_builder,
                array_info.size * array_info.element_size * 8, // size in bits
                array_info.alignment * 8, // alignment in bits
                element_type,
                &subscript,
                1,
            ))
        }
    }

    /// Create pointer debug type
    fn create_pointer_debug_type(&mut self, pointer_info: &PointerInfo) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        unsafe {
            let pointee_type = self.create_debug_type(&Some(pointer_info.pointee_type.clone()))?;
            
            Ok(LLVMDIBuilderCreatePointerType(
                debug_builder,
                pointee_type,
                64, // pointer size in bits
                64, // alignment in bits
                0,  // address space
                std::ffi::CString::new("*")?.as_ptr(),
                1,
            ))
        }
    }

    /// Create function debug type
    fn create_function_debug_type(&mut self, function_info: &FunctionInfo) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        unsafe {
            let return_type = self.create_debug_type(&function_info.return_type)?;
            let mut param_types = vec![return_type];
            
            for param_type in &function_info.parameter_types {
                param_types.push(self.create_debug_type(&Some(param_type.clone()))?);
            }

            Ok(LLVMDIBuilderCreateSubroutineType(
                debug_builder,
                self.source_files.values().next().unwrap().debug_file.unwrap(),
                param_types.as_ptr(),
                param_types.len() as u32,
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
            ))
        }
    }

    /// Get or create basic type
    fn get_or_create_basic_type(&mut self, type_name: &str) -> Result<LLVMMetadataRef, Box<dyn std::error::Error>> {
        if let Some(cached_type) = self.debug_context.type_cache.get(type_name) {
            return Ok(*cached_type);
        }

        let debug_builder = self.debug_builder.ok_or("Debug builder not initialized")?;
        
        let (size, encoding) = match type_name {
            "normie" | "i32" => (32, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_signed),
            "thicc" | "i64" => (64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_signed),
            "drip" | "f32" => (32, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_float),
            "meal" | "f64" => (64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_float),
            "lit" | "bool" => (1, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_boolean),
            "tea" | "string" => (64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_UTF),
            "sip" | "char" => (8, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_UTF),
            _ => (64, llvm_sys::debuginfo::LLVMDWARFTypeEncoding::LLVMDWARFTypeEncoding_address),
        };

        unsafe {
            let debug_type = LLVMDIBuilderCreateBasicType(
                debug_builder,
                std::ffi::CString::new(type_name)?.as_ptr(),
                type_name.len(),
                size,
                encoding,
                llvm_sys::debuginfo::LLVMDIFlags::LLVMDIFlagZero,
            );

            self.debug_context.type_cache.insert(type_name.to_string(), debug_type);
            Ok(debug_type)
        }
    }

    /// Calculate file checksum
    fn calculate_file_checksum(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        use sha2::{Sha256, Digest};
        
        let content = fs::read(file_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Generate debug summary
    fn generate_debug_summary(&self) -> Result<(), Box<dyn std::error::Error>> {
        let summary_path = self.config.output_path.join("debug_summary.json");
        
        let summary = serde_json::json!({
            "debug_level": format!("{:?}", self.config.debug_level),
            "dwarf_version": self.config.dwarf_version,
            "source_files": self.source_files.len(),
            "debug_variables": self.debug_context.variable_cache.len(),
            "debug_types": self.debug_context.type_cache.len(),
            "line_table_entries": self.line_table.len(),
            "files": self.source_files.keys().collect::<Vec<_>>(),
        });

        fs::write(summary_path, serde_json::to_string_pretty(&summary)?)?;
        Ok(())
    }
}

impl DebugContext {
    fn new() -> Self {
        Self {
            compile_unit: None,
            current_scope: None,
            current_function: None,
            type_cache: HashMap::new(),
            variable_cache: HashMap::new(),
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            debug_level: DebugLevel::Full,
            include_source: true,
            include_variables: true,
            include_types: true,
            include_inlined_functions: true,
            dwarf_version: 4,
            output_path: PathBuf::from("debug_output"),
            source_map: true,
            optimization_level: 0,
        }
    }
}

// Placeholder types for compilation
#[derive(Debug, Clone)]
pub enum TypeInfo {
    Basic(BasicType),
    Struct(StructInfo),
    Array(ArrayInfo),
    Pointer(PointerInfo),
    Function(FunctionInfo),
}

#[derive(Debug, Clone)]
pub enum BasicType {
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    String,
    Char,
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub size: u64,
    pub alignment: u64,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: TypeInfo,
    pub offset: u64,
    pub size: u64,
    pub alignment: u64,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct ArrayInfo {
    pub element_type: TypeInfo,
    pub size: u64,
    pub element_size: u64,
    pub alignment: u64,
}

#[derive(Debug, Clone)]
pub struct PointerInfo {
    pub pointee_type: TypeInfo,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub return_type: Option<TypeInfo>,
    pub parameter_types: Vec<TypeInfo>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: TypeInfo,
    pub optional: bool,
    pub default_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_debug_generator_creation() {
        let config = DebugConfig::default();
        let debug_gen = DebugInfoGenerator::new(config);
        
        assert_eq!(debug_gen.config.debug_level, DebugLevel::Full);
        assert_eq!(debug_gen.config.dwarf_version, 4);
    }

    #[test]
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        
        assert_eq!(config.debug_level, DebugLevel::Full);
        assert!(config.include_source);
        assert!(config.include_variables);
        assert!(config.include_types);
    }
}
