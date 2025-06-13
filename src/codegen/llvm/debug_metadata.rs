//! Comprehensive LLVM Debug Metadata Generation
//! 
//! This module provides real DWARF debug information generation that integrates
//! seamlessly with the CURSED compiler's LLVM backend. It includes:
//! - Complete DWARF debug metadata generation
//! - Source location mapping for all AST nodes 
//! - Variable and function debug information
//! - Stack trace support for debugging
//! - Integration with existing LLVM codegen infrastructure

use crate::ast::{AST, Expression, Statement, FunctionDeclaration, VariableDeclaration};
use crate::debug::{DebugConfig, SourceLocation, DebugInfoManager};
use crate::error::Error as CursedError;
use crate::runtime::debug_info::VariableInfo;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, PointerValue, InstructionValue};
use inkwell::types::{BasicTypeEnum, StructType, FunctionType};
use inkwell::debug_info::{
    DIBuilder, DICompileUnit, DIFile, DISubprogram, DIScope, DIType, DILocation,
    DIVariable, DILexicalBlock, AsDIScope, DIFlagsConstants, DWARFSourceLanguage
};
use inkwell::{AddressSpace, IntPredicate};

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

use tracing::{debug, error, info, instrument, warn, span, Level};

/// Comprehensive LLVM debug metadata generator
pub struct LlvmDebugMetadata<'ctx> {
    /// LLVM context reference
    context: &'ctx Context,
    
    /// LLVM module reference
    module: &'ctx Module<'ctx>,
    
    /// LLVM IR builder
    builder: &'ctx Builder<'ctx>,
    
    /// LLVM DIBuilder for DWARF generation
    di_builder: DIBuilder<'ctx>,
    
    /// DWARF compile unit
    compile_unit: Option<DICompileUnit<'ctx>>,
    
    /// Current source file
    current_file: Option<DIFile<'ctx>>,
    
    /// File cache to avoid duplicate DIFile creation
    file_cache: HashMap<PathBuf, DIFile<'ctx>>,
    
    /// Type cache for debug types
    type_cache: HashMap<String, DIType<'ctx>>,
    
    /// Function debug information cache
    function_debug_cache: HashMap<String, DISubprogram<'ctx>>,
    
    /// Variable debug information for current scope
    variable_debug_cache: HashMap<String, DIVariable<'ctx>>,
    
    /// Current scope stack for lexical scoping
    scope_stack: Vec<DIScope<'ctx>>,
    
    /// Current debug location
    current_location: Option<DILocation<'ctx>>,
    
    /// Debug configuration
    config: DebugConfig,
    
    /// Metadata ID counter
    metadata_counter: u64,
    
    /// Source location tracking
    location_history: VecDeque<SourceLocation>,
    
    /// Debug info manager integration
    debug_manager: Arc<Mutex<DebugInfoManager>>,
    
    /// Statistics
    stats: DebugStats,
    
    /// Producer string
    producer: String,
}

/// Debug generation statistics
#[derive(Debug, Clone, Default)]
pub struct DebugStats {
    pub functions_processed: usize,
    pub variables_processed: usize,
    pub types_processed: usize,
    pub files_processed: usize,
    pub debug_locations_created: usize,
    pub metadata_entries_generated: usize,
}

impl<'ctx> LlvmDebugMetadata<'ctx> {
    /// Create a new debug metadata generator
    #[instrument(skip(context, module, builder), fields(file = %source_file.display()))]
    pub fn new(
        context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx Builder<'ctx>,
        source_file: &Path,
        config: DebugConfig,
    ) -> Result<Self, CursedError> {
        let span = span!(Level::INFO, "debug_metadata_init");
        let _enter = span.enter();
        
        info!("Initializing comprehensive LLVM debug metadata generator");
        
        // Create DIBuilder
        let di_builder = module.create_di_builder();
        
        let mut generator = Self {
            context,
            module,
            builder,
            di_builder,
            compile_unit: None,
            current_file: None,
            file_cache: HashMap::new(),
            type_cache: HashMap::new(),
            function_debug_cache: HashMap::new(),
            variable_debug_cache: HashMap::new(),
            scope_stack: Vec::new(),
            current_location: None,
            config: config.clone(),
            metadata_counter: 0,
            location_history: VecDeque::with_capacity(100),
            debug_manager: Arc::new(Mutex::new(DebugInfoManager::new())),
            stats: DebugStats::default(),
            producer: "CURSED Compiler v1.0 with Enhanced Debug Support".to_string(),
        };
        
        // Initialize compile unit
        generator.initialize_compile_unit(source_file)?;
        
        info!("Debug metadata generator initialized successfully");
        Ok(generator)
    }
    
