//! LLVM module linking and symbol resolution for CURSED packages
//!
//! This module handles the linking of separately compiled LLVM modules,
//! resolving imports and exports between packages, and managing symbol
//! visibility and dependencies.

use crate::codegen::llvm::separate_compilation::PackageMetadata;
use crate::error::Error;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::values::{FunctionValue, GlobalValue, BasicValueEnum, InstructionOpcode};
use std::collections::{HashMap, HashSet};
use tracing::{debug, error, info, instrument, warn};

/// Symbol information for linking
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name
    pub name: String,
    /// Package that defines this symbol
    pub defining_package: String,
    /// Whether this symbol is exported
    pub is_exported: bool,
    /// Whether this symbol is imported
    pub is_imported: bool,
    /// Symbol type (function, global, etc.)
    pub symbol_type: SymbolType,
}

/// Types of symbols that can be linked
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Global,
    Type,
}

/// Manages symbol resolution and module linking
pub struct ModuleLinker<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// Symbol table for all packages
    symbol_table: HashMap<String, SymbolInfo>,
    /// Package metadata
    packages: HashMap<String, PackageMetadata>,
    /// Import/export relationships
    dependencies: HashMap<String, HashSet<String>>,
}

impl<'ctx> ModuleLinker<'ctx> {
    /// Create a new module linker
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            symbol_table: HashMap::new(),
            packages: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Add package metadata for linking
    #[instrument(skip(self), level = "debug")]
    pub fn add_package(&mut self, metadata: PackageMetadata) -> Result<(), Error> {
        debug!(package = metadata.name, "Adding package to linker");

        // Register package dependencies
        if !metadata.dependencies.is_empty() {
            self.dependencies.insert(metadata.name.clone(), metadata.dependencies.iter().cloned().collect());
        }

        // Register exported symbols
        for export in &metadata.exports {
            let symbol_info = SymbolInfo {
                name: export.clone(),
                defining_package: metadata.name.clone(),
                is_exported: true,
                is_imported: false,
                symbol_type: SymbolType::Function, // For now, assume all exports are functions
            };
            self.symbol_table.insert(export.clone(), symbol_info);
        }

        self.packages.insert(metadata.name.clone(), metadata);
        Ok(())
    }

    /// Resolve all imports and exports
    #[instrument(skip(self), level = "info")]
    pub fn resolve_symbols(&mut self) -> Result<(), Error> {
        info!("Resolving symbols across all packages");

        // Check that all imported symbols are available
        for (package_name, deps) in &self.dependencies.clone() {
            for dep_name in deps {
                if !self.packages.contains_key(dep_name) {
                    return Err(Error::from_str(&format!(
                        "Package '{}' imports '{}' but '{}' is not available",
                        package_name, dep_name, dep_name
                    )));
                }

                // Mark symbols as imported
                if let Some(dep_metadata) = self.packages.get(dep_name) {
                    for export in &dep_metadata.exports {
                        if let Some(symbol) = self.symbol_table.get_mut(export) {
                            symbol.is_imported = true;
                        }
                    }
                }
            }
        }

        info!(symbols_resolved = self.symbol_table.len(), "Symbol resolution completed");
        Ok(())
    }

