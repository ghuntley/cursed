//! Debug information generation for LLVM code.
//!
//! This module provides functionality for adding debug information to
//! LLVM modules, enabling source-level debugging of compiled CURSED programs.

use crate::error::Error;

use inkwell::context::Context;
use inkwell::debug_info::{
    DWARFEmissionKind, DIBuilder, DICompileUnit, DIFile, DILocation, DIScope,
    DISubprogram, DIType,
};
use inkwell::module::Module;
use inkwell::values::{FunctionValue, InstructionValue};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Debug information context for a single compilation unit.
///
/// This struct manages the debug information for a CURSED source file being compiled.
/// It provides methods for creating debug metadata for types, functions, and variables,
/// and for attaching source location information to LLVM instructions.
#[derive(Debug)]
pub struct DebugInfoContext<'ctx> {
    /// The LLVM debug information builder
    di_builder: DIBuilder<'ctx>,
    
    /// The compilation unit metadata
    compile_unit: DICompileUnit<'ctx>,
    
    /// The file metadata for the main source file
    file: DIFile<'ctx>,
    
    /// The source file path
    source_path: PathBuf,
    
    /// Cache of created DIType objects
    type_cache: HashMap<String, DIType<'ctx>>,
    
    /// Cache of function DISubprogram objects
    subprogram_cache: HashMap<String, DISubprogram<'ctx>>,
}

impl<'ctx> DebugInfoContext<'ctx> {
    /// Creates a new debug information context.
    ///
    /// # Arguments
    ///
    /// * `context` - The LLVM context
    /// * `module` - The LLVM module to add debug info to
    /// * `source_path` - Path to the source file
    /// * `optimize` - Whether optimization is enabled
    ///
    /// # Returns
    ///
    /// A new DebugInfoContext instance
    #[tracing::instrument(level = "debug", skip(context, module))]
    pub fn new(
        context: &'ctx Context,
        module: &Module<'ctx>,
        source_path: &Path,
        optimize: bool,
    ) -> Result<Self, Error> {
        // Create a new debug info builder
        let di_builder = DIBuilder::create(module);
        
        // Get the filename component
        let file_name = source_path
            .file_name()
            .ok_or_else(|| Error::from_str("Source path has no filename"))?
            .to_string_lossy()
            .into_owned();
        
        // Get the directory component
        let directory = source_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .into_owned();
        
        // Create debug info for the source file
        let file = di_builder.create_file(&file_name, &directory);
        
        // Create the compile unit
        let compile_unit = di_builder.create_compile_unit(
            DWARFEmissionKind::Full,  // Full DWARF info
            file,                      // Source file
            "cursed compiler",       // Producer
            optimize,                  // Optimized
            "",                      // Flags
            0,                         // Runtime version
            "",                      // Split name
            1,                         // DWARF version
            0,                         // DWARF debug abbrev
            0,                         // Debug line info
            None,                      // Source filename
            None,                      // SDK
        );
        
        tracing::debug!("Created debug info context for source file: {:?}", source_path);
        
        Ok(Self {
            di_builder,
            compile_unit,
            file,
            source_path: source_path.to_path_buf(),
            type_cache: HashMap::new(),
            subprogram_cache: HashMap::new(),
        })
    }
    