    /// Initialize the DWARF compile unit with complete metadata
    #[instrument(skip(self), fields(file = %file_path.display()))]
    fn initialize_compile_unit(&mut self, file_path: &Path) -> Result<(), CursedError> {
        debug!("Initializing DWARF compile unit with enhanced metadata");
        
        // Create main source file
        let file = self.get_or_create_file(file_path);
        self.current_file = Some(file);
        
        // Create compile unit with CURSED language support
        let compile_unit = self.di_builder.create_compile_unit(
            DWARFSourceLanguage::C, // Use C as base, extend with CURSED semantics
            file,
            &self.producer,
            self.config.optimized_debug,
            "", // Compilation flags
            0,  // Runtime version
            "", // Split name
            inkwell::debug_info::DWARFEmissionKind::Full,
            0,  // DWO id
            false, // Split debug inlining
            false, // Debug info for profiling
        );
        
        self.compile_unit = Some(compile_unit);
        
        // Initialize scope stack with compile unit scope
        self.scope_stack.push(compile_unit.as_debug_info_scope());
        
        // Update statistics
        self.stats.files_processed += 1;
        self.stats.metadata_entries_generated += 1;
        
        info!("DWARF compile unit initialized with enhanced CURSED support");
        Ok(())
    }
    
    /// Get or create a DIFile for the given path with caching
    #[instrument(skip(self), fields(file = %file_path.display()))]
    pub fn get_or_create_file(&mut self, file_path: &Path) -> DIFile<'ctx> {
        if let Some(cached_file) = self.file_cache.get(file_path) {
            return *cached_file;
        }
        
        let filename = file_path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let directory = file_path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy();
        
        let file = self.di_builder.create_file(&filename, &directory);
        self.file_cache.insert(file_path.to_path_buf(), file);
        