    /// Link multiple modules together
    #[instrument(skip(self, modules), fields(module_count = modules.len()), level = "info")]
    pub fn link_modules(&self, modules: Vec<Module<'ctx>>) -> Result<Module<'ctx>, Error> {
        info!("Linking {} modules", modules.len());

        if modules.is_empty() {
            return Err(Error::from_str("No modules to link"));
        }

        // Create the main linked module
        let linked_module = self.context.create_module("linked_program");

        // Link each module
        for (i, module) in modules.iter().enumerate() {
            debug!(module_index = i, module_name = module.get_name().to_string_lossy().as_ref(), "Processing module for linking");
            
            self.process_module_for_linking(&linked_module, module)?;
        }

        // Resolve cross-module references
        self.resolve_cross_module_references(&linked_module)?;

        // Set appropriate linkages
        self.set_symbol_linkages(&linked_module)?;

        info!("Module linking completed successfully");
        Ok(linked_module)
    }

    /// Process a single module for linking
    #[instrument(skip(self, target_module, source_module), level = "debug")]
    fn process_module_for_linking(&self, target_module: &Module<'ctx>, source_module: &Module<'ctx>) -> Result<(), Error> {
        let source_name = source_module.get_name().to_string_lossy();
        debug!(source = source_name.as_ref(), "Processing module for linking");

        // Get package name from module metadata
        let package_name = self.extract_package_name_from_module(source_module)?;

        // Copy functions
        self.copy_functions(target_module, source_module, &package_name)?;

        // Copy global variables
        self.copy_globals(target_module, source_module, &package_name)?;

        // Copy type information (simplified)
        self.copy_types(target_module, source_module)?;

        debug!(package = package_name, "Module processing completed");
        Ok(())
    }

    /// Copy functions from source to target module
    #[instrument(skip(self, target_module, source_module), level = "debug")]
    fn copy_functions(&self, target_module: &Module<'ctx>, source_module: &Module<'ctx>, package_name: &str) -> Result<(), Error> {
        let mut functions_copied = 0;

        // Iterate through all functions in source module
        let mut current_function = source_module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            // Determine if this function should be copied
            if self.should_copy_function(&func_name, package_name) {
                self.copy_function_to_module(target_module, &function, package_name)?;
                functions_copied += 1;
            }

            current_function = function.get_next_function();
        }

        debug!(package = package_name, functions_copied, "Functions copied");
        Ok(())
    }

    /// Copy a single function to target module
    #[instrument(skip(self, target_module, function), level = "trace")]
    fn copy_function_to_module(&self, target_module: &Module<'ctx>, function: &FunctionValue<'ctx>, package_name: &str) -> Result<(), Error> {
        let func_name = function.get_name().to_string_lossy();
        
        // Create mangled name for the function
        let mangled_name = self.mangle_function_name(&func_name, package_name);

        // Check if function already exists in target
        if target_module.get_function(&mangled_name).is_some() {
            debug!(function = func_name.as_ref(), "Function already exists in target module");
            return Ok(());
        }

        // Get function type
        let func_type = function.get_type();

        // Add function declaration to target module
        let new_function = target_module.add_function(&mangled_name, func_type, None);

        // Copy function attributes
        self.copy_function_attributes(function, &new_function);

        // Copy function body if it exists
        if function.count_basic_blocks() > 0 {
            self.copy_function_body(function, &new_function, target_module)?;
        }

        debug!(function = func_name.as_ref(), mangled_name, "Function copied successfully");
        Ok(())
    }

    /// Copy function attributes
    #[instrument(skip(self, source_function, target_function), level = "trace")]
    fn copy_function_attributes(&self, source_function: &FunctionValue<'ctx>, target_function: &FunctionValue<'ctx>) {
        // Copy linkage
        target_function.set_linkage(source_function.get_linkage());
        
        // Copy calling convention
        target_function.set_call_conventions(source_function.get_call_conventions());
        
        // Note: visibility methods are not available in this LLVM version
        // target_function.set_visibility(source_function.get_visibility());

        debug!("Function attributes copied");
    }

    /// Copy function body from source to target
    #[instrument(skip(self, source_function, target_function, target_module), level = "trace")]
    fn copy_function_body(
        &self,
        source_function: &FunctionValue<'ctx>,
        target_function: &FunctionValue<'ctx>,
        target_module: &Module<'ctx>,
    ) -> Result<(), Error> {
        use inkwell::values::BasicValueEnum;
        use std::collections::HashMap;

        // Map to track cloned values
        let mut value_map: HashMap<String, BasicValueEnum<'ctx>> = HashMap::new();

        // Copy parameters
        for (i, param) in source_function.get_params().iter().enumerate() {
            let target_param = target_function.get_nth_param(i as u32).unwrap();
            // Note: BasicValueEnum doesn't have get_name method, so we use index-based naming
            target_param.set_name(&format!("param_{}", i));
            value_map.insert(
                format!("param_{}", i),
                target_param
            );
        }

        // Copy basic blocks
        let mut block_map: HashMap<String, inkwell::basic_block::BasicBlock<'ctx>> = HashMap::new();
        
        // First pass: create all basic blocks
        let mut current_block = source_function.get_first_basic_block();
        while let Some(block) = current_block {
            let block_name = block.get_name().to_string_lossy();
            let new_block = self.context.append_basic_block(*target_function, &block_name);
            block_map.insert(block_name.to_string(), new_block);
            current_block = block.get_next_basic_block();
        }

        // Second pass: copy instructions
        current_block = source_function.get_first_basic_block();
        let builder = self.context.create_builder();
        
        while let Some(block) = current_block {
            let block_name = block.get_name().to_string_lossy();
            let target_block = block_map.get(&block_name.to_string()).unwrap();
            builder.position_at_end(*target_block);

            // Copy instructions in this block
            self.copy_block_instructions(&block, &builder, &mut value_map, &block_map, target_module)?;

            current_block = block.get_next_basic_block();
        }

        debug!("Function body copied successfully");
        Ok(())
    }