    /// Finalizes the debug information and adds it to the module.
    ///
    /// This method must be called after all debug info has been added
    /// and before the module is compiled to machine code.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn finalize(&self) {
        tracing::debug!("Finalizing debug information");
        self.di_builder.finalize();
    }
    
    /// Creates debug information for a function.
    ///
    /// # Arguments
    ///
    /// * `function` - The LLVM function value
    /// * `name` - The function name
    /// * `line` - The line number where the function is defined
    /// * `return_type` - The DIType for the function's return type
    /// * `param_types` - The DITypes for the function's parameters
    ///
    /// # Returns
    ///
    /// The created DISubprogram
    #[tracing::instrument(level = "debug", skip(self, function, return_type, param_types))]
    pub fn create_function_info(
        &mut self,
        function: FunctionValue<'ctx>,
        name: &str,
        line: u32,
        return_type: DIType<'ctx>,
        param_types: Vec<DIType<'ctx>>,
    ) -> DISubprogram<'ctx> {
        tracing::debug!("Creating debug info for function: {}", name);
        
        // Create a function type for the debug info
        let function_type = self.di_builder.create_subroutine_type(
            self.file,
            return_type,
            param_types.as_slice(),
            0, // Flags
        );
        
        // Create the subprogram
        let subprogram = self.di_builder.create_function(
            self.compile_unit.as_debug_info_scope(),
            name,
            None,
            self.file,
            line,
            function_type,
            true,  // Is local to unit
            true,  // Is definition
            line,  // Scope line
            0,     // Flags
            false, // Is optimized
        );
        
        // Set the subprogram as the function's debug scope
        function.set_subprogram(subprogram);
        
        // Cache the subprogram
        self.subprogram_cache.insert(name.to_string(), subprogram);
        
        subprogram
    }
    
    /// Creates debug information for a local variable.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name
    /// * `var_type` - The DIType for the variable's type
    /// * `scope` - The DIScope the variable is defined in
    /// * `line` - The line number where the variable is defined
    ///
    /// # Returns
    ///
    /// The created DILocalVariable
    #[tracing::instrument(level = "debug", skip(self, var_type, scope))]
    pub fn create_local_variable(
        &self,
        name: &str,
        var_type: DIType<'ctx>,
        scope: DIScope<'ctx>,
        line: u32,
    ) {
        tracing::debug!("Creating debug info for local variable: {}", name);
        
        self.di_builder.create_parameter_variable(
            scope,
            name,
            0,      // Arg number
            self.file,
            line,
            var_type,
            true,   // Always preserve
            0,      // Flags
        );
    }
    
    /// Creates debug info for primitive types (used for caching common types).
    ///
    /// # Returns
    ///
    /// The created primitive types (int, bool, etc.)
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn create_primitive_types(&mut self) {
        tracing::debug!("Creating debug info for primitive types");
        
        // Basic numeric types
        let di_int32_type = self.di_builder.create_basic_type(
            "int",
            32,     // Size in bits
            0,      // Alignment
            0,      // Flags
        );
        self.type_cache.insert("int".to_string(), di_int32_type);
        
        let di_bool_type = self.di_builder.create_basic_type(
            "bool",
            1,      // Size in bits
            0,      // Alignment
            0,      // Flags
        );
        self.type_cache.insert("bool".to_string(), di_bool_type);
        
        let di_float_type = self.di_builder.create_basic_type(
            "float",
            32,     // Size in bits
            0,      // Alignment
            0,      // Flags
        );
        self.type_cache.insert("float".to_string(), di_float_type);
        
        let di_double_type = self.di_builder.create_basic_type(
            "double",
            64,     // Size in bits
            0,      // Alignment
            0,      // Flags
        );
        self.type_cache.insert("double".to_string(), di_double_type);
        
        let di_char_type = self.di_builder.create_basic_type(
            "char",
            8,      // Size in bits
            0,      // Alignment
            0,      // Flags
        );
        self.type_cache.insert("char".to_string(), di_char_type);
        
        // Create a void type for functions that don't return a value
        let di_void_type = self.di_builder.create_unspecified_type("void");
        self.type_cache.insert("void".to_string(), di_void_type);
    }
    
    /// Sets the current debug location for the following instructions.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number in the source file
    /// * `column` - The column number in the source file
    /// * `scope` - The DIScope the location is in
    ///
    /// # Returns
    ///
    /// The created DILocation
    #[tracing::instrument(level = "debug", skip(self, scope))]
    pub fn set_debug_location(
        &self,
        line: u32,
        column: u32,
        scope: DIScope<'ctx>,
    ) -> DILocation<'ctx> {
        tracing::debug!("Setting debug location: line {}, column {}", line, column);
        
        self.di_builder.create_debug_location(
            self.context(), 
            line, 
            column, 
            scope, 
            None
        )
    }
    
    /// Attaches a debug location to an instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The LLVM instruction
    /// * `location` - The DILocation to attach
    #[tracing::instrument(level = "debug", skip(self, instruction, location))]
    pub fn set_instruction_location(
        &self,
        instruction: InstructionValue<'ctx>,
        location: DILocation<'ctx>,
    ) {
        instruction.set_debug_loc(location);
    }
    
    /// Gets the LLVM context.
    ///
    /// # Returns
    ///
    /// The LLVM context
    pub fn context(&self) -> &'ctx Context {
        self.di_builder.get_context()
    }
    
    /// Gets the DIType for a type name from the cache.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the type
    ///
    /// # Returns
    ///
    /// The DIType if found, None otherwise
    pub fn get_type(&self, type_name: &str) -> Option<DIType<'ctx>> {
        self.type_cache.get(type_name).copied()
    }
    
    /// Gets the DISubprogram for a function name from the cache.
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function
    ///
    /// # Returns
    ///
    /// The DISubprogram if found, None otherwise
    pub fn get_subprogram(&self, function_name: &str) -> Option<DISubprogram<'ctx>> {
        self.subprogram_cache.get(function_name).copied()
    }
    
    /// Creates a DIType for an array type.
    ///
    /// # Arguments
    ///
    /// * `element_type` - The DIType of the array elements
    /// * `element_count` - The number of elements in the array
    ///
    /// # Returns
    ///
    /// The created DIType for the array
    pub fn create_array_type(
        &self,
        element_type: DIType<'ctx>,
        element_count: u32,
    ) -> DIType<'ctx> {
        let subscript = self.di_builder.create_subroutine_type(
            self.file,
            self.get_type("void").unwrap(),
            &[],
            0,
        );
        
        self.di_builder.create_array_type(
            element_type,
            element_count,
            8,      // Alignment in bits
            &[subscript],
        )
    }
    
    /// Creates a DIType for a struct type.
    ///
    /// # Arguments
    ///
    /// * `struct_name` - The name of the struct
    /// * `member_types` - The DITypes of the struct members
    /// * `member_names` - The names of the struct members
    /// * `size_in_bits` - The total size of the struct in bits
    ///
    /// # Returns
    ///
    /// The created DIType for the struct
    pub fn create_struct_type(
        &mut self,
        struct_name: &str,
        member_types: &[DIType<'ctx>],
        member_names: &[&str],
        size_in_bits: u64,
    ) -> DIType<'ctx> {
        let scope = self.compile_unit.as_debug_info_scope();
        
        let struct_type = self.di_builder.create_struct_type(
            scope,
            struct_name,
            self.file,
            0,      // Line number
            size_in_bits,
            8,      // Alignment in bits
            0,      // Flags
            None,   // Derived from
            &[],    // Elements - filled in below
            0,      // Runtime language
            None,   // VTable holder
        );
        
        // Add struct members
        let mut members = Vec::new();
        let mut offset = 0;
        
        for (i, (member_type, member_name)) in member_types.iter().zip(member_names).enumerate() {
            let member = self.di_builder.create_member_type(
                scope,
                member_name,
                self.file,
                0,      // Line number
                member_type.get_size_in_bits(),
                8,      // Alignment in bits
                offset, // Offset in bits
                0,      // Flags
                *member_type,
            );
            
            members.push(member);
            offset += member_type.get_size_in_bits();
        }
        
        // Add members to the struct type
        struct_type.replace_elements(&members);
        
        // Cache the struct type
        self.type_cache.insert(struct_name.to_string(), struct_type);
        
        struct_type
    }
    
    /// Creates a DIType for a pointer type.
    ///
    /// # Arguments
    ///
    /// * `pointee_type` - The DIType of the type being pointed to
    /// * `name` - Optional name for the pointer type
    ///
    /// # Returns
    ///
    /// The created DIType for the pointer
    pub fn create_pointer_type(
        &self,
        pointee_type: DIType<'ctx>,
        name: Option<&str>,
    ) -> DIType<'ctx> {
        let ptr_name = name.unwrap_or("");
        
        self.di_builder.create_pointer_type(
            ptr_name,
            64,     // Size in bits (assuming 64-bit pointers)
            8,      // Alignment in bits
            0,      // Address space
            None,   // Optional name
            pointee_type,
        )
    }
}