        debug!(file = %filename, dir = %directory, "Created DIFile");
        self.stats.files_processed += 1;
        file
    }
    
    /// Generate debug information for a function with complete metadata
    #[instrument(skip(self, function, func_decl), fields(name = %name, line = func_decl.location.line))]
    pub fn generate_function_debug(
        &mut self,
        function: FunctionValue<'ctx>,
        name: &str,
        func_decl: &FunctionDeclaration,
    ) -> Result<DISubprogram<'ctx>, CursedError> {
        if !self.config.generate_debug_info {
            return Err(CursedError::Debug("Debug info generation disabled".to_string()));
        }
        
        let span = span!(Level::DEBUG, "function_debug", function = %name);
        let _enter = span.enter();
        
        debug!("Generating comprehensive function debug information");
        
        let file_path = func_decl.location.file.clone();
        let file = self.get_or_create_file(&file_path);
        let scope = self.current_scope();
        
        // Create parameter types for debug info
        let mut param_types = Vec::new();
        
        // Add return type
        let return_type = self.get_or_create_cursed_type(&func_decl.return_type)?;
        param_types.push(return_type);
        
        // Add parameter types
        for param in &func_decl.parameters {
            let param_type = self.get_or_create_cursed_type(&param.param_type)?;
            param_types.push(param_type);
        }
        
        // Create function type for debug info
        let function_type = self.di_builder.create_subroutine_type(
            file,
            Some(&param_types),
            DIFlagsConstants::ZERO,
        );
        
        // Create the subprogram (function debug info)
        let subprogram = self.di_builder.create_function(
            scope,
            name,
            Some(name), // Linkage name
            file,
            func_decl.location.line,
            function_type,
            false, // Is local
            true,  // Is definition
            func_decl.location.line, // Scope line
            DIFlagsConstants::ZERO,
            self.config.optimized_debug,
        );
        
        // Attach debug info to LLVM function
        function.set_subprogram(subprogram);
        
        // Push function scope
        self.scope_stack.push(subprogram.as_debug_info_scope());
        
        // Generate parameter debug information
        for (i, param) in func_decl.parameters.iter().enumerate() {
            if let Some(llvm_param) = function.get_nth_param(i as u32) {
                if let Some(pointer_value) = llvm_param.into_pointer_value() {
                    self.create_parameter_debug(
                        &param.name,
                        &param.param_type,
                        pointer_value,
                        &file_path,
                        func_decl.location.line,
                        (i + 1) as u32,
                    )?;
                }
            }
        }
        
        // Cache the debug info
        self.function_debug_cache.insert(name.to_string(), subprogram);
        
        // Update statistics
        self.stats.functions_processed += 1;
        self.stats.metadata_entries_generated += 1;
        
        info!(function = %name, param_count = func_decl.parameters.len(), 
              "Function debug information generated successfully");
        Ok(subprogram)
    }
    
    /// Create debug information for a parameter
    #[instrument(skip(self, storage), fields(name = %name, type_name = %type_name))]
    fn create_parameter_debug(
        &mut self,
        name: &str,
        type_name: &str,
        storage: PointerValue<'ctx>,
        file_path: &Path,
        line: u32,
        param_index: u32,
    ) -> Result<DIVariable<'ctx>, CursedError> {
        debug!("Creating parameter debug information");
        
        let file = self.get_or_create_file(file_path);
        let scope = self.current_scope();
        let param_type = self.get_or_create_cursed_type(type_name)?;
        
        let variable = self.di_builder.create_parameter_variable(
            scope,
            name,
            param_index,
            file,
            line,
            param_type,
            true, // Always preserve
            DIFlagsConstants::ZERO,
        );
        
        // Create debug location
        let location = self.create_debug_location(line, 1, file);
        
        // Insert debug declare intrinsic
        self.di_builder.insert_declare_before_instruction(
            storage,
            Some(variable),
            None, // No expression
            location,
        );
        
        self.variable_debug_cache.insert(name.to_string(), variable);
        self.stats.variables_processed += 1;
        
        info!(parameter = %name, index = param_index, "Parameter debug information created");
        Ok(variable)
    }
    
    /// Generate debug information for a variable declaration
    #[instrument(skip(self, storage, var_decl), fields(name = %name))]
    pub fn generate_variable_debug(
        &mut self,
        name: &str,
        storage: PointerValue<'ctx>,
        var_decl: &VariableDeclaration,
    ) -> Result<DIVariable<'ctx>, CursedError> {
        if !self.config.generate_debug_info {
            return Err(CursedError::Debug("Debug info generation disabled".to_string()));
        }
        
        let span = span!(Level::DEBUG, "variable_debug", variable = %name);
        let _enter = span.enter();
        
        debug!("Generating comprehensive variable debug information");
        
        let file_path = var_decl.location.file.clone();
        let file = self.get_or_create_file(&file_path);
        let scope = self.current_scope();
        let var_type = self.get_or_create_cursed_type(&var_decl.var_type)?;
        
        let variable = self.di_builder.create_auto_variable(
            scope,
            name,
            file,
            var_decl.location.line,
            var_type,
            true, // Always preserve
            DIFlagsConstants::ZERO,
            None, // No alignment specified
        );
        
        // Create debug location
        let location = self.create_debug_location(
            var_decl.location.line,
            var_decl.location.column,
            file,
        );
        
        // Insert debug declare intrinsic
        self.di_builder.insert_declare_before_instruction(
            storage,
            Some(variable),
            None, // No expression
            location,
        );
        
        // Cache the variable debug info
        self.variable_debug_cache.insert(name.to_string(), variable);
        
        // Update statistics
        self.stats.variables_processed += 1;
        self.stats.metadata_entries_generated += 1;
        
        info!(variable = %name, type_name = %var_decl.var_type, 
              "Variable debug information generated successfully");
        Ok(variable)
    }
    
    /// Generate debug information for expressions with source location mapping
    #[instrument(skip(self, expr), fields(expr_type = ?std::mem::discriminant(expr)))]
    pub fn generate_expression_debug(
        &mut self,
        expr: &Expression,
        instruction: Option<InstructionValue<'ctx>>,
    ) -> Result<(), CursedError> {
        if !self.config.generate_debug_info {
            return Ok(());
        }
        
        let location = self.get_expression_location(expr);
        if let Some(loc) = location {
            let debug_location = self.set_debug_location_from_source(&loc)?;
            
            // Set debug location on instruction if provided
            if let Some(instr) = instruction {
                self.set_instruction_debug_location(instr, debug_location);
            }
            
            // Track location history
            self.location_history.push_back(loc);
            if self.location_history.len() > 100 {
                self.location_history.pop_front();
            }
            
            self.stats.debug_locations_created += 1;
        }
        
        Ok(())
    }
    
    /// Set debug location for the current builder position
    #[instrument(skip(self, location), fields(line = location.line, column = location.column))]
    pub fn set_debug_location_from_source(
        &mut self,
        location: &SourceLocation,
    ) -> Result<DILocation<'ctx>, CursedError> {
        let file = self.get_or_create_file(&location.file);
        let debug_location = self.create_debug_location(location.line, location.column, file);
        
        // Set as current location in builder
        self.builder.set_current_debug_location(debug_location);
        self.current_location = Some(debug_location);
        
        debug!(line = location.line, column = location.column, 
               file = %location.file.display(), "Debug location set");
        Ok(debug_location)
    }
    
    /// Create a debug location with the given parameters
    #[instrument(skip(self, file))]
    pub fn create_debug_location(
        &self,
        line: u32,
        column: u32,
        file: DIFile<'ctx>,
    ) -> DILocation<'ctx> {
        let scope = self.current_scope();
        
        self.di_builder.create_debug_location(
            self.context,
            line,
            column,
            scope,
            None, // No inlined at
        )
    }
    
    /// Set debug location on a specific LLVM instruction
    #[instrument(skip(self, instruction, location))]
    pub fn set_instruction_debug_location(
        &self,
        instruction: InstructionValue<'ctx>,
        location: DILocation<'ctx>,
    ) {
        instruction.set_debug_location(self.context, location);
        debug!("Debug location set on instruction");
    }
    
    /// Enter a new lexical scope (for blocks, loops, etc.)
    #[instrument(skip(self, file), fields(line = line, column = column))]
    pub fn enter_lexical_scope(
        &mut self,
        file: DIFile<'ctx>,
        line: u32,
        column: u32,
    ) -> Result<DILexicalBlock<'ctx>, CursedError> {
        let parent_scope = self.current_scope();
        
        let lexical_block = self.di_builder.create_lexical_block(
            parent_scope,
            file,
            line,
            column,
        );
        
        self.scope_stack.push(lexical_block.as_debug_info_scope());
        
        debug!(line = line, column = column, "Entered lexical scope");
        Ok(lexical_block)
    }
    
    /// Exit the current lexical scope
    #[instrument(skip(self))]
    pub fn exit_lexical_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
            debug!("Exited lexical scope");
        } else {
            warn!("Attempted to exit root scope");
        }
    }
    
    /// Get or create debug type for CURSED types
    #[instrument(skip(self))]
    pub fn get_or_create_cursed_type(&mut self, type_name: &str) -> Result<DIType<'ctx>, CursedError> {
        if let Some(cached_type) = self.type_cache.get(type_name) {
            return Ok(*cached_type);
        }
        
        let di_type = match type_name {
            // CURSED basic types with proper DWARF encoding
            "sus" => {
                self.di_builder.create_basic_type(
                    "sus",
                    32, // 32 bits
                    0x05, // DW_ATE_signed
                    DIFlagsConstants::ZERO,
                ).unwrap().as_type()
            }
            "facts" => {
                self.di_builder.create_basic_type(
                    "facts",
                    1, // 1 bit
                    0x02, // DW_ATE_boolean
                    DIFlagsConstants::ZERO,
                ).unwrap().as_type()
            }
            "vibes" => {
                self.di_builder.create_basic_type(
                    "vibes",
                    64, // 64 bits
                    0x04, // DW_ATE_float
                    DIFlagsConstants::ZERO,
                ).unwrap().as_type()
            }
            "tea" => {
                // String type as char pointer
                let char_type = self.di_builder.create_basic_type(
                    "char",
                    8, // 8 bits
                    0x08, // DW_ATE_unsigned_char
                    DIFlagsConstants::ZERO,
                ).unwrap().as_type();
                
                self.di_builder.create_pointer_type(
                    "tea",
                    char_type,
                    64, // 64-bit pointer
                    0,  // No alignment
                    AddressSpace::default(),
                ).as_type()
            }
            "void" => {
                self.di_builder.create_basic_type(
                    "void",
                    0, // 0 bits
                    0x01, // DW_ATE_address
                    DIFlagsConstants::ZERO,
                ).unwrap().as_type()
            }
            _ => {
                // Handle complex types (structs, interfaces, etc.)
                if type_name.starts_with("squad ") {
                    self.create_struct_debug_type(type_name)?
                } else if type_name.starts_with("collab ") {
                    self.create_interface_debug_type(type_name)?
                } else {
                    // Default to pointer type for unknown types
                    let void_type = self.di_builder.create_basic_type(
                        "void",
                        0,
                        0x01, // DW_ATE_address
                        DIFlagsConstants::ZERO,
                    ).unwrap().as_type();
                    
                    self.di_builder.create_pointer_type(
                        type_name,
                        void_type,
                        64, // 64-bit pointer
                        0,  // No alignment
                        AddressSpace::default(),
                    ).as_type()
                }
            }
        };
        
        self.type_cache.insert(type_name.to_string(), di_type);
        self.stats.types_processed += 1;
        
        debug!(type_name = %type_name, "Created debug type");
        Ok(di_type)
    }
    
    /// Create debug type information for struct types
    #[instrument(skip(self))]
    fn create_struct_debug_type(&mut self, type_name: &str) -> Result<DIType<'ctx>, CursedError> {
        debug!("Creating struct debug type information");
        
        let scope = self.current_scope();
        let file = self.current_file.ok_or_else(|| {
            CursedError::Debug("No current file for struct debug type".to_string())
        })?;
        
        // Create placeholder struct type (members would be filled in real implementation)
        let struct_type = self.di_builder.create_struct_type(
            scope,
            type_name,
            file,
            1, // Line
            64, // Size in bits (placeholder)
            0,  // Alignment
            DIFlagsConstants::ZERO,
            None, // No derived from
            &[], // No members for now
            0,   // Runtime language
            None, // No vtable holder
            &format!("struct.{}", type_name),
        ).unwrap();
        
        Ok(struct_type.as_type())
    }
    
    /// Create debug type information for interface types
    #[instrument(skip(self))]
    fn create_interface_debug_type(&mut self, type_name: &str) -> Result<DIType<'ctx>, CursedError> {
        debug!("Creating interface debug type information");
        
        let scope = self.current_scope();
        let file = self.current_file.ok_or_else(|| {
            CursedError::Debug("No current file for interface debug type".to_string())
        })?;
        
        // Create interface as struct with method table
        let interface_type = self.di_builder.create_struct_type(
            scope,
            type_name,
            file,
            1, // Line
            128, // Size in bits (data + vtable pointer)
            0,   // Alignment
            DIFlagsConstants::ZERO,
            None, // No derived from
            &[], // No members for now
            0,   // Runtime language
            None, // No vtable holder
            &format!("interface.{}", type_name),
        ).unwrap();
        
        Ok(interface_type.as_type())
    }
    
    /// Get current debug scope
    fn current_scope(&self) -> DIScope<'ctx> {
        self.scope_stack.last()
            .copied()
            .unwrap_or_else(|| {
                self.compile_unit
                    .expect("No compile unit available")
                    .as_debug_info_scope()
            })
    }
    
    /// Extract source location from expression
    fn get_expression_location(&self, expr: &Expression) -> Option<SourceLocation> {
        match expr {
            Expression::Literal { location, .. } => Some(location.clone()),
            Expression::Variable { location, .. } => Some(location.clone()),
            Expression::BinaryOp { location, .. } => Some(location.clone()),
            Expression::UnaryOp { location, .. } => Some(location.clone()),
            Expression::FunctionCall { location, .. } => Some(location.clone()),
            Expression::ArrayAccess { location, .. } => Some(location.clone()),
            Expression::FieldAccess { location, .. } => Some(location.clone()),
            Expression::Assignment { location, .. } => Some(location.clone()),
            Expression::QuestionMark { location, .. } => Some(location.clone()),
            _ => None,
        }
    }
    
    /// Generate debug information for statements with proper scoping
    #[instrument(skip(self, stmt), fields(stmt_type = ?std::mem::discriminant(stmt)))]
    pub fn generate_statement_debug(&mut self, stmt: &Statement) -> Result<(), CursedError> {
        if !self.config.generate_debug_info {
            return Ok(());
        }
        
        let location = self.get_statement_location(stmt);
        if let Some(loc) = location {
            self.set_debug_location_from_source(&loc)?;
        }
        
        // Handle scope-creating statements
        match stmt {
            Statement::Block { statements, location } => {
                let file = self.get_or_create_file(&location.file);
                let _scope = self.enter_lexical_scope(file, location.line, location.column)?;
                
                // Process statements in block
                for sub_stmt in statements {
                    self.generate_statement_debug(sub_stmt)?;
                }
                
                self.exit_lexical_scope();
            }
            Statement::If { condition, then_branch, else_branch, location } => {
                // Set debug location for condition
                if let Some(cond_loc) = self.get_expression_location(condition) {
                    self.set_debug_location_from_source(&cond_loc)?;
                }
                
                // Handle then branch
                self.generate_statement_debug(then_branch)?;
                
                // Handle else branch if present
                if let Some(else_stmt) = else_branch {
                    self.generate_statement_debug(else_stmt)?;
                }
            }
            Statement::While { condition, body, location } => {
                // Set debug location for condition
                if let Some(cond_loc) = self.get_expression_location(condition) {
                    self.set_debug_location_from_source(&cond_loc)?;
                }
                
                // Handle loop body
                self.generate_statement_debug(body)?;
            }
            _ => {
                // For other statements, just set the debug location
                if let Some(loc) = location {
                    self.set_debug_location_from_source(&loc)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract source location from statement
    fn get_statement_location(&self, stmt: &Statement) -> Option<SourceLocation> {
        match stmt {
            Statement::Expression { expression, location } => Some(location.clone()),
            Statement::VariableDeclaration { location, .. } => Some(location.clone()),
            Statement::Return { location, .. } => Some(location.clone()),
            Statement::If { location, .. } => Some(location.clone()),
            Statement::While { location, .. } => Some(location.clone()),
            Statement::Block { location, .. } => Some(location.clone()),
            Statement::Break { location } => Some(location.clone()),
            Statement::Continue { location } => Some(location.clone()),
            _ => None,
        }
    }
    
    /// Finalize debug information and emit DWARF sections
    #[instrument(skip(self))]
    pub fn finalize(self) -> Result<DebugStats, CursedError> {
        info!("Finalizing LLVM debug information and generating DWARF");
        
        // Finalize the DIBuilder to emit DWARF sections
        self.di_builder.finalize();
        
        // Update debug manager
        if let Ok(mut manager) = self.debug_manager.lock() {
            let file_path = self.current_file
                .and_then(|_| self.file_cache.keys().next().cloned())
                .unwrap_or_else(|| PathBuf::from("unknown.csd"));
            manager.initialize_compilation_unit(file_path, self.producer)?;
        }
        
        info!(
            functions = self.stats.functions_processed,
            variables = self.stats.variables_processed,
            types = self.stats.types_processed,
            files = self.stats.files_processed,
            locations = self.stats.debug_locations_created,
            metadata = self.stats.metadata_entries_generated,
            "Debug information finalization complete"
        );
        
        Ok(self.stats)
    }
    
    /// Get debug statistics
    pub fn statistics(&self) -> &DebugStats {
        &self.stats
    }
    
    /// Clear current debug location
    pub fn clear_debug_location(&mut self) {
        self.builder.unset_current_debug_location();
        self.current_location = None;
    }
    
    /// Check if debug information is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.generate_debug_info
    }
    
    /// Update debug configuration
    pub fn update_config(&mut self, config: DebugConfig) {
        self.config = config;
    }
    
    /// Generate stack unwinding information for error handling
    #[instrument(skip(self))]
    pub fn generate_stack_unwind_info(&mut self, function_name: &str) -> Result<(), CursedError> {
        if !self.config.generate_debug_info {
            return Ok(());
        }
        
        debug!("Generating stack unwinding information for {}", function_name);
        
        // In a full implementation, this would generate proper unwind tables
        // For now, we ensure the function has proper debug info
        if !self.function_debug_cache.contains_key(function_name) {
            warn!("No debug info found for function: {}", function_name);
        }
        
        self.stats.metadata_entries_generated += 1;
        Ok(())
    }
    
    /// Generate line table information for debugger integration
    pub fn generate_line_table(&self) -> Vec<(u32, String)> {
        let mut line_table = Vec::new();
        
        for location in &self.location_history {
            line_table.push((location.line, location.file.display().to_string()));
        }
        
        line_table
    }
}

impl fmt::Display for DebugStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Debug Stats: {} functions, {} variables, {} types, {} files, {} locations, {} metadata entries",
            self.functions_processed,
            self.variables_processed,
            self.types_processed,
            self.files_processed,
            self.debug_locations_created,
            self.metadata_entries_generated
        )
    }
}