    /// Copy instructions from a basic block
    #[instrument(skip(self, source_block, builder, value_map, block_map, target_module), level = "trace")]
    fn copy_block_instructions(
        &self,
        source_block: &inkwell::basic_block::BasicBlock<'ctx>,
        builder: &inkwell::builder::Builder<'ctx>,
        value_map: &mut HashMap<String, BasicValueEnum<'ctx>>,
        block_map: &HashMap<String, inkwell::basic_block::BasicBlock<'ctx>>,
        target_module: &Module<'ctx>,
    ) -> Result<(), Error> {
        use inkwell::values::InstructionValue;

        // Get first instruction
        let mut current_instruction = source_block.get_first_instruction();
        
        while let Some(instruction) = current_instruction {
            self.copy_single_instruction(&instruction, builder, value_map, block_map, target_module)?;
            current_instruction = instruction.get_next_instruction();
        }

        debug!("Block instructions copied");
        Ok(())
    }

    /// Copy a single instruction
    #[instrument(skip(self, instruction, builder, value_map, block_map, target_module), level = "trace")]
    fn copy_single_instruction(
        &self,
        instruction: &inkwell::values::InstructionValue<'ctx>,
        builder: &inkwell::builder::Builder<'ctx>,
        value_map: &mut HashMap<String, BasicValueEnum<'ctx>>,
        block_map: &HashMap<String, inkwell::basic_block::BasicBlock<'ctx>>,
        target_module: &Module<'ctx>,
    ) -> Result<(), Error> {
        use inkwell::values::{BasicValueEnum, InstructionOpcode};

        let opcode = instruction.get_opcode();
        
        match opcode {
            InstructionOpcode::Return => {
                // For now, just build a void return for simplicity
                // In a full implementation, we'd handle return values properly
                builder.build_return(None)
                    .map_err(|e| Error::from_str(&format!("Failed to build void return: {}", e)))?;
            }
            InstructionOpcode::Alloca => {
                // Handle alloca instruction
                if let Ok(allocated_type) = instruction.get_allocated_type() {
                    let alloca = builder.build_alloca(allocated_type, "")
                        .map_err(|e| Error::from_str(&format!("Failed to build alloca: {}", e)))?;
                    
                    if let Some(inst_name_cstr) = instruction.get_name() {
                        let inst_name = inst_name_cstr.to_string_lossy();
                        if !inst_name.is_empty() {
                            alloca.set_name(&inst_name);
                            value_map.insert(inst_name.to_string(), alloca.into());
                        }
                    }
                }
            }
            InstructionOpcode::Store => {
                // Skip store instruction copying for simplicity
                debug!("Store instruction skipped in function copying");
            }
            InstructionOpcode::Load => {
                // Skip load instruction copying for simplicity
                debug!("Load instruction skipped in function copying");
            }
            InstructionOpcode::Add => {
                // Skip add instruction copying for simplicity
                debug!("Add instruction skipped in function copying");
            }
            _ => {
                // For unhandled instructions, skip with warning
                warn!(opcode = ?opcode, "Instruction type not yet supported in function copying");
            }
        }

        Ok(())
    }