/// Adds debug information to an LLVM module.
///
/// This function creates and attaches debug information to an LLVM module
/// based on the specified debug level.
///
/// # Arguments
///
/// * `context` - The LLVM context
/// * `module` - The LLVM module to add debug info to
/// * `source_path` - Path to the source file
/// * `debug_level` - The level of debug information to add
/// * `optimize` - Whether optimization is enabled
///
/// # Returns
///
/// Result<Option<DebugInfoContext>, Error> - The debug info context if created, or None if debug is disabled
#[tracing::instrument(level = "info", skip(context, module))]
pub fn add_debug_info(
    context: &Context,
    module: &Module,
    source_path: &Path,
    debug_level: &super::binary_compiler::DebugInfoLevel,
    optimize: bool,
) -> Result<Option<DebugInfoContext>, Error> {
    match debug_level {
        super::binary_compiler::DebugInfoLevel::None => {
            tracing::info!("Debug information disabled");
            Ok(None)
        },
        super::binary_compiler::DebugInfoLevel::LineInfo | super::binary_compiler::DebugInfoLevel::Full => {
            tracing::info!("Adding debug information at level: {:?}", debug_level);
            let debug_info = DebugInfoContext::new(context, module, source_path, optimize)?;
            
            // Create primitive type debug info
            debug_info.create_primitive_types();
            
            Ok(Some(debug_info))
        }
    }
}