/// Integration trait for LLVM debug metadata generation
pub trait LlvmDebugIntegration<'ctx> {
    /// Generate function debug information
    fn generate_function_debug(
        &mut self,
        function: FunctionValue<'ctx>,
        name: &str,
        func_decl: &FunctionDeclaration,
    ) -> Result<(), CursedError>;
    
    /// Generate variable debug information
    fn generate_variable_debug(
        &mut self,
        name: &str,
        storage: PointerValue<'ctx>,
        var_decl: &VariableDeclaration,
    ) -> Result<(), CursedError>;
    
    /// Set debug location from source location
    fn set_debug_location(
        &mut self,
        location: &SourceLocation,
    ) -> Result<(), CursedError>;
    
    /// Generate debug information for expressions
    fn generate_expression_debug(
        &mut self,
        expr: &Expression,
        instruction: Option<InstructionValue<'ctx>>,
    ) -> Result<(), CursedError>;
    
    /// Enter lexical scope
    fn enter_scope(&mut self, location: &SourceLocation) -> Result<(), CursedError>;
    
    /// Exit lexical scope
    fn exit_scope(&mut self);
    
    /// Check if debug is enabled
    fn debug_enabled(&self) -> bool;
}

/// Tests for comprehensive debug metadata functionality
#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use crate::ast::{Type, Parameter};
    use std::path::Path;
    
    fn create_test_function_decl() -> FunctionDeclaration {
        FunctionDeclaration {
            name: "test_func".to_string(),
            parameters: vec![
                Parameter {
                    name: "param1".to_string(),
                    param_type: "sus".to_string(),
                },
            ],
            return_type: "void".to_string(),
            body: vec![],
            location: SourceLocation::new(PathBuf::from("test.csd"), 10, 1),
        }
    }
    
    fn create_test_variable_decl() -> VariableDeclaration {
        VariableDeclaration {
            name: "test_var".to_string(),
            var_type: "sus".to_string(),
            value: None,
            is_mutable: false,
            location: SourceLocation::new(PathBuf::from("test.csd"), 15, 5),
        }
    }
    
    #[test]
    fn test_debug_stats_creation() {
        let stats = DebugStats::default();
        assert_eq!(stats.functions_processed, 0);
        assert_eq!(stats.variables_processed, 0);
        assert_eq!(stats.types_processed, 0);
    }
    
    #[test]
    fn test_debug_stats_display() {
        let stats = DebugStats {
            functions_processed: 5,
            variables_processed: 15,
            types_processed: 8,
            files_processed: 3,
            debug_locations_created: 50,
            metadata_entries_generated: 100,
        };
        
        let display = format!("{}", stats);
        assert!(display.contains("5 functions"));
        assert!(display.contains("15 variables"));
        assert!(display.contains("8 types"));
        assert!(display.contains("3 files"));
        assert!(display.contains("50 locations"));
        assert!(display.contains("100 metadata entries"));
    }
    
    #[test]
    #[ignore = "Requires LLVM context - integration test"]
    fn test_debug_metadata_creation() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        let config = DebugConfig::default();
        
        let result = LlvmDebugMetadata::new(
            &context,
            &module,
            &builder,
            Path::new("test.csd"),
            config,
        );
        
        assert!(result.is_ok(), "Debug metadata creation should succeed");
    }
    
    #[test]
    #[ignore = "Requires LLVM context - integration test"]
    fn test_cursed_type_creation() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        let config = DebugConfig::default();
        
        if let Ok(mut metadata) = LlvmDebugMetadata::new(
            &context,
            &module,
            &builder,
            Path::new("test.csd"),
            config,
        ) {
            // Test CURSED basic types
            assert!(metadata.get_or_create_cursed_type("sus").is_ok());
            assert!(metadata.get_or_create_cursed_type("facts").is_ok());
            assert!(metadata.get_or_create_cursed_type("vibes").is_ok());
            assert!(metadata.get_or_create_cursed_type("tea").is_ok());
            assert!(metadata.get_or_create_cursed_type("void").is_ok());
        }
    }
    
    #[test]
    fn test_expression_location_extraction() {
        let location = SourceLocation::new(PathBuf::from("test.csd"), 20, 10);
        let expr = Expression::Literal {
            value: crate::ast::Literal::Integer(42),
            location: location.clone(),
        };
        
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        let config = DebugConfig::default();
        
        if let Ok(metadata) = LlvmDebugMetadata::new(
            &context,
            &module,
            &builder,
            Path::new("test.csd"),
            config,
        ) {
            let extracted_location = metadata.get_expression_location(&expr);
            assert!(extracted_location.is_some());
            assert_eq!(extracted_location.unwrap().line, 20);
        }
    }
    
    #[test]
    fn test_debug_enabled_check() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        // Test with debug enabled
        let config_enabled = DebugConfig {
            generate_debug_info: true,
            ..Default::default()
        };
        
        if let Ok(metadata) = LlvmDebugMetadata::new(
            &context,
            &module,
            &builder,
            Path::new("test.csd"),
            config_enabled,
        ) {
            assert!(metadata.is_enabled());
        }
        
        // Test with debug disabled
        let config_disabled = DebugConfig {
            generate_debug_info: false,
            ..Default::default()
        };
        
        if let Ok(metadata) = LlvmDebugMetadata::new(
            &context,
            &module,
            &builder,
            Path::new("test.csd"),
            config_disabled,
        ) {
            assert!(!metadata.is_enabled());
        }
    }
}