    /// Simplified instruction copying - placeholder for future implementation
    #[allow(dead_code)]
    fn map_operand(
        &self,
        operand: inkwell::values::BasicValueEnum<'ctx>,
        _value_map: &HashMap<String, BasicValueEnum<'ctx>>,
        _target_module: &Module<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        use inkwell::values::BasicValueEnum;

        match operand {
            // Constants can be used directly
            BasicValueEnum::IntValue(int_val) if int_val.is_const() => Some(operand),
            BasicValueEnum::FloatValue(float_val) if float_val.is_const() => Some(operand),
            
            // For complex value mapping, return None for now
            _ => None,
        }
    }

    /// Copy global variables from source to target module
    #[instrument(skip(self, target_module, source_module), level = "debug")]
    fn copy_globals(&self, target_module: &Module<'ctx>, source_module: &Module<'ctx>, package_name: &str) -> Result<(), Error> {
        let mut globals_copied = 0;

        // Iterate through all globals in source module
        let mut current_global = source_module.get_first_global();
        while let Some(global) = current_global {
            let global_name = global.get_name().to_string_lossy();
            
            if self.should_copy_global(&global_name, package_name) {
                self.copy_global_to_module(target_module, &global, package_name)?;
                globals_copied += 1;
            }

            current_global = global.get_next_global();
        }

        debug!(package = package_name, globals_copied, "Global variables copied");
        Ok(())
    }

    /// Copy a single global variable to target module
    #[instrument(skip(self, target_module, global), level = "trace")]
    fn copy_global_to_module(&self, target_module: &Module<'ctx>, global: &GlobalValue<'ctx>, package_name: &str) -> Result<(), Error> {
        let global_name = global.get_name().to_string_lossy();
        let mangled_name = self.mangle_global_name(&global_name, package_name);

        // Check if global already exists
        if target_module.get_global(&mangled_name).is_some() {
            debug!(global = global_name.as_ref(), "Global already exists in target module");
            return Ok(());
        }

        // Get global type (simplified approach)
        let i8_type = self.context.i8_type();
        let global_type = i8_type;

        // Add global to target module
        let new_global = target_module.add_global(global_type, None, &mangled_name);

        // Copy initializer if present
        if let Some(initializer) = global.get_initializer() {
            new_global.set_initializer(&initializer);
        }

        debug!(global = global_name.as_ref(), mangled_name, "Global variable added");
        Ok(())
    }

    /// Copy type definitions (simplified implementation)
    #[instrument(skip(self, target_module, source_module), level = "debug")]
    fn copy_types(&self, target_module: &Module<'ctx>, source_module: &Module<'ctx>) -> Result<(), Error> {
        // Type copying is complex and depends on the specific type system
        // For now, we'll implement a placeholder
        debug!("Type copying not yet fully implemented");
        Ok(())
    }

    /// Resolve references between modules
    #[instrument(skip(self, module), level = "debug")]
    fn resolve_cross_module_references(&self, module: &Module<'ctx>) -> Result<(), Error> {
        debug!("Resolving cross-module references");

        // This would involve:
        // - Finding unresolved external references
        // - Mapping them to the correct mangled names
        // - Updating call instructions and other references

        // For now, this is a placeholder
        debug!("Cross-module reference resolution not yet fully implemented");
        Ok(())
    }

    /// Set appropriate linkages for symbols
    #[instrument(skip(self, module), level = "debug")]
    fn set_symbol_linkages(&self, module: &Module<'ctx>) -> Result<(), Error> {
        debug!("Setting symbol linkages");

        // Iterate through functions and set linkage
        let mut current_function = module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            if self.is_exported_symbol(&func_name) {
                function.set_linkage(Linkage::External);
            } else {
                function.set_linkage(Linkage::Internal);
            }

            current_function = function.get_next_function();
        }

        // Iterate through globals and set linkage
        let mut current_global = module.get_first_global();
        while let Some(global) = current_global {
            let global_name = global.get_name().to_string_lossy();
            
            if self.is_exported_symbol(&global_name) {
                global.set_linkage(Linkage::External);
            } else {
                global.set_linkage(Linkage::Internal);
            }

            current_global = global.get_next_global();
        }

        debug!("Symbol linkages set");
        Ok(())
    }

    /// Extract package name from module metadata
    fn extract_package_name_from_module(&self, module: &Module<'ctx>) -> Result<String, Error> {
        // Try to extract package name from module metadata
        // For now, we'll use the module name as a fallback
        let module_name = module.get_name().to_string_lossy();
        
        if let Some(package_name) = module_name.strip_prefix("module_") {
            Ok(package_name.to_string())
        } else {
            Ok("main".to_string())
        }
    }

    /// Check if a function should be copied during linking
    fn should_copy_function(&self, func_name: &str, package_name: &str) -> bool {
        // Skip LLVM intrinsics and other special functions
        if func_name.starts_with("llvm.") {
            return false;
        }

        // For now, copy all user-defined functions
        true
    }

    /// Check if a global should be copied during linking
    fn should_copy_global(&self, global_name: &str, package_name: &str) -> bool {
        // Skip LLVM-generated globals
        if global_name.starts_with("llvm.") {
            return false;
        }

        true
    }

    /// Mangle function name for package-local scope
    fn mangle_function_name(&self, func_name: &str, package_name: &str) -> String {
        if func_name == "main" && package_name == "main" {
            // Keep main function unmangled for the main package
            "main".to_string()
        } else {
            format!("_{}_{}", package_name, func_name)
        }
    }

    /// Mangle global variable name for package-local scope
    fn mangle_global_name(&self, global_name: &str, package_name: &str) -> String {
        format!("_{}_{}", package_name, global_name)
    }

    /// Check if a symbol is exported
    fn is_exported_symbol(&self, symbol_name: &str) -> bool {
        self.symbol_table.get(symbol_name)
            .map(|info| info.is_exported)
            .unwrap_or(false)
    }

    /// Get symbol information
    pub fn get_symbol_info(&self, symbol_name: &str) -> Option<&SymbolInfo> {
        self.symbol_table.get(symbol_name)
    }

    /// Get all symbols
    pub fn get_all_symbols(&self) -> &HashMap<String, SymbolInfo> {
        &self.symbol_table
    }

    /// Get package dependencies
    pub fn get_dependencies(&self, package_name: &str) -> Option<&HashSet<String>> {
        self.dependencies.get(package_name)
    }
}

/// Convenience function to link a set of modules with metadata
#[instrument(skip(context, modules, metadata_list), fields(module_count = modules.len()), level = "info")]
pub fn link_modules_with_metadata<'ctx>(
    context: &'ctx Context,
    modules: Vec<Module<'ctx>>,
    metadata_list: Vec<PackageMetadata>,
) -> Result<Module<'ctx>, Error> {
    info!("Linking modules with metadata");

    let mut linker = ModuleLinker::new(context);

    // Add all package metadata
    for metadata in metadata_list {
        linker.add_package(metadata)?;
    }

    // Resolve symbols
    linker.resolve_symbols()?;

    // Link modules
    linker.link_modules(modules)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::separate_compilation::PackageMetadata;
    use inkwell::context::Context;
    use std::path::PathBuf;

    #[test]
    fn test_symbol_resolution() {
        let context = Context::create();
        let mut linker = ModuleLinker::new(&context);

        // Add package metadata
        let pkg_a = PackageMetadata {
            name: "a".to_string(),
            source_path: PathBuf::from("a.csd"),
            dependencies: vec!["b".to_string()],
            exports: vec!["func_a".to_string()],
            module_name: "module_a".to_string(),
        };

        let pkg_b = PackageMetadata {
            name: "b".to_string(),
            source_path: PathBuf::from("b.csd"),
            dependencies: vec![],
            exports: vec!["func_b".to_string()],
            module_name: "module_b".to_string(),
        };

        linker.add_package(pkg_a).unwrap();
        linker.add_package(pkg_b).unwrap();
        linker.resolve_symbols().unwrap();

        // Check symbol table
        assert!(linker.get_symbol_info("func_a").is_some());
        assert!(linker.get_symbol_info("func_b").is_some());

        // Check dependencies
        let deps = linker.get_dependencies("a").unwrap();
        assert!(deps.contains("b"));
    }

    #[test]
    fn test_function_name_mangling() {
        let context = Context::create();
        let linker = ModuleLinker::new(&context);

        assert_eq!(linker.mangle_function_name("main", "main"), "main");
        assert_eq!(linker.mangle_function_name("helper", "utils"), "_utils_helper");
        assert_eq!(linker.mangle_function_name("func", "pkg"), "_pkg_func");
    }
}